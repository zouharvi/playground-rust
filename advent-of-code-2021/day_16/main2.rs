use std::cmp::Ordering;
use std::collections::BinaryHeap;

struct Point {
    danger: u32,
    cost: u32,
}

impl Point {
    fn new(danger: u32) -> Point {
        Point {
            danger,
            cost: u32::MAX,
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: u32,
    point_x: usize,
    point_y: usize,
}

// The priority queue depends on `Ord`.
// Explicitly implement the trait so the queue becomes a min-heap
// instead of a max-heap.
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // Notice that the we flip the ordering on costs.
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.point_x.cmp(&other.point_x))
            .then_with(|| self.point_y.cmp(&other.point_y))
    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// taken from https://doc.rust-lang.org/std/collections/binary_heap/index.html
fn shortest_path(data: &mut Vec<Vec<Point>>, goal_x: usize, goal_y: usize) -> Option<u32> {
    let mut heap = BinaryHeap::new();

    // todo rewrite this to store data in the struct
    // We're at `start`, with a zero cost
    data[0][0].cost = 0;
    heap.push(State {
        cost: 0,
        point_x: 0,
        point_y: 0,
    });

    // Examine the frontier with lower cost nodes first (min-heap)
    while let Some(State {
        cost,
        point_x,
        point_y,
    }) = heap.pop()
    {
        // Alternatively we could have continued to find all shortest paths
        if point_x == goal_x && point_y == goal_y {
            return Some(cost);
        }

        // Important as we may have already found a better way
        if cost > data[point_y][point_x].cost {
            continue;
        }

        // For each node we can reach, see if we can find a way with
        // a lower cost going through this node
        for (off_x, off_y) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
            // filter ivalid neighbours
            let x = (point_x as i32) + off_x;
            let y = (point_y as i32) + off_y;
            if x < 0 || x >= (data[0].len() as i32) || y < 0 || y >= (data.len() as i32) {
                continue;
            }

            // construct new state
            let neighbour_point = &data[y as usize][x as usize];
            let next = State {
                cost: cost + neighbour_point.danger,
                point_x: x as usize,
                point_y: y as usize,
            };
            if next.cost < data[next.point_y][next.point_x].cost {
                heap.push(next);
                // Relaxation, we have now found a better way
                data[next.point_y][next.point_x].cost = next.cost;
            }
        }
    }

    // Goal not reachable
    None
}

fn main() {
    let data = include_str!("input2")
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| Point::new(c.to_digit(10).unwrap() as u32))
                .collect()
        })
        .collect::<Vec<Vec<Point>>>();

    // tile data
    let mut new_data = (0..5).map(|j| {
        data.iter().map(|row| {
            (0..5)
                .map(|i| {
                    row.iter()
                        .map(|point| Point {
                            danger: (point.danger + ((i + j) as u32) - 1) % 9 + 1,
                            cost: point.cost,
                        })
                        .collect::<Vec<Point>>()
                })
                .flatten()
                .collect::<Vec<Point>>()
        })
        .collect::<Vec<Vec<Point>>>()
    })
    .flatten()
    .collect::<Vec<Vec<Point>>>();

    let dim_x = new_data[0].len();
    let dim_y = new_data.len();

    let out = shortest_path(&mut new_data, dim_x - 1, dim_y - 1);
    println!("{}", out.unwrap());
}
