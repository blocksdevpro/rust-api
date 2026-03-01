use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone)]
#[allow(dead_code)]
pub struct TodoJson {
    pub id: u16,
    pub title: String,
    pub completed: bool,
}
