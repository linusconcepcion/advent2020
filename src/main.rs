use std::env;
mod util;

fn main() {
    let args: Vec<String> = env::args().collect();
    let days = [day4, day5, day6];

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
