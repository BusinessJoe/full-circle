use std::collections::HashMap;
use std::sync::Arc;

use futures_util::{stream::SplitStream, StreamExt};
use rand::{distributions::Alphanumeric, Rng};
use serde::{Deserialize, Serialize};
use tokio::sync::{mpsc, RwLock};
use tokio_stream::wrappers::UnboundedReceiverStream;
use warp::ws::{Message, WebSocket};
use warp::Filter;

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
}

type Rooms = Arc<RwLock<HashMap<String, Room>>>;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    let rooms = Rooms::default();

    let rooms_clone = rooms.clone();
    let rooms_filter = warp::any().map(move || rooms_clone.clone());

    // GET /room -> websocket upgrade
    let room = warp::path("room")
        // The `ws()` filter will prepare Websocket handshake...
        .and(warp::ws())
        .and(rooms_filter)
        .map(|ws: warp::ws::Ws, rooms| {
            // This will call our function if the handshake succeeds.
            ws.on_upgrade(move |socket| host_connected(socket, rooms))
        });

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

    // GET / -> index html
    //let index = warp::path::end().map(|| warp::reply::html(INDEX_HTML));

    // let routes = room.or(join);
    let routes = join.or(room);

    warp::serve(routes).run(([0, 0, 0, 0], 3001)).await;
}

async fn host_connected(ws: WebSocket, rooms: Rooms) {
    let room_id: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(7)
        .map(char::from)
        .collect();
    rooms.write().await.insert(
        room_id.clone(),
        Room {
            room_id: room_id.clone(),
            players: Players::default(),
            image_dimensions: None,
        },
    );

    println!("New room: {}", &room_id);

    // Split the socket into a sender and receiver of messages.
    let (client_ws_sender, client_ws_rcv) = ws.split();
    let (client_sender, client_rcv) = mpsc::unbounded_channel();
    let client_rcv = UnboundedReceiverStream::new(client_rcv);

    tokio::task::spawn(client_rcv.map(Ok).forward(client_ws_sender));

    client_sender
        .send(Message::text(
            serde_json::to_string(&res::WsEvent::RoomPath(format!("/join/{}/", &room_id))).unwrap(),
        ))
        .expect("Error sending room_id");

    let new_player = Player::new(true, client_sender);

    connect_player(new_player, rooms, room_id, client_ws_rcv).await;
}

async fn player_connected(ws: WebSocket, rooms: Rooms, room_id: String) {
    // First check that the provided room id matches an existing room.
    //if let Some(mut room) = rooms.read().await.get(&room_id) {

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

    let player = Player::new(false, client_sender);

    connect_player(player, rooms, room_id, client_ws_rcv).await;

    //} else {
    //    println!("Rejecting connection to: {}", room_id);
    //    ws.close().await.expect("Error rejecting connection");
    //}
}

async fn connect_player(
    player: Player,
    rooms: Rooms,
    room_id: String,
    mut client_ws_rcv: SplitStream<WebSocket>,
) {
    let player_id = player.info.id.clone();
    {
        let rooms = rooms.read().await;
        let room = rooms.get(&room_id).unwrap();
        let mut players = room.players.write().await;
        players.push(player);
        println!("Player {} joined room {}", &player_id, &room.room_id);
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
                let new_message = serde_json::to_string(&event).unwrap();

                // Send circle to all players
                let rooms = rooms.read().await;
                let room = rooms.get(room_id).unwrap();
                let players = &room.players;
                for p in players.read().await.iter() {
                    if let Err(_disconnected) = p.sender.send(Message::text(&new_message)) {
                        // The tx is disconnected, our `user_disconnected` code
                        // should be happening in another task, nothing more to
                        // do here.
                    }
                }
            }
            WsEvent::NewImage { dimensions } => {
                let mut rooms = rooms.write().await;
                let mut room = rooms.get_mut(room_id).unwrap();
                room.image_dimensions = Some(dimensions);
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

async fn user_disconnected(my_id: &str, rooms: &Rooms, room_id: &str) {
    let delete_room = {
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

    if delete_room {
        let mut rooms = rooms.write().await;
        let room_id = {
            let room = rooms.get(room_id).unwrap();
            room.room_id.clone()
        };
        eprintln!("deleting room {}", &room_id);
        rooms.remove(&room_id);
    }
}
