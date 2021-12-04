use std::fs::File;
use std::io::{BufReader,BufRead};

fn main() {
    let file = File::open("day_03/input2").expect("file wasn't found.");
    let reader = BufReader::new(file);

    let lines = reader.lines().map(|line| line.unwrap()).collect::<Vec<String>>(); 
    let mut data = vec![0; lines[0].len()];

    for line in &lines {
        for (i, c) in line.chars().into_iter().enumerate() {
            if c == '1' {
                data[i] += 1;
            }
        }
    }

    
    let mut gamma : u32 = 0;
    let mut epsilon : u32 = 0;
    let base : u32 = 2;
    for (i, c) in data.into_iter().rev().enumerate() {
        if c > lines.len()/2 {
            gamma += base.pow(i.try_into().unwrap());
        } else {
            epsilon += base.pow(i.try_into().unwrap());
        }
    }
    
    println!("{}", gamma*epsilon);
}