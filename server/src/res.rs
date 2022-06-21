use crate::{Player, PlayerInfo};
use serde::{Deserialize, Serialize};
use shape_evolution::random_shape::RandomCircle;

#[derive(Serialize, Deserialize)]
pub enum WsEvent<'a> {
    Circle(RandomCircle),

    #[serde(skip_deserializing)]
    CircleSequence(Vec<RandomCircle>),
    NewImage { dimensions: (u32, u32) },
    #[serde(skip_deserializing)]
    RoomPath(String),
    #[serde(skip_deserializing)]
    PlayerList(Vec<PlayerInfo>),
    #[serde(skip_deserializing)]
    PrivateInfo(&'a Player),

    #[serde(skip_serializing)]
    PlayerName(String),
}
