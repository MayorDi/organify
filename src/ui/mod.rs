use debug_tools::DebugWindow;
use egui::{vec2, Context, Pos2, Rect};
use egui_glfw::{self as egui_backend, EguiInputState, Painter};
use glfw::{PWindow, WindowEvent};
use std::{cell::RefCell, rc::Rc};

pub mod debug_tools;

use crate::{
    cell::Cell, control::{Camera, Mouse, Tool}, idx_obj_vec::IdxObjVec, world::World
};

pub struct Info {
    camera: Rc<RefCell<Camera>>,
    mouse: Rc<RefCell<Mouse>>,
    world: Rc<RefCell<World>>,
    cells: Rc<RefCell<IdxObjVec<Cell>>>,
}

impl Info {
    pub fn new(
        camera: Rc<RefCell<Camera>>,
        mouse: Rc<RefCell<Mouse>>,
        world: Rc<RefCell<World>>,
        cells: Rc<RefCell<IdxObjVec<Cell>>>,
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
            ui.label(format!("Count cells: {}", cells.count_objects() - cells.count_idxs()).as_str());
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
    pub debug_window: bool,
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
                        ui.checkbox(&mut self.ui_view.debug_window, "Debug window");
                    });
                });
            },
        );
    }
}

pub struct MetaDataRender {
    pub painter: Rc<RefCell<Painter>>,
    pub ctx: Context,
    pub egui_input_state: EguiInputState,
    pub native_pixels_per_point: f32,
}

impl MetaDataRender {
    pub fn event_handler(&mut self, event: WindowEvent) {
        egui_backend::handle_event(event, &mut self.egui_input_state);
    }

    pub fn begin_frame(&mut self) {
        self.ctx.begin_frame(self.egui_input_state.input.take());
    }
}

pub fn init_egui_ctx(window: &mut PWindow) -> MetaDataRender {
    let painter = Rc::new(RefCell::new(egui_backend::Painter::new(window)));
    let p = painter.clone();
    window.set_framebuffer_size_callback(move |_, w, h| unsafe {
        gl::Viewport(0, 0, w, h);
        (*p).borrow_mut().canvas_width = w as u32;
        (*p).borrow_mut().canvas_height = h as u32;
    });
    let egui_ctx = egui::Context::default();

    let (width, height) = window.get_framebuffer_size();
    let native_pixels_per_point = window.get_content_scale().0;

    let egui_input_state = egui_backend::EguiInputState::new(egui::RawInput {
        screen_rect: Some(Rect::from_min_size(
            Pos2::new(0f32, 0f32),
            vec2(width as f32, height as f32) / native_pixels_per_point,
        )),

        ..Default::default()
    });

    MetaDataRender {
        painter,
        ctx: egui_ctx,
        egui_input_state,
        native_pixels_per_point,
    }
}

pub fn ui_render(
    menu: &mut Menu,
    info: &Info,
    tools: &Tools,
    debug_window: &mut DebugWindow,
    time: f32,
    meta_data_render: &mut MetaDataRender,
) {
    menu.ui_render(&meta_data_render.ctx);
    if menu.ui_view.info_window {
        info.ui_render(&meta_data_render.ctx, time);
    }

    if menu.ui_view.tools_window {
        tools.ui_render(&meta_data_render.ctx);
    }

    if menu.ui_view.debug_window {
        debug_window.ui_render(&meta_data_render.ctx);
    }

    let egui::FullOutput {
        platform_output,
        textures_delta,
        shapes,
        ..
    } = meta_data_render.ctx.end_frame();

    //Handle cut, copy text from egui
    if !platform_output.copied_text.is_empty() {
        egui_backend::copy_to_clipboard(
            &mut meta_data_render.egui_input_state,
            platform_output.copied_text,
        );
    }

    let clipped_shapes = meta_data_render
        .ctx
        .tessellate(shapes, meta_data_render.native_pixels_per_point);
    (*meta_data_render.painter)
        .borrow_mut()
        .paint_and_update_textures(
            meta_data_render.native_pixels_per_point,
            &clipped_shapes,
            &textures_delta,
        );
}
