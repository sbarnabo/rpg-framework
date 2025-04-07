mod api;
mod db;
mod engine;
mod loader;
mod models;

use loader::artifacts::load_artifacts_from_dir;

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
