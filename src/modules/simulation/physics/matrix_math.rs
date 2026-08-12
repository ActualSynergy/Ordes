// Matrix mathematics for transformations and linear algebra

use super::vector_math::Vector3;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Matrix3x3 {
    pub data: [[f64; 3]; 3],
}

impl Matrix3x3 {
    pub fn new(data: [[f64; 3]; 3]) -> Self {
        Self { data }
    }

    pub fn identity() -> Self {
        Self {
            data: [
                [1.0, 0.0, 0.0],
                [0.0, 1.0, 0.0],
                [0.0, 0.0, 1.0],
            ],
        }
    }

    pub fn zero() -> Self {
        Self {
            data: [
                [0.0, 0.0, 0.0],
                [0.0, 0.0, 0.0],
                [0.0, 0.0, 0.0],
            ],
        }
    }

    pub fn multiply_vector(&self, v: &Vector3) -> Vector3 {
        Vector3::new(
            self.data[0][0] * v.x + self.data[0][1] * v.y + self.data[0][2] * v.z,
            self.data[1][0] * v.x + self.data[1][1] * v.y + self.data[1][2] * v.z,
            self.data[2][0] * v.x + self.data[2][1] * v.y + self.data[2][2] * v.z,
        )
    }
}
