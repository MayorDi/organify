use cell::Cell;
use nalgebra::Vector2;
use vector_growing::VecGrow;

use crate::logic::traits::Physics;

pub mod cell;

#[derive(Debug, Clone)]
pub struct Grid {
    used_cells: Vec<(usize, usize)>,
    cells: Vec<Vec<Cell>>,
}

impl Grid {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            used_cells: Vec::with_capacity(width * height),
            cells: vec![vec![Cell::new(); height]; width],
        }
    }

    pub fn push(&mut self, index: usize, position: Vector2<f32>) {
        let pos = (position.x as usize, position.y as usize);
        self.cells[pos.0][pos.1].push(index);
        self.used_cells.push(pos);
    }

    pub fn clear(&mut self) {
        for pos in self.used_cells.iter() {
            self.cells[pos.0][pos.1].clear();
        }

        self.used_cells.clear();
    }

    pub fn update<T: Physics>(&mut self, objects: VecGrow<T>) {
        for (index, obj) in objects.iter().enumerate() {
            self.push(index, obj.position());
        }
    }

    pub fn get(&self, pos: Vector2<f32>) -> &Cell {
        &self.cells[pos.x as usize][pos.y as usize]
    }
}
