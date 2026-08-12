// Chip entity - Integrated circuit design

use super::base_entity::{BaseEntity, EntityType};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Chip {
    pub base: BaseEntity,
    pub specifications: ChipSpecifications,
    pub component_ids: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChipSpecifications {
    pub process_node: String,   // nm (7nm, 14nm, etc)
    pub transistor_count: u64,
    pub frequency: f64,         // GHz
    pub thermal_design_power: f64, // W
}

impl Chip {
    pub fn new(name: String, description: String) -> Self {
        Self {
            base: BaseEntity::new(name, description, EntityType::Chip),
            specifications: ChipSpecifications {
                process_node: "Unknown".to_string(),
                transistor_count: 0,
                frequency: 0.0,
                thermal_design_power: 0.0,
            },
            component_ids: Vec::new(),
        }
    }
}
