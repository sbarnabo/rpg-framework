mod api;
mod db;
mod engine;
mod loader;
mod models;

use engine::map_graph::MapGraph;
use loader::artifacts::load_artifacts_from_dir;
use loader::dungeons::load_regions_from_dir;

#[tokio::main]
async fn main() {
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

    // Placeholder for Axum server setup
    println!("🚀 Axum API not started yet – coming soon!");
}
