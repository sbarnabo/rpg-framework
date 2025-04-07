// src/procedural/world_generator.rs
use rand::{rngs::StdRng, Rng, SeedableRng};
use crate::models::dungeon::{DungeonRegion, EnvironmentType, Portal};

pub struct GenerationConfig {
    pub seed: Option<u64>,
    pub region_count: usize,
}

pub fn generate_world(config: GenerationConfig) -> Vec<DungeonRegion> {
    // Initialize RNG with a seed for reproducibility.
    let seed = config.seed.unwrap_or_else(|| rand::random());
    let mut rng = StdRng::seed_from_u64(seed);
    
    let mut regions = Vec::new();

    for i in 0..config.region_count {
        let id = format!("proc_region_{}", i);
        let name = format!("Procedural Region {}", i);
        let description = "A dynamically generated region.".to_string();

        // Randomly pick an environment type.
        let environment = match rng.gen_range(0..3) {
            0 => EnvironmentType::Fantasy,
            1 => EnvironmentType::Technology,
            _ => EnvironmentType::Hybrid,
        };

        // Create dummy portals connecting regions (ensure connectivity later).
        let portal = Portal {
            id: format!("portal_{}_to_next", i),
            name: format!("Portal from {} to {}", id, id),
            leads_to: if i + 1 < config.region_count {
                format!("proc_region_{}", i + 1)
            } else {
                "nexus".to_string()  // last region loops back to nexus
            },
            required_level: rng.gen_range(1..10),
        };

        regions.push(DungeonRegion {
            id,
            name,
            description,
            environment,
            portals: vec![portal],
            anchor_point: None,
        });
    }
    
    regions
}