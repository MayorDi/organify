use std::{cell::RefCell, mem::size_of, rc::Rc};

use nalgebra::Vector2;

use crate::{
    control::Camera, opengl::prelude::{get_location, Build, GetId, Program, Shader}, render_data::RenderData, traits::Behavior
};

#[derive(Debug, Clone, Default)]
pub struct Cell {
    pub position: Vector2<f32>,
    pub velocity: Vector2<f32>,
    pub mass: f32,
    pub radius: f32,
    pub direction: Vector2<f32>,
}

impl Cell {
    pub fn new(position: Vector2<f32>) -> Self {
        Self {
            position,
            radius: 5.0,
            mass: 4.7,
            ..Default::default()
        }
    }
}

impl Behavior for Cell {
    fn update(&mut self) {
        self.position += self.velocity;
        self.velocity *= 0.9;
    }
}

impl Cell {
    pub fn render_init(camera: Option<Rc<RefCell<Camera>>>) -> RenderData {
        let mut vao @ mut vbo @ mut ebo = 0;
        let idxs = [0u8, 1, 2, 1, 2, 3];

        unsafe {
            gl::GenVertexArrays(1, &mut vao);
            gl::GenBuffers(1, &mut vbo);
            gl::GenBuffers(1, &mut ebo);

            gl::BindVertexArray(vao);
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (4 * 4 * size_of::<f32>()) as isize,
                std::ptr::null(),
                gl::DYNAMIC_DRAW,
            );

            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                idxs.len() as isize,
                idxs.as_ptr() as _,
                gl::DYNAMIC_DRAW,
            );

            gl::VertexAttribPointer(
                0,
                2,
                gl::FLOAT,
                gl::FALSE,
                (4 * size_of::<f32>()) as _,
                std::ptr::null(),
            );
            gl::EnableVertexAttribArray(0);

            gl::VertexAttribPointer(
                1,
                2,
                gl::FLOAT,
                gl::FALSE,
                (4 * size_of::<f32>()) as _,
                (2 * size_of::<f32>()) as _,
            );
            gl::EnableVertexAttribArray(1);

            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
            gl::BindVertexArray(0);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0);
        }

        RenderData {
            vao,
            vbo,
            program: create_program_shader_cells(),
            camera,
        }
    }

    pub fn render(&self, render_data: &RenderData, time: f32) {
        let camera = render_data.camera.as_ref().unwrap();
        let vertex_data = [
            self.position.x - self.radius,
            self.position.y - self.radius,
            0.0,
            0.0,
            self.position.x + self.radius,
            self.position.y - self.radius,
            1.0,
            0.0,
            self.position.x - self.radius,
            self.position.y + self.radius,
            0.0,
            1.0,
            self.position.x + self.radius,
            self.position.y + self.radius,
            1.0,
            1.0,
        ];

        unsafe {
            let mut size_viewport = [0, 0, 0, 0];
            gl::GetIntegerv(gl::VIEWPORT, &mut size_viewport[0]);

            gl::UseProgram(render_data.program.id());
            gl::Uniform2f(
                get_location(&render_data.program, "u_resolution"),
                size_viewport[2] as f32,
                size_viewport[3] as f32,
            );
            // gl::Uniform1f(get_location(&render_data.program, "u_radius"), self.radius);
            gl::Uniform1f(get_location(&render_data.program, "u_time"), time);

            gl::Uniform2fv(
                get_location(&render_data.program, "u_camera.position"),
                1,
                [
                    camera.borrow().position.x,
                    camera.borrow().position.y,
                ].as_ptr() as _
            );

            gl::Uniform1fv(
                get_location(&render_data.program, "u_camera.scale"),
                1,
                [
                    camera.borrow().scale,
                ].as_ptr() as _
            );

            gl::BindVertexArray(render_data.vao);
            gl::BindBuffer(gl::ARRAY_BUFFER, render_data.vbo);
            gl::BufferSubData(
                gl::ARRAY_BUFFER,
                0,
                (4 * 4 * size_of::<f32>()) as isize,
                vertex_data.as_ptr() as _,
            );
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);

            gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_BYTE, std::ptr::null());
            gl::BindVertexArray(0);
            gl::UseProgram(0);
        }
    }
}

fn create_program_shader_cells() -> Program<Shader> {
    let mut prog = Program::new();
    let vs_src = include_bytes!("../../res/shaders/cell.vert");
    let fs_src = include_bytes!("../../res/shaders/cell.frag");
    let vs = Shader::new(gl::VERTEX_SHADER, vs_src.to_vec());
    let fs = Shader::new(gl::FRAGMENT_SHADER, fs_src.to_vec());
    prog.push_shader(vs);
    prog.push_shader(fs);
    prog.build().unwrap();

    prog
}
