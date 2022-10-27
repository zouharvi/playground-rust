use std::fs::File;
use std::io::{BufRead, BufReader};

struct Octopus {
    energy: u32,
    flashed: bool,
    x: usize,
    y: usize,
}

fn apply_all<F>(data: &mut Vec<Vec<Octopus>>, mut f: F)
where
    F: FnMut(&mut Octopus, usize, usize),
{
    let dim_x = data[0].len();
    let dim_y = data.len();
    for row in data.iter_mut() {
        for octopus in row.iter_mut() {
            f(octopus, dim_x, dim_y);
        }
    }
}

fn main() {
    let file = File::open("day_11/input2").unwrap();
    let reader = BufReader::new(file);
    // load and parse data
    let mut data: Vec<Vec<Octopus>> = reader
        .lines()
        .enumerate()
        .map(|(line_i, line)| {
            line.unwrap()
                .chars()
                .enumerate()
                .map(|(c_i, c)| Octopus {
                    energy: c.to_digit(10).unwrap(),
                    flashed: false,
                    x: c_i,
                    y: line_i,
                })
                .collect()
        })
        .collect();

    let mut flashes = 0;
    for step in 0..1000 {
        let mut to_increase = Vec::<(usize, usize)>::new();

        // add all to to_increase
        apply_all(&mut data, |octopus, _, _| {
            to_increase.push((octopus.y, octopus.x))
        });

        let mut flashes_this_step = 0;
        while !to_increase.is_empty() {
            // increase energy level
            for (y, x) in to_increase.iter() {
                data[*y][*x].energy += 1;
            }

            // clear queue
            to_increase = Vec::<(usize, usize)>::new();

            apply_all(&mut data, |octopus, dim_x, dim_y| {
                if octopus.energy > 9 && !octopus.flashed {
                    flashes += 1;
                    flashes_this_step += 1;
                    octopus.flashed = true;

                    // left side
                    if octopus.x > 0 {
                        to_increase.push((octopus.y, octopus.x - 1));
                        if octopus.y > 0 {
                            to_increase.push((octopus.y - 1, octopus.x - 1));
                        }
                        if octopus.y < dim_y - 1 {
                            to_increase.push((octopus.y + 1, octopus.x - 1));
                        }
                    }

                    // right side
                    if octopus.x < dim_x - 1 {
                        to_increase.push((octopus.y, octopus.x + 1));
                        if octopus.y > 0 {
                            to_increase.push((octopus.y - 1, octopus.x + 1));
                        }
                        if octopus.y < dim_y - 1 {
                            to_increase.push((octopus.y + 1, octopus.x + 1));
                        }
                    }

                    // top
                    if octopus.y > 0 {
                        to_increase.push((octopus.y - 1, octopus.x));
                    }
                    // bottom
                    if octopus.y < dim_y - 1 {
                        to_increase.push((octopus.y + 1, octopus.x));
                    }
                }
            });
        }

        if flashes_this_step == data.len() * data[0].len() {
            println!("Sync flashed at {} (add 1 to this for answer)", step);
            return;
        }

        // nullify flashed octopuses for the next step
        apply_all(&mut data, |octopus, _, _| {
            if octopus.flashed {
                octopus.energy = 0;
                octopus.flashed = false;
            }
        });
    }

    println!("{}", flashes);
}
