use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("day_08/input2").unwrap();
    let reader = BufReader::new(file);
    let data = reader
        .lines()
        .map(|l| { 
            let l = l.unwrap();
            let m = l.split(" | ").collect::<Vec<&str>>();
            (
                m[0].split(" ").map(|x| String::from(x)).collect(),
                m[1].split(" ").map(|x| String::from(x)).collect()
            )
        })
        .collect::<Vec<(String, Vec<String>)>>();

    let total = data.iter().map(|(_signal, output)| {
        output.iter().map(|digit| 
            match digit.len() {
                // number 1
                2 => 1,
                // number 4
                4 => 1,
                // number 7
                3 => 1,
                // number 8
                7 => 1,
                _ => 0,
            }
        ).sum::<u32>()
    }
    ).sum::<u32>();
    println!("{}", total);
}
