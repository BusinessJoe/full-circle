use std::collections::HashMap;
use std::sync::Arc;

use futures::future::{AbortHandle, Abortable};
use futures_util::{stream::SplitStream, StreamExt};
use rand::{distributions::Alphanumeric, Rng};
use serde::{Deserialize, Serialize};
use shape_evolution::random_shape::RandomCircle;
use tokio::sync::{mpsc, RwLock};
use tokio::time::Duration;
use tokio_stream::wrappers::UnboundedReceiverStream;
use warp::ws::{Message, WebSocket};
use warp::{http::Response, Filter};

mod res;
use res::WsEvent;

#[derive(Clone, Serialize, Deserialize)]
pub struct PlayerInfo {
    id: String,
    is_host: bool,
}
pub struct Player {
    info: PlayerInfo,
    sender: mpsc::UnboundedSender<Message>,
}
impl Player {
    pub fn new(is_host: bool, sender: mpsc::UnboundedSender<Message>) -> Self {
        let id = uuid::Uuid::new_v4().to_string();
        Player {
            info: PlayerInfo { id, is_host },
            sender,
        }
    }
}

type Players = Vec<Player>;

pub struct Room {
    id: String,
    players: Players,
    image_dimensions: Option<(u32, u32)>,
    circles: Vec<RandomCircle>,
    cleanup_abort_handle: Option<AbortHandle>,
}
impl Room {
    pub fn new(id: String) -> Self {
        Room {
            id,
            players: Players::default(),
            image_dimensions: None,
            circles: Vec::new(),
            cleanup_abort_handle: None,
        }
    }

    pub fn cancel_cleanup(&mut self) {
        if let Some(abort_handle) = &self.cleanup_abort_handle {
            println!("Cancelling cleanup for room {}", &self.id);
            abort_handle.abort();
            self.cleanup_abort_handle = None;
        }
    }
}

type Rooms = Arc<RwLock<HashMap<String, Arc<RwLock<Room>>>>>;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    let rooms = Rooms::default();

    let rooms_clone = rooms.clone();
    let rooms_filter = warp::any().map(move || rooms_clone.clone());

    let cors = warp::cors().allow_any_origin();

    // GET /room -> creates a room and returns a path for joining it
    let room = warp::path("room")
        .and(rooms_filter)
        .and_then(|rooms| async move { new_room(rooms).await })
        .with(cors);

    let rooms_clone = rooms.clone();
    let rooms_filter = warp::any().map(move || rooms_clone.clone());

    // GET /join -> websocket upgrade
    let join = warp::path!("join" / String)
        // The `ws()` filter will prepare Websocket handshake...
        .and(warp::ws())
        .and(rooms_filter)
        .map(|room_id: String, ws: warp::ws::Ws, rooms| {
            // This will call our function if the handshake succeeds.
            ws.on_upgrade(move |socket| player_connected(socket, rooms, room_id))
        });

    let routes = room.or(join);

    warp::serve(routes).run(([0, 0, 0, 0], 3001)).await;
}

async fn new_room(rooms: Rooms) -> Result<Response<String>, warp::Rejection> {
    // Generate a 7 character alphanumeric id for the room
    let room_id: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(7)
        .map(char::from)
        .collect();

    let mut new_room = Room::new(room_id.clone());

    let (abort_handle, abort_registration) = AbortHandle::new_pair();
    new_room.cleanup_abort_handle = Some(abort_handle);

    // Wrap room in a reference counted lock
    let new_room = Arc::new(RwLock::new(new_room));

    // Start an abortable cleanup task for this room.
    {
        let rooms = rooms.clone();
        let room = new_room.clone();
        let future = Abortable::new(
            async move {
                tokio::time::sleep(Duration::new(5, 0)).await;
                let room = room.read().await;
                delete_room(&rooms, &room).await;
            },
            abort_registration,
        );
        tokio::task::spawn(future);
    }

    rooms.write().await.insert(room_id.clone(), new_room);
    eprintln!("New room: {}", &room_id);

    Ok(Response::builder()
        .body(format!("/join/{}/", &room_id))
        .unwrap())
}

async fn player_connected(ws: WebSocket, rooms: Rooms, room_id: String) {
    // First check that the provided room id matches an existing room.
    let room = {
        if let Some(room) = rooms.read().await.get(&room_id) {
            room.clone()
        } else {
            // The room doesn't exist, notify the browser somehow
            todo!();
        }
    };

    // Split the socket into a sender and receiver of messages.
    let (client_ws_sender, client_ws_rcv) = ws.split();
    let (client_sender, client_rcv) = mpsc::unbounded_channel();
    let client_rcv = UnboundedReceiverStream::new(client_rcv);

    tokio::task::spawn(client_rcv.map(Ok).forward(client_ws_sender));

    // Send room's current image dimensions (if they exist) to the new player
    if let Some(dimensions) = room.read().await.image_dimensions {
        client_sender
            .send(Message::text(
                serde_json::to_string(&res::WsEvent::NewImage { dimensions }).unwrap(),
            ))
            .expect("Error sending new-image");
    }

    let is_host = {
        let players = &room.read().await.players;
        players.is_empty()
    };

    let player = Player::new(is_host, client_sender);

    connect_player(player, rooms, room, client_ws_rcv).await;
}

