use std::collections::HashMap;
use std::collections::hash_map;
use cell::State;


//HashMap storing the world chunks
pub struct HashWorld {
    pub cells: HashMap<(isize, isize), State>
}

pub trait World {
    fn get_cell(&self, x: isize, y: isize) -> State;

    fn set_cell(&mut self, x: isize, y: isize);

    fn kill_cell(&mut self, x: isize, y: isize);

    fn num_adjacent(&self, x: isize, y: isize) -> isize {
        let mut count = 0is;
        for i in range(-1is, 2is) {
            for j in range(-1is, 2is) {
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
    fn iter(&self) -> hash_map::Iter<(isize, isize), State>;
}

impl HashWorld {
    pub fn new() -> HashWorld {
        HashWorld { cells: HashMap::new() }
    }
}

impl World for HashWorld {

    fn get_cell(&self, x: isize, y: isize) -> State {
        match self.cells.get(&(x, y)) {
            Some(cell) => *cell,
            None => State::Dead
        }
    }

    fn set_cell(&mut self, x: isize, y: isize) {
        self.cells.insert((x, y), State::Alive);
    }

    fn kill_cell(&mut self, x: isize, y: isize) {
        self.cells.remove(&(x, y));
    }

    fn iter(&self) -> hash_map::Iter<(isize, isize), State> {
        self.cells.iter()
    }
}
