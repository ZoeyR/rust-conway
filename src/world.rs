use std::collections::HashMap;
use std::collections::hash_map;
use cell::State;


//HashMap storing the world chunks
pub struct HashWorld {
    pub cells: HashMap<(int, int), State>
}

pub trait World {
    fn get_cell(&self, x: int, y: int) -> State;

    fn set_cell(&mut self, x: int, y: int);

    fn kill_cell(&mut self, x: int, y: int);

    fn num_adjacent(&self, x: int, y: int) -> int {
        let mut count = 0i;
        for i in range(-1i, 2i) {
            for j in range(-1i, 2i) {
                if i == 0 && j == 0 {
                    continue;
                }
                if self.get_cell(x - i, y - j) == State::Alive {
                    count += 1;
                }
            }
        }
        count
    }

    ///Returns a HashMap iterator of live cells.
    ///This will stay in this format for the forseeable future.
    ///Returning a generic iterator adds too much overhead to the
    ///iter() function
    fn iter(&self) -> hash_map::Entries<(int, int), State>;
}

impl HashWorld {
    pub fn new() -> HashWorld {
        HashWorld { cells: HashMap::new() }
    }
}

impl World for HashWorld {

    fn get_cell(&self, x: int, y: int) -> State {
        match self.cells.get(&(x, y)) {
            Some(cell) => *cell,
            None => State::Dead
        }
    }

    fn set_cell(&mut self, x: int, y: int) {
        self.cells.insert((x, y), State::Alive);
    }

    fn kill_cell(&mut self, x: int, y: int) {
        self.cells.remove(&(x, y));
    }

    fn iter(&self) -> hash_map::Entries<(int, int), State> {
        self.cells.iter()
    }
}
