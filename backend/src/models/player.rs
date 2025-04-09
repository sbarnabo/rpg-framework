use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Player {
    pub id: Uuid,
    pub username: String,
    pub level: i32,
    pub experience: i32,
    pub current_region: String,
    pub inventory: Vec<Uuid>, // Artifact IDs
    pub class_id: Uuid,
}
