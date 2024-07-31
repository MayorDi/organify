#[cfg(feature = "debug")]
use nalgebra::Vector2;
#[cfg(feature = "debug")]
use std::mem::size_of;

use crate::consts::SIZE_GRID;

#[cfg(feature = "debug")]
use crate::{
    opengl::prelude::{get_location, GetId},
    opengl::prelude::{Build, Program, Shader},
    render_data::RenderData,
    traits::Render,
};

pub type Index = usize;

#[derive(Debug)]
pub struct Grid {
    cells: Vec<Vec<Vec<Index>>>,

    #[cfg(feature = "debug")]
    render_data: RenderData,
    #[cfg(feature = "debug")]
    world_position: Vector2<f32>,
    #[cfg(feature = "debug")]
    world_radius: f32,
}

impl Grid {
    #[cfg(not(feature = "debug"))]
    pub fn new() -> Self {
        Self {
            cells: vec![vec![vec![]; SIZE_GRID[1]]; SIZE_GRID[0]],
            #[cfg(feature = "debug")]
            render_data: RenderData::default(),
        }
    }

    #[cfg(feature = "debug")]
    pub fn new(world_position: Vector2<f32>, world_radius: f32) -> Self {
        Self {
            cells: vec![vec![vec![]; SIZE_GRID[1]]; SIZE_GRID[0]],
            #[cfg(feature = "debug")]
            render_data: RenderData::default(),
            world_position,
            world_radius,
        }
    }

    pub fn get(&self, x: usize, y: usize) -> &Vec<Index> {
        &self.cells[x][y]
    }

    pub fn clear(&mut self) {
        for x in 0..SIZE_GRID[0] {
            for y in 0..SIZE_GRID[1] {
                if !self.cells[x][y].is_empty() {
                    self.cells[x][y].clear();
                }
            }
        }
    }

    pub fn push_idx(&mut self, idx: Index, x: usize, y: usize) {
        self.cells[x][y].push(idx);
    }
}

#[cfg(feature = "debug")]
impl Render for Grid {
    fn render_init(&mut self) {
        let vs_src = include_bytes!("../res/shaders/grid.vert");
        let fs_src = include_bytes!("../res/shaders/grid.frag");
        let vs = Shader::new(gl::VERTEX_SHADER, vs_src.to_vec());
        let fs = Shader::new(gl::FRAGMENT_SHADER, fs_src.to_vec());
        self.render_data.program.push_shader(vs);
        self.render_data.program.push_shader(fs);
        self.render_data.program.build().unwrap();

        unsafe {
            gl::GenVertexArrays(1, &mut self.render_data.vao);
            gl::GenBuffers(1, &mut self.render_data.vbo);

            gl::BindVertexArray(self.render_data.vao);
            gl::BindBuffer(gl::ARRAY_BUFFER, self.render_data.vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (4 * 2 * size_of::<f32>()) as isize,
                std::ptr::null(),
                gl::DYNAMIC_DRAW,
            );

            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
            gl::BindVertexArray(0);
        }
    }

    fn render(&self) {
        let start_point = self.world_position.x - self.world_radius;
        unsafe {
            let mut size_viewport = [0, 0, 0, 0];
            gl::GetIntegerv(gl::VIEWPORT, &mut size_viewport[0]);

            gl::UseProgram(self.render_data.program.id());
            gl::Uniform2f(
                get_location(&self.render_data.program, "resolution"),
                size_viewport[2] as f32,
                size_viewport[3] as f32,
            );
            gl::BindVertexArray(self.render_data.vao);
            for x in 0..SIZE_GRID[0] {
                for y in 0..SIZE_GRID[1] {
                    let dx = start_point + x as f32 * 10.0;
                    let dy = start_point + y as f32 * 10.0;
                    let vertex_data = [dx, dy, dx + 10.0, dy, dx + 10.0, dy + 10.0, dx, dy + 10.0];

                    gl::BindBuffer(gl::ARRAY_BUFFER, self.render_data.vbo);
                    gl::BufferSubData(
                        gl::ARRAY_BUFFER,
                        0,
                        (4 * 2 * size_of::<f32>()) as isize,
                        vertex_data.as_ptr() as _,
                    );

                    gl::VertexAttribPointer(0, 2, gl::FLOAT, gl::FALSE, 0, std::ptr::null());
                    gl::EnableVertexAttribArray(0);
                    gl::BindBuffer(gl::ARRAY_BUFFER, 0);

                    gl::Uniform1i(
                        get_location(&self.render_data.program, "is_empty"),
                        self.get(x, y).is_empty() as _,
                    );
                    gl::DrawArrays(gl::LINE_LOOP, 0, 4);
                }
            }
            gl::BindVertexArray(0);
            gl::UseProgram(0);
        }
    }
}
