use sqlx::FromRow;
use serde::{Serialize, Deserialize};

#[derive(FromRow, Serialize, Deserialize, Debug, Clone)]
pub struct Quest {
    pub id: i32,
    pub title: String,
    pub description: String,
    pub completed: bool,
}

impl Quest {
    pub fn complete(&mut self) {
        self.completed = true;
    }
}
