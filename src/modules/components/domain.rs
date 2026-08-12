// Domain models for components

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Component {
    pub id: String,
    pub name: String,
    pub component_type: ComponentType,
    pub mass: f64,              // kg
    pub power_consumption: f64, // W
    pub dimensions: Dimensions,
    pub specifications: HashMap<String, serde_json::Value>,
    pub created_by: String,
    pub is_public: bool,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ComponentType {
    Processor,
    Sensor,
    Battery,
    SolarPanel,
    Antenna,
    Actuator,
    Thruster,
    ThermalControl,
    PowerDistribution,
    Other(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Dimensions {
    pub width: f64,   // mm
    pub height: f64,  // mm
    pub depth: f64,   // mm
}

impl Component {
    pub fn new(
        name: String,
        component_type: ComponentType,
        mass: f64,
        created_by: String,
    ) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            name,
            component_type,
            mass,
            power_consumption: 0.0,
            dimensions: Dimensions {
                width: 0.0,
                height: 0.0,
                depth: 0.0,
            },
            specifications: HashMap::new(),
            created_by,
            is_public: false,
            created_at: chrono::Utc::now().to_rfc3339(),
        }
    }
}
