use std::fs;

fn main() {
    let input = fs::read_to_string("src/input5.txt")
        .expect("Could not read the input file.");

    let mut seatids = input.split('\n').map(|s| { get_seatid(s) }).collect::<Vec<i32>>();
    seatids.sort();

    for index in 0..seatids.len() {
        let seatid = seatids[index];
        if index>0 && seatids[index-1]!=(seatid-1) {
            println!("Missing: {}", seatid-1);
            break;
        }
    }
}

fn get_seatid(input: &str) -> i32 {
    let row = get_num(&input[0..7], 'F', 'B', 0, 127);
    let col = get_num(&input[7..10], 'L', 'R', 0, 7);
    (row * 8) + col
}

fn get_num(input: &str, upchar: char, downchar: char, mut start: i32, mut end: i32) -> i32 {
    for ch in input.chars() {
        if ch==upchar {
            let half = (end - start + 1)/2;
            end = start + half - 1;
        } else if ch==downchar {
            let half = (end - start + 1)/2;
            start = start + half;
        }
        else {
            panic!("Invalid character {}", ch);
        }
    }
    if start != end {
        panic!("Not enough characters");
    }
    start
}

#[test]
fn test_get_num() {
    assert_eq!(get_num("BFFFBBF", 'F', 'B', 0, 127), 70);
    assert_eq!(get_num("FFFBBBF", 'F', 'B', 0, 127), 14);
}