async fn connect_player(
    player: Player,
    rooms: Rooms,
    room: Arc<RwLock<Room>>,
    mut client_ws_rcv: SplitStream<WebSocket>,
) {
    let player_id = player.info.id.clone();
    {
        let mut room = room.write().await;
        // Cancel the room's cleanup process
        room.cancel_cleanup();

        // Add the player to the room
        room.players.push(player);

        let player = room
            .players
            .last()
            .expect("There should always be a player after the push");

        println!("Player {} joined room {}", player.info.id, &room.id);

        // Tell the player what their id is
        send_player_id(player);
        send_current_circles(player, &room);
        broadcast_player_list(&room);
    }

    // Every time the host sends a message, broadcast it to
    // all other users
    while let Some(result) = client_ws_rcv.next().await {
        let msg = match result {
            Ok(msg) => msg,
            Err(e) => {
                eprintln!("websocket error(uid={}): {}", player_id, e);
                break;
            }
        };
        handle_user_message(&player_id, msg, room.clone()).await;
    }

    // user_ws_rx stream will keep processing as long as the user stays
    // connected. Once they disconnect, then...
    user_disconnected(&player_id, &rooms, room).await;
}

async fn handle_user_message(player_id: &str, msg: Message, room: Arc<RwLock<Room>>) {
    // Skip any non-Text messages
    let msg = if let Ok(s) = msg.to_str() {
        s
    } else {
        return;
    };

    // Only handle host's messages
    {
        let room = room.read().await;
        if let Some(player) = room.players.iter().find(|&p| p.info.id == player_id) {
            if !player.info.is_host {
                eprintln!("Rejecting message from player {}", player_id);
                return;
            }
        }
    }

    match serde_json::from_str::<WsEvent>(msg) {
        Ok(event) => match event {
            WsEvent::Circle(ref circle) => {
                let mut room = room.write().await;
                room.circles.push(circle.clone());
                // Let everyone know there's a new circle
                broadcast_ws_event(event, &room);
            }
            WsEvent::NewImage { dimensions } => {
                {
                    let mut room = room.write().await;
                    room.image_dimensions = Some(dimensions);
                }
                // Let everyone know there's a new image
                let room = room.read().await;
                broadcast_ws_event(event, &room);
            }
            _ => {
                eprintln!("Unsupported message type");
            }
        },
        Err(e) => {
            eprintln!("Failed to deserialize message {:?}", e);
        }
    }
}

async fn delete_room(rooms: &Rooms, room: &Room) {
    {
        let mut rooms = rooms.write().await;
        rooms.remove(&room.id);
    }
    eprintln!("Deleted room {}", &room.id);
}

async fn user_disconnected(my_id: &str, rooms: &Rooms, room: Arc<RwLock<Room>>) {
    {
        let mut room = room.write().await;

        eprintln!("good bye player: {}", my_id);
        // Stream closed up, so remove from the user list.
        // Acquire write lock. The lock will be dropped on function end.
        let players = &mut room.players;
        let mut removed_player = None;
        let mut i = 0;
        while i < players.len() {
            if players[i].info.id == my_id {
                removed_player = Some(players.remove(i));
                break;
            } else {
                i += 1;
            }
        }

        let removed_player = removed_player.expect("A player should always be removed");

        // We might need to assign a new host
        if !players.is_empty() && removed_player.info.is_host {
            // Wrap index around to 0 if the host was the last player in vector
            if i == players.len() {
                i = 0;
            }
            players[i].info.is_host = true;
        }
    }

    let room = room.read().await;
    if room.players.is_empty() {
        // Delete the room if it has no more players
        delete_room(rooms, &room).await;
    } else {
        // otherwise let everyone know who is still connected
        broadcast_player_list(&room);
    }
}

// Send a WsEvent to a single player
fn send_ws_event(event: WsEvent, player: &Player) {
    let message = serde_json::to_string(&event).expect("failed to serialize event");
    player.sender.send(Message::text(&message));
}

fn send_current_circles(player: &Player, room: &Room) {
    // No need to send an empty sequence
    if room.circles.is_empty() {
        return;
    }

    let event = WsEvent::CircleSequence(room.circles.clone());
    send_ws_event(event, player);
}

fn send_player_id(player: &Player) {
    let event = WsEvent::PlayerId(player.info.id.clone());
    send_ws_event(event, player);
}

// Broadcast a WsEvent to every player in a room
fn broadcast_ws_event(event: WsEvent, room: &Room) {
    let message = serde_json::to_string(&event).unwrap();
    for p in room.players.iter() {
        if let Err(_disconnected) = p.sender.send(Message::text(&message)) {
            // The tx is disconnected, our `user_disconnected` code
            // should be happening in another task, nothing more to
            // do here.
        }
    }
}

fn broadcast_player_list(room: &Room) {
    let player_list = WsEvent::PlayerList(room.players.iter().map(|p| p.info.clone()).collect());

    broadcast_ws_event(player_list, room);
}
