use crate::{
    Room, 
    Rooms, 
    InboundWsEvent,
    OutboundWsEvent,
    broadcast_server_message, 
    broadcast_player_list,
    broadcast_ws_event,
    handle_chat_message,
    make_hint,
};
use std::sync::Arc;
use tokio::sync::RwLock;
use std::time::Duration;


pub async fn handle_user_disconnected(private_id: &str, rooms: Rooms, room: Arc<RwLock<Room>>) {
    let mut room = room.write().await;

    eprintln!("good bye player: {}", private_id);
    // Stream closed up, so remove from the user list.
    // Acquire write lock. The lock will be dropped on function end.
    let players = &mut room.players;
    let mut removed_player = None;
    let mut i = 0;
    while i < players.len() {
        if players[i].private_id == private_id {
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

    broadcast_server_message(&format!("{} left", removed_player.info.name), &room);

    if room.players.is_empty() {
        // Start a cleanup process
        room.wait_then_cleanup(rooms, Duration::from_secs(60));
    } else {
        // otherwise let everyone know who is still connected
        broadcast_player_list(&room);
    }
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


pub async fn handle_binary_message(private_id: &str, msg: Vec<u8>, room: Arc<RwLock<Room>>) {
    eprintln!("Got binary data");
    let mut room = room.write().await;
    let player = match room.get_player_mut(private_id) {
        Some(p) => p,
        None => return,
    };

    if player.info.is_host {
        room.image_data = Some(msg);
    }
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

    // Handle WsEvents which can come from any user
    let mut room = room.write().await;

    match event {
        InboundWsEvent::ChatMessage(message) => {
            if let Err(e) = handle_chat_message(message, private_id, &mut room) {
                eprintln!("Error: {}", e);
            }
        }
        _ => {
            let player = match room.get_player_mut(private_id) {
                Some(p) => p,
                None => return,
            };

            // Try handling event as host
            if !player.info.is_host {
                eprintln!("Rejecting message from player {}", private_id);
                return;
            }

            match event {
                InboundWsEvent::Circle(circle) => {
                    room.circles.push(circle.clone());
                    // Let everyone know there's a new circle
                    let event = OutboundWsEvent::Circle(circle);
                    broadcast_ws_event(event, &room);
                }
                InboundWsEvent::NewImage { dimensions, answer } => {
                    room.image_dimensions = Some(dimensions);
                    room.answer = Some(answer.to_lowercase());
                    room.circles.clear();
                    // Let everyone know there's a new image

                    let answer_hint = make_hint(answer);
                    let event = OutboundWsEvent::NewImage {
                        dimensions,
                        answer_hint: &answer_hint,
                    };
                    broadcast_ws_event(event, &room);
                }
                _ => {
                    eprintln!("Unsupported message type");
                }
            }
        }
    }
}
