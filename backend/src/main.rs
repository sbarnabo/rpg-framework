mod api;
mod db;
mod engine;
mod loader;
mod models;
mod startup;
mod routes;
mod config;

use crate::db::{init_db, seed_data, check_db_health};
use crate::config::{load_env, get_database_url};
use crate::startup::create_router;

use sqlx::postgres::PgPoolOptions;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    load_env();

    let database_url = get_database_url();
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("❌ Failed to connect to the database");

    if check_db_health(&pool).await {
        println!("✅ Database is healthy");
    }

    if cfg!(debug_assertions) {
        if let Err(e) = seed_data(&pool).await {
            eprintln!("⚠️ Failed to seed database: {}", e);
        }
    }

    let app = create_router(pool);
    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    println!("🚀 Server running at http://{}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
