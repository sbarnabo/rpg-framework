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

use dotenvy::dotenv;
use sqlx::PgPool;
use std::{env, net::SocketAddr};
use std::sync::Arc;
use std::collections::HashMap;

#[tokio::main]
async fn main() {
    dotenv().ok(); // Load env vars from `.env`

    // Initialize database connection pool
    let db = init_db().await;

    // Seed data if needed (for debugging purposes)
    if cfg!(debug_assertions) {
        if let Err(e) = seed_data(&db).await {
            eprintln!("⚠️ Failed to seed database: {}", e);
        }
    }

    // Optional health check log
    if check_db_health(&db).await {
        println!("✅ Database is healthy");
    } else {
        eprintln!("❌ Database connection failed");
    }

    // Create Axum app with routes and shared database pool
    let app = Router::new()
        .route("/health", get(health_check))
        .route("/auth/login", get(login))  // Add login route
        .route("/player/:id", get(get_player))  // Get player by id route
        .route("/players", get(get_players))  // Get multiple players route
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
async fn health_check() -> impl IntoResponse {
    StatusCode::OK
}
