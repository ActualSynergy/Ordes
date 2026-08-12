// Merge strategy for conflicting changes

use serde_json::Value;

pub enum MergeStrategy {
    Ours,
    Theirs,
    Manual,
}

pub struct MergeConflict {
    pub field: String,
    pub ours: Value,
    pub theirs: Value,
}

pub struct MergeEngine;

impl MergeEngine {
    pub fn merge(
        base: &Value,
        ours: &Value,
        theirs: &Value,
        strategy: MergeStrategy,
    ) -> Result<Value, Vec<MergeConflict>> {
        // TODO: Implement merge algorithm with conflict detection
        match strategy {
            MergeStrategy::Ours => Ok(ours.clone()),
            MergeStrategy::Theirs => Ok(theirs.clone()),
            MergeStrategy::Manual => Err(vec![]),
        }
    }
}
