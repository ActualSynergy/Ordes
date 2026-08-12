// Power simulation

use crate::modules::simulation::models::PowerBudget;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PowerSimulation {
    pub budget: PowerBudget,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PowerSimulationResult {
    pub generation_profile: Vec<f64>,
    pub consumption_profile: Vec<f64>,
    pub soc_profile: Vec<f64>,
}

impl PowerSimulation {
    pub fn new(budget: PowerBudget) -> Self {
        Self { budget }
    }

    pub async fn run(&self, duration: f64, time_step: f64) -> PowerSimulationResult {
        let steps = (duration / time_step) as usize;
        
        PowerSimulationResult {
            generation_profile: vec![50.0; steps],
            consumption_profile: vec![30.0; steps],
            soc_profile: vec![0.8; steps],
        }
    }
}
