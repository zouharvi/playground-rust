// use std::collections::{HashMap, HashSet};

struct Point {
    x: usize,
    y: usize,
}

#[derive(Clone, Copy)]
enum Fold {
    AlongY(usize),
    AlongX(usize),
}

fn matrix_transpose(m: Vec<Vec<bool>>) -> Vec<Vec<bool>> {
    let mut t = vec![Vec::with_capacity(m.len()); m[0].len()];
    for r in m {
        for i in 0..r.len() {
            t[i].push(r[i]);
        }
    }
    t
}

fn matrix_print(data: &Vec<Vec<bool>>) {
    for row in data {
        for c in row {
            if *c {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
    println!();
}

fn apply_fold(data_old: Vec<Vec<bool>>, fold: Fold) -> Vec<Vec<bool>> {
    match fold {
        Fold::AlongY(pivot) => {
            let mut data_new = vec![vec![false; data_old[0].len()]; pivot];
            for (y, row) in data_new.iter_mut().enumerate() {
                for (x, val) in row.iter_mut().enumerate() {
                    *val = data_old[y][x] || data_old[data_old.len() - y - 1][x];
                }
            }
            return data_new;
        }
        Fold::AlongX(i) => {
            let data_old = matrix_transpose(data_old);
            let data_new = apply_fold(data_old, Fold::AlongY(i));
            return matrix_transpose(data_new);
        }
    };
}

fn main() {
    let mut points = Vec::<Point>::new();
    let mut folds = Vec::<Fold>::new();

    for line in include_str!("input2").lines() {
        if line.contains("fold along x=") {
            folds.push(Fold::AlongX(
                line.split_once('=').unwrap().1.parse().unwrap(),
            ));
        } else if line.contains("fold along y=") {
            folds.push(Fold::AlongY(
                line.split_once('=').unwrap().1.parse().unwrap(),
            ));
        } else if line.len() > 0 {
            let (x, y) = line.split_once(',').unwrap();
            points.push(Point {
                x: x.parse().unwrap(),
                y: y.parse().unwrap(),
            });
        }
    }

    let max_x = points.iter().map(|p| p.x).max().unwrap();
    let max_y = points.iter().map(|p| p.y).max().unwrap();

    // load data
    let mut data = vec![vec![false; max_x+1]; max_y+1];
    for p in points {
        data[p.y][p.x] = true;
    }

    for fold in folds {
        data = apply_fold(data, fold);
    }
    matrix_print(&data);
}
