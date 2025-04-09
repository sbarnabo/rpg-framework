// player.rs
use axum::{
    extract::{Path, Query},
    http::StatusCode,
    response::{IntoResponse, Json},
};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use std::sync::Arc;

#[derive(Serialize, Deserialize)]
pub struct Player {
    pub id: i32,
    pub username: String,
    pub email: String,
}

pub async fn get_player(
    pool: Arc<PgPool>,
    Path(player_id): Path<i32>,
) -> impl IntoResponse {
    // Query player from the database
    let player = sqlx::query_as!(
        Player,
        "SELECT id, username, email FROM players WHERE id = $1",
        player_id
    )
    .fetch_optional(&*pool)
    .await
    .unwrap();

    match player {
        Some(p) => Json(p),
        None => StatusCode::NOT_FOUND.into_response(),
    }
}

pub async fn get_players(pool: Arc<PgPool>, Query(params): Query<HashMap<String, String>>) -> impl IntoResponse {
    // Fetch multiple players from database with optional filters
    let players = sqlx::query_as!(
        Player,
        "SELECT id, username, email FROM players WHERE username LIKE $1",
        params.get("username").unwrap_or(&"".to_string())
    )
    .fetch_all(&*pool)
    .await
    .unwrap();

    Json(players)
}
