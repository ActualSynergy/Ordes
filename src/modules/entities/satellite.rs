// Satellite entity - Orbital design

use super::base_entity::{BaseEntity, EntityType};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Satellite {
    pub base: BaseEntity,
    pub orbital_parameters: OrbitalParameters,
    pub component_ids: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrbitalParameters {
    pub altitude: f64,          // km
    pub inclination: f64,       // degrees
    pub eccentricity: f64,
    pub orbital_period: f64,    // minutes
}

impl Satellite {
    pub fn new(name: String, description: String) -> Self {
        Self {
            base: BaseEntity::new(name, description, EntityType::Satellite),
            orbital_parameters: OrbitalParameters {
                altitude: 0.0,
                inclination: 0.0,
                eccentricity: 0.0,
                orbital_period: 0.0,
            },
            component_ids: Vec::new(),
        }
    }
}
