use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("day_03/input2").expect("file wasn't found.");
    let reader = BufReader::new(file);

    let mut lines_gamma = reader
        .lines()
        .map(|line| line.unwrap().chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    let mut lines_epsilon = lines_gamma.clone();
    
    for pos in 0..(lines_gamma[0].len()) {
        let count_1 : i32 = lines_gamma.iter().map(|line| match line[pos] {
            '0' => 0,
            '1' => 1,
            _ => panic!("unknown input"),
        }).sum();
        let cur_len : i32 = lines_gamma.len().try_into().unwrap();
        let count_0 : i32 = cur_len - count_1;
        let key = if count_1 >= count_0 { '1' } else { '0' };
        lines_gamma = lines_gamma.into_iter().filter(|line| line[pos] == key).collect();
    }
    let gamma = isize::from_str_radix(lines_gamma[0].clone().into_iter().collect::<String>().as_str(), 2).unwrap();

    for pos in 0..(lines_epsilon[0].len()) {
        let count_1 : i32 = lines_epsilon.iter().map(|line| match line[pos] {
            '0' => 0,
            '1' => 1,
            _ => panic!("unknown input"),
        }).sum();
        let cur_len : i32 = lines_epsilon.len().try_into().unwrap();
        let count_0 : i32 = cur_len - count_1;
        let key = if cur_len == 1 {
            lines_epsilon[0][pos]
        } else {
            if count_1 < count_0 { '1' } else { '0' }
        };
        lines_epsilon = lines_epsilon.into_iter().filter(|line| line[pos] == key).collect();
    }
    let epsilon = isize::from_str_radix(lines_epsilon[0].clone().into_iter().collect::<String>().as_str(), 2).unwrap();

    println!("{}", gamma* epsilon);
}
