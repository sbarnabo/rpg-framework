use rand::seq::SliceRandom;
use rand::Rng;
use std::fs;
use std::path::Path;
use std::io::Write;
use crate::models::{DungeonRegion, EnvironmentType, Portal};
use anyhow::Result;

const ENV_TYPES: &[EnvironmentType] = &[
    EnvironmentType::Fantasy,
    EnvironmentType::Technology,
    EnvironmentType::RealLife,
    EnvironmentType::Hybrid,
];

/// Generate a random environment type
fn random_env() -> EnvironmentType {
    ENV_TYPES.choose(&mut rand::thread_rng()).unwrap().clone()
}

/// Generate a new region with a unique name and portals
pub fn generate_region(existing_region_ids: &[String]) -> DungeonRegion {
    let mut rng = rand::thread_rng();
    let id = format!("region_{}", rng.gen_range(1000..9999));
    let name = format!("Mystery Zone {}", rng.gen_range(1..100));
    let description = format!("A procedurally generated zone of strangeness and wonder.");
    let environment = random_env();

    let mut portals = vec![];

    // Connect to one or two existing regions at random
    let connections = existing_region_ids.choose_multiple(&mut rng, 2);
    for conn in connections {
        portals.push(Portal {
            id: format!("portal_to_{}", conn),
            name: format!("Portal to {}", conn),
            leads_to: conn.clone(),
            required_level: rng.gen_range(1..10),
        });
    }

    DungeonRegion {
        id,
        name,
        description,
        environment,
        portals,
        anchor_point: None,
    }
}

/// Save the new region as a TOML file
pub fn save_region(region: &DungeonRegion, dir: &str) -> Result<()> {
    let toml_str = toml::to_string_pretty(&region)?;
    let file_path = Path::new(dir).join(format!("{}.toml", region.id));
    let mut file = fs::File::create(file_path)?;
    file.write_all(toml_str.as_bytes())?;
    Ok(())
}
