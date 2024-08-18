use glfw::MouseButton;
use nalgebra::Vector2;

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct Camera {
    pub position: Vector2<f32>,
    pub scale: f32,
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            position: Default::default(),
            scale: 1.0,
        }
    }
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct Mouse {
    pub world_position: Vector2<f32>,
    pub position: Vector2<f32>,
    pub old_position: Vector2<f32>,
    pub button: MouseButton,
    pub pressed: bool,
}

impl Mouse {
    pub fn update_world_position(&mut self, current_window_size: (i32, i32), camera: &Camera) {
        let size = (current_window_size.0 as f32, current_window_size.1 as f32);
        let world_mouse_pos = Vector2::new(
            camera.position.x + (self.position.x * 2.0 - size.0) / camera.scale,
            camera.position.y + (-self.position.y * 2.0 + size.1) / camera.scale,
        );

        self.world_position = world_mouse_pos;
    }

    pub fn delta(&self) -> Vector2<f32> {
        self.position - self.old_position
    }
}

impl Default for Mouse {
    fn default() -> Self {
        Self {
            world_position: Default::default(),
            position: Default::default(),
            old_position: Default::default(),
            button: MouseButton::Button1,
            pressed: false,
        }
    }
}
