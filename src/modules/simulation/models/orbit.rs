// Orbital mechanics models and calculations

use crate::modules::simulation::physics::{
    constants::*, vector_math::Vector3,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrbitalElements {
    pub semi_major_axis: f64,
    pub eccentricity: f64,
    pub inclination: f64,
    pub raan: f64,
    pub argument_of_perigee: f64,
    pub true_anomaly: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrbitalState {
    pub position: Vector3,
    pub velocity: Vector3,
    pub time: f64,
}

impl OrbitalElements {
    pub fn period(&self) -> f64 {
        2.0 * std::f64::consts::PI * (self.semi_major_axis.powi(3) / GM_EARTH).sqrt()
    }

    pub fn perigee_altitude(&self) -> f64 {
        self.semi_major_axis * (1.0 - self.eccentricity) - EARTH_RADIUS
    }

    pub fn apogee_altitude(&self) -> f64 {
        self.semi_major_axis * (1.0 + self.eccentricity) - EARTH_RADIUS
    }
}
