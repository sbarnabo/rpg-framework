use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterClass {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub base_health: i32,
    pub base_mana: i32,
    pub starting_artifacts: Vec<Uuid>,
}
