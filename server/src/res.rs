use crate::{Player, PlayerInfo};
use serde::{Deserialize, Serialize};
use shape_evolution::random_shape::RandomCircle;
use std::borrow::Cow;

#[derive(Debug, Serialize)]
pub enum OutboundWsEvent<'a> {
    Circle(RandomCircle),
    CircleSequence(Vec<RandomCircle>),
    NewImage {
        dimensions: (u32, u32),
        answer_hint: &'a str,
    },
    PlayerList(Vec<&'a PlayerInfo>),
    // Info which also contains a player's private id
    PrivateInfo(&'a Player),
    ChatMessage {
        name: &'a str,
        text: &'a str,
    },
    SecretChatMessage {
        name: &'a str,
        text: &'a str,
    },
    ServerMessage(&'a str),
    Answer(&'a str),
}

#[derive(Debug, Deserialize)]
pub enum InboundWsEvent<'a> {
    Circle(RandomCircle),
    // This cannot just be a `&'a str` type because it would not be able to handle escaped text.
    // The text needs to be processed (to turn things strings `a\"b` into `a"b`) so a simple
    // reference does not work.
    ChatMessage(Cow<'a, str>),
    // TODO: check if this is still used
    PlayerName(&'a str),
}
