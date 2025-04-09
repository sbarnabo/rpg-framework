mod api;
mod db;
mod engine;
mod loader;
mod models;

use db::init_db;
use axum::Extension;
use engine::map_graph::MapGraph;
use loader::artifacts::load_artifacts_from_dir;
use loader::dungeons::load_regions_from_dir;

use axum::{
    extract::Extension,
    http::StatusCode,
    response::IntoResponse,
    routing::get,
    Router,
};
use dotenvy::dotenv;
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use std::{env, net::SocketAddr};

#[tokio::main]
async fn main() {
    dotenv().ok(); // Load env vars from `.env`
    
    let db = init_db().await;
    let app = Router::new()
        .route("/health", get(health_check))
        .layer(Extension(db));
    
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to connect to the database");

    // Load regions (dungeons)
    let regions = match load_regions_from_dir("content/regions") {
        Ok(r) => r,
        Err(e) => {
            eprintln!("⚠️ Failed to load dungeon regions: {}", e);
            return;
        }
    };

    println!("✅ Loaded {} regions", regions.len());
    for region in &regions {
        println!("Region: {} ({:?})", region.name, region.environment);
    }

    // Build world graph
    let graph = MapGraph::new(regions.clone());
    if let Some(portals) = graph.get_portals("nexus") {
        println!("Nexus portals:");
        for p in portals {
            println!("- {} (to {}, requires level {})", p.name, p.leads_to, p.required_level);
        }
    }

    let broken = graph.validate_links();
    if broken.is_empty() {
        println!("✅ All portals are valid!");
    } else {
        println!("⚠️ Found broken links:");
        for err in broken {
            println!("{}", err);
        }
    }

    // Load artifacts
    match load_artifacts_from_dir("content/artifacts") {
        Ok(artifacts) => {
            println!("✅ Loaded {} artifacts", artifacts.len());
            for artifact in &artifacts {
                println!("{:?}", artifact);
            }
        }
        Err(e) => eprintln!("⚠️ Error loading artifacts: {}", e),
    }

    // Setup Axum API with shared DB pool
    let app = Router::new()
        .route("/health", get(health_check))
        .layer(Extension(pool));

    // Launch the server
    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    println!("🚀 Server running at http://{}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

// Health check with DB connectivity
async fn health_check(Extension(pool): Extension<PgPool>) -> impl IntoResponse {
    match sqlx::query("SELECT 1").execute(&pool).await {
        Ok(_) => StatusCode::OK,
        Err(_) => StatusCode::SERVICE_UNAVAILABLE,
    }
}
