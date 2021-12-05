use std::fs;
use crate::util;

const WIDTH: usize = 95;
const HEIGHT: usize = 95;
type Grid = [[char; WIDTH]; HEIGHT];

pub fn go() {
    println!("Day 11");

    let input = fs::read_to_string("inputs/input11.txt")
        .expect("Could not read the input file.");

    let mut grid: Grid = [['.'; WIDTH]; HEIGHT];
    for (y,line) in input.split(util::LINE_ENDING).into_iter().enumerate() {
        for (x, char) in line.chars().enumerate() {
            grid[y][x] = char;
        } 
    }

    let mut rounds = 0;
    loop {
        rounds+=1;
        let (newgrid, changes) = run(grid);
        if changes==0 {
            break;
        }
        grid = newgrid;
    }

    let occupied:usize = grid.iter().map(|&r| {
        r.iter().filter(|&c| { *c=='#' }).count()
    }).sum();

    println!("{:?}", grid);
    println!("Occupied: {}, Round: {}", occupied, rounds);
}

fn grid_xy(grid: Grid, x: i32, y: i32) -> char {
    if x < 0 || x >= WIDTH as i32 || y < 0 || y>=HEIGHT as i32 {
        return '.';
    }

    grid[y as usize][x as usize]
}

fn look_around(grid: Grid, x: i32, y: i32) -> usize {
    let checks = [[-1,-1],[0,-1],[1,-1],[-1,0],[1,0],[-1,1],[0,1],[1,1]];
    checks.iter().filter(|&p| {
        grid_xy(grid, x+p[0], y+p[1])=='#'
    }).count()    
}

fn change(grid: Grid, x: i32, y: i32) -> (char, char) {
    let curxy = grid_xy(grid, x, y);
    if curxy=='L' {
        if look_around(grid, x, y)==0 {
            return ('L', '#');
        }
    } else if curxy=='#' {
        if look_around(grid, x, y)>=4 {
            return ('#', 'L');
        }
    }

    (curxy, curxy)
}

fn run(grid: Grid) -> (Grid, i32) {
    let mut newgrid = grid;
    let mut changes = 0;

    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let (curxy,newxy) = change(grid, x as i32, y as i32);
            if curxy!=newxy {
                changes += 1;
                newgrid[y][x] = newxy;
            }
        }
    }
    (newgrid, changes)
}
