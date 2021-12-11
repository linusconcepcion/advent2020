use std::fs;
use std::collections::HashMap;
use crate::util;

pub fn go() {
    println!("Day 14");

    let input = fs::read_to_string("inputs/input14.txt")
        .expect("Could not read the input file.");

    let mut masks : Vec<String> = Vec::new();
    let mut mem : HashMap<i64, i64> = HashMap::new();

    let lines : Vec<&str> = input.split(util::LINE_ENDING).collect();
    for line in lines {

        let parts : Vec<&str> = line.split(" = ").collect();
        match parts[0] {
            "mask" => {
                masks = mask_permutations(parts[1])
            }
            _ => {
                let nums = &parts[0][4..parts[0].len()-1];
                let to_mask = str::parse::<i64>(nums).unwrap();
                let val = str::parse::<i64>(parts[1]).unwrap();

                for mask in &masks {
                    let key = apply_bitmask(mask, to_mask);
                    mem.insert(key, val);
                }
            }
        }
    }

    let total : i64 = mem.values().sum();
    println!("total: {}", total);
}

fn mask_permutations(mask: &str) -> Vec<String> {
    let mut result : Vec<String> = Vec::new();
    if mask.len()==0 {
        return result;
    }

    let firstchar = mask.chars().nth(0).unwrap();
    let digits = match firstchar {
        '0' => ["X"].to_vec(),
        '1' => ["1"].to_vec(),
        'X' => ["0","1"].to_vec(),
        _ => panic!("unexpected character: {}", firstchar)
    };

    let rest = mask_permutations(&mask[1..]);
    for digit in digits {
        if rest.len()==0 {
            result.push(digit.to_string());
        } else {
            for add in &rest {
                let combined = digit.to_owned() + add;
                result.push(combined);
            }
        }
    }

    result
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