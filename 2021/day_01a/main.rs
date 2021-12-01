use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

fn main() {
    let file = File::open("input2").expect("file wasn't found.");
    let reader = BufReader::new(file);

    let numbers_1: Vec<u32> = reader
        .lines()
        .map(|line| line.unwrap().parse::<u32>().unwrap())
        .collect();

    let result = numbers_1[1..].into_iter().zip(&numbers_1).map(|(a, b)| {
        if b < a {
            1
        } else {
            0
        }
    }).sum::<u32>();
    
    println!("{}", result);
}