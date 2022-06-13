use crate::PlayerInfo;
use serde::{Deserialize, Serialize};
use shape_evolution::random_shape;

#[derive(Serialize, Deserialize)]
pub enum WsEvent {
    Circle(random_shape::RandomCircle),
    NewImage { dimensions: (u32, u32) },
    RoomPath(String),
    PlayerList(Vec<PlayerInfo>),
    PlayerId(String),
}
