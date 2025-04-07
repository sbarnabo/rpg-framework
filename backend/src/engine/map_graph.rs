use std::collections::HashMap;
use crate::models::{DungeonRegion, Portal};

#[derive(Debug)]
pub struct MapGraph {
    pub regions: HashMap<String, DungeonRegion>,
    pub connections: HashMap<String, Vec<Portal>>, // region_id -> portals
}

impl MapGraph {
    pub fn new(regions: Vec<DungeonRegion>) -> Self {
        let mut region_map = HashMap::new();
        let mut conn_map = HashMap::new();

        for region in regions {
            conn_map.insert(region.id.clone(), region.portals.clone());
            region_map.insert(region.id.clone(), region);
        }

        MapGraph {
            regions: region_map,
            connections: conn_map,
        }
    }

    /// Returns the portals (exits) from a given region
    pub fn get_portals(&self, from_region: &str) -> Option<&Vec<Portal>> {
        self.connections.get(from_region)
    }

    /// Returns a reference to a DungeonRegion by ID
    pub fn get_region(&self, id: &str) -> Option<&DungeonRegion> {
        self.regions.get(id)
    }

    /// Validate that all portals lead to existing region IDs
    pub fn validate_links(&self) -> Vec<String> {
        let mut broken_links = Vec::new();

        for (from_id, portals) in &self.connections {
            for portal in portals {
                if !self.regions.contains_key(&portal.leads_to) {
                    broken_links.push(format!(
                        "🔗 Broken portal from '{}' to unknown region '{}'",
                        from_id, portal.leads_to
                    ));
                }
            }
        }

        broken_links
    }
}
