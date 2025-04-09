use sqlx::FromRow;
use serde::{Serialize, Deserialize};

#[derive(FromRow, Serialize, Deserialize, Debug, Clone)]
pub struct Item {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub item_type: String, // "Weapon", "Potion", etc.
    pub value: i32,
    pub durability: Option<i32>, // Durability for items like weapons and armor
    pub is_magical: bool, // Flag for magical items
}

impl Item {
    pub fn use_item(&mut self, player: &mut Player) {
        if let Some(durability) = self.durability {
            if durability <= 0 {
                println!("The item {} is broken and can no longer be used.", self.name);
                return;
            } else {
                self.durability = Some(durability - 1); // Decrease durability
                println!("Using item: {}. Remaining durability: {}", self.name, self.durability.unwrap());
            }
        }

        if self.is_magical {
            println!("The item {} is magical! It grants special abilities!", self.name);
        }

        match self.item_type.as_str() {
            "Potion" => {
                println!("Using potion: {}", self.name);
                player.heal(self.value);
            },
            "Weapon" => {
                println!("Equipping weapon: {}", self.name);
            },
            _ => println!("Using item: {}", self.name),
        }
    }
}
