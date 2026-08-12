// Ordes - Orbital Design Suite
// Main library entry point

pub mod modules {
    pub mod entities;
    pub mod components;
    pub mod assembly;
    pub mod simulation;
    pub mod collaboration;
    pub mod rendering;
    pub mod shared;
}

pub mod config;
pub mod layers;

// Re-export commonly used items
pub use modules::{
    entities::*,
    components::*,
    assembly::*,
    simulation::*,
    collaboration::*,
    rendering::*,
    shared::*,
};
