pub struct WindowComponents {
    pub(crate) glfw: glfw::Glfw,
    pub(crate) window: glfw::PWindow,
    pub(crate) events: glfw::GlfwReceiver<(f64, glfw::WindowEvent)>,
}

pub struct EguiComponents {
    pub(crate) painter: egui_glfw::Painter,
    pub(crate) egui_ctx: egui_glfw::egui::Context,
    pub(crate) native_pixels_per_point: f32,
    pub(crate) egui_input_state: egui_glfw::EguiInputState,
}
