// Collision detection

use crate::modules::simulation::physics::Vector3;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollisionDetection {
    pub components: Vec<ComponentBoundingBox>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentBoundingBox {
    pub component_id: String,
    pub position: Vector3,
    pub dimensions: (f64, f64, f64),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollisionResult {
    pub has_collisions: bool,
    pub collision_pairs: Vec<(String, String)>,
}

impl CollisionDetection {
    pub fn new() -> Self {
        Self {
            components: Vec::new(),
        }
    }

    pub fn check_collisions(&self) -> CollisionResult {
        let mut collision_pairs = Vec::new();

        for i in 0..self.components.len() {
            for j in (i + 1)..self.components.len() {
                if self.aabb_intersect(&self.components[i], &self.components[j]) {
                    collision_pairs.push((
                        self.components[i].component_id.clone(),
                        self.components[j].component_id.clone(),
                    ));
                }
            }
        }

        CollisionResult {
            has_collisions: !collision_pairs.is_empty(),
            collision_pairs,
        }
    }

    fn aabb_intersect(&self, a: &ComponentBoundingBox, b: &ComponentBoundingBox) -> bool {
        let a_min_x = a.position.x - a.dimensions.0 / 2.0;
        let a_max_x = a.position.x + a.dimensions.0 / 2.0;
        let a_min_y = a.position.y - a.dimensions.1 / 2.0;
        let a_max_y = a.position.y + a.dimensions.1 / 2.0;
        let a_min_z = a.position.z - a.dimensions.2 / 2.0;
        let a_max_z = a.position.z + a.dimensions.2 / 2.0;

        let b_min_x = b.position.x - b.dimensions.0 / 2.0;
        let b_max_x = b.position.x + b.dimensions.0 / 2.0;
        let b_min_y = b.position.y - b.dimensions.1 / 2.0;
        let b_max_y = b.position.y + b.dimensions.1 / 2.0;
        let b_min_z = b.position.z - b.dimensions.2 / 2.0;
        let b_max_z = b.position.z + b.dimensions.2 / 2.0;

        a_min_x <= b_max_x && a_max_x >= b_min_x &&
        a_min_y <= b_max_y && a_max_y >= b_min_y &&
        a_min_z <= b_max_z && a_max_z >= b_min_z
    }
}
