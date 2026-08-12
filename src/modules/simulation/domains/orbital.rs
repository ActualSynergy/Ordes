// Orbital mechanics simulation

use crate::modules::simulation::models::OrbitalElements;
use crate::modules::simulation::physics::Vector3;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrbitalSimulation {
    pub elements: OrbitalElements,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrbitalSimulationResult {
    pub trajectory: Vec<(f64, Vector3, Vector3)>,
    pub altitudes: Vec<f64>,
    pub velocities: Vec<f64>,
    pub max_altitude: f64,
    pub min_altitude: f64,
    pub orbital_period: f64,
}

impl OrbitalSimulation {
    pub fn new(elements: OrbitalElements) -> Self {
        Self { elements }
    }

    pub async fn run(&self, duration: f64, time_step: f64) -> OrbitalSimulationResult {
        let mut trajectory = Vec::new();
        let mut altitudes = Vec::new();
        let mut velocities = Vec::new();

        let mut time = 0.0;
        while time < duration {
            let altitude = (self.elements.semi_major_axis * (1.0 - self.elements.eccentricity)) - 6.371e6;
            altitudes.push(altitude);
            velocities.push(7800.0);
            trajectory.push((time, Vector3::new(0.0, 0.0, 0.0), Vector3::new(7800.0, 0.0, 0.0)));
            time += time_step;
        }

        OrbitalSimulationResult {
            trajectory,
            altitudes: altitudes.clone(),
            velocities,
            max_altitude: altitudes.iter().cloned().fold(f64::NEG_INFINITY, f64::max),
            min_altitude: altitudes.iter().cloned().fold(f64::INFINITY, f64::min),
            orbital_period: self.elements.period(),
        }
    }
}
