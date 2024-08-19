use nalgebra::Vector2;

#[derive(Debug, Clone, Copy)]
pub struct Light {
    pub position: Vector2<f32>,
    pub radius: f32,
    pub intensity: f32,
}

impl Light {
    pub fn new(position: Vector2<f32>) -> Self {
        Self {
            position,
            radius: 400.0,
            intensity: 1.0,
        }
    }
}
