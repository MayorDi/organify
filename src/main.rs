use std::{cell::RefCell, rc::Rc, time::Instant};

use glfw::Context;
use nalgebra::Vector2;
use organify::{grid::Grid, traits::Render, world::World};

use egui::{vec2, Color32, Image, Pos2, Rect};
use egui_glfw as egui_backend;

fn main() {
    let mut glfw = glfw::init(glfw::fail_on_errors).unwrap();

    #[cfg(feature = "log")]
    {
        std::env::set_var("RUST_LOG", "INFO");
        env_logger::init();
        log::info!("The log init");
    }

    glfw.window_hint(glfw::WindowHint::ContextVersionMajor(3));
    glfw.window_hint(glfw::WindowHint::ContextVersionMinor(3));
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(
        glfw::OpenGlProfileHint::Core,
    ));
    glfw.window_hint(glfw::WindowHint::DoubleBuffer(true));
    #[cfg(feature = "log")]
    log::info!("OpenGL 3.3 core");

    let (mut window, events) = glfw
        .create_window(1200, 600, "Organify", glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window.");
    #[cfg(feature = "log")]
    log::info!("Window init");

    window.set_all_polling(true);
    window.make_current();

    gl::load_with(|s| window.get_proc_address(s));
    #[cfg(feature = "log")]
    log::info!("load OpenGL functions");
    glfw.set_swap_interval(glfw::SwapInterval::Sync(1));

    let painter = Rc::new(RefCell::new(egui_backend::Painter::new(&mut window)));
    let p = painter.clone();
    window.set_framebuffer_size_callback(move |_, w, h| unsafe {
        gl::Viewport(0, 0, w, h);
        (*p).borrow_mut().canvas_width = w as u32;
        (*p).borrow_mut().canvas_height = h as u32;
    });
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

    // egui_input_state.input.time = Some(0.01);

    let mut world = World::new(Vector2::new(0.0, 0.0));
    world.render_init();

    unsafe {
        gl::Enable(gl::BLEND);
        gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
    }

    #[cfg(feature = "log")]
    log::info!("Run the main loop");
    while !window.should_close() {
        glfw.poll_events();
        egui_ctx.begin_frame(egui_input_state.input.take());
        
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
            gl::ClearColor(0.1, 0.1, 0.1, 1.0);

            world.render();
        }

        egui::Window::new("Egui in Organify").show(&egui_ctx, |ui|{
            ui.label("Hello Organify");
        });
        

        let egui::FullOutput {
            platform_output,
            textures_delta,
            shapes, .. } = egui_ctx.end_frame();

        //Handle cut, copy text from egui
        if !platform_output.copied_text.is_empty() {
            egui_backend::copy_to_clipboard(&mut egui_input_state, platform_output.copied_text);
        }

        let clipped_shapes = egui_ctx.tessellate(shapes, native_pixels_per_point);
        (*painter).borrow_mut().paint_and_update_textures(native_pixels_per_point, &clipped_shapes, &textures_delta);

        for (_, event) in glfw::flush_messages(&events) {
            match event {
                glfw::WindowEvent::Close => window.set_should_close(true),
                _ => {
                    egui_backend::handle_event(event, &mut egui_input_state);
                }
            }
        }
        window.swap_buffers();
    }
    #[cfg(feature = "log")]
    log::info!("End the main loop");
}
