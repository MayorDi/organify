use std::ffi::CString;

use gl::types::{GLchar, GLint};

use crate::opengl::prelude::StatusShader;

use super::prelude::{Build, Delete, GetId, Status};

#[derive(Debug, Clone)]
pub struct Program<S: Build + GetId + Status + Delete> {
    id: u32,
    status: bool,
    shaders: Vec<S>,
}

impl<S: Build + GetId + Status + Delete> Program<S> {
    pub fn new() -> Self {
        unsafe {
            Self {
                id: gl::CreateProgram(),
                status: false,
                shaders: vec![],
            }
        }
    }

    pub fn push_shader(&mut self, shader: S) {
        self.shaders.push(shader);
    }
}

impl<S: Build + GetId + Status + Delete> GetId for Program<S> {
    fn id(&self) -> u32 {
        self.id
    }
}

impl<S> Build for Program<S>
where
    S: Build + GetId + Status<Output = StatusShader> + Delete,
{
    fn build(&mut self) -> Result<(), String> {
        let id = self.id();
        unsafe {
            for shader in self.shaders.iter_mut() {
                if let StatusShader::NotCompiled = shader.status() {
                    shader.build()?;
                } else if let StatusShader::ErrorCompile(err) = shader.status() {
                    return Err(err);
                }

                gl::AttachShader(id, shader.id());
            }
        }

        unsafe {
            gl::LinkProgram(id);

            let mut status = gl::FALSE as GLint;
            gl::GetProgramiv(id, gl::LINK_STATUS, &mut status);

            if status != (gl::TRUE as GLint) {
                let mut len: GLint = 0;
                gl::GetProgramiv(id, gl::INFO_LOG_LENGTH, &mut len);

                let mut buf = Vec::with_capacity(len as usize);
                buf.set_len((len as usize) - 1);

                gl::GetProgramInfoLog(
                    id,
                    len,
                    std::ptr::null_mut(),
                    buf.as_mut_ptr() as *mut GLchar,
                );

                self.status = false;

                return Err(format!(
                    "{}",
                    std::str::from_utf8(&buf)
                        .ok()
                        .expect("ProgramInfoLog not valid utf8")
                ));
            }
        }

        self.status = true;

        Ok(())
    }
}

impl<S> Status for Program<S>
where
    S: Build + GetId + Status<Output = StatusShader> + Delete,
{
    type Output = bool;

    fn status(&self) -> Self::Output {
        self.status
    }
}

impl<S> Delete for Program<S>
where
    S: Build + GetId + Status<Output = StatusShader> + Delete,
{
    fn delete(self) {
        unsafe {
            let id = self.id();
            for shader in self.shaders {
                shader.delete();
            }

            gl::DeleteProgram(id);
        }
    }
}

pub fn get_location<T: GetId>(program: &T, name: &str) -> i32 {
    unsafe { gl::GetUniformLocation(program.id(), std::ffi::CStr::as_ptr(&CString::new(name).unwrap())) }
}
