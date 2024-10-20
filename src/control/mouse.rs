use nalgebra::Vector2;

#[derive(Debug, Clone, Default)]
pub struct Mouse {
    pub action: Action,
    pub mouse_button: MouseButton,
    position: Vector2<f32>,
    old_position: Vector2<f32>,
}

impl Mouse {
    pub fn delta(&self) -> Vector2<f32> {
        self.position - self.old_position
    }

    pub fn update_position(&mut self, new_position: Vector2<f32>) {
        self.old_position = self.position;
        self.position = new_position;
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub enum Action {
    Release,
    Press,
    #[default]
    None,
}

#[derive(Debug, Clone, Copy, Default)]
pub enum MouseButton {
    #[default]
    Button1,
    Button2,
    Button3,
}
