use egui::Context;

#[derive(Debug)]
pub struct DebugWindow {
    pub direction_cells: bool,
    pub view_grid: bool,
}

impl DebugWindow {
    pub fn new() -> Self {
        Self {
            direction_cells: false,
            view_grid: false,
        }
    }

    pub fn ui_render(&mut self, ctx: &Context) {
        egui::Window::new("Debug").show(ctx, |ui| {
            ui.checkbox(&mut self.direction_cells, "Direction cells");
            ui.checkbox(&mut self.view_grid, "View grid");
        });
    }
}
