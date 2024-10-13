use nalgebra::Vector2;

#[derive(Debug, Clone, Default)]
pub struct Camera {
    pub position: Vector2<f32>,
    pub scale: f32,
}
