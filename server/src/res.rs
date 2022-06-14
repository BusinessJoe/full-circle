use crate::PlayerInfo;
use serde::{Deserialize, Serialize};
use shape_evolution::random_shape::RandomCircle;

#[derive(Serialize, Deserialize)]
pub enum WsEvent {
    Circle(RandomCircle),
    CircleSequence(Vec<RandomCircle>),
    NewImage { dimensions: (u32, u32) },
    RoomPath(String),
    PlayerList(Vec<PlayerInfo>),
    PlayerId(String),
}
