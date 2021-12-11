use std::fs;
use std::collections::HashMap;
use crate::util;

pub fn go() {
    println!("Day 14");

    let input = fs::read_to_string("inputs/input14.txt")
        .expect("Could not read the input file.");

    let mut mask = "".to_string();
    let mut mem : HashMap<i32, i64> = HashMap::new();

    let lines : Vec<&str> = input.split(util::LINE_ENDING).collect();
    for line in lines {

        let parts : Vec<&str> = line.split(" = ").collect();
        match parts[0] {
            "mask" => mask = parts[1].to_string(),
            _ => {
                let nums = &parts[0][4..parts[0].len()-1];
                let key = str::parse::<i32>(nums).unwrap();
                let to_mask = str::parse::<i64>(parts[1]).unwrap();

                mem.insert(key, apply_bitmask(&mask, to_mask));
            }
        }
    }

    let total : i64 = mem.values().sum();
    println!("total: {}", total);
}

fn apply_bitmask(mask: &str, to_mask: i64) -> i64 {
    let bits = format!("{:b}", to_mask);
    let diflen = mask.len() - bits.len();

    let mut newbits = "".to_string();

    for n in 0..mask.len() {
        let maskchar = mask.chars().nth(n).unwrap();
        let pos: i32 = n as i32 - diflen as i32;

        newbits.push(match maskchar {
            'X' => {
                if pos<0 {
                    '0'
                } else {
                    bits.chars().nth(pos as usize).unwrap()
                }
            },
            ch => ch
        })
    }

    i64::from_str_radix(&newbits, 2).unwrap()
}