use crate::{
    broadcast_player_list, broadcast_server_message, broadcast_ws_event, handle_chat_message,
    InboundWsEvent, OutboundWsEvent, Room, Rooms, Round,
};
use http::StatusCode;
use serde::Serialize;
use std::convert::Infallible;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::RwLock;
use warp::{Rejection, Reply};

/// An API error serializable to JSON.
#[derive(Serialize)]
struct ErrorMessage {
    code: u16,
    message: String,
}

pub async fn handle_user_disconnected(private_id: &str, rooms: Rooms, room: Arc<RwLock<Room>>) {
    eprintln!("good bye player: {}", private_id);

    let mut room = room.write().await;

    // Find the player to be removed, as well as their index in the players vector
    if let Some((i, disconnecting_player)) = room
        .players
        .iter()
        .enumerate()
        .find(|(_i, player)| player.private_id == private_id)
    {
        let info = disconnecting_player.info.clone();

        if info.is_host {
            room.end_round();
        }
        room.players.remove(i);

        if room.players.is_empty() {
            // Start a cleanup process if nobody is left in the room
            room.wait_then_cleanup(rooms, Duration::from_secs(60));
        } else {
            // otherwise let everyone know who is still connected
            broadcast_server_message(&format!("{} left", info.name), &room);
            broadcast_player_list(&room);
        }
    }
}

pub async fn handle_upload_round_data(
    rooms: Rooms,
    room_id: String,
    private_player_id: String,
    round: Round,
) -> Result<impl warp::Reply, warp::Rejection> {
    eprintln!("handling round data upload");
    let rooms = rooms.read().await;

    let room = match rooms.get(&room_id) {
        Some(r) => r,
        None => return Ok(http::StatusCode::NOT_FOUND),
    };
    let mut room = room.write().await;

    match room.get_player_from_private_id(&private_player_id) {
        Some(player) if player.info.is_host => {
            // Request is validated, now do some stuff
            if room.round.is_none() {
                room.round = Some(round);

                let round = room.round.as_ref().unwrap();
                // Let people know about the new round's image dimensions and answer hint
                let event = OutboundWsEvent::NewImage {
                    dimensions: round.image_dimensions,
                    answer_hint: &round.answer_hint(),
                };
                broadcast_ws_event(event, &room);
            }
            Ok(http::StatusCode::OK)
        }
        _ => return Ok(http::StatusCode::NOT_FOUND),
    }
}

pub async fn handle_rejection(err: Rejection) -> Result<impl Reply, Infallible> {
    let code;
    let message;

    if err.is_not_found() {
        code = StatusCode::NOT_FOUND;
        message = "NOT_FOUND";
    } else if let Some(_) = err.find::<warp::reject::UnsupportedMediaType>() {
        code = StatusCode::UNSUPPORTED_MEDIA_TYPE;
        message = "UNSUPPORTED_MEDIA_TYPE";
    } else if let Some(_) = err.find::<warp::reject::MethodNotAllowed>() {
        // We can handle a specific error, here METHOD_NOT_ALLOWED,
        // and render it however we want
        code = StatusCode::METHOD_NOT_ALLOWED;
        message = "METHOD_NOT_ALLOWED";
    } else {
        eprintln!("unhandled rejection: {:?}", err);
        code = StatusCode::INTERNAL_SERVER_ERROR;
        message = "UNHANDLED_REJECTION";
    }

    let json = warp::reply::json(&ErrorMessage {
        code: code.as_u16(),
        message: message.into(),
    });

    Ok(warp::reply::with_status(json, code))
}

#[allow(unused_variables)]
pub async fn handle_binary_message(private_id: &str, msg: Vec<u8>, room: Arc<RwLock<Room>>) {
    eprintln!("Ignoring binary data");
}

async fn handle_inbound_ws_event(private_id: &str, event: InboundWsEvent<'_>, room: &mut Room) -> Result<(), String> {
    match event {
        InboundWsEvent::ChatMessage(message) => {
            handle_chat_message(&message, private_id, room)?;
        }
        InboundWsEvent::GiveUp => {
            if room.round.is_none() {
                return Err("Cannot give up before round starts".to_string());
            } else if room.get_player_from_private_id(private_id)
                .ok_or("Player not found".to_string())?
                .info.is_host
            {
                return Err("Only non-hosts can give up".to_string());
            }

            {
                let mut player = room.get_player_mut_from_private_id(private_id)
                    .ok_or("Player not found".to_string())?;
                player.info.has_answer = true;
            }
            let player = room.get_player_from_private_id(private_id)
                .ok_or("Player not found".to_string())?;
            room.send_player_answer(player)?;
            broadcast_server_message(&format!("{} gave up", player.info.name), room);
            broadcast_player_list(room);

            // Check if all non-host players have finished
            let round_over = room
                .players
                .iter()
                .filter(|p| !p.info.is_host)
                .all(|p| p.info.has_answer);
            if round_over {
                room.end_round();
            }
        }
        InboundWsEvent::Circle(circle) => {
            let player = room.get_player_from_private_id(private_id)
                .ok_or("Player not found".to_string())?;
            if !player.info.is_host {
                return Err("Only hosts can send circles".to_string());
            }

            if let Some(ref mut round) = room.round {
                round.add_circle(circle.clone());
                // Let everyone know there's a new circle
                let event = OutboundWsEvent::Circle(circle);
                broadcast_ws_event(event, &room);
            }
        }
        InboundWsEvent::Pass => {
            let player = room.get_player_from_private_id(private_id)
                .ok_or("Player not found".to_string())?;
            if !player.info.is_host {
                return Err("Only hosts can pass".to_string());
            } else if room.round.is_some() {
                return Err("Cannot pass while round is in progress".to_string());
            }

            room.advance_host();
            broadcast_player_list(room);
        }
        InboundWsEvent::PlayerName(_) => {
            eprintln!("PlayerName is not implemented");
            return Err("PlayerName is not implemented".to_string());
        }
    }

    Ok(())
}

pub async fn handle_text_message(private_id: &str, msg: &str, room: Arc<RwLock<Room>>) {
    // Try parsing the message into a WsEvent
    let event: InboundWsEvent = match serde_json::from_str(msg) {
        Ok(event) => event,
        Err(e) => {
            eprintln!("Failed to deserialize message {:?}", e);
            return;
        }
    };

    let mut room = room.write().await;
    handle_inbound_ws_event(private_id, event, &mut room).await;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Player;
    use tokio::sync::mpsc;

    #[tokio::test]
    async fn test_player_removed() {
        let rooms = Rooms::default();
        let (sender, _) = mpsc::unbounded_channel();
        let player = Player::new("alice".to_string(), true, sender);
        let private_id = player.private_id.clone();

        {
            let mut rooms = rooms.write().await;
            let mut room = Room::new("foo".to_string());

            room.insert_player(player);
            rooms.insert("foo".to_string(), Arc::new(RwLock::new(room)));
        }
        {
            let rooms = rooms.read().await;
            let room = rooms.get("foo").unwrap();
            let room = room.read().await;
            assert_eq!(room.players.len(), 1);
        }

        let rooms_clone = rooms.clone();
        {
            let rooms = rooms.read().await;
            let room = rooms.get("foo").unwrap();
            handle_user_disconnected(&private_id, rooms_clone, room.clone()).await;
            {
                let room = room.read().await;
                assert_eq!(room.players.len(), 0);
            }
        }
    }
}
