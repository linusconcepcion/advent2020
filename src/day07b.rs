use std::fs;
use std::collections::HashMap;
use crate::util;

pub fn go() {
    println!("Day 7b");

    let input = fs::read_to_string("inputs/input07.txt")
        .expect("Could not read the input file.");

    let rules : HashMap<String, Rule> = input.split(util::LINE_ENDING).map(|s| { 
        let rule = parse_rule(s);
        (rule.bag_name.clone(), rule)
    } ).collect();

    let shiny_gold = rules.get("shiny gold").unwrap();
    let count = count_bags_recursive(shiny_gold, &rules);

    println!("Result: {}", count-1);
}

fn count_bags_recursive(rule: &Rule, all: &HashMap<String, Rule>) -> i32 {
    if rule.contains.len()==0 {
        return 1;
    } else {
        let mut total = 1;
        for inner in &rule.contains {
            let inner_rule = all.get(&inner.bag_name).unwrap();
            total += inner.max_count * count_bags_recursive(inner_rule, all);
        }
        total
    }
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
