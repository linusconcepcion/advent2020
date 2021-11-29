use std::fs;
use regex::Regex;
use crate::util;

pub fn go() {
    println!("Day 4");

    let input = fs::read_to_string("inputs/input04.txt")
        .expect("Could not read the input file.");
    
    let passports = input.split(util::LINE_ENDING2);
    let mut index = 1;
    let mut legal = 0;
    for pass in passports {
        if is_legal(pass) {
            println!("#{} Legal: {}\n", index, pass);
            legal += 1;
        } else {
            println!("#{} Illegal: {}\n", index, pass);
        }
        index += 1;
    }
    println!("{}", legal);
}

fn is_legal(str: &str) -> bool {
    let checks = ["byr:","iyr:","eyr:","hgt:","hcl:","ecl:","pid:"];
    if checks.into_iter().any(| p| { !str.contains(p) }) {
        return false;
    }

    !str.split(|c| { c==' ' || c=='\n' || c=='\r'}).any(|s| { !is_valid_part(s) })    
}

fn is_valid_part(str: &str) -> bool {
    if str.len()==0 {
        return true;
    }

    let parts: Vec<&str> = str.trim().split(':').collect();
    if parts.len()!=2 {
        return false;
    }
    let code = parts[0];
    let val = parts[1];
    match code {
        "byr" => is_num_legal(val, 1920, 2002),
        "iyr" => is_num_legal(val, 2010, 2020),
        "eyr" => is_num_legal(val, 2020, 2030),
        "hgt" => is_hgt_legal(val),
        "hcl" => is_hcl_legal(val),
        "ecl" => is_ecl_legal(val),
        "pid" => is_pid_legal(val),
        "cid" => true,
        _ => false
    }
}

fn is_num_legal(str: &str, min: i32, max: i32) -> bool {
    match str.parse::<i32>() {
        Err(_) => false,
        Ok(n) => min <= n && n <= max
    }
}

fn is_hgt_legal(str: &str) -> bool {
    if str.len()<3 {
        return false;
    }
    let num = &str[0..(str.len()-2)];
    let unit = &str[(str.len()-2)..];   
    match unit {
        "in" => is_num_legal(num, 59, 76),
        "cm" => is_num_legal(num, 150, 193),
        _ => false
    }
}

fn is_hcl_legal(str: &str) -> bool {
    let re = Regex::new("^#(?:[0-9a-fA-F]{6})$").unwrap();
    re.is_match(str)
}

fn is_ecl_legal(str: &str) -> bool {
    let checks = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
    checks.into_iter().any(| p| { str == p })
}

fn is_pid_legal(str: &str) -> bool {
    let re = Regex::new("^(?:[0-9]{9})$").unwrap();
    re.is_match(str)
}

#[test]
fn test_height() {
    assert!(is_hgt_legal("59in"));
    assert!(!is_hgt_legal("abc"));
    assert!(is_hgt_legal("193cm"));
    assert!(!is_hgt_legal("200cm"));
}

#[test]
fn test_color() {
    assert!(is_hcl_legal("#FF1243"));
    assert!(!is_hcl_legal("#FF12243"));
    assert!(is_hcl_legal("#Ff1243"));
    assert!(!is_hcl_legal("#FG1243"));
}




