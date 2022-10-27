use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;

fn main() {
    let file = File::open("day_06/input2").unwrap();
    let reader = BufReader::new(file);
    let fish_raw = reader
        .lines().map(|l| l.unwrap()).collect::<Vec<String>>()[0].split(',')
        .map(|num| {
            num.parse().unwrap()
        })
        .collect::<Vec<u32>>();

    let mut fish = HashMap::<u32,usize>::new();
    for f in fish_raw {
        *fish.entry(f).or_insert(0) += 1;
    }
    
    for day in 0..256 {
        let mut fish_next = HashMap::<u32,usize>::new();
        println!("day {}", day);
        let mut new_fish = 0;
        for (f,count) in fish.iter_mut() {
            if *f == 0 {
                new_fish += *count;
                *fish_next.entry(6).or_insert(0) += *count;
            } else {
                *fish_next.entry(*f-1).or_insert(0) += *count;
            }
        }
        // spawn new fish
        *fish_next.entry(8).or_insert(0) += new_fish;

        // replace old container
        fish = fish_next;
    }

    println!("{}", fish.iter().map(|(f,count)| count).sum::<usize>());
}
