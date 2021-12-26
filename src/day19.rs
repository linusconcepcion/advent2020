use std::fs;
use std::collections::HashMap;
use crate::util;

pub fn go() {
    println!("Day 19");

    let input = fs::read_to_string("inputs/input19.txt")
        .expect("Could not read the input file.");

    let lines : Vec<&str> = input.split(util::LINE_ENDING).collect();
    let mut rules : HashMap<i32, Rule> = lines.into_iter().filter(|&l| {l.contains(":")})
        .map(|l| {Rule::create_tuple_mut(l)})
        .collect();

    let rulezero = rules.get_mut(&0).unwrap();
    let possible = rulezero.get_possible_values(&mut rules);
}

struct Rule {
    number: i32,
    definition: String,
    possible_values: Vec<String>
}

impl Rule {
    fn new(line: &str) -> Self {
        let parts: Vec<&str> = line.split(": ").collect();
        Rule { number: parts[0].parse::<i32>().unwrap(), definition: parts[1].to_string(), possible_values: vec![]}
    }

    fn create_tuple_mut(line: &str) -> (i32, Rule) {
        let mut rule = Rule::new(line);
        (rule.number, rule)
    }

    fn get_possible_values(&mut self, all: &mut HashMap<i32, Rule>) -> &Vec<String> {
        if self.possible_values.len()==0 {
            // process the definition
            if self.definition.contains("\"") {
                self.possible_values.push(self.definition[1..2].to_string());
            } else {
                let ors = self.definition.split(" | ");
                self.possible_values = ors.map(|o| { Rule::expand_value(o, all) })
                    .flatten().collect();
            }
        }

        &self.possible_values
    }

    fn expand_value(ids: &str, all: &mut HashMap<i32, Rule>) -> Vec<String> {
        let idvals: Vec<i32> = ids.split(' ').map(|s| { s.parse::<i32>().unwrap() }).collect();
        if idvals.len()==1 {
            let x = idvals[0];
            return all.get_mut(&x).unwrap().get_possible_values(all).to_vec();
        }
        else if idvals.len()==2 {
            let x = idvals[0];
            let y = idvals[1];
            let mut result = Vec::<String>::new();
            for x1 in all.get_mut(&x).unwrap().get_possible_values(all) {
                for y1 in all.get_mut(&y).unwrap().get_possible_values(all) {
                    result.push(x1.to_owned() + y1);
                }
            }
            return result;
        }
        else if idvals.len()==3 {
            let x = idvals[0];
            let y = idvals[1];
            let z = idvals[2];
            let mut result = Vec::<String>::new();
            for x1 in all.get_mut(&x).unwrap().get_possible_values(all) {
                for y1 in all.get_mut(&y).unwrap().get_possible_values(all) {
                    for z1 in all.get_mut(&z).unwrap().get_possible_values(all) {
                        result.push(x1.to_owned() + y1 + z1);
                    }
                }
            }
            return result;
        }
        else {
            panic!("Unexpected length: {}", idvals.len());
        }
    } 
}