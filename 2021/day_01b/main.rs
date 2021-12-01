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

    let numbers_x = numbers_1[..].into_iter().zip(&numbers_1[1..]).zip(&numbers_1[2..]).map(|((a,b), c)| {
        a + b + c
    }).collect::<Vec<u32>>();

    let result = numbers_x[1..].into_iter().zip(&numbers_x).map(|(a, b)| {
        if b < a {
            1
        } else {
            0
        }
    }).sum::<u32>();
    
    println!("{}", result);
}