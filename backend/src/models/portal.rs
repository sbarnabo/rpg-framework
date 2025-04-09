use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Portal {
    pub name: String,
    pub from_region: String,
    pub leads_to: String,
    pub required_level: i32,
}
