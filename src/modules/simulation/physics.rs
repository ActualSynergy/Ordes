// Physics calculations for orbital mechanics

const EARTH_RADIUS: f64 = 6371.0; // km
const GRAVITATIONAL_CONSTANT: f64 = 6.674e-11; // m^3 kg^-1 s^-2
const EARTH_MASS: f64 = 5.972e24; // kg

pub struct OrbitalMechanics;

impl OrbitalMechanics {
    /// Calculate orbital period from altitude
    pub fn calculate_period(altitude_km: f64) -> f64 {
        let semi_major_axis = EARTH_RADIUS + altitude_km;
        let mu = GRAVITATIONAL_CONSTANT * EARTH_MASS / 1e9; // Convert to km^3/s^2
        
        let period_seconds = 2.0 * std::f64::consts::PI * (semi_major_axis.powi(3) / mu).sqrt();
        period_seconds / 60.0 // Convert to minutes
    }

    /// Calculate orbital velocity
    pub fn calculate_velocity(altitude_km: f64) -> f64 {
        let orbital_radius = EARTH_RADIUS + altitude_km;
        let mu = GRAVITATIONAL_CONSTANT * EARTH_MASS / 1e9;
        
        (mu / orbital_radius).sqrt()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_orbital_period() {
        // ISS altitude is approximately 408 km
        let period = OrbitalMechanics::calculate_period(408.0);
        // Period should be around 92 minutes
        assert!((period - 92.0).abs() < 2.0);
    }
}
