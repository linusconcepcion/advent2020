use std::fs;
use std::collections::HashMap;
use crate::util;

pub fn go() {
    println!("Day 10");

    let input = fs::read_to_string("inputs/input10.txt")
        .expect("Could not read the input file.");

    let mut jolts: Vec<i32> = input.split(util::LINE_ENDING).into_iter().map(|s| {s.parse::<i32>().unwrap()}).collect();
    jolts.sort();

    jolts.insert(0, 0);
    jolts.push(jolts.iter().max().unwrap()+3);

    // part a
    // let mut ones = 0;
    // let mut threes = 0;
    // let mut last = 0;

    // for jolt in jolts {
    //     let diff = jolt - last;
    //     if diff==1 {
    //         ones += 1;
    //     } else if diff==3 {
    //         threes += 1;
    //     } else {
    //         panic!("shouldn't get here?");
    //     }
    //     last = jolt;
    // }

    // threes+=1;
    // println!("ones: {}, threes: {}, mult: {}", ones, threes, ones * threes);

    let mut cache : HashMap<i32, i64> = HashMap::new();
    let perms = count_permutations_recursive(&jolts[..], &mut cache);
    println!("permutations: {}", perms);
}

// build a tree and then count the leaves
fn count_permutations_recursive(slice: &[i32], cache: &mut HashMap<i32, i64>) -> i64 {
    if slice.len()==0 {
        return 1;
    }

    if let Some(searched) = cache.get(&slice[0]) {
        return *searched;
    }

    // find next branch point
    let mut start = 0;
    loop {
        if slice.len()<=start+2 {
            return 1;
        } else if slice[start+2]-slice[start]>3 {
            start+=1;
        }
        else {
            break;
        }
    }

    let mut branches = 0i64;
    for i in start+1..start+4 {
        if i>=slice.len() {
            break;
        }
        let test = slice[i];
        if test>slice[start] + 3 {
            break;
        }
        branches += count_permutations_recursive(&slice[i..], cache);
    }

    cache.insert(slice[0], branches);
    branches
}


