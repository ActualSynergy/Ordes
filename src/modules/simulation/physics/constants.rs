// Physical constants used throughout simulations

// Gravitational constants
pub const GRAVITATIONAL_CONSTANT: f64 = 6.67430e-11;  // m^3 kg^-1 s^-2
pub const EARTH_MASS: f64 = 5.972e24;                 // kg
pub const EARTH_RADIUS: f64 = 6.371e6;               // m
pub const EARTH_RADIUS_KM: f64 = 6371.0;             // km
pub const EARTH_ROTATION_PERIOD: f64 = 86164.0905;   // s (sidereal day)

// Sun constants
pub const SUN_MASS: f64 = 1.989e30;                  // kg
pub const SUN_RADIUS: f64 = 6.96e8;                  // m
pub const SOLAR_CONSTANT: f64 = 1361.0;             // W/m^2 at Earth distance
pub const AU: f64 = 1.496e11;                        // m (Astronomical Unit)

// Moon constants
pub const MOON_MASS: f64 = 7.342e22;                 // kg
pub const MOON_RADIUS: f64 = 1.737e6;               // m
pub const EARTH_MOON_DISTANCE: f64 = 3.844e8;       // m

// Atmosphere
pub const EARTH_SCALE_HEIGHT: f64 = 8500.0;         // m (exponential atmosphere model)
pub const SEA_LEVEL_DENSITY: f64 = 1.225;           // kg/m^3

// Thermal constants
pub const STEFAN_BOLTZMANN: f64 = 5.670374419e-8;   // W m^-2 K^-4
pub const BOLTZMANN_CONSTANT: f64 = 1.380649e-23;   // J K^-1

// Electromagnetic constants
pub const SPEED_OF_LIGHT: f64 = 2.99792458e8;       // m/s
pub const PERMITTIVITY_VACUUM: f64 = 8.8541878128e-12;  // F/m
pub const PERMEABILITY_VACUUM: f64 = 1.25663706212e-6;  // H/m

// Orbital element constants
pub const GM_EARTH: f64 = GRAVITATIONAL_CONSTANT * EARTH_MASS;  // m^3/s^2

// Time constants
pub const J2000_EPOCH: f64 = 2451545.0;             // JD2000
pub const SECONDS_PER_DAY: f64 = 86400.0;
pub const SECONDS_PER_HOUR: f64 = 3600.0;
pub const SECONDS_PER_MINUTE: f64 = 60.0;

// Unit conversions
pub const DEG_TO_RAD: f64 = std::f64::consts::PI / 180.0;
pub const RAD_TO_DEG: f64 = 180.0 / std::f64::consts::PI;

// J2 perturbation (Earth oblateness)
pub const EARTH_J2: f64 = 1.08262668e-3;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_constants_sanity() {
        assert!(GRAVITATIONAL_CONSTANT > 0.0);
        assert!(EARTH_MASS > 0.0);
        assert!(GM_EARTH > 0.0);
        assert!(SOLAR_CONSTANT > 1000.0);
    }
}
