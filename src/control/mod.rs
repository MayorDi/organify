use glfw::MouseButton;
use nalgebra::Vector2;

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct Camera {
    pub position: Vector2<f32>,
    pub scale: f32
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            position: Default::default(),
            scale: 1.0
        }
    }
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct Mouse {
    pub position: Vector2<f32>,
    pub old_position: Vector2<f32>,
    pub button: MouseButton,
    pub pressed: bool,
}

impl Mouse {
    pub fn delta(&self) -> Vector2<f32> {
        self.position - self.old_position
    }
}

impl Default for Mouse {
    fn default() -> Self {
        Self {
            position: Default::default(),
            old_position: Default::default(),
            button: MouseButton::Button1,
            pressed: false,
        }
    }
}
