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
use models::item::describe_item; // Adjust the path depending on where describe_item is located

use dotenvy::dotenv;
use sqlx::{PgPool, migrate::Migrator};
use std::{env, net::SocketAddr, sync::Arc};


#[tokio::main]
async fn main() {
    dotenv().ok(); // Load env vars from `.env`

    // Initialize database connection pool
    let db = init_db().await;

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPool::connect(&database_url)
        .await
        .expect("Failed to connect to the database");

    // Apply migrations automatically on startup
    let migrator = Migrator::new(std::path::Path::new("migrations")).await.unwrap();
    migrator.run(&pool).await.unwrap();

    println!("Migrations applied successfully");

    // Continue with the rest of your app initialization...
    if cfg!(debug_assertions) {
    if let Err(e) = db::seed::seed_items(&db).await {
        eprintln!("⚠️ Failed to seed items: {}", e);
    }
}

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
let item = Item {
    id: 1,
    name: "Ancient Sword".to_string(),
    description: "A sword from a forgotten era.".to_string(),
    item_type: "Weapon".to_string(),
    value: 100,
    durability: Some(50),
    is_magical: true,
    is_cursed: true,
};

describe_item(&item);
// Health check with DB connectivity
async fn health_check() -> impl axum::response::IntoResponse {
    StatusCode::OK
}
