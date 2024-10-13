use nalgebra::Vector2;

#[derive(Debug)]
pub struct World {
    position: Vector2<f32>,
    radius: f32,
}

impl World {
    pub fn new(position: Vector2<f32>, radius: f32) -> Self {
        Self {
            position,
            radius
        }
    }
}
