use serde::{Deserialize, Deserializer, Serialize};
use shape_evolution::random_shape;

#[derive(Serialize, Deserialize)]
pub struct Event<T> {
    pub topic: String,
    pub payload: T,
}

#[derive(Serialize, Deserialize)]
pub struct NewImage {
    pub dimensions: (u32, u32)
}
