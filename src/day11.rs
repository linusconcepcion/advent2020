use std::fs;
use crate::util;

const WIDTH: usize = 10;
const HEIGHT: usize = 10;
type Grid = [[char; WIDTH]; HEIGHT];

pub fn go() {
    println!("Day 11");

    let input = fs::read_to_string("inputs/input11.txt")
        .expect("Could not read the input file.");

    let mut grid: Grid;
    

}
