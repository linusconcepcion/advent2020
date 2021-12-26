use std::fs;
use std::collections::HashMap;
use crate::util;

pub fn go() {
    println!("Day 19");

    let input = fs::read_to_string("inputs/input19.txt")
        .expect("Could not read the input file.");

    let lines : Vec<&str> = input.split(util::LINE_ENDING).collect();
    let newlines = lines.clone();

    let rules : HashMap<i32, Rule> = newlines.into_iter().filter(|&l| {l.contains(":")})
        .map(|l| {Rule::create_tuple(l)})
        .collect();

    let mut cache : HashMap<i32, Vec<String>> = HashMap::new();
    let possible : HashMap<String, i32> = Rule::get_possible_values_from_id(&rules, &mut cache, 0)
        .into_iter().map(|s| {(s, 0)}).collect();

    println!("all possible entries found.");

    let count = lines.into_iter().filter(|&l| {(l.starts_with("a") || l.starts_with("b")) && possible.contains_key(l)}).count();
    println!("count: {}", count);
}

struct Rule {
    number: i32,
    definition: String
}

impl Rule {
    fn new(line: &str) -> Self {
        let parts: Vec<&str> = line.split(": ").collect();
        Rule { number: parts[0].parse::<i32>().unwrap(), definition: parts[1].to_string() }
    }

    fn create_tuple(line: &str) -> (i32, Rule) {
        let rule = Rule::new(line);
        (rule.number, rule)
    }

    fn get_possible_values_from_id(all: &HashMap<i32, Rule>, cache: &mut HashMap<i32, Vec<String>>, id: i32) -> Vec<String> {
        let rulezero = all.get(&id).unwrap();
        rulezero.get_possible_values(all, cache)
    }

    fn get_possible_values(&self, all: &HashMap<i32, Rule>, cache: &mut HashMap<i32, Vec<String>>) -> Vec<String> {
        let result = match cache.get(&self.number) {
            Some(s) => s,
            None => {
                if self.definition.contains("\"") {
                    return vec![self.definition[1..2].to_string()];
                } else {
                    let ors = self.definition.split(" | ");
                    return ors.map(|o| { Rule::expand_value(o, all, cache) })
                        .flatten().collect();
                }
            }
        };
        
        result.clone()
    }

    fn expand_value(ids: &str, all: &HashMap<i32, Rule>, cache: &mut HashMap<i32, Vec<String>>) -> Vec<String> {
        let idvals: Vec<i32> = ids.split(' ').map(|s| { s.parse::<i32>().unwrap() }).collect();
        if idvals.len()==1 {
            let x = idvals[0];
            return Rule::get_possible_values_from_id(all, cache, x).to_vec();   //all.get_mut(&x).unwrap().get_possible_values(all).to_vec();
        }
        else if idvals.len()==2 {
            let x = idvals[0];
            let y = idvals[1];
            let mut result = Vec::<String>::new();
            for x1 in Rule::get_possible_values_from_id(all, cache, x) { //all.get_mut(&x).unwrap().get_possible_values(all) {
                for y1 in Rule::get_possible_values_from_id(all, cache, y) { //all.get_mut(&y).unwrap().get_possible_values(all) {
                    result.push(x1.clone() + &y1);
                }
            }
            return result;
        }
        else if idvals.len()==3 {
            let x = idvals[0];
            let y = idvals[1];
            let z = idvals[2];
            let mut result = Vec::<String>::new();
            for x1 in Rule::get_possible_values_from_id(all, cache, x) {//all.get_mut(&x).unwrap().get_possible_values(all) {
                for y1 in Rule::get_possible_values_from_id(all, cache, y) {//all.get_mut(&y).unwrap().get_possible_values(all) {
                    for z1 in Rule::get_possible_values_from_id(all, cache, z) {//all.get_mut(&z).unwrap().get_possible_values(all) {
                        result.push(x1.clone() + &y1 + &z1);
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
