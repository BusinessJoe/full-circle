use std::collections::HashMap;
use std::sync::Arc;

use futures_util::{SinkExt, StreamExt, TryFutureExt, stream::SplitStream};
use tokio::sync::{mpsc, RwLock};
use tokio_stream::wrappers::UnboundedReceiverStream;
use warp::ws::{Message, WebSocket};
use warp::Filter;
use rand::{Rng, distributions::Alphanumeric};

use shape_evolution::random_shape::{RandomCircle};

pub struct Player {
    player_id: String,
    is_host: bool,
    sender: mpsc::UnboundedSender<Message>,
}
impl Player {
    pub fn new(is_host: bool, sender: mpsc::UnboundedSender<Message>) -> Self {
        let player_id = uuid::Uuid::new_v4().to_string();
        Player {player_id, is_host, sender}
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
        }
        );

    println!("New room: {}", room_id);

    // Split the socket into a sender and receiver of messages.
    let (client_ws_sender, mut client_ws_rcv) = ws.split();
    let (client_sender, client_rcv) = mpsc::unbounded_channel();
    let mut client_rcv = UnboundedReceiverStream::new(client_rcv);

    tokio::task::spawn(client_rcv.map(Ok).forward(client_ws_sender));
    //tokio::task::spawn(client_rcv.forward(client_ws_sender).map(|result| {
    //    if let Err(e) = result {
    //        eprintln!("error sending websocket msg: {}", e);
    //    }
    //}));

    let rooms_read = rooms.read().await;
    let room = rooms_read.get(&room_id).unwrap();
    let new_player = Player::new(true, client_sender);

    connect_player(new_player, &room, client_ws_rcv).await;
}

async fn player_connected(ws: WebSocket, rooms: Rooms, room_id: String) {
    // First check that the provided room id matches an existing room.
    if let Some(room) = rooms.read().await.get(&room_id) {
        // Split the socket into a sender and receiver of messages.
        let (mut client_ws_sender, mut client_ws_rcv) = ws.split();
        let (client_sender, client_rcv) = mpsc::unbounded_channel();
        let mut client_rcv = UnboundedReceiverStream::new(client_rcv);

        tokio::task::spawn(async move {
            while let Some(message) = client_rcv.next().await {
                client_ws_sender
                    .send(message)
                    .unwrap_or_else(|e| {
                        eprintln!("websocket send error: {}", e);
                    })
                .await;
            }
        });

        let player = Player::new(false, client_sender);

        connect_player(player, &room, client_ws_rcv).await;
    } else {
        println!("Rejecting connection to: {}", room_id);
        ws.close().await;
    }
}

async fn connect_player(player: Player, room: &Room, mut client_ws_rcv: SplitStream<WebSocket>) {
    let player_id = player.player_id.clone();
    {
        let mut players = room.players.write().await;
        players.push(player);
    }

    println!("Player {} joined room {}", &player_id, &room.room_id);

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
        user_message(&player_id, msg, &room).await;
    }

    // user_ws_rx stream will keep processing as long as the user stays
    // connected. Once they disconnect, then...
    user_disconnected(&player_id, &room).await;
}

async fn user_message(my_id: &str, msg: Message, room: &Room) {
    // Skip any non-Text messages...
    let msg = if let Ok(s) = msg.to_str() {
        s
    } else {
        return;
    };

    let new_msg = match serde_json::from_str::<RandomCircle>(msg) {
        Ok(circle) => serde_json::to_string(&circle).unwrap(),
        Err(e) => {
            println!("{:?}", e);
            String::from("Error parsing circle")
        }
    };

    let players = &room.players;

    // New message from this user, send it to everyone...
    for p in players.read().await.iter() {
        if let Err(_disconnected) = p.sender.send(Message::text(&new_msg)) {
            // The tx is disconnected, our `user_disconnected` code
            // should be happening in another task, nothing more to
            // do here.
        }
    }
}

async fn user_disconnected(my_id: &str, room: &Room) {
    eprintln!("good bye player: {}", my_id);

    // Stream closed up, so remove from the user list.
    // Acquire write lock. The lock will be dropped on function end.
    let mut players = room.players.write().await;

    let mut i = 0;
    while i < players.len() {
        if &mut players[i].player_id == my_id {
            players.remove(i);
        } else {
            i += 1;
        }
    }
}

