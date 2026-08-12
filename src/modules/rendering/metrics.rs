// Calculate metrics for visualization and analysis

use crate::modules::satellite::Satellite;

#[derive(Debug, Clone)]
pub struct SatelliteMetrics {
    pub total_mass: f64,
    pub center_of_mass: (f64, f64, f64),
    pub bounding_box: (f64, f64, f64),
    pub component_count: usize,
}

pub struct MetricsCalculator;

impl MetricsCalculator {
    pub fn calculate(satellite: &Satellite) -> SatelliteMetrics {
        let total_mass = satellite.mass;
        let component_count = satellite.components.len();

        SatelliteMetrics {
            total_mass,
            center_of_mass: (0.0, 0.0, 0.0),
            bounding_box: (100.0, 100.0, 100.0),
            component_count,
        }
    }
}
