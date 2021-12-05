use std::fs;
use crate::util;

pub fn go() {
    println!("Day 9");

    let input = fs::read_to_string("inputs/input09.txt")
        .expect("Could not read the input file.");

    let xmas: Vec<i64> = input.split(util::LINE_ENDING).into_iter().map(|s| {s.parse::<i64>().unwrap()}).collect();
    // part a
    // let size = 25;
    // for i in size..xmas.len() {
    //     if !test(xmas[i], &xmas[i-size..i]) {
    //         println!("Invalid number: {}", xmas[i]);
    //         return;
    //     }
    // }

    let check = 23278925;
    let mut start = 0;
    let mut end = 1;
    loop {
        let slice = &xmas[start..end+1];
        let total:i64 = slice.iter().sum();

        if total==check {
            let min = slice.iter().min().unwrap();
            let max = slice.iter().max().unwrap();
            println!("start: {}, end: {}, sum: {}", start, end, min + max);
            return;
        }
        else if total<check {
            // eat more
            end += 1;
        }
        else if total>check {
            start += 1;
            if start==end {
                end += 1;
            }
        }

        if end==xmas.len() {
            break;
        }
    }

    println!("None found.");
}

// part a
fn test(value: i64, priors: &[i64]) -> bool {
    priors.iter().any(|&p| {
        let find = value - p;
        find!=value && priors.contains(&find)
    })
}


