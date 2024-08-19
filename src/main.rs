use std::{cell::RefCell, rc::Rc};

use glfw::{Action, Context, Key, MouseButton};
use nalgebra::Vector2;
use organify::{
    cell::Cell,
    control::{Camera, Mouse},
    grid::Grid,
    traits::{Behavior, Render},
    world::World,
};

use egui::{vec2, Pos2, Rect};
use egui_glfw as egui_backend;
use rand::Rng;

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

    let mut time = 0.0;

    let camera = Rc::new(RefCell::new(Camera::default()));
    let mut mouse = Mouse::default();

    #[cfg(not(feature = "debug"))]
    let mut grid = Grid::new();

    let mut cells = vec![Cell::new(Vector2::new(50.0, 50.0))];
    let rd_cells = Cell::render_init(Some(Rc::clone(&camera)));

    let mut world = World::new(Vector2::new(0.0, 0.0));
    world.render_init();
    world.render_data.camera = Some(Rc::clone(&camera));

    #[cfg(feature = "debug")]
    let mut grid = Grid::new(world.position, world.radius);

    #[cfg(feature = "debug")]
    grid.render_init();
    #[cfg(feature = "debug")]
    {
        grid.render_data.camera = Some(Rc::clone(&camera));
    }

    unsafe {
        gl::Enable(gl::BLEND);
        gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
    }

    #[cfg(feature = "log")]
    log::info!("Run the main loop");
    while !window.should_close() {
        glfw.poll_events();
        egui_ctx.begin_frame(egui_input_state.input.take());

        grid.update_cells(&cells);
        grid.find_collisions_grid(&mut cells);

        for cell in cells.iter_mut() {
            cell.update();
        }

        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
            gl::ClearColor(0.1, 0.1, 0.1, 1.0);

            world.render();

            #[cfg(feature = "debug")]
            grid.render();

            for cell in cells.iter() {
                cell.render(&rd_cells, time);
            }
        }

        let camera = &mut *(*camera).borrow_mut();

        ui_render(
            &egui_ctx,
            time,
            camera,
            &mouse,
            &world,
            &cells,
            painter.clone(),
            &mut egui_input_state,
            native_pixels_per_point,
        );

        for (_, event) in glfw::flush_messages(&events) {
            egui_backend::handle_event(event.clone(), &mut egui_input_state);

            mouse.update_world_position(window.get_size(), &camera);
            match event {
                glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                    window.set_should_close(true);
                }

                glfw::WindowEvent::MouseButton(button, action, _) => {
                    mouse.button = button;

                    match action {
                        Action::Press => mouse.pressed = true,
                        _ => mouse.pressed = false,
                    }

                    match mouse.button {
                        MouseButton::Button2 if mouse.pressed => {
                            cells.push(Cell::new(mouse.world_position));
                        }
                        _ => {}
                    }
                }

                glfw::WindowEvent::Scroll(_, y) => {
                    if (camera.scale + y as f32) > 0.0 {
                        camera.scale += y as f32;
                    }
                }

                glfw::WindowEvent::CursorPos(x, y) => {
                    mouse.old_position = mouse.position;
                    mouse.position = Vector2::new(x as f32, y as f32);

                    match mouse.button {
                        glfw::MouseButton::Button1 if mouse.pressed => {
                            camera.position +=
                                Vector2::new(-mouse.delta().x, mouse.delta().y) / camera.scale
                        }
                        _ => {}
                    }
                }
                glfw::WindowEvent::Close => window.set_should_close(true),
                _ => {}
            }
        }
        window.swap_buffers();
        time += 0.05;
    }
    #[cfg(feature = "log")]
    log::info!("End the main loop");
}

fn ui_render(
    egui_ctx: &egui::Context,
    time: f32,
    camera: &Camera,
    mouse: &Mouse,
    world: &World,
    cells: &Vec<Cell>,

    painter: Rc<RefCell<egui_backend::Painter>>,
    egui_input_state: &mut egui_glfw::EguiInputState,
    native_pixels_per_point: f32,
) {
    egui::Window::new("Info").show(&egui_ctx, |ui| {
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
                camera.position.x, camera.position.y
            )
            .as_str(),
        );
        ui.label(format!("Camera scale: {:.1}", camera.scale).as_str());
        ui.separator();
        ui.label(
            format!(
                "World position: (x: {:.2}, y: {:.2})",
                world.position.x, world.position.y
            )
            .as_str(),
        );
        ui.label(format!("World radius: {:.2}", world.radius).as_str());
    });

    let egui::FullOutput {
        platform_output,
        textures_delta,
        shapes,
        ..
    } = egui_ctx.end_frame();

    //Handle cut, copy text from egui
    if !platform_output.copied_text.is_empty() {
        egui_backend::copy_to_clipboard(egui_input_state, platform_output.copied_text);
    }

    let clipped_shapes = egui_ctx.tessellate(shapes, native_pixels_per_point);
    (*painter).borrow_mut().paint_and_update_textures(
        native_pixels_per_point,
        &clipped_shapes,
        &textures_delta,
    );
}
