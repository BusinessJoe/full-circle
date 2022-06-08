use std::collections::HashMap;
use std::sync::Arc;

use futures_util::{SinkExt, StreamExt, TryFutureExt};
use tokio::sync::{mpsc, RwLock};
use tokio_stream::wrappers::UnboundedReceiverStream;
use warp::ws::{Message, WebSocket};
use warp::Filter;

use shape_evolution::random_shape::{RandomCircle};

pub struct Player {
    player_id: String,
    is_host: bool,
    sender: mpsc::UnboundedSender<Message>,
}

/// Our state of currently connected users.
///
/// - Key is their id
/// - Value is a `Player` struct, which contains a sender of `warp::ws::Message`
type Players = Arc<RwLock<HashMap<String, Player>>>;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    let host: Option<Player> = None;
    let players = Players::default();

    // Turn our "state" into a new Filter...
    let players_clone = players.clone();
    let players_filter = warp::any().map(move || players_clone.clone());

    // GET /room -> websocket upgrade
    // let room = warp::path("room")
    //     // The `ws()` filter will prepare Websocket handshake...
    //     .and(warp::ws())
    //     .and(players_filter)
    //     .map(|ws: warp::ws::Ws, players| {
    //         // This will call our function if the handshake succeeds.
    //         ws.on_upgrade(move |socket| host_connected(socket, players))
    //     });

    let players_filter = warp::any().map(move || players.clone());

    // GET /join -> websocket upgrade
    let join = warp::path("join")
        // The `ws()` filter will prepare Websocket handshake...
        .and(warp::ws())
        .and(players_filter)
        .map(|ws: warp::ws::Ws, players| {
            // This will call our function if the handshake succeeds.
            ws.on_upgrade(move |socket| player_connected(socket, players))
        });

    // GET / -> index html
    //let index = warp::path::end().map(|| warp::reply::html(INDEX_HTML));

    // let routes = room.or(join);
    let routes = join;

    warp::serve(routes).run(([0, 0, 0, 0], 3001)).await;
}

async fn host_connected(ws: WebSocket, players: Players) {
    let id = uuid::Uuid::new_v4().to_string();

    // Split the socket into a sender and receive of messages.
    let (mut player_ws_tx, mut player_ws_rx) = ws.split();

    // Use an unbounded channel to handle buffering and flushing of messages
    // to the websocket...
    let (tx, rx) = mpsc::unbounded_channel();
    let mut rx = UnboundedReceiverStream::new(rx);

    // Save the sender in our list of connected users.
    players.write().await.insert(
        id.clone(),
        Player {
            player_id: id,
            is_host: true,
            sender: tx,
        },
    );
}

async fn player_connected(ws: WebSocket, players: Players) {
    let id = uuid::Uuid::new_v4().to_string();

    eprintln!("Player {} connected", &id);

    // Split the socket into a sender and receive of messages.
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

    // Save the sender in our list of connected users.
    players.write().await.insert(
        id.clone(),
        Player {
            player_id: id.clone(),
            is_host: true,
            sender: client_sender,
        },
    );

    // Every time the user sends a message, broadcast it to
    // all other users...
    while let Some(result) = client_ws_rcv.next().await {
        let msg = match result {
            Ok(msg) => msg,
            Err(e) => {
                eprintln!("websocket error(uid={}): {}", id.clone(), e);
                break;
            }
        };
        user_message(&id, msg, &players).await;
    }

    // user_ws_rx stream will keep processing as long as the user stays
    // connected. Once they disconnect, then...
    user_disconnected(&id, &players).await;
}

async fn user_message(my_id: &str, msg: Message, players: &Players) {
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
    // New message from this user, send it to everyone...
    for (uid, p) in players.read().await.iter() {
        if let Err(_disconnected) = p.sender.send(Message::text(&new_msg)) {
            // The tx is disconnected, our `user_disconnected` code
            // should be happening in another task, nothing more to
            // do here.
        }
    }
}

async fn user_disconnected(my_id: &str, players: &Players) {
    eprintln!("good bye player: {}", my_id);

    // Stream closed up, so remove from the user list
    players.write().await.remove(my_id);
}
