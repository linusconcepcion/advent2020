use std::fs;
use crate::util;

pub fn go() {
    println!("Day 12");

    let input = fs::read_to_string("inputs/input12.txt")
        .expect("Could not read the input file.");

    let actions: Vec<&str> = input.split(util::LINE_ENDING).collect();

    let mut position = (0, 0);
    let mut waypoint = (10, 1);

    for action in actions {
        let cmd = &action[0..1];
        let num = &action[1..].parse::<i32>().unwrap();

        match cmd {
            "F" => position = move_forward(position, waypoint, *num),
            "L" => waypoint = turn_left(waypoint, *num),
            "R" => waypoint = turn_right(waypoint, *num),

            "N" => waypoint = (waypoint.0, waypoint.1+num),
            "S" => waypoint = (waypoint.0, waypoint.1-num),
            "E" => waypoint = (waypoint.0+num, waypoint.1),
            "W" => waypoint = (waypoint.0-num, waypoint.1),
            _ => panic!("Unknown cmd: {}", cmd)
        }
    }

    println!("Distance: {}", position.0.abs() + position.1.abs());
}

fn move_forward(position: (i32, i32), waypoint: (i32, i32), num: i32) -> (i32, i32) {
    (position.0 + (waypoint.0 * num), position.1 + (waypoint.1 * num))
}

fn turn_left(waypoint: (i32, i32), num: i32) -> (i32, i32) {
    match num {
        270 => (waypoint.1, -waypoint.0),
        180 => (-waypoint.0, -waypoint.1),
        90 => (-waypoint.1, waypoint.0),
        _ => panic!("Unknown deg: {}", num)
    }    
}

fn turn_right(waypoint: (i32, i32), num: i32) -> (i32, i32) {
    match num {
        90 => (waypoint.1, -waypoint.0),
        180 => (-waypoint.0, -waypoint.1),
        270 => (-waypoint.1, waypoint.0),
        _ => panic!("Unknown deg: {}", num)
    }
}
