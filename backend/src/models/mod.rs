use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Artifact {
    pub id: String,
    pub name: String,
    pub description: String,
    pub power: u32,
    pub rarity: String,
}
