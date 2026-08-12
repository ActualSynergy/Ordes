// Assembly domain models

use serde::{Deserialize, Serialize};
use crate::modules::components::ComponentInstance;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Assembly {
    pub id: String,
    pub entity_id: String,              // Reference to Satellite, Spacecraft, or Chip
    pub components: Vec<ComponentInstance>,
    pub total_mass: f64,                // kg (calculated)
    pub total_power: f64,               // W (calculated)
    pub created_at: String,
    pub updated_at: String,
}

impl Assembly {
    pub fn new(entity_id: String) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            entity_id,
            components: Vec::new(),
            total_mass: 0.0,
            total_power: 0.0,
            created_at: chrono::Utc::now().to_rfc3339(),
            updated_at: chrono::Utc::now().to_rfc3339(),
        }
    }

    pub fn add_component(&mut self, component: ComponentInstance) {
        self.components.push(component);
        self.updated_at = chrono::Utc::now().to_rfc3339();
    }

    pub fn remove_component(&mut self, component_id: &str) -> Option<ComponentInstance> {
        if let Some(pos) = self.components.iter().position(|c| c.id == component_id) {
            self.updated_at = chrono::Utc::now().to_rfc3339();
            return Some(self.components.remove(pos));
        }
        None
    }
}
