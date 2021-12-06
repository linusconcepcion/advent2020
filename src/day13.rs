use std::fs;
use crate::util;

pub fn go() {
    println!("Day 13");

    let input = fs::read_to_string("inputs/input13.txt")
        .expect("Could not read the input file.");

    let lines : Vec<&str> = input.split(util::LINE_ENDING).collect();

    let mintime = lines.get(0).unwrap().parse::<i32>().unwrap();
    let buses = lines.get(1).unwrap();

    let mut minbusid = 0;
    let mut minbustime = 100000000;  // just a big number

    for busid in buses.split(',') {
        if busid=="x" {
            continue;
        }

        let bus = busid.parse::<i32>().unwrap();
        let ratio = (mintime as f32 / bus as f32).ceil();

        let time = (ratio as i32)*bus;
        if time < minbustime {
            minbustime = time;
            minbusid = bus;
        }
    }

    let minutes = minbustime - mintime;
    println!("Minutes: {}, Bus: {}, Anser: {}", minutes, minbusid, minbusid * minutes);
}