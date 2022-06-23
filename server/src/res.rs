use crate::{Player, PlayerInfo};
use serde::{Deserialize, Serialize};
use shape_evolution::random_shape::RandomCircle;

#[derive(Debug, Serialize)]
pub enum OutboundWsEvent<'a> {
    Circle(RandomCircle),
    CircleSequence(Vec<RandomCircle>),
    NewImage {
        dimensions: (u32, u32),
        answer_hint: &'a str,
    },
    PlayerList(Vec<&'a PlayerInfo>),
    PrivateInfo(&'a Player),
    ChatMessage {
        name: &'a str,
        text: &'a str,
    },
    Answer(&'a str),
}

#[derive(Debug, Deserialize)]
pub enum InboundWsEvent<'a> {
    Circle(RandomCircle),
    ChatMessage(&'a str),
    NewImage {
        dimensions: (u32, u32),
        answer: &'a str,
    },
    PlayerName(&'a str),
}
