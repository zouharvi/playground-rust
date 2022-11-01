pub fn find_ball_one(grid: &Vec<Vec<i32>>, mut pos: usize) -> i32 {
    let width = grid[0].len();
    for row in 0..grid.len() {
        let direction = grid[row][pos];
        if direction == 1 {
            if pos == width - 1 {
                return -1;
            } else {
                if grid[row][pos + 1] == -1 {
                    return -1;
                }
                // move to right
                pos += 1;
            }
        }
        if direction == -1 {
            if pos == 0 {
                return -1;
            } else {
                if grid[row][pos - 1] == 1 {
                    return -1;
                }
                // move to left
                pos -= 1;
            }
        }
    }
    pos as i32
}

pub fn find_ball(grid: Vec<Vec<i32>>) -> Vec<i32> {
    (0..grid[0].len())
        .map(|x| find_ball_one(&grid, x))
        .collect()
}

pub fn main() {
    let grid = [
        [1, 1, 1, -1, -1],
        [1, 1, 1, -1, -1],
        [-1, -1, -1, 1, 1],
        [1, 1, 1, 1, -1],
        [-1, -1, -1, -1, -1],
    ]
    .iter()
    .map(|x| x.to_vec())
    .collect();
    println!("{:?}", find_ball(grid));

    let grid = [[-1]].iter().map(|x| x.to_vec()).collect();
    println!("{:?}", find_ball(grid));

    let grid = [
        [1, 1, 1, 1, 1, 1],
        [-1, -1, -1, -1, -1, -1],
        [1, 1, 1, 1, 1, 1],
        [-1, -1, -1, -1, -1, -1],
    ]
    .iter()
    .map(|x| x.to_vec())
    .collect();
    println!("{:?}", find_ball(grid));
}
