use std::collections::HashMap;
use cell;


//HashMap storing the world chunks
pub struct World {
    pub cells: HashMap<(int, int), cell::Cell>
}

impl World {
    fn next(&self) {
    }
    pub fn new() -> World {
        World { cells: HashMap::new() }
    }

    pub fn get_cell(&self, x: int, y: int) -> cell::Cell {
        match self.cells.find(&(x, y)) {
            Some(cell) => *cell,
            None => cell::Dead
        }
    }

    pub fn set_cell(&mut self, x: int, y: int) {
        self.cells.insert((x, y), cell::Alive);
    }

    pub fn kill_cell(&mut self, x: int, y: int) {
        self.cells.remove(&(x, y));
    }
}
