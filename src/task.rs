// src/task.rs
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Task {
    pub id: u32,
    pub description: String,
    pub status: String,
}

impl Task {
    pub fn new(id: u32, description: String) -> Task {
        Task {
            id,
            description,
            status: "en_espera".to_string(),
        }
    }
}
