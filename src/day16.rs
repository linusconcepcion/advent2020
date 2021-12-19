use std::fs;
use std::cmp;
use crate::util;

type Ticket = Vec<i32>;

pub fn go() {
    println!("Day 16");

    let input = fs::read_to_string("inputs/input16.txt")
        .expect("Could not read the input file.");

    let mut fields : Vec<Field> = Vec::new();
    
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

        fields.push(Field::new(line));
    }

    let mut all_tickets: Vec<Ticket> = Vec::new();

    // my ticket
    all_tickets.push(create_ticket(lines[start]));
    start += 3;

    // nearby tickets
    for row in start..lines.len() {
        all_tickets.push(create_ticket(lines[row]));
    }

    let valid_tickets: Vec<Ticket> = all_tickets.iter().filter(|&t| { sum_of_invalid_numbers(t, &fields)==0 })
        .cloned().collect();

    println!("Valid tickets: {}", valid_tickets.len());

    let myticket = &all_tickets[0];
    let mut mytotal = 0i32;

    set_possible_indexes(&mut fields, &valid_tickets);
    process_possible_indexes(&mut fields);

    for field in fields {
        println!("{}: {:?}", field.name, field.possible_indexes);
    }

}

fn set_possible_indexes(fields: &mut Vec<Field>, valid_tickets: &Vec<Ticket>) {
    for mut field in fields {
        field.possible_indexes = find_possible_indexes(field.ranges, valid_tickets);
        println!("{}: {:?}", field.name, field.possible_indexes);
    }
}

fn process_possible_indexes(fields: &mut Vec<Field>) {
    println!("Processing fields..");
    loop {
        for &mut field in fields.iter_mut() {
            if field.possible_indexes.len()==1 {
                continue;
            }

            let mut ind = field.possible_indexes_mut();
            for pos in 0..field.possible_indexes.len() {
                // see if the number exists as a single in 
                if is_single_number(field.possible_indexes[pos], &fields) {
                    field.possible_indexes.remove(pos);
                }
            }
        }

        // only stop when all fields have only one possible index
        if all_complete(&fields) {
            break;
        }
    }
}

fn is_single_number(num: usize, fields: &Vec<Field>) -> bool {
    fields.iter().any(|f| {f.possible_indexes.len()==1 && f.possible_indexes[0]==num})
}

fn all_complete(fields: &Vec<Field>) -> bool {
    !fields.iter().any(|f| {f.possible_indexes.len()>1})
}

fn find_possible_indexes(ranges: [Range;2], valid_tickets: &Vec<Ticket>) -> Vec<usize> {
    let mut result: Vec<usize> = Vec::new();

    for index in 0..valid_tickets[0].len() {
        let mut valid = true;
        for ticket in valid_tickets {
            let amount = ticket[index];
            if !ranges[0].contains(amount) && !ranges[1].contains(amount) {
                valid = false;
                break;
            }
        }

        if valid {
            result.push(index);
        }
    }

    result
}

fn create_ticket(line: &str) -> Ticket {
    line.split(",").map(|s| { s.parse::<i32>().unwrap() }).collect()
}

fn sum_of_invalid_numbers(ticket: &Ticket, fields: &Vec<Field>) -> i32 {
    ticket.iter().filter(|&n| { is_invalid(*n, fields)}).sum()
}

fn is_invalid(num: i32, fields: &Vec<Field>) -> bool {
    for field in fields {
        for range in field.ranges {
            if range.contains(num) {
                return false;
            }
        }
    }

    true
}

struct Field {
    name: String,
    ranges: [Range;2],
    possible_indexes: Vec<usize>
}

impl Field {
    fn new(line: &str) -> Self {
        let parts: Vec<&str> = line.split(": ").collect();
        let name = parts[0];
        let ranges: Vec<&str> = parts[1].split(" or ").collect();
        if ranges.len()!=2 {
            panic!("Unexpected number of ranges");
        }

        Field { name: name.to_string(), ranges: [Range::new(ranges[0]), Range::new(ranges[1])], possible_indexes: Vec::new() }
    }    

    fn possible_indexes_mut(&mut self) -> &mut Vec<usize> {
        &mut self.possible_indexes
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
