use sqlx::FromRow;
use serde::{Serialize, Deserialize};

#[derive(FromRow, Serialize, Deserialize, Debug, Clone)]
pub struct Player {
    pub id: i32,
    pub username: String,
    pub level: i32,
    pub health: i32,
    pub max_health: i32,
    pub experience: i32,
}

impl Player {
    pub fn level_up(&mut self) {
        self.level += 1;
        self.max_health += 10;
        self.health = self.max_health;
        self.experience = 0;
    }

    pub fn take_damage(&mut self, amount: i32) {
        self.health -= amount;
        if self.health < 0 {
            self.health = 0;
        }
    }

    pub fn heal(&mut self, amount: i32) {
        self.health += amount;
        if self.health > self.max_health {
            self.health = self.max_health;
        }
    }
}
