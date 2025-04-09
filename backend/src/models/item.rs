use sqlx::FromRow;
use serde::{Serialize, Deserialize};

#[derive(FromRow, Serialize, Deserialize, Debug, Clone)]
pub struct Item {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub item_type: String,  // e.g. "Weapon", "Potion", etc.
    pub value: i32,
}

impl Item {
    pub fn use_item(&self, player: &mut Player) {
        match self.item_type.as_str() {
            "Potion" => player.heal(self.value),
            "Weapon" => println!("Equipping weapon: {}", self.name),
            _ => println!("Using item: {}", self.name),
        }
    }
}
