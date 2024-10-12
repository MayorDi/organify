use nalgebra::Vector2;

#[derive(Debug, Clone)]
pub struct Cell {
    // Physical properties
    position: Vector2<f32>,
    velocity: Vector2<f32>,
    mass: f32,
}
