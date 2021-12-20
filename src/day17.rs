use std::fs;
use crate::util;

type Ticket = Vec<i32>;

pub fn go() {
    println!("Day 17");

    let input = fs::read_to_string("inputs/input17.txt")
        .expect("Could not read the input file.");

    

}

struct Grid3 {
    start_x: i32,
    start_y: i32,
    start_z: i32,
    width: i32,
    height: i32,
    depth: i32,
    state: Vec<bool>
}

impl Grid3 {
    fn new(startx: i32, starty: i32, startz: i32, width: i32, height: i32, depth: i32) -> Self {
        Grid3 {
            start_x: startx,
            start_y: starty,
            start_z: startz,
            width,
            height,
            depth,
            state: vec![false; (width * height * depth) as usize]
        }
    }

    fn wrap(grid: &Grid3) -> Self {
        let mut new_grid = Grid3::new(grid.start_x-1, grid.start_y-1, grid.start_z-1,
            grid.width+2, grid.height+2, grid.depth+2);

        // copy the contents of the inner grid
        
        new_grid
    }

    fn get_index(&self, x: i32, y: i32, z: i32) -> usize {
        if (x<self.start_x || x>=self.start_x+self.width) ||
           (y<self.start_y || y>=self.start_y+self.height) ||
           (z<self.start_z || z>=self.start_z+self.depth) {
            panic!("Coordinates out of range: ({},{},{})", x, y, z);
        }

        ((x - self.start_x) * (y - self.start_y) * (z - self.start_z)) as usize
    }

    fn get_value_at(&self, x: i32, y: i32, z: i32) -> bool {
        self.state[self.get_index(x, y, z)]
    }

    fn set_value_at(&mut self, x: i32, y: i32, z: i32, val: bool) {
        let index = self.get_index(x, y, z);
        self.state[index] = val;
    }
}

