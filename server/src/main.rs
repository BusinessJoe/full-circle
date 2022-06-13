use std::collections::HashMap;
use std::sync::Arc;

use futures::future::{Abortable, AbortHandle};
use futures_util::{stream::SplitStream, StreamExt};
use rand::{distributions::Alphanumeric, Rng};
use serde::{Deserialize, Serialize};
use tokio::sync::{mpsc, RwLock};
use tokio::time::Duration;
use tokio_stream::wrappers::UnboundedReceiverStream;
use warp::ws::{Message, WebSocket};
use warp::{Filter, http::Response};

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

/// Our state of currently connected users.
///
/// - Key is their id
/// - Value is a `Player` struct, which contains a sender of `warp::ws::Message`
type Players = Arc<RwLock<Vec<Player>>>;

pub struct Room {
    room_id: String,
    players: Players,
    image_dimensions: Option<(u32, u32)>,
    cleanup_abort_handle: Option<AbortHandle>,
}
impl Room {
    pub fn new(room_id: String) -> Self {
        Room {
            room_id,
            players: Players::default(),
            image_dimensions: None,
            cleanup_abort_handle: None,
        }
    }

    pub fn cancel_cleanup(&mut self) {
        if let Some(abort_handle) = &self.cleanup_abort_handle {
            println!("Cancelling cleanup for room {}", &self.room_id);
            abort_handle.abort();
            self.cleanup_abort_handle = None;
        }
    }
}

type Rooms = Arc<RwLock<HashMap<String, Room>>>;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    let rooms = Rooms::default();

    let rooms_clone = rooms.clone();
    let rooms_filter = warp::any().map(move || rooms_clone.clone());

    let cors = warp::cors()
        .allow_any_origin();

    // GET /room -> creates a room and returns a path for joining it
    let room = warp::path("room")
        .and(rooms_filter)
        .and_then(|rooms| async move {
            new_room(rooms).await
        })
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

    // Start an abortable cleanup task for this room.
    {
        let room_id = room_id.clone();
        let rooms = rooms.clone();
        let future = Abortable::new(async move {
            tokio::time::sleep(Duration::new(5, 0)).await;
            delete_room(&rooms, &room_id).await;
        }, abort_registration);
        tokio::task::spawn(future);
    }

    rooms.write().await.insert(
        room_id.clone(),
        new_room,
    );
    println!("New room: {}", &room_id);

    Ok(Response::builder()
        .body(format!("/join/{}/", &room_id))
        .unwrap())
}

async fn player_connected(ws: WebSocket, rooms: Rooms, room_id: String) {
    // First check that the provided room id matches an existing room.
    if let None = rooms.read().await.get(&room_id) {
        unimplemented!();
    }

    // Split the socket into a sender and receiver of messages.
    let (client_ws_sender, client_ws_rcv) = ws.split();
    let (client_sender, client_rcv) = mpsc::unbounded_channel();
    let client_rcv = UnboundedReceiverStream::new(client_rcv);

    tokio::task::spawn(client_rcv.map(Ok).forward(client_ws_sender));

    {
        let rooms = rooms.read().await;
        let room = rooms.get(&room_id).unwrap();
        if let Some(dimensions) = room.image_dimensions {
            client_sender
                .send(Message::text(
                    serde_json::to_string(&res::WsEvent::NewImage { dimensions }).unwrap(),
                ))
                .expect("Error sending new-image");
        }
    }

    let is_host = {
        let rooms = rooms.read().await;
        let room = rooms.get(&room_id).unwrap();
        let players = room.players.read().await;
        
        players.is_empty()
    };

    let player = Player::new(is_host, client_sender);

    connect_player(player, rooms, room_id, client_ws_rcv).await;
}

async fn connect_player(
    player: Player,
    rooms: Rooms,
    room_id: String,
    mut client_ws_rcv: SplitStream<WebSocket>,
) {
    let player_id = player.info.id.clone();
    {
        let mut rooms = rooms.write().await;
        let room = rooms.get_mut(&room_id).unwrap();
        // Cancel the room's cleanup process
        // TODO: Make this work without needing a write lock on rooms
        room.cancel_cleanup();

        // Tell the player what their id is
        send_player_id(&player);

        {
            let mut players = room.players.write().await;
            players.push(player);
        }
        println!("Player {} joined room {}", &player_id, &room.room_id);
        broadcast_player_list(room).await;
    }

    // Every time the user sends a message, broadcast it to
    // all other users...
    while let Some(result) = client_ws_rcv.next().await {
        let msg = match result {
            Ok(msg) => msg,
            Err(e) => {
                eprintln!("websocket error(uid={}): {}", &player_id, e);
                break;
            }
        };
        user_message(&player_id, msg, &rooms, &room_id).await;
    }

    // user_ws_rx stream will keep processing as long as the user stays
    // connected. Once they disconnect, then...
    user_disconnected(&player_id, &rooms, &room_id).await;
}

async fn user_message(my_id: &str, msg: Message, rooms: &Rooms, room_id: &str) {
    // Skip any non-Text messages...
    let msg = if let Ok(s) = msg.to_str() {
        s
    } else {
        return;
    };

    match serde_json::from_str::<WsEvent>(msg) {
        Ok(event) => match event {
            WsEvent::Circle(_) => {
                // Send circle to all players
                let rooms = rooms.read().await;
                let room = rooms.get(room_id).unwrap();

                // Let everyone know there's a new circle
                broadcast_ws_event(event, room).await;
            }
            WsEvent::NewImage { dimensions } => {
                let mut rooms = rooms.write().await;
                let mut room = rooms.get_mut(room_id).unwrap();
                room.image_dimensions = Some(dimensions);

                // Let everyone know there's a new image
                broadcast_ws_event(event, room).await;
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

async fn delete_room(rooms: &Rooms, room_id: &str) {
        let mut rooms = rooms.write().await;
        let room_id = {
            let room = rooms.get(room_id).unwrap();
            room.room_id.clone()
        };
        eprintln!("deleting room {}", &room_id);
        rooms.remove(&room_id);
}

async fn user_disconnected(my_id: &str, rooms: &Rooms, room_id: &str) {
    let should_delete_room = {
        let rooms = rooms.read().await;
        let room = rooms.get(room_id).unwrap();
        eprintln!("good bye player: {}", my_id);

        // Stream closed up, so remove from the user list.
        // Acquire write lock. The lock will be dropped on function end.
        let mut players = room.players.write().await;

        let mut i = 0;
        while i < players.len() {
            if players[i].info.id == my_id {
                players.remove(i);
            } else {
                i += 1;
            }
        }

        players.is_empty()
    };

    if should_delete_room {
        delete_room(rooms, room_id).await;
    } else {
        let rooms = rooms.read().await;
        let room = rooms.get(room_id).unwrap();
        broadcast_player_list(room).await;
    }
}

fn send_player_id(player: &Player) {
    let message = serde_json::to_string(&WsEvent::PlayerId(player.info.id.clone())).unwrap();
    player.sender.send(Message::text(&message));
}

async fn broadcast_ws_event(event: WsEvent, room: &Room) {
    let message = serde_json::to_string(&event).unwrap();
    let players = room.players.read().await;
    for p in players.iter() {
        if let Err(_disconnected) = p.sender.send(Message::text(&message)) {
            // The tx is disconnected, our `user_disconnected` code
            // should be happening in another task, nothing more to
            // do here.
        }
    }
}

async fn broadcast_player_list(room: &Room) {
    let players = room.players.read().await;
    let player_list = WsEvent::PlayerList(players.iter().map(|p| p.info.clone()).collect());

    broadcast_ws_event(player_list, room).await;
}
