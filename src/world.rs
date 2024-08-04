use std::mem::size_of;

use nalgebra::Vector2;

use crate::{
    consts::RADIUS_WORLD,
    opengl::prelude::{get_location, Build, GetId, Shader},
    render_data::RenderData,
    traits::Render,
};

#[derive(Debug)]
pub struct World {
    pub position: Vector2<f32>,
    pub radius: f32,
    render_data: RenderData,
}

impl World {
    pub fn new(position: Vector2<f32>) -> Self {
        Self {
            position,
            radius: RADIUS_WORLD,
            render_data: RenderData::default(),
        }
    }
}

impl Render for World {
    fn render_init(&mut self) {
        let vertex_data = [
            self.position.x - self.radius,
            self.position.y - self.radius,
            self.position.x + self.radius,
            self.position.y - self.radius,
            self.position.x - self.radius,
            self.position.y + self.radius,
            self.position.x + self.radius,
            self.position.y + self.radius,
        ];

        let idxs: [u8; 6] = [0, 1, 2, 1, 2, 3];

        let vs_src = include_bytes!("../res/shaders/world.vert");
        let fs_src = include_bytes!("../res/shaders/world.frag");
        let vs = Shader::new(gl::VERTEX_SHADER, vs_src.to_vec());
        let fs = Shader::new(gl::FRAGMENT_SHADER, fs_src.to_vec());
        self.render_data.program.push_shader(vs);
        self.render_data.program.push_shader(fs);
        self.render_data.program.build().unwrap();

        unsafe {
            let mut ebo = 0;
            gl::GenVertexArrays(1, &mut self.render_data.vao);
            gl::GenBuffers(1, &mut self.render_data.vbo);
            gl::GenBuffers(1, &mut ebo);

            gl::BindVertexArray(self.render_data.vao);
            gl::BindBuffer(gl::ARRAY_BUFFER, self.render_data.vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (vertex_data.len() * size_of::<f32>()) as isize,
                vertex_data.as_ptr() as _,
                gl::STATIC_DRAW,
            );

            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                idxs.len() as isize,
                idxs.as_ptr() as _,
                gl::STATIC_DRAW,
            );

            gl::VertexAttribPointer(0, 2, gl::FLOAT, gl::FALSE, 0, std::ptr::null());
            gl::EnableVertexAttribArray(0);
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
            gl::BindVertexArray(0);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0);
        }
    }

    fn render(&self) {
        unsafe {
            let mut size_viewport = [0, 0, 0, 0];
            gl::GetIntegerv(gl::VIEWPORT, &mut size_viewport[0]);

            gl::UseProgram(self.render_data.program.id());
            gl::Uniform2f(
                get_location(&self.render_data.program, "u_resolution"),
                size_viewport[2] as f32,
                size_viewport[3] as f32,
            );
            gl::Uniform1f(
                get_location(&self.render_data.program, "u_radius"),
                self.radius,
            );
            gl::BindVertexArray(self.render_data.vao);
            gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_BYTE, std::ptr::null());
            gl::BindVertexArray(0);
            gl::UseProgram(0);
        }
    }
}
