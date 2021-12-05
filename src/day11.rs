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
        printgrid(grid);

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

    println!("Occupied: {}, Round: {}", occupied, rounds);
}

fn printgrid(grid: Grid) {
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            print!("{}", grid[y][x]);
        }
        println!("");
    }
    println!("");
}

fn grid_xy(grid: Grid, x: i32, y: i32) -> char {
    if x < 0 || x >= WIDTH as i32 || y < 0 || y>=HEIGHT as i32 {
        return 'W'; // wall
    }

    grid[y as usize][x as usize]
}

fn look_around(grid: Grid, x: i32, y: i32) -> usize {
    let checks = [[-1,-1],[0,-1],[1,-1],[-1,0],[1,0],[-1,1],[0,1],[1,1]];
    checks.iter().filter(|&[dx,dy]| {
        is_direction_occupied(grid, x, y, *dx, *dy)
    }).count()    
}

fn is_direction_occupied(grid: Grid, mut x: i32, mut y: i32, dx: i32, dy: i32) -> bool {
    loop {
        x += dx;
        y += dy;
        let pos = grid_xy(grid, x, y);
        if pos=='W' || pos=='L' {
            break;
        } else if pos=='#' {
            return true;
        }
    }
    false
}

fn change(grid: Grid, x: i32, y: i32) -> (char, char) {
    let curxy = grid_xy(grid, x, y);
    if curxy=='L' {
        if look_around(grid, x, y)==0 {
            return ('L', '#');
        }
    } else if curxy=='#' {
        if look_around(grid, x, y)>=5 {
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

