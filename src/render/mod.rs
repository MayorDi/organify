use std::collections::HashMap;

use gl::types::GLuint;

use crate::opengl::prelude::{Program, Shader};

pub mod traits;
pub mod ui;

pub struct RenderContext {
    pub vaos: HashMap<&'static str, GLuint>,
    pub vbos: HashMap<&'static str, GLuint>,
    pub programs: HashMap<&'static str, Program<Shader>>,
}
