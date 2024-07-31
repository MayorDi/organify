use crate::opengl::prelude::{Program, Shader};

#[derive(Debug, Clone)]
pub struct RenderData {
    pub vao: u32,
    pub vbo: u32,
    pub program: Program<Shader>
}

impl Default for RenderData {
    fn default() -> Self {
        Self {
            vao: 0,
            vbo: 0,
            program: Program::new(),
        }
    }
}
