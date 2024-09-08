use egui::Context;

#[derive(Debug)]
pub struct DebugWindow {
    direction_cells: bool,
}

impl DebugWindow {
    pub fn new() -> Self {
        Self {
            direction_cells: false,
        }
    }

    pub fn ui_render(&mut self, ctx: &Context) {
        egui::Window::new("Debug").show(ctx, |ui| {
            ui.checkbox(&mut self.direction_cells, "Direction cells");
        });
    }
}
