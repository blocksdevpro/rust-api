use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone)]

pub struct TodoJson {
    pub id: u32,
    pub title: String,
    pub completed: bool,
}
