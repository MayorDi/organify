use std::{cell::RefCell, rc::Rc};

use egui::Context;

use crate::{
    cell::Cell,
    control::{Camera, Mouse, Tool},
    world::World,
};

pub struct Info {
    camera: Rc<RefCell<Camera>>,
    mouse: Rc<RefCell<Mouse>>,
    world: Rc<RefCell<World>>,
    cells: Rc<RefCell<Vec<Cell>>>,
}

impl Info {
    pub fn new(
        camera: Rc<RefCell<Camera>>,
        mouse: Rc<RefCell<Mouse>>,
        world: Rc<RefCell<World>>,
        cells: Rc<RefCell<Vec<Cell>>>,
    ) -> Self {
        Self {
            camera,
            mouse,
            world,
            cells,
        }
    }

    pub fn ui_render(&self, ctx: &Context, time: f32) {
        let mouse = &mut *(*self.mouse).borrow_mut();
        let world = &mut *(*self.world).borrow_mut();
        let cells = &mut *(*self.cells).borrow_mut();

        egui::Window::new("Info").show(ctx, |ui| {
            ui.label(format!("Time: {:.2}", time).as_str());
            ui.label(format!("Count cells: {}", cells.len()).as_str());
            ui.label(
                format!(
                    "Mouse world position: (x: {:.2}, y: {:.2})",
                    mouse.world_position.x, mouse.world_position.y
                )
                .as_str(),
            );
            ui.label(
                format!(
                    "Camera position: (x: {:.2}, y: {:.2})",
                    self.camera.borrow().position.x,
                    self.camera.borrow().position.y
                )
                .as_str(),
            );
            ui.label(format!("Camera scale: {:.1}", self.camera.borrow().scale).as_str());
            ui.label(
                format!(
                    "World position: (x: {:.2}, y: {:.2})",
                    world.position.x, world.position.y
                )
                .as_str(),
            );
            ui.label(format!("World radius: {:.2}", world.radius).as_str());
        });
    }
}

pub struct Tools {
    tool: Rc<RefCell<Tool>>,
}

impl Tools {
    pub fn new(tool: Rc<RefCell<Tool>>) -> Self {
        Self { tool }
    }

    pub fn ui_render(&self, ctx: &Context) {
        egui::Window::new("Tools").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label("The current tool:");
                ui.code(self.tool.borrow().to_string());
            });
            ui.horizontal(|ui| {
                if ui.button("Move cell").clicked() {
                    *(*self.tool).borrow_mut() = Tool::MoveCell;
                }

                if ui.button("Add cell").clicked() {
                    *(*self.tool).borrow_mut() = Tool::AddCell;
                }

                if ui.button("Select cell").clicked() {
                    *(*self.tool).borrow_mut() = Tool::SelectCell;
                }

                if ui.button("None").clicked() {
                    *(*self.tool).borrow_mut() = Tool::None;
                }
            });
        });
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct UiView {
    pub info_window: bool,
    pub tools_window: bool,
}

pub struct Menu {
    pub ui_view: UiView,
}

impl Menu {
    pub fn new(ui_view: UiView) -> Self {
        Self { ui_view }
    }

    pub fn ui_render(&mut self, ctx: &Context) {
        egui::TopBottomPanel::new(egui::containers::panel::TopBottomSide::Top, "top_panel").show(
            ctx,
            |ui| {
                egui::menu::bar(ui, |ui| {
                    ui.menu_button("File", |ui| {
                        if ui.button("Open").clicked() {
                            unimplemented!()
                        }

                        if ui.button("Save").clicked() {
                            unimplemented!()
                        }
                    });

                    ui.menu_button("View", |ui| {
                        ui.checkbox(&mut self.ui_view.info_window, "Info window");
                        ui.checkbox(&mut self.ui_view.tools_window, "Tools window");
                    });
                });
            },
        );
    }
}
