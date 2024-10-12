use nalgebra::Vector2;

pub trait Behavior {
    fn update(&mut self);
}

pub trait Physics {
    fn mass(&self) -> f32;
    fn position(&self) -> Vector2<f32>;
}
