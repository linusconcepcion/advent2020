#![allow(dead_code)]

use std::env;
mod util;

fn main() {
    let args: Vec<String> = env::args().collect();
    let days = [day4, day5, day6, day7, day8, day9, day10, day11, day12, day13, day14, day15, day16];

    if args.len()==1 {
        days.last().unwrap()();
    } else {
        let index = (args[1].parse::<i32>().unwrap()-4) as usize;
        days[index]();
    }
}

mod day04;
fn day4() {
    day04::go();
}

mod day05;
fn day5() {
    day05::go();
}

mod day06;
fn day6() {
    day06::go();
}

mod day07;
fn day7() {
    day07::go();
}

mod day07b;
fn day7b() {
    day07b::go();
}

mod day08;
fn day8() {
    day08::go();
}

mod day09;
fn day9() {
    day09::go();
}

mod day10;
fn day10() {
    day10::go();
}

mod day11;
fn day11() {
    day11::go();
}

mod day12;
fn day12() {
    day12::go();
}

mod day13;
fn day13() {
    day13::go();
}

mod day14;
fn day14() {
    day14::go();
}

mod day15;
fn day15() {
    day15::go();
}

mod day16;
fn day16() {
    day16::go();
}