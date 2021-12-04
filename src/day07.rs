use std::fs;
use std::collections::HashMap;
use crate::util;

pub fn go() {
    println!("Day 7");

    let input = fs::read_to_string("inputs/input07.txt")
        .expect("Could not read the input file.");

    let rules : HashMap<String, Rule> = input.split(util::LINE_ENDING).map(|s| { 
        let rule = parse_rule(s);
        (rule.bag_name.clone(), rule)
    } ).collect();

    let mut count = 0;
    let mut processed: HashMap<String, bool> = HashMap::new();

    for bag in &rules {
        if process_bag(bag.1, &mut processed, &rules) {
            count += 1;
        }
    }

    println!("Result: {}", count);
}

fn process_bag(rule: &Rule, processed: &mut HashMap<String, bool>, rules: &HashMap<String, Rule>) -> bool {
    match processed.get(&rule.bag_name) {
        None => {
            let check = process_bag_impl(rule, processed, rules);
            //processed.insert(rule.bag_name, check);
            check            
        },
        Some(&result) => result
    }
}

fn process_bag_impl(rule: &Rule, processed: &mut HashMap<String, bool>, rules: &HashMap<String, Rule>) -> bool {
    if rule.contains.len()==0 {
        return false;
    }

    for bags in &rule.contains {
        if &bags.bag_name == "shiny gold" {
            return true;
        }

        let can_have = match rules.get(&bags.bag_name) {
            Some(findrule) => process_bag(findrule, processed, rules),
            None => false
        };

        if can_have {
            return true;
        }
    }

    false
}

#[derive(Debug)]
struct Rule {
    bag_name: String,
    contains: Vec<BagCount>
}

impl Rule {
    fn new(name: String, bags: String) -> Self {
        Rule { bag_name: name, contains: Rule::parse_bags(bags) }
    }

    fn parse_bags(bags: String) -> Vec<BagCount> {
        let bags : Vec<&str> = bags.split(", ").collect();
        if bags[0]=="no other" {
            return Vec::new();
        } 

        bags.iter().map(|s| { BagCount::new(s.to_string()) }).collect()
    }
}

#[derive(Debug)]
struct BagCount {
    bag_name: String,
    max_count: i32,
}

impl BagCount {
    fn new(bagcount: String) -> Self {
        let bag : Vec<&str> = bagcount.split(" ").collect();
        let count = bag[0].parse::<i32>().unwrap();
        let desc =  bag[1..].join(" ");

        BagCount { bag_name: desc, max_count: count }
    }
}

fn parse_rule(line: &str) -> Rule {
    let repl = line.replace(" bags", "").replace(" bag", "").replace(".", "");
    let parts: Vec<&str> = repl.split(" contain ").collect();

    if parts.len()!=2 {
        panic!("Expected two parts to rule {}:", line);
    }

    let key = parts[0].trim().to_string();
    Rule::new(key, parts[1].to_string())
}
