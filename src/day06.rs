use std::fs;

pub fn go() {
    println!("Day 6");

    let input = fs::read_to_string("inputs/input06.txt")
        .expect("Could not read the input file.");

    let groups = input.split("\n\n");
    println!("Distinct sum: {}", groups.map(|s| { count_letters(s) }).sum::<i32>());
}

fn count_letters(input: &str) -> i32 {
    let people : Vec<&str> = input.split('\n').collect();
    if people.len()==1 {
        return people[0].len() as i32;
    }
    else {
        let test = people[0];
        let mut count = 0;
        for ch in test.chars() {
            let mut found = true;
            for index in 1..people.len() {
                if !people[index].contains(ch) {
                    found = false;
                    break;
                }
            }

            if found {
                count+=1;
            }
        }
        count
    }
}