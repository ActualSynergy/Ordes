// Validation logic for simulations

use crate::modules::entities::BaseEntity;
use crate::modules::components::Component;

pub struct SimulationValidator;

impl SimulationValidator {
    pub fn validate_entity(entity: &BaseEntity) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();

        if entity.mass <= 0.0 {
            errors.push("Entity mass must be positive".to_string());
        }

        if entity.name.is_empty() {
            errors.push("Entity name cannot be empty".to_string());
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }

    pub fn validate_component(component: &Component) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();

        if component.name.is_empty() {
            errors.push("Component name cannot be empty".to_string());
        }

        if component.mass < 0.0 {
            errors.push("Component mass cannot be negative".to_string());
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}
