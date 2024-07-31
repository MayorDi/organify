use nalgebra::Vector2;

use crate::consts::RADIUS_WORLD;

#[derive(Debug)]
pub struct World {
    position: Vector2<f32>,
    radius: f32,
}

impl World {
    pub fn new(position: Vector2<f32>) -> Self {
        Self {
            position,
            radius: RADIUS_WORLD
        }
    }
}


