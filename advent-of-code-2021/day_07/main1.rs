use std::fs::File;
use std::io::{BufRead, BufReader};

fn comp_fuel(crabs: &Vec<i32>, pos: i32) -> i32 {
    crabs.iter().map(|crab| (crab-pos).abs()).sum()
}

fn main() {
    let file = File::open("day_07/input2").unwrap();
    let reader = BufReader::new(file);
    let crabs = reader
        .lines().map(|l| l.unwrap()).collect::<Vec<String>>()[0].split(',')
        .map(|num| {
            num.parse().unwrap()
        })
        .collect::<Vec<i32>>();

    let mut min_fuel = i32::MAX;
    let mut min_fuel_pos = 0;
    for pos in (*crabs.iter().min().unwrap())..(*crabs.iter().max().unwrap()+1) {
        let cur_fuel = comp_fuel(&crabs, pos);
        if cur_fuel < min_fuel {
            min_fuel_pos = pos;
            min_fuel = cur_fuel;
        }
    }
        

    println!("{} {}", min_fuel_pos, min_fuel);
}
