use crate::models::DungeonRegion;
use std::fs;
use std::path::Path;
use anyhow::Result;

pub fn load_regions_from_dir(dir_path: &str) -> Result<Vec<DungeonRegion>> {
    let mut regions = Vec::new();
    let entries = fs::read_dir(dir_path)?;

    for entry in entries {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() && path.extension().map_or(false, |ext| ext == "toml") {
            let content = fs::read_to_string(&path)?;
            let region: DungeonRegion = toml::from_str(&content)?;
            regions.push(region);
        }
    }

    Ok(regions)
}
