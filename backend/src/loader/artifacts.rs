use crate::models::Artifact;
use std::fs;
use std::path::Path;
use anyhow::Result;

pub fn load_artifacts_from_dir(dir_path: &str) -> Result<Vec<Artifact>> {
    let mut artifacts = Vec::new();
    let entries = fs::read_dir(dir_path)?;

    for entry in entries {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() && path.extension().map_or(false, |ext| ext == "toml") {
            let content = fs::read_to_string(&path)?;
            let artifact: Artifact = toml::from_str(&content)?;
            artifacts.push(artifact);
        }
    }

    Ok(artifacts)
}
