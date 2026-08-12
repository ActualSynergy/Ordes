// Component instance - Specific placement of a component in an entity

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentInstance {
    pub id: String,
    pub component_template_id: String,   // Reference to component template
    pub position: Vec3,
    pub rotation: Quaternion,
    pub properties_override: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Quaternion {
    pub w: f64,
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl ComponentInstance {
    pub fn new(component_template_id: String) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            component_template_id,
            position: Vec3 { x: 0.0, y: 0.0, z: 0.0 },
            rotation: Quaternion { w: 1.0, x: 0.0, y: 0.0, z: 0.0 },
            properties_override: HashMap::new(),
        }
    }
}
