use sqlx::FromRow;
use serde::{Serialize, Deserialize};

#[derive(FromRow, Serialize, Deserialize, Debug, Clone)]
pub struct Inventory {
    pub player_id: i32,
    pub item_id: i32,
    pub quantity: i32,
    pub durability: Option<i32>, // Durability for items like weapons and armor
}


impl Inventory {
    pub fn add_item(&mut self, quantity: i32, durability: Option<i32>) {
        self.quantity += quantity;
        if let Some(durability) = durability {
            self.durability = Some(durability);
        }
    }

    pub fn remove_item(&mut self, quantity: i32) {
        if self.quantity >= quantity {
            self.quantity -= quantity;
        } else {
            self.quantity = 0;
        }
    }
}

