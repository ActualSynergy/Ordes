// Thermal simulation

use crate::modules::simulation::models::ThermalNode;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThermalSimulation {
    pub nodes: HashMap<String, ThermalNode>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThermalSimulationResult {
    pub max_temperature: f64,
    pub min_temperature: f64,
    pub average_temperature: f64,
}

impl ThermalSimulation {
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
        }
    }

    pub async fn run(&self, duration: f64, time_step: f64) -> ThermalSimulationResult {
        let temps: Vec<f64> = (0..((duration / time_step) as usize)).map(|_| 293.0).collect();

        ThermalSimulationResult {
            max_temperature: 350.0,
            min_temperature: 200.0,
            average_temperature: temps.iter().sum::<f64>() / temps.len() as f64,
        }
    }
}
