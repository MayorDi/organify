use std::{cell::RefCell, rc::Rc};

use glfw::{Action, Context, Key, MouseButton};
use nalgebra::Vector2;
use organify::{
    cell::Cell,
    control::{Camera, Mouse, Tool},
    grid::Grid,
    traits::{Behavior, Render},
    ui::{ui_render, Info, Menu, Tools, UiView},
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

    // --------------------------------
    let mut time = 0.0;

    let camera = Rc::new(RefCell::new(Camera::default()));
    let mouse = Rc::new(RefCell::new(Mouse::default()));

    #[cfg(not(feature = "debug"))]
    let mut grid = Grid::new();

    let mut cells = vec![];
    for _ in 0..1000 {
        cells.push(Cell::new(Vector2::new(
            rand::thread_rng().gen_range(-100.0..100.0),
            rand::thread_rng().gen_range(-100.0..100.0),
        )));
    }
    let cells = Rc::new(RefCell::new(cells));

    let rd_cells = Cell::render_init(Some(Rc::clone(&camera)));

    let mut world = World::new(Vector2::new(0.0, 0.0));
    world.render_init();
    world.render_data.camera = Some(Rc::clone(&camera));
    let world = Rc::new(RefCell::new(world));

    let tool = Rc::new(RefCell::new(Tool::None));

    #[cfg(feature = "debug")]
    let mut grid = Grid::new(world.position, world.radius);

    #[cfg(feature = "debug")]
    grid.render_init();
    #[cfg(feature = "debug")]
    {
        grid.render_data.camera = Some(Rc::clone(&camera));
    }

    // ui init data

    let info = Info::new(camera.clone(), mouse.clone(), world.clone(), cells.clone());
    let tools = Tools::new(tool.clone());
    let mut menu = Menu::new(UiView::default());

    // ---

    unsafe {
        gl::Enable(gl::BLEND);
        gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
    }

    #[cfg(feature = "log")]
    log::info!("Run the main loop");
    while !window.should_close() {
        glfw.poll_events();
        egui_ctx.begin_frame(egui_input_state.input.take());

        {
            let cells = &mut *(*cells).borrow_mut();

            grid.update_cells(&cells);
            grid.find_collisions_grid(cells);

            {
                let mut len = cells.len();
                let mut i = 0;
                while i < len {
                    cells[i].update();
                    cells[i].check_alive();

                    if !cells[i].is_alive {
                        cells.remove(i);
                        len -= 1;
                        continue;
                    }

                    i += 1;
                }
            }
        }

        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
            gl::ClearColor(0.1, 0.1, 0.1, 1.0);

            (*world).borrow().render();

            #[cfg(feature = "debug")]
            grid.render();

            let cells = &*(*cells).borrow();
            Cell::render(cells, &rd_cells, time);
        }

        ui_render(
            &egui_ctx,
            &mut menu,
            &info,
            &tools,
            time,
            painter.clone(),
            &mut egui_input_state,
            native_pixels_per_point,
        );

        for (_, event) in glfw::flush_messages(&events) {
            egui_backend::handle_event(event.clone(), &mut egui_input_state);
            let mouse = &mut *(*mouse).borrow_mut();

            mouse.update_world_position(window.get_size(), &*camera.borrow());
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
                        MouseButton::Button2
                            if mouse.pressed && *tool.borrow() == Tool::AddCell =>
                        {
                            (*cells).borrow_mut().push(Cell::new(mouse.world_position));
                        }
                        _ => {}
                    }
                }

                glfw::WindowEvent::Scroll(_, y) => {
                    if (camera.borrow().scale + y as f32) > 0.0 {
                        camera.borrow_mut().scale += y as f32;
                    }
                }

                glfw::WindowEvent::CursorPos(x, y) => {
                    mouse.old_position = mouse.position;
                    mouse.position = Vector2::new(x as f32, y as f32);

                    match mouse.button {
                        glfw::MouseButton::Button3 if mouse.pressed => {
                            let camera = &mut *(*camera).borrow_mut();
                            camera.position +=
                                Vector2::new(-mouse.delta().x, mouse.delta().y)
                                    / camera.scale
                        }

                        glfw::MouseButton::Button1 if mouse.pressed => {}

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
