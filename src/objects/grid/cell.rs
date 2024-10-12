#[derive(Debug, Clone)]
pub struct Cell {
    pub indexes: Vec<usize>
}

impl Cell {
    pub fn new() -> Self {
        Self {
            indexes: vec![]
        }
    }

    pub fn clear(&mut self) {
        self.indexes.clear();
    }

    pub fn push(&mut self, index: usize) {
        self.indexes.push(index);
    }
}
