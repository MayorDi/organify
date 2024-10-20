use gl::types::{GLchar, GLint};

use super::prelude::{Build, Delete, GetId, Status};

#[derive(Debug, Clone)]
pub struct Shader {
    id: u32,
    status: StatusShader,
}

impl Shader {
    pub fn new(type_shader: gl::types::GLenum, src: Vec<u8>) -> Self {
        unsafe {
            let id = gl::CreateShader(type_shader);
            gl::ShaderSource(
                id,
                1,
                &std::ffi::CStr::as_ptr(&std::ffi::CString::new(src.clone()).unwrap()),
                std::ptr::null(),
            );

            Self {
                id,
                status: Default::default(),
            }
        }
    }
}

impl GetId for Shader {
    fn id(&self) -> u32 {
        self.id
    }
}

impl Status for Shader {
    type Output = StatusShader;
    fn status(&self) -> Self::Output {
        self.status.clone()
    }
}

impl Build for Shader {
    fn build(&mut self) -> Result<(), String> {
        unsafe {
            gl::CompileShader(self.id());

            let mut status = gl::FALSE as GLint;
            gl::GetShaderiv(self.id(), gl::COMPILE_STATUS, &mut status);

            if status != (gl::TRUE as GLint) {
                let mut len = 0;
                gl::GetShaderiv(self.id(), gl::INFO_LOG_LENGTH, &mut len);

                let mut buf = Vec::with_capacity(len as usize);
                buf.set_len((len as usize) - 1);

                gl::GetShaderInfoLog(
                    self.id(),
                    len,
                    std::ptr::null_mut(),
                    buf.as_mut_ptr() as *mut GLchar,
                );

                let err = format!(
                    "{}",
                    std::str::from_utf8(&buf)
                        .ok()
                        .expect("ShaderInfoLog not valid utf8")
                );

                self.status = StatusShader::ErrorCompile(err.clone());
                return Err(err);
            }

            self.status = StatusShader::CompiledSuccessfully;
            Ok(())
        }
    }
}

impl Delete for Shader {
    fn delete(self) {
        unsafe {
            gl::DeleteShader(self.id());
        }
    }
}

#[derive(Debug, Clone, Default)]
pub enum StatusShader {
    ErrorCompile(String),
    #[default]
    NotCompiled,
    CompiledSuccessfully,
}
