use std::fs;
use crate::util;

pub fn go() {
    println!("Day 8");

    let input = fs::read_to_string("inputs/input08.txt")
        .expect("Could not read the input file.");

    let instructions: Vec<&str> = input.split(util::LINE_ENDING).collect();

    let mut change_index = 1;
    loop {
        let (finish, accumulator) = run(&instructions, change_index);
        if finish {
            println!("Accumulator: {}", accumulator);
            break;
        }

        change_index += 1;
        if change_index as usize>=instructions.len() {
            println!("None found.");
            break;
        }
    }
}

fn run(instructions: &Vec<&str>, change_index: i32) -> (bool, i32) {
    let mut accumulator = 0;
    let mut pos: i32 = 0;
    let mut skip: i32 = 0;
    let mut processed: Vec<i32> = Vec::new();

    loop {
        if pos as usize>=instructions.len() {
            return (true, accumulator);
        }
        if processed.contains(&pos) {
            return (false, 0);
        }
        processed.push(pos);

        let instruction: &str = instructions[pos as usize];
        let cmd = &instruction[..3];
        let num = &instruction[4..].trim().parse::<i32>().unwrap();

        if cmd=="nop" {
            skip += 1;
            if skip==change_index {
                // do a jmp instead
                pos += num;
            } else {
                pos += 1;
            }
        }
        else if cmd=="acc" {
            accumulator += num;
            pos += 1;
        }
        else if cmd=="jmp" {
            skip += 1;
            if skip==change_index {
                // do a nop instead
                pos += 1;
            } else {
                pos += num;
            }
        }
    }
}
