use std::fs::File;
use std::io::{BufRead, BufReader};

struct Line {
    x0: usize,
    x1: usize,
    y0: usize,
    y1: usize,
}

fn main() {
    let file = File::open("day_05/input2").unwrap();
    let reader = BufReader::new(file);
    let mut lines = reader
        .lines()
        .map(|line| {
            let line = line.unwrap();
            let line_vec = line.split(" -> ").collect::<Vec<&str>>();
            let seg0 = line_vec[0].split(',').collect::<Vec<&str>>();
            let seg1 = line_vec[1].split(',').collect::<Vec<&str>>();
            Line {
                x0 : seg0[0].parse().unwrap(),
                y0 : seg0[1].parse().unwrap(),
                x1 : seg1[0].parse().unwrap(),
                y1 : seg1[1].parse().unwrap(),
            }
        })
        .collect::<Vec<Line>>();

    // just use the constant 1000 for now
    let mut data = vec![vec![0; 1000]; 1000];

    for line in lines.iter_mut() {
        // normalize lines
        if line.x0 > line.x1 {
            let tmp = line.x0;
            line.x0 = line.x1;
            line.x1 = tmp;
        }
        if line.y0 > line.y1 {
            let tmp = line.y0;
            line.y0 = line.y1;
            line.y1 = tmp;
        }

        // skip nonstraight lines
        if (line.x0 != line.x1) && (line.y0 != line.y1) {
            continue;
        }

        // this fills a rectangle but if we are given only straight lines it works
        for col_i in line.x0..(line.x1+1) {
            for row_i in line.y0..(line.y1+1) {
                data[row_i][col_i] += 1;
            }
        }
    }



    let mut total_dangerous = 0;
    for row in data {
        for val in row {
            // print!("{}", val);
            if val >= 2 {
                total_dangerous += 1;
            }
        }
        // println!();
    }

    println!("{}", total_dangerous);
}
