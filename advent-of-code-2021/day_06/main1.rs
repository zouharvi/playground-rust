use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("day_06/input2").unwrap();
    let reader = BufReader::new(file);
    let mut fish = reader
        .lines().map(|l| l.unwrap()).collect::<Vec<String>>()[0].split(',')
        .map(|num| {
            num.parse().unwrap()
        })
        .collect::<Vec<u32>>();


    for day in 0..80 {
        let mut new_fish = Vec::<u32>::new();
        for f in fish.iter_mut() {
            if *f == 0 {
                new_fish.push(8);
                *f = 6;
            } else {
                *f -= 1;
            }
        }
        fish.append(&mut new_fish);
    }

    println!("{}", fish.len());
}
