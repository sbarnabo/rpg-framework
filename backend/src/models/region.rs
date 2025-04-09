use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Environment {
    Fantasy,
    Technology,
    Realistic,
    Hybrid,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Region {
    pub id: String,
    pub name: String,
    pub environment: Environment,
    pub description: String,
}
