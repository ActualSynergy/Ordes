// Base entity trait for all design types

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BaseEntity {
    pub id: String,
    pub name: String,
    pub description: String,
    pub entity_type: EntityType,
    pub mass: f64,                          // kg
    pub power_budget: f64,                  // W
    pub dimensions: Dimensions,
    pub metadata: HashMap<String, serde_json::Value>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum EntityType {
    Satellite,
    Spacecraft,
    Chip,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Dimensions {
    pub width: f64,   // mm
    pub height: f64,  // mm
    pub depth: f64,   // mm
}

impl BaseEntity {
    pub fn new(name: String, description: String, entity_type: EntityType) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            name,
            description,
            entity_type,
            mass: 0.0,
            power_budget: 0.0,
            dimensions: Dimensions {
                width: 0.0,
                height: 0.0,
                depth: 0.0,
            },
            metadata: HashMap::new(),
            created_at: chrono::Utc::now().to_rfc3339(),
            updated_at: chrono::Utc::now().to_rfc3339(),
        }
    }
}
