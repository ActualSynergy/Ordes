// Numerical solvers for differential equations

use super::vector_math::Vector3;

/// Runge-Kutta 4th order integrator for orbital mechanics
pub struct RK4Integrator;

impl RK4Integrator {
    /// Integrate a system of ODEs using RK4 method
    pub fn step<F>(
        position: &Vector3,
        velocity: &Vector3,
        time: f64,
        dt: f64,
        mut f: F,
    ) -> (Vector3, Vector3)
    where
        F: FnMut(&Vector3, &Vector3, f64) -> Vector3,
    {
        let k1_v = f(position, velocity, time);
        let k1_p = *velocity;

        let p2 = *position + (k1_p * (dt / 2.0));
        let v2 = *velocity + (k1_v * (dt / 2.0));
        let k2_v = f(&p2, &v2, time + dt / 2.0);
        let k2_p = v2;

        let p3 = *position + (k2_p * (dt / 2.0));
        let v3 = *velocity + (k2_v * (dt / 2.0));
        let k3_v = f(&p3, &v3, time + dt / 2.0);
        let k3_p = v3;

        let p4 = *position + (k3_p * dt);
        let v4 = *velocity + (k3_v * dt);
        let k4_v = f(&p4, &v4, time + dt);
        let k4_p = v4;

        let new_position = *position
            + ((k1_p + (k2_p * 2.0) + (k3_p * 2.0) + k4_p) * (dt / 6.0));
        let new_velocity = *velocity
            + ((k1_v + (k2_v * 2.0) + (k3_v * 2.0) + k4_v) * (dt / 6.0));

        (new_position, new_velocity)
    }
}
