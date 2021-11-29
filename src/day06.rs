use std::fs;
use crate::util;

pub fn go() {
    println!("Day 6");

    let input = fs::read_to_string("inputs/input06.txt")
        .expect("Could not read the input file.");

    let groups = input.split(util::LINE_ENDING2);
    println!("Distinct sum: {}", groups.map(|s| { count_letters(s) }).sum::<i32>());
}

fn count_letters(input: &str) -> i32 {
    let people : Vec<&str> = input.split(util::LINE_ENDING).collect();
    if people.len()==1 {
        return people[0].len() as i32;
    }
    else {
        let test = people[0];
        test.chars().filter(|&c| {
            !people.iter().any(|&s| { !s.contains(c) })
        }).count() as i32
    }
}