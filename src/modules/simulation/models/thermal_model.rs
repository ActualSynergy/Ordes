// Thermal analysis model

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThermalNode {
    pub id: String,
    pub mass: f64,
    pub specific_heat: f64,
    pub emissivity: f64,
    pub absorptivity: f64,
    pub temperature: f64,
    pub internal_power: f64,
}

impl ThermalNode {
    pub fn new(id: String, mass: f64, specific_heat: f64) -> Self {
        Self {
            id,
            mass,
            specific_heat,
            emissivity: 0.8,
            absorptivity: 0.8,
            temperature: 293.0,
            internal_power: 0.0,
        }
    }
}
