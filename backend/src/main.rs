mod api;
mod db;
mod engine;
mod loader;
mod models;

use axum::{Router, Extension};
use axum::routing::{get};
use db::{init_db, check_db_health, seed_data};
use api::auth::login;
use api::player::{get_player, get_players};
use api::game::{start_combat, complete_quest_route};
use api::inventory::{add_item, remove_item}; // Add this line

use dotenvy::dotenv;
use sqlx::PgPool;
use std::{env, net::SocketAddr};
use std::sync::Arc;

#[tokio::main]
async fn main() {
    dotenv().ok(); // Load env vars from `.env`

    // Initialize database connection pool
    let db = init_db().await;

    // Create Axum app with routes and shared database pool
    let app = Router::new()
        .route("/health", get(health_check))
        .route("/auth/login", get(login))  // Add login route
        .route("/player/:id", get(get_player))  // Get player by id route
        .route("/players", get(get_players))  // Get multiple players route
        .route("/combat/:player_id/:monster_health", get(start_combat))  // Combat route
        .route("/quest/:player_id/:quest_id", get(complete_quest_route))  // Complete quest route
        .route("/inventory/add/:player_id", get(add_item))  // Add item to inventory
        .route("/inventory/remove/:player_id", get(remove_item))  // Remove item from inventory
        .layer(Extension(Arc::new(db)));

    // Launch the server
    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    println!("🚀 Server running at http://{}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

// Health check with DB connectivity
async fn health_check() -> impl axum::response::IntoResponse {
    StatusCode::OK
}
