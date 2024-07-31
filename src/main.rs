use glfw::{Action, Context, Key};
use nalgebra::Vector2;
use organify::{grid::Grid, traits::Render, world::World};

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
    #[cfg(feature = "log")]
    log::info!("OpenGL 3.3 core | Samples 4");

    let (mut window, events) = glfw
        .create_window(1200, 600, "Organify", glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window.");
    #[cfg(feature = "log")]
    log::info!("Window init");

    window.set_all_polling(true);
    window.make_current();

    gl::load_with(|s| window.get_proc_address(s));
    window.set_framebuffer_size_callback(|_, w, h| unsafe {
        gl::Viewport(0, 0, w, h);
    });
    #[cfg(feature = "log")]
    log::info!("load OpenGL functions");

    let mut world = World::new(Vector2::new(0.0, 0.0));
    world.render_init();

    #[cfg(feature = "debug")]
    let mut grid = Grid::new(world.position, world.radius);
    #[cfg(feature = "debug")]
    grid.render_init();

    #[cfg(feature = "log")]
    log::info!("Run the main loop");

    unsafe {
        gl::Enable(gl::BLEND);
        gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
    }

    while !window.should_close() {
        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            match event {
                glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                    window.set_should_close(true);
                }
                _ => {}
            }
        }

        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
            gl::ClearColor(0.1, 0.1, 0.1, 1.0);

            world.render();
            #[cfg(feature = "debug")]
            grid.render();
        }

        window.swap_buffers();

        std::thread::sleep(std::time::Duration::from_nanos(1_000_000_000 / 60));
    }
    #[cfg(feature = "log")]
    log::info!("End the main loop");
}
