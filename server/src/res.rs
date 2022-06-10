use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Event<T> {
    pub topic: String,
    pub payload: T,
}

