// Spacecraft entity - Space vehicles design

use super::base_entity::{BaseEntity, EntityType};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Spacecraft {
    pub base: BaseEntity,
    pub propulsion: PropulsionSystem,
    pub component_ids: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PropulsionSystem {
    pub thrust: f64,            // N
    pub specific_impulse: f64,  // s
    pub fuel_capacity: f64,     // kg
}

impl Spacecraft {
    pub fn new(name: String, description: String) -> Self {
        Self {
            base: BaseEntity::new(name, description, EntityType::Spacecraft),
            propulsion: PropulsionSystem {
                thrust: 0.0,
                specific_impulse: 0.0,
                fuel_capacity: 0.0,
            },
            component_ids: Vec::new(),
        }
    }
}
