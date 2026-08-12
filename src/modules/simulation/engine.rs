// Main simulation engine

use serde::{Deserialize, Serialize};
use crate::modules::simulation::domains::{
    OrbitalSimulation, ThermalSimulation, PowerSimulation,
    OrbitalSimulationResult, ThermalSimulationResult, PowerSimulationResult,
};
use crate::modules::simulation::models::{OrbitalElements, PowerBudget};
use crate::modules::simulation::physics::Vector3;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimulationRequest {
    pub entity_id: String,
    pub assembly_id: String,
    pub scenario: SimulationScenario,
    pub enabled_simulations: Vec<SimulationType>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimulationScenario {
    pub duration: f64,
    pub time_step: f64,
    pub sun_direction: Vector3,
    pub atmospheric_density: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SimulationType {
    Orbital,
    Thermal,
    Power,
    Collision,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimulationOutput {
    pub orbital: Option<OrbitalSimulationResult>,
    pub thermal: Option<ThermalSimulationResult>,
    pub power: Option<PowerSimulationResult>,
    pub timestamp: String,
}

pub struct SimulationEngine;

impl SimulationEngine {
    pub fn new() -> Self {
        Self
    }

    pub async fn run_simulation(
        &self,
        request: SimulationRequest,
    ) -> Result<SimulationOutput, String> {
        let mut output = SimulationOutput {
            orbital: None,
            thermal: None,
            power: None,
            timestamp: chrono::Utc::now().to_rfc3339(),
        };

        for sim_type in &request.enabled_simulations {
            match sim_type {
                SimulationType::Orbital => {
                    let elements = OrbitalElements {
                        semi_major_axis: 6.741e6,
                        eccentricity: 0.0,
                        inclination: 0.0,
                        raan: 0.0,
                        argument_of_perigee: 0.0,
                        true_anomaly: 0.0,
                    };
                    let sim = OrbitalSimulation::new(elements);
                    let result = sim.run(request.scenario.duration, request.scenario.time_step).await;
                    output.orbital = Some(result);
                }
                SimulationType::Thermal => {
                    let sim = ThermalSimulation::new();
                    let result = sim.run(request.scenario.duration, request.scenario.time_step).await;
                    output.thermal = Some(result);
                }
                SimulationType::Power => {
                    let budget = PowerBudget::new();
                    let sim = PowerSimulation::new(budget);
                    let result = sim.run(request.scenario.duration, request.scenario.time_step).await;
                    output.power = Some(result);
                }
                _ => {}
            }
        }

        Ok(output)
    }
}
