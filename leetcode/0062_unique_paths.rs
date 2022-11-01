pub fn unique_paths(m: i32, n: i32) -> i32 {
    let m = m as usize;
    let n = n as usize;
    let mut grid: Vec<Vec<i32>> = (0..m).map(|_| (0..n).map(|_| 0).collect()).collect();

    for r in 0..m {
        grid[r][0] = 1;
    }

    for c in 1..n {
        grid[0][c] = grid[0][c-1];
        for r in 1..m {
            grid[r][c] = grid[r][c - 1] + grid[r - 1][c];
        }
    }

    grid[m - 1][n - 1]
}

pub fn main() {
    println!("{}", unique_paths(3, 7));
    println!("{}", unique_paths(3, 2));
}
