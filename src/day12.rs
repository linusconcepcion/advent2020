use std::fs;
use crate::util;

pub fn go() {
    println!("Day 12");

    let input = fs::read_to_string("inputs/input12.txt")
        .expect("Could not read the input file.");

    let actions: Vec<&str> = input.split(util::LINE_ENDING).collect();

    let mut position = (0, 0);
    let mut facing = 90;  // east

    for action in actions {
        let cmd = &action[0..1];
        let num = &action[1..].parse::<i32>().unwrap();

        match cmd {
            "F" => position = move_forward(position, facing, *num),
            "L" => facing = turn_left(facing, *num),
            "R" => facing = turn_right(facing, *num),
            "N" => position = (position.0, position.1+num),
            "S" => position = (position.0, position.1-num),
            "E" => position = (position.0+num, position.1),
            "W" => position = (position.0-num, position.1),
            _ => panic!("Unknown cmd: {}", cmd)
        }
    }

    println!("Distance: {}", position.0.abs() + position.1.abs());
}

fn move_forward(position: (i32, i32), facing: i32, num: i32) -> (i32, i32) {
    let dir = match facing {
        0 => (0, 1),
        90 => (1, 0),
        180 => (0, -1),
        270 => (-1, 0),
        _ =>
            panic!("Unexpected facing: {}", facing)
    };

    (position.0 + (dir.0 * num), position.1 + (dir.1 * num))
}

fn turn_left(facing: i32, num: i32) -> i32 {
    (facing - num + 360) % 360
}

fn turn_right(facing: i32, num: i32) -> i32 {
    (facing + num) % 360
}
