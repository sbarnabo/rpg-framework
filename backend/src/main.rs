mod api;
mod db;
mod engine;
mod loader;
mod models;

use loader::artifacts::load_artifacts_from_dir;
use loader::dungeons::load_regions_from_dir;
use engine::map_graph::MapGraph;
use loader::dungeons::load_regions_from_dir;

fn main() {
    let regions = load_regions_from_dir("content/regions")
        .expect("Failed to load dungeon regions");

    let graph = MapGraph::new(regions);

    // ✅ Print nexus portals
    if let Some(portals) = graph.get_portals("nexus") {
        println!("Nexus portals:");
        for p in portals {
            println!("- {} (to {}, requires level {})", p.name, p.leads_to, p.required_level);
        }
    }

    // 🔎 Validate portals
    let broken = graph.validate_links();
    if broken.is_empty() {
        println!("✅ All portals are valid!");
    } else {
        println!("⚠️ Found broken links:");
        for err in broken {
            println!("{}", err);
        }
    }
}


fn main() {
    let nexus_path = "content/regions";
    match load_regions_from_dir(nexus_path) {
        Ok(regions) => {
            println!("✅ Loaded {} regions", regions.len());
            for region in &regions {
                println!("Region: {} ({:?})", region.name, region.environment);
            }
        }
        Err(e) => eprintln!("⚠️ Failed to load regions: {}", e),
    }
}

#[tokio::main]
async fn main() {
    match load_artifacts_from_dir("content/artifacts") {
        Ok(artifacts) => {
            println!("✅ Loaded {} artifacts", artifacts.len());
            for artifact in &artifacts {
                println!("{:?}", artifact);
            }
        }
        Err(e) => eprintln!("⚠️ Error loading artifacts: {}", e),
    }

    // axum HTTP setup goes here
}
