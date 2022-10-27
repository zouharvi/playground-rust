use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

enum Instruction {
    Down(i32),
    Up(i32),
    Horizontal(i32),
}

fn main() {
    let file = File::open("day_02/input2").expect("file wasn't found.");
    let reader = BufReader::new(file);

    let instructions: Vec<Instruction> = reader
        .lines()
        .map(|line| {
            let line = line.unwrap();
            let line = line.split_ascii_whitespace().collect::<Vec<&str>>();
            let num = line[1].parse::<i32>().unwrap();
            match line[0] {
                "forward" => Instruction::Horizontal(num), 
                "down" => Instruction::Down(num), 
                "up" => Instruction::Up(num), 
                x => panic!("unknown command {}", x),
            }
        })
        .collect();

    // It was probably not necessary to split the instruction loading and execution

    let mut horizontal : i32 = 0;
    let mut depth : i32 = 0;
    let mut aim : i32 = 0;
    for i in instructions {
        match i {
            Instruction::Horizontal(num) => {
                horizontal += num;
                depth += aim * num;
            },
            Instruction::Down(num) => {
                // depth += num;
                aim += num;
            },
            Instruction::Up(num) => {
                // depth -= num;
                aim -= num;
            },
        };
    }
    println!("{}", horizontal*depth);
}