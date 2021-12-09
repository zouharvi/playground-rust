use std::fs::File;
use std::io::{BufRead, BufReader};

type Map = Vec<Vec<Point>>;

#[derive(Clone)]
struct Point {
    height: i32,
    low: bool,
    risk: i32,
}

fn get_neighbours<'a>(data: &'a Map, pos_x: usize, pos_y: usize) -> Vec<Point> {
    let point_left = if pos_x == 0 {
        None
    } else {
        Some(data[pos_y][pos_x - 1].clone())
    };
    let point_right = if pos_x == data[0].len() - 1 {
        None
    } else {
        Some(data[pos_y][pos_x + 1].clone())
    };
    let point_up = if pos_y == 0 {
        None
    } else {
        Some(data[pos_y - 1][pos_x].clone())
    };
    let point_down = if pos_y == data.len() - 1 {
        None
    } else {
        Some(data[pos_y + 1][pos_x].clone())
    };

    vec![point_left, point_right, point_up, point_down]
        .into_iter()
        .filter_map(|x| x)
        .collect()
}

fn mark_low_risk(point: &mut Point, neighbours: Vec<Point>) {
    if neighbours.iter().all(|x| x.height > point.height) {
        point.low = true;
        point.risk = point.height + 1;
    }
}

fn main() {
    let file = File::open("day_09/input2").unwrap();
    let reader = BufReader::new(file);
    // load and parse data
    let mut data: Map = reader
        .lines()
        .map(|l| {
            let l: String = l.unwrap();
            l.chars()
                .map(|x| Point {
                    height: x.to_digit(10).unwrap() as i32,
                    low: false,
                    risk: -1,
                })
                .collect()
        })
        .collect();

    let mut total = 0;
    for pos_x in 0..data[0].len() {
        for pos_y in 0..data.len() {
            let neighbours = get_neighbours(&data, pos_x, pos_y);
            mark_low_risk(&mut data[pos_y][pos_x], neighbours);
            if data[pos_y][pos_x].low {
                total += data[pos_y][pos_x].risk;
            }
        }
    }
    println!("{}", total);
}
