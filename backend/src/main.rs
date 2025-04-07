use axum::{Router, routing::get};
use std::net::SocketAddr;
use dotenvy::dotenv;
use tracing_subscriber;

mod api;
mod engine;
mod loader;
mod models;
mod db;

#[tokio::main]
async fn main() {
    dotenv().ok();
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/", get(|| async { "Text RPG Backend is running" }))
        .merge(api::routes());

    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    println!("🚀 Backend running on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
