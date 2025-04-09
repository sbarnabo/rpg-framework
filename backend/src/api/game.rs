use axum::{Json, extract::Path};
use crate::models::{Player, Item, Quest};
use crate::engine::game_logic::{handle_combat, complete_quest};
use axum::http::StatusCode;

pub async fn start_combat(Path(player_id): Path<i32>, monster_health: i32) -> impl axum::response::IntoResponse {
    let mut player = get_player_from_db(player_id).await;
    let remaining_health = handle_combat(&mut player, monster_health);
    update_player_in_db(&player).await;
    Json(remaining_health)
}

pub async fn complete_quest_route(Path(player_id): Path<i32>, quest_id: i32) -> impl axum::response::IntoResponse {
    let mut player = get_player_from_db(player_id).await;
    let mut quest = get_quest_from_db(quest_id).await;
    complete_quest(&mut player, &mut quest);
    update_player_in_db(&player).await;
    Json(quest)
}
