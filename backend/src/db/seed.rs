// src/db/seed.rs
use sqlx::{PgPool, Error};

#[derive(Debug)]
struct Item<'a> {
    name: &'a str,
    description: &'a str,
    durability: Option<i32>,
    is_magical: bool,
    is_cursed: bool,
    item_type: &'a str,
    power: i32,
    value: i32,
}

pub async fn seed_items(pool: &PgPool) -> Result<(), Error> {
   let items = vec![
        Item { name: "Iron Sword", description: "A basic iron sword.", durability: Some(100), is_magical: false, is_cursed: false, item_type: "Weapon", power: 10, value: 50 },
        Item { name: "Healing Potion", description: "Restores a small amount of health.", durability: None, is_magical: false, is_cursed: false, item_type: "Consumable", power: 0, value: 20 },
        Item { name: "Staff of Fire", description: "A magical staff that casts fire.", durability: Some(80), is_magical: true, is_cursed: false, item_type: "Weapon", power: 25, value: 150 },
        Item { name: "Cursed Ring", description: "A ring that binds the soul.", durability: None, is_magical: true, is_cursed: true, item_type: "Accessory", power: 5, value: 5 },
        Item { name: "Leather Armor", description: "Basic leather protection.", durability: Some(150), is_magical: false, is_cursed: false, item_type: "Armor", power: 0, value: 75 },
        Item { name: "Pegasus Saddle", description: "Used to mount a pegasus.", durability: Some(60), is_magical: false, is_cursed: false, item_type: "MountAccessory", power: 0, value: 100 },
        Item { name: "Battlemech Chip", description: "Activates a battlemech.", durability: None, is_magical: false, is_cursed: false, item_type: "TechKey", power: 0, value: 250 },
        Item { name: "Elven Cloak", description: "A magical cloak that boosts agility.", durability: Some(120), is_magical: true, is_cursed: false, item_type: "Armor", power: 2, value: 90 },
        Item { name: "Necromancer Skull", description: "Used to summon undead minions.", durability: None, is_magical: true, is_cursed: true, item_type: "MagicItem", power: 30, value: 300 },
        Item { name: "Explorer's Compass", description: "Helps navigate hybrid worlds.", durability: None, is_magical: false, is_cursed: false, item_type: "Tool", power: 0, value: 40 },
    ];

    for item in items {
        sqlx::query!(
            r#"
            INSERT INTO items (name, description, durability, is_magical, is_cursed, item_type, power, value)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            "#,
            item.name,
            item.description,
            item.durability,
            item.is_magical,
            item.is_cursed,
            item.item_type,
            item.power,
            item.value
        )
        .execute(pool)
        .await?;
    }

    println!("✅ Seeded sample items.");
    Ok(())
}
