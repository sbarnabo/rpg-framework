use crate::models::{Player, Item, Inventory};
use sqlx::PgPool;

pub async fn add_item_to_inventory(pool: &PgPool, player_id: i32, item: Item, quantity: i32) {
    let mut inventory = get_inventory_for_player(pool, player_id).await;
    match inventory.iter_mut().find(|i| i.item_id == item.id) {
        Some(existing_item) => existing_item.add_item(quantity),
        None => {
            let new_item = Inventory {
                player_id,
                item_id: item.id,
                quantity,
            };
            inventory.push(new_item);
        }
    }

    // Update inventory in DB
    update_inventory_in_db(pool, inventory).await;
}

pub async fn remove_item_from_inventory(pool: &PgPool, player_id: i32, item: Item, quantity: i32) {
    let mut inventory = get_inventory_for_player(pool, player_id).await;
    if let Some(existing_item) = inventory.iter_mut().find(|i| i.item_id == item.id) {
        existing_item.remove_item(quantity);
    }

    // Update inventory in DB
    update_inventory_in_db(pool, inventory).await;
}

// Helper functions to interact with DB
async fn get_inventory_for_player(pool: &PgPool, player_id: i32) -> Vec<Inventory> {
    sqlx::query_as::<_, Inventory>("SELECT * FROM inventory WHERE player_id = $1")
        .bind(player_id)
        .fetch_all(pool)
        .await
        .unwrap_or_default()
}

async fn update_inventory_in_db(pool: &PgPool, inventory: Vec<Inventory>) {
    // Update DB logic here (upsert or delete if item quantity is zero)
}
