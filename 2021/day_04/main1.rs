use std::fs::File;
use std::io::{BufRead, BufReader};

type Table = Vec<Vec<(u32, bool)>>;

fn check_winning(table: &Table) -> bool {
    // find succes in rows
    for row in table {
        if row.into_iter().all(|(_val, drawn)| *drawn) {
            return true;
        }
    }

    // find success in columns
    for col_i in 0..table[0].len() {
        let mut strike = true;
        for row in table {
            let (val, drawn) = row[col_i];
            if !drawn {
                strike = false;
                break;
            }
        }
        if strike {
            return true;
        }
    }

    false
}

fn unmarked_sum(table: &Table) -> u32 {
    let mut sum: u32 = 0;
    for row in table {
        sum += row
            .into_iter()
            .filter(|(val, drawn)| !drawn)
            .map(|(val, drawn)| val)
            .sum::<u32>();
    }
    return sum;
}

fn apply_val(table: &mut Table, val: u32) {
    for row in table {
        for i in 0..row.len() {
            if row[i].0 == val {
                row[i].1 = true;
            }
        }
    }
}

fn main() {
    let file = File::open("day_04/input2").expect("file wasn't found.");
    let reader = BufReader::new(file);
    let lines = reader
        .lines()
        .map(|line| line.unwrap())
        .collect::<Vec<String>>();

    let drawn_nums: Vec<u32> = lines[0]
        .split(',')
        .map(|num| num.parse().unwrap())
        .collect();

    let mut tables: Vec<Table> = lines[1..]
        .chunks(6)
        .map(|chunk| {
            chunk[1..]
                .into_iter()
                .map(|line| {
                    line.split(' ')
                        .filter(|num| num.len() != 0)
                        .map(|num| (num.parse().unwrap(), false))
                        .collect()
                })
                .collect()
        })
        .collect();

    for drawn_num in drawn_nums {
        for table in tables.iter_mut() {
            apply_val(table, drawn_num);
            if check_winning(table) {
                println!("{}", unmarked_sum(table) * drawn_num);
                return;
            }
        }
    }
}
