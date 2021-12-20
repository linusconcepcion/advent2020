use std::fs;
use crate::util;

pub fn go() {
    println!("Day 18");

    let input = fs::read_to_string("inputs/input18.txt")
        .expect("Could not read the input file.");

    let lines : Vec<&str> = input.split(util::LINE_ENDING).collect();
    let mut sum = 0i64;
    for line in lines {
        let result = process_line(line);
        println!("{} = {}", line, result);

        sum += result;
    }

    println!();
    println!("Total: {}", sum);
}

fn process_line(line: &str) -> i64 {
    let mut pos = 0;
    let mut result = 0;
    let mut last_op = ' ';

    loop {
        let (val, nextpos) = get_value_at(&line, pos);

        if last_op == '+' {
            result = result + val;
        } else if last_op == '*' {
            result = result * val;
        } else {
            result = val;
        }

        if nextpos>=line.len() {
            break;
        }
        pos = nextpos;

        let op = &line[pos..pos+3];
        if op == " + " {
            last_op = '+';
        } else if op == " * " {
            last_op = '*';
        }
        pos += 3;
    }

    result
}

fn get_value_at(line: &str, pos: usize) -> (i64, usize) {
    let ch = line.chars().nth(pos).unwrap();
    if ch=='(' {
        // find the matching close paren
        let mut count = 1;
        let mut close = 0usize;
        for i in pos+1..line.len() {
            let cl = line.chars().nth(i).unwrap();
            if cl=='(' {
                count += 1;
            } else if cl==')' {
                count -= 1;
                if count==0 {
                    close = i;
                    break;
                }
            }
        }

        if count!=0 {
            panic!("No matching closing paren found.");
        }

        (process_line(&line[pos+1..close]), close+1)

    } else if ch>='0' && ch<='9' {
        (ch.to_digit(10).unwrap() as i64, pos+1)
    } else {
        panic!("Unexpected character!");
    }
}



