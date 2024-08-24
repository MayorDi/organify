#[cfg(feature = "debug")]
use nalgebra::Vector2;
use nalgebra::{ComplexField, Vector2};
#[cfg(feature = "debug")]
use std::mem::size_of;

use crate::{
    cell::Cell,
    consts::{RADIUS_WORLD, SIZE_GRID},
    control::Camera,
};

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
    pub cells_used: Vec<(usize, usize)>,
    #[cfg(feature = "debug")]
    pub render_data: RenderData,
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
            cells_used: vec![],
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
        for idx in self.cells_used.iter() {
            self.cells[idx.0][idx.1].clear();
        }

        self.cells_used.clear();
    }

    pub fn push_idx(&mut self, idx: Index, x: usize, y: usize) {
        self.cells[x][y].push(idx);
        self.cells_used.push((x, y));
    }

    pub fn update_cells(&mut self, cells: &Vec<Cell>) {
        self.clear();

        for (idx, cell) in cells.iter().enumerate() {
            self.push_idx(
                idx,
                (50.0 + cell.position.x / 10.0) as usize,
                (50.0 + cell.position.y / 10.0) as usize,
            );
        }
    }

    pub fn find_collisions_grid(&self, cells: &mut Vec<Cell>) {
        for x in 1..(SIZE_GRID[0] - 1) {
            for y in 1..(SIZE_GRID[1] - 1) {
                let current_cell_grid = self.get(x, y);
                if current_cell_grid.is_empty() {
                    continue;
                }

                for dx in -1..=1 {
                    for dy in -1..=1 {
                        let other_cell_grid =
                            self.get((x as i32 + dx) as usize, (y as i32 + dy) as usize);

                        self.check_cells_collisions(current_cell_grid, other_cell_grid, cells);
                    }
                }
            }
        }
    }

    pub fn check_cells_collisions(
        &self,
        idxs_cells1: &Vec<usize>,
        idxs_cells2: &Vec<usize>,
        cells: &mut Vec<Cell>,
    ) {
        if idxs_cells1.is_empty() && idxs_cells2.is_empty() {
            return;
        }

        for idx1 in idxs_cells1.iter() {
            for idx2 in idxs_cells2.iter() {
                if Self::is_out_world(&cells[*idx1]) {
                    Self::solve_collide_border_world(*idx1, cells)
                }

                if *idx1 != *idx2 && Self::collide(*idx1, *idx2, cells) {
                    Self::solve_collide(*idx1, *idx2, cells);
                }
            }
        }
    }

    pub fn is_out_world(cell: &Cell) -> bool {
        let radius_world = RADIUS_WORLD - cell.radius * 3.0;
        let radius_world = radius_world * radius_world;
        let len_dist_center_world =
            cell.position.x * cell.position.x + cell.position.y * cell.position.y;

        len_dist_center_world >= radius_world
    }

    pub fn collide(idx1: usize, idx2: usize, cells: &Vec<Cell>) -> bool {
        let cell1 = &cells[idx1];
        let cell2 = &cells[idx2];

        let dist = cell2.position - cell1.position;
        let r = dist.x * dist.x + dist.y * dist.y;

        let diam = cell1.radius*2.0;
        r <= diam*diam
    }

    pub fn solve_collide(idx1: usize, idx2: usize, cells: &mut Vec<Cell>) {
        let cell1 = &cells[idx1];
        let cell2 = &cells[idx2];

        let dist = cell2.position - cell1.position;
        let r = dist.x * dist.x + dist.y * dist.y;

        let diam = cell1.radius*2.0;
        let f = (diam*diam - r) / (diam*diam) * cell1.mass / (1.0 + r);
        cells[idx1].velocity -= dist * f;
    }

    pub fn solve_collide_border_world(idx: usize, cells: &mut Vec<Cell>) {
        let cell = &cells[idx];
        let radius_world = RADIUS_WORLD - cell.radius * 3.0;
        let r = (cell.position.x * cell.position.x + cell.position.y * cell.position.y).sqrt();
        let cof = radius_world/r;
        cells[idx].position *= cof;
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
        let camera = self.render_data.camera.as_ref().unwrap();
        let start_point = self.world_position.x - self.world_radius;
        unsafe {
            let mut size_viewport = [0, 0, 0, 0];
            gl::GetIntegerv(gl::VIEWPORT, &mut size_viewport[0]);

            gl::UseProgram(self.render_data.program.id());
            gl::Uniform2f(
                get_location(&self.render_data.program, "u_resolution"),
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
                        get_location(&self.render_data.program, "u_is_empty"),
                        self.get(x, y).is_empty() as _,
                    );
                    gl::Uniform2fv(
                        get_location(&self.render_data.program, "u_camera.position"),
                        1,
                        [camera.borrow().position.x, camera.borrow().position.y].as_ptr() as _,
                    );

                    gl::Uniform1fv(
                        get_location(&self.render_data.program, "u_camera.scale"),
                        1,
                        [camera.borrow().scale].as_ptr() as _,
                    );
                    gl::DrawArrays(gl::LINE_LOOP, 0, 4);
                }
            }
            gl::BindVertexArray(0);
            gl::UseProgram(0);
        }
    }
}
