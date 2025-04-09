use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Character {
    pub id: i32,
    pub name: String,
    pub class: String,
    pub level: i32,
    pub experience: i32,
}
