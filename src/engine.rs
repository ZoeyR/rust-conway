use std::collections::DList;

use cell;
use world;

///The main implementation of Conway's game of life. This engine utilizes a list of updates to
///track interesting areas in the world. Only cells adjacent to cells that updated during the last
///generation are evaluated for a new state
pub struct ConwayEngine {
    generation
    updated: DList<(cell::Cell, (int, int))>
}

impl ConwayEngine {

    ///Create a new instance of the engine, this should be used
    ///on a world with an initial setup of cells.
    pub fn new(world: &world::World) -> ConwayEngine {
        let mut first_list = DList::new();
        for(location, cell) in world.cells.iter() {
            first_list.push((*cell, *location));
        }
        for cell in first_list.iter() {
            let (_, (x, y)) = *cell;
            print!("[x:{}, y:{}] ", x, y);
        }
        println!("");
        ConwayEngine { updated: first_list}
    }

    ///Calculate the next generation of cells
    pub fn next_generation(&mut self, world: &mut world::World) {
        //new list of updates
        let mut new_list = DList::new();

        for cell in self.updated.iter() {
            let (_, (x, y)) = *cell;
            print!("[x:{}, y:{}] ", x, y);
        }
        println!("");
        //loop through all the updated cells
        for cell in self.updated.iter() {
            let (_, (x, y)) = *cell;
            //check for new states on all adjacent cells
            for i in range(-1i, 2i) {
                for j in range(-1i, 2i) {

                    //ignore the center cell
                    if i == 0 && j == 0 {
                        continue;
                    }

                    //get the new state
                    let state = world.get_cell(x - i, y - j);
                    let new_state = self.new_state((world.get_cell(x - i, y - j), (x - i, y - j)), world);

                    //if the cell changed, update the world and list
                    if state != new_state {
                        new_list.push((new_state, (x - i, y - j)));
                    }
                }
            }
        }

        //update the world with new list
        for cell in new_list.iter() {
            match *cell {
                (cell::Dead, (x, y)) => world.kill_cell(x, y),
                (_, (x, y))         => world.set_cell(x, y)
            }
           /* if state == cell::Dead {
                world.kill_cell(x, y);
            } else {
                world.set_cell(x, y);
            }*/
        }
        self.updated = new_list;
    }

    
    fn new_state(&self, cell: (cell::Cell, (int, int)), world: &world::World) -> cell::Cell {
        let (state, (x, y)) = cell;

        //count of sourrounding live cells
        let mut count = 0i;

        //count the surrounding cells
        for i in range(-1i, 2i) {
            for j in range(-1i, 2i) {
                if i == 0 && j == 0 {
                    continue;
                }
                if world.get_cell(x - i, y - j) == cell::Alive {
                    count += 1;
                }
            }
        }

        //apply conways rules
        if count == 3 {
            return cell::Alive;
        }
        if count == 2 && state == cell::Alive {
            return cell::Alive;
        }
        cell::Dead
    }
}
