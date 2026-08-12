// Diff calculation between versions

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Diff {
    pub added: Vec<String>,
    pub removed: Vec<String>,
    pub modified: Vec<Change>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Change {
    pub field: String,
    pub old_value: serde_json::Value,
    pub new_value: serde_json::Value,
}

pub struct DiffCalculator;

impl DiffCalculator {
    pub fn calculate_diff(
        old: &serde_json::Value,
        new: &serde_json::Value,
    ) -> Result<Diff, String> {
        // TODO: Implement proper diff algorithm
        Ok(Diff {
            added: Vec::new(),
            removed: Vec::new(),
            modified: Vec::new(),
        })
    }
}
