use std::fs;
use std::cmp;
use crate::util;

pub fn go() {
    println!("Day 16");

    let input = fs::read_to_string("inputs/input16.txt")
        .expect("Could not read the input file.");

    let mut tickets : Vec<Ticket> = Vec::new();
    
    // read the ticket values
    let lines : Vec<&str> = input.split(util::LINE_ENDING).collect();
    let mut start = 0usize;

    for row in 0usize.. {
        let line = lines[row];
        if line.len()==0 {
            continue;
        }

        if line=="your ticket:" {
            start = row+1;
            break;
        }

        tickets.push(Ticket::new(line));
    }

    let my_ticket: Vec<i32> = lines[start].split(",").map(|s| { s.parse::<i32>().unwrap() }).collect();
    start += 3;
    
    let mut nearby_tickets: Vec<i32> = Vec::new();
    for row in start..lines.len() {
        let line = lines[row];
        line.split(",").for_each(|s| {
            nearby_tickets.push(s.parse::<i32>().unwrap());
        });
    }

    let total: i32 = nearby_tickets.iter().filter(|&n | { is_invalid(*n, &tickets) }).sum();
    println!("Sum of invalid: {}", total);
}

fn is_invalid(num: i32, tickets: &Vec<Ticket>) -> bool {
    for ticket in tickets {
        for range in ticket.ranges {
            if range.contains(num) {
                return false;
            }
        }
    }

    true
}


struct Ticket {
    name: String,
    ranges: [Range;2]
}

impl Ticket {
    fn new(line: &str) -> Self {
        let parts: Vec<&str> = line.split(": ").collect();
        let name = parts[0];
        let ranges: Vec<&str> = parts[1].split(" or ").collect();
        if ranges.len()!=2 {
            panic!("Unexpected number of ranges");
        }

        Ticket { name: name.to_string(), ranges: [Range::new(ranges[0]), Range::new(ranges[1])] }
    }    
}

#[derive(Clone,Copy)]
struct Range {
    min: i32,
    max: i32
}

impl Range {
    fn new(range: &str) -> Self {
        let parts: Vec<&str> = range.split("-").collect();
        Range { min: parts[0].parse::<i32>().unwrap(), max: parts[1].parse::<i32>().unwrap() }
    }

    fn contains(&self, num: i32) -> bool {
        self.min <= num && self.max >= num
    }

    fn connects(&self, other: Range) -> bool {
        self.min == other.max+1 || self.max == other.min-1
    }
    
    fn overlaps(&self, other: Range) -> bool {
        self.min <= other.max && other.min <= self.max
    }

    fn combine(&self, other: Range) -> Range {
        Range {
            min: cmp::min(self.min, other.min),
            max: cmp::max(self.max, other.max)
        }
    }
}
