use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum EnvironmentType {
    Fantasy,
    Technology,
    RealLife,
    Hybrid,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Portal {
    pub id: String,
    pub name: String,
    pub leads_to: String, // region_id
    pub required_level: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DungeonRegion {
    pub id: String,
    pub name: String,
    pub description: String,
    pub environment: EnvironmentType,
    pub portals: Vec<Portal>,
    pub anchor_point: Option<String>, // anchor ID in the nexus
}
