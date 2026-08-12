// Power budget model

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PowerBudget {
    pub sources: HashMap<String, f64>,
    pub loads: HashMap<String, f64>,
    pub battery_capacity: f64,
    pub battery_state_of_charge: f64,
}

impl PowerBudget {
    pub fn new() -> Self {
        Self {
            sources: HashMap::new(),
            loads: HashMap::new(),
            battery_capacity: 100.0,
            battery_state_of_charge: 1.0,
        }
    }

    pub fn total_generation(&self) -> f64 {
        self.sources.values().sum()
    }

    pub fn total_consumption(&self) -> f64 {
        self.loads.values().sum()
    }

    pub fn power_balance(&self) -> f64 {
        self.total_generation() - self.total_consumption()
    }
}
