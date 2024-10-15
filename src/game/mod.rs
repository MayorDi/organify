use components::{EguiComponents, WindowComponents};
use egui_glfw::{self as egui_backend};

use egui_backend::egui::{self, vec2, Pos2, Rect};
use egui_glfw::glfw::Context;
use glfw::Window;

mod components;

pub struct Game {
    window_components: WindowComponents,
    egui_components: EguiComponents,
}

impl Game {
    pub fn init() -> Self {
        let mut wc = Self::init_window_components();

        gl::load_with(|symbol| wc.window.get_proc_address(symbol) as *const _);
        wc.window.set_all_polling(true);
        wc.window.make_current();

        let egui_c = Self::init_gui_components(&mut wc.window);

        Self {
            window_components: wc,
            egui_components: egui_c,
        }
    }

    fn init_window_components() -> WindowComponents {
        let mut glfw = glfw::init(glfw::fail_on_errors).unwrap();
        glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
        glfw.window_hint(glfw::WindowHint::OpenGlProfile(
            glfw::OpenGlProfileHint::Core,
        ));
        glfw.window_hint(glfw::WindowHint::DoubleBuffer(true));
        glfw.window_hint(glfw::WindowHint::Resizable(true));

        let (window, events) = glfw
            .create_window(600, 600, "Organify", glfw::WindowMode::Windowed)
            .expect("Failed to create GLFW window.");

        WindowComponents {
            glfw,
            window,
            events,
        }
    }

    fn init_gui_components(window: &mut Window) -> EguiComponents {
        let painter = egui_backend::Painter::new(window);
        let egui_ctx = egui::Context::default();

        let (width, height) = window.get_framebuffer_size();
        let native_pixels_per_point = window.get_content_scale().0;

        let mut egui_input_state = egui_backend::EguiInputState::new(egui::RawInput {
            screen_rect: Some(Rect::from_min_size(
                Pos2::new(0f32, 0f32),
                vec2(width as f32, height as f32) / native_pixels_per_point,
            )),
            ..Default::default()
        });

        egui_input_state.input.time = Some(0.01);

        EguiComponents {
            painter,
            egui_ctx,
            native_pixels_per_point,
            egui_input_state,
        }
    }

    pub fn run(self) {
        let WindowComponents {
            mut glfw,
            mut window,
            events,
        } = self.window_components;

        let mut egui_components = self.egui_components;

        while !window.should_close() {
            glfw.poll_events();

            for (_, event) in glfw::flush_messages(&events) {
                match event {
                    glfw::WindowEvent::Close => window.set_should_close(true),
                    _ => {
                        egui_backend::handle_event(event, &mut egui_components.egui_input_state);
                    }
                }
            }

            unsafe {
                gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
                gl::ClearColor(0.1, 0.1, 0.1, 1.0);
            }

            Self::render_ui(&mut egui_components);

            window.swap_buffers();
        }
    }

    fn create_ui(ctx: &egui::Context) {
        egui::Window::new("Egui with GLFW").show(ctx, |ui| {
            ui.label("Hello!");
        });
    }

    fn render_ui(egui_components: &mut EguiComponents) {
        let EguiComponents {
            egui_ctx,
            painter,
            native_pixels_per_point,
            egui_input_state,
        } = egui_components;

        egui_ctx.begin_frame(egui_input_state.input.take());

        Self::create_ui(egui_ctx);

        let egui::FullOutput {
            platform_output,
            textures_delta,
            shapes,
            ..
        } = egui_ctx.end_frame();

        if !platform_output.copied_text.is_empty() {
            egui_backend::copy_to_clipboard(egui_input_state, platform_output.copied_text);
        }

        let clipped_shapes = egui_ctx.tessellate(shapes, *native_pixels_per_point);
        painter.paint_and_update_textures(
            *native_pixels_per_point,
            &clipped_shapes,
            &textures_delta,
        );
    }
}
