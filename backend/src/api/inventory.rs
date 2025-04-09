use axum::{Json, extract::Path};
use crate::engine::inventory_logic::{add_item_to_inventory, remove_item_from_inventory};
use crate::models::{Player, Item};
use axum::http::StatusCode;
use axum::response::IntoResponse;

pub async fn add_item(Path(player_id): Path<i32>, item: Item, quantity: i32) -> impl IntoResponse {
    // Add the item to the player's inventory
    add_item_to_inventory(&pool, player_id, item, quantity).await;
    StatusCode::OK
}

pub async fn remove_item(Path(player_id): Path<i32>, item: Item, quantity: i32) -> impl IntoResponse {
    // Remove the item from the player's inventory
    remove_item_from_inventory(&pool, player_id, item, quantity).await;
    StatusCode::OK
}
