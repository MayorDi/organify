use crate::consts::SIZE_GRID;

pub type Index = usize;

#[derive(Debug)]
pub struct Grid {
    cells: [[Vec<Index>; SIZE_GRID[1]]; SIZE_GRID[0]]
}

impl Grid {
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
