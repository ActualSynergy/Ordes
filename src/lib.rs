// Ordes - Orbital Design Suite
// Main library entry point

pub mod modules {
    pub mod satellite;
    pub mod components;
    pub mod simulation;
    pub mod collaboration;
    pub mod rendering;
    pub mod shared;
}

pub mod config;

// Re-export commonly used items
pub use modules::{
    satellite::*,
    components::*,
    simulation::*,
    collaboration::*,
    rendering::*,
    shared::*,
};
