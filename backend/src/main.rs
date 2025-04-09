mod api;
mod db;
mod engine;
mod loader;
mod models;

use db::{init_db, check_db_health, seed_data};
use axum::{
    extract::Extension,
    http::StatusCode,
    response::IntoResponse,
    routing::get,
    Router,
};
use engine::map_graph::MapGraph;
use loader::artifacts::load_artifacts_from_dir;
use loader::dungeons::load_regions_from_dir;
use std::{net::SocketAddr};

#[tokio::main]
async fn main() {
    // Initialize DB + migrations
    let db = init_db().await;

    // Seed (in debug mode only)
    if cfg!(debug_assertions) {
        if let Err(e) = seed_data(&db).await {
            eprintln!("⚠️ Failed to seed database: {}", e);
        }
    }

    // Health check logging
    if check_db_health(&db).await {
        println!("✅ Database is healthy");
    } else {
        eprintln!("❌ Database connection failed");
    }

    // Load dungeon regions
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

    // Build map graph
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

    // Setup Axum API
    let app = Router::new()
        .route("/health", get(health_check))
        .layer(Extension(db));

    // Launch the server
    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    println!("🚀 Server running at http://{}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

// Health check route
async fn health_check(Extension(pool): Extension<db::DbPool>) -> impl IntoResponse {
    match sqlx::query("SELECT 1").execute(pool.as_ref()).await {
        Ok(_) => StatusCode::OK,
        Err(_) => StatusCode::SERVICE_UNAVAILABLE,
    }
}
