use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

enum Instruction {
    Vertical(i32),
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
                "down" => Instruction::Vertical(num), 
                "up" => Instruction::Vertical(-num), 
                x => panic!("unknown command {}", x),
            }
        })
        .collect();

    let mut horizontal : i32 = 0;
    let mut vertical : i32 = 0;
    for i in instructions {
        match i {
            Instruction::Vertical(num) => vertical += num,
            Instruction::Horizontal(num) => horizontal += num,
        };
    }
    println!("{}", horizontal*vertical);
}