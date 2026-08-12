// Calculate metrics for visualization and analysis

use crate::modules::entities::BaseEntity;

#[derive(Debug, Clone)]
pub struct EntityMetrics {
    pub total_mass: f64,
    pub center_of_mass: (f64, f64, f64),
    pub bounding_box: (f64, f64, f64),
}

pub struct MetricsCalculator;

impl MetricsCalculator {
    pub fn calculate(entity: &BaseEntity) -> EntityMetrics {
        let total_mass = entity.mass;

        EntityMetrics {
            total_mass,
            center_of_mass: (0.0, 0.0, 0.0),
            bounding_box: (entity.dimensions.width, entity.dimensions.height, entity.dimensions.depth),
        }
    }
}
