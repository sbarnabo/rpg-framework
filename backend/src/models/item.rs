use sqlx::FromRow;
use serde::{Serialize, Deserialize};
use crate::models::Item; // Assuming Item is in the models module


#[derive(FromRow, Serialize, Deserialize, Debug, Clone)]
pub struct Item {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub item_type: String, // "Weapon", "Potion", etc.
    pub value: i32,
    pub durability: Option<i32>, // Nullable for durability (for items like weapons and armor)
    pub is_magical: bool, // Flag for magical items
    pub is_cursed: bool,  // Flag for cursed items
}
// Function to describe an item, including its properties
pub fn describe_item(item: &Item) {
    println!("Item: {}", item.name);
    println!("Description: {}", item.description);
    println!("Type: {}", item.item_type);
    println!("Value: {}", item.value);

    if let Some(durability) = item.durability {
        println!("Durability: {}", durability);
    }

    if item.is_magical {
        println!("This item is magical!");
    }

    if item.is_cursed {
        println!("This item is cursed!");
    }
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
            // Magical effect, can be expanded
            println!("The item {} is magical! It grants special abilities!", self.name);
            if self.item_type == "Potion" {
                // Magical potion might heal more
                player.heal(self.value * 2); // Example effect: heals double
            }
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
