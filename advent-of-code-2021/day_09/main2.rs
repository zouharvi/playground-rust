use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

type Map = Vec<Vec<Point>>;

#[derive(Clone, Hash)]
struct Point {
    pos_x: usize,
    pos_y: usize,

    height: i32,
    visited: bool,
}


impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.pos_x == other.pos_x && self.pos_y == other.pos_y
    }
}
impl Eq for Point {}

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

fn low_risk(point: &mut Point, neighbours: Vec<Point>) -> bool {
    neighbours.iter().all(|x| x.height > point.height)
}

fn main() {
    let file = File::open("day_09/input2").unwrap();
    let reader = BufReader::new(file);
    // load and parse data
    let mut data: Map = reader
        .lines()
        .enumerate()
        .map(|(y_i, l)| {
            let l: String = l.unwrap();
            l.chars()
                .enumerate()
                .map(|(x_i, x)| Point {
                    pos_x: x_i,
                    pos_y: y_i,
                    height: x.to_digit(10).unwrap() as i32,
                    visited: false,
                })
                .collect()
        })
        .collect();

    // find low points
    let mut basins = Vec::<Point>::new();
    for pos_x in 0..data[0].len() {
        for pos_y in 0..data.len() {
            let neighbours = get_neighbours(&data, pos_x, pos_y);
            if low_risk(&mut data[pos_y][pos_x], neighbours) {
                basins.push(data[pos_y][pos_x].clone());
            }
        }
    }

    let mut basin_sizes = Vec::<usize>::new();

    // BFS
    for b_lowpoint in &basins {
        let mut visited = HashSet::<Point>::new();
        let mut queue = Vec::<Point>::new();
        queue.push(b_lowpoint.clone());
        loop {
            let mut to_add = Vec::<Point>::new();
            for el in &queue {
                visited.insert(el.clone());
                let neighbours = get_neighbours(&data, el.pos_x, el.pos_y)
                    .iter()
                    .filter(|x| !visited.contains(x))
                    .filter(|x| x.height != 9)
                    .map(|x| x.clone())
                    .collect::<Vec<Point>>();
                for neighbour in neighbours {
                    to_add.push(neighbour);
                }
            }            
            queue = to_add;

            // finish with this basin
            if queue.is_empty() {
                basin_sizes.push(visited.len());
                break;
            }
        }
    }

    basin_sizes.sort();
    let ans = [1, 2, 3].iter().map(|x| basin_sizes[basin_sizes.len() - x]).product::<usize>();
    println!("{}", ans);
}
