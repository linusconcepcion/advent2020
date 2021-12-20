use std::fs;
use crate::util;

type Ticket = Vec<i32>;

pub fn go() {
    println!("Day 17");

    let input = fs::read_to_string("inputs/input17.txt")
        .expect("Could not read the input file.");

    let lines : Vec<&str> = input.split(util::LINE_ENDING).collect();
         
    let mut grid = Grid4::new(-3, -3, 0, 0, 8, 8, 1, 1);
    for y in 0..grid.height as usize {
        let line = lines[y];
        for x in 0..grid.width as usize {
            let val = line.chars().nth(x).unwrap()=='#';
            grid.set_value_at(x as i32 + grid.start_x, y as i32 + grid.start_y, 0, 0, val);
        }
    }

    for cycle in 0..6 {
        println!();
        println!("Cycle #{}", cycle+1);

        let mut new_grid = Grid4::wrap(&grid);
        new_grid.cycle(&grid);

        println!("Active: {}", new_grid.count_active());
        
        grid = new_grid;
    }
}

struct Grid4 {
    start_x: i32,
    start_y: i32,
    start_z: i32,
    start_w: i32,
    width: i32,
    height: i32,
    depth: i32,
    fourth: i32,
    state: Vec<bool>
}

impl Grid4 {
    fn new(startx: i32, starty: i32, startz: i32, startw: i32, width: i32, height: i32, depth: i32, fourth: i32) -> Self {
        Grid4 {
            start_x: startx,
            start_y: starty,
            start_z: startz,
            start_w: startw,
            width: width,
            height: height,
            depth: depth,
            fourth: fourth,
            state: vec![false; (width * height * depth * fourth) as usize]
        }
    }

    fn wrap(grid: &Grid4) -> Self {
        let mut new_grid = Grid4::new(grid.start_x-1, grid.start_y-1, grid.start_z-1, grid.start_w-1,
            grid.width+2, grid.height+2, grid.depth+2, grid.fourth+2);

        // copy the contents of the inner grid
        for w in grid.start_w..grid.start_w+grid.fourth {
            for z in grid.start_z..grid.start_z+grid.depth {
                for y in grid.start_y..grid.start_y+grid.height {
                    for x in grid.start_x..grid.start_x+grid.width {
                        let val = grid.get_value_at(x, y, z, w);
                        new_grid.set_value_at(x, y,z, w, val);
                    }
                }
            }
        }
        
        new_grid
    }

    // fn clone(grid: &Grid3) -> Self {
    //     let mut new_grid = Grid3::new(grid.start_x, grid.start_y, grid.start_z,
    //         grid.width, grid.height, grid.depth);
    //     new_grid.state = grid.state.clone();
    //     new_grid
    // }

    fn count_active(&self) -> usize {
        self.state.iter().filter(|&s| {*s}).count()
    }

    fn get_index(&self, x: i32, y: i32, z: i32, w: i32) -> Option<usize> {
        if (x<self.start_x || x>=self.start_x+self.width) ||
           (y<self.start_y || y>=self.start_y+self.height) ||
           (z<self.start_z || z>=self.start_z+self.depth) ||
           (w<self.start_w || w>=self.start_w+self.fourth) {
            return None;
        }

        Some((
             ((w - self.start_w) * (self.width * self.height * self.depth)) +
             ((z - self.start_z) * (self.width * self.height)) + 
             ((y - self.start_y) * (self.width)) + 
             (x - self.start_x)) as usize)
    }

    fn get_value_at(&self, x: i32, y: i32, z: i32, w: i32) -> bool {
        match self.get_index(x, y, z, w) {
            None => false,
            Some(n) => self.state[n]
        }
    }

    fn set_value_at(&mut self, x: i32, y: i32, z: i32, w: i32, val: bool) {
        match self.get_index(x, y, z, w) {
            None => panic!("Invalid coords {}, {}, {}, {}", x, y, z, w),
            Some(n) => {
                self.state[n] = val;
            }
        }
    }

    fn cycle(&mut self, old_state: &Grid4) {
        for w in self.start_w..self.start_w+self.fourth {
            for z in self.start_z..self.start_z+self.depth {
                for y in self.start_y..self.start_y+self.height {
                    for x in self.start_x..self.start_x+self.width {
                        let cur_active = old_state.count_surrounding(x, y, z, w);
                        let cur_state = old_state.get_value_at(x, y, z, w);
                        if cur_state {
                            if cur_active==2 || cur_active==3 {
                                self.set_value_at(x, y, z, w, true);
                            } else {
                                self.set_value_at(x, y, z, w, false);
                            }
                        } else {
                            if cur_active==3 {
                                self.set_value_at(x, y, z, w, true);
                            } else {
                                self.set_value_at(x, y, z, w, false);
                            }
                        }
                    }
                }
            }
        }
    }

    fn count_surrounding(&self, x: i32, y: i32, z: i32, w: i32) -> i32 {
        let mut result = 0;

        for dw in -1..2 {
            for dz in -1..2 {
                for dy in -1..2 {
                    for dx in -1..2 {
                        if dx==0 && dy==0 && dz==0 && dw==0 {
                            continue;
                        }

                        if self.get_value_at(x + dx, y + dy, z + dz, w + dw) {
                            result += 1;
                        }
                    } 
                }
            }
        }
        result
    }

    // fn print(&self) {
    //     for z in self.start_z..self.start_z+self.depth {
    //         println!();
    //         print!("Z={}", z);
    //         for y in self.start_y..self.start_y+self.height {
    //             println!();
    //             for x in self.start_x..self.start_x+self.width {
    //                 let val = self.get_value_at(x, y, z);
    //                 print!("{}", if val { '#' } else { '.' });
    //             }
    //         }
    //     }
    //     println!();
    // }
}

