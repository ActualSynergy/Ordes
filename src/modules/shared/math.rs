// Mathematical utilities for orbital mechanics and physics

use std::f64::consts::PI;

pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * (PI / 180.0)
}

pub fn radians_to_degrees(radians: f64) -> f64 {
    radians * (180.0 / PI)
}

pub fn distance_3d(x1: f64, y1: f64, z1: f64, x2: f64, y2: f64, z2: f64) -> f64 {
    ((x2 - x1).powi(2) + (y2 - y1).powi(2) + (z2 - z1).powi(2)).sqrt()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_degrees_to_radians() {
        assert!((degrees_to_radians(180.0) - PI).abs() < 1e-10);
    }
}
