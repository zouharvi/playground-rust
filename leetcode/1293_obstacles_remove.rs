pub fn shortest_path(grid: Vec<Vec<i32>>, k: i32) -> i32 {
    let n = grid.len();
    let m = grid[0].len();
    let k = k as usize;
    let mut queue = vec![(0, 0, k)];
    let mut result = 0;

    let mut min_path = i32::MAX;
    // which points and with which ks were searched
    let mut grid_searched = vec![vec![vec![false; k + 1]; m]; n];
    while !queue.is_empty() {
        let mut todo_queue = vec![];
        for (i, j, k_left) in queue.into_iter() {
            // found goal
            if i + 1 == n && j + 1 == m {
                return result;
            }
            // invalid bounds
            if i == n || j == m || i < 0 || j < 0{
                continue;
            }
            // return if we searched this already
            if grid_searched[i][j][k_left] {
                continue;
            }

            grid_searched[i][j][k_left] = true;
            let val = grid[i][j];
            
            // obstacle
            if val == 1 && k_left > 0 {
                todo_queue.push((i + 1, j, k_left - 1));
                todo_queue.push((i, j + 1, k_left - 1));
                if i > 0 {
                    todo_queue.push((i - 1, j, k_left - 1));
                }
                if j > 0 {
                    todo_queue.push((i, j - 1, k_left - 1));
                }
            }
            // no obstacle
            if val == 0 {
                todo_queue.push((i + 1, j, k_left));
                todo_queue.push((i, j + 1, k_left));
                if i > 0 {
                    todo_queue.push((i - 1, j, k_left));
                }
                if j > 0 {
                    todo_queue.push((i, j - 1, k_left));
                }
            }
        }
        queue = todo_queue;
        result += 1;
    }
    if min_path != i32::MAX {
        return min_path;
    }
    -1
}

pub fn main() {
    let field = [[0, 0, 0], [1, 1, 0], [0, 0, 0], [0, 1, 1], [0, 0, 0]];
    println!(
        "{}",
        shortest_path(field.iter().map(|x| x.to_vec()).collect(), 1)
    );
    let field = [[0, 0], [0, 0]];
    println!(
        "{}",
        shortest_path(field.iter().map(|x| x.to_vec()).collect(), 1)
    );
    let field = [[0, 1, 1], [1, 1, 1], [1, 0, 0]];
    println!(
        "{}",
        shortest_path(field.iter().map(|x| x.to_vec()).collect(), 1)
    );
    let field = [
        [0, 1, 0, 1],
        [0, 1, 0, 0],
        [0, 0, 1, 0],
        [1, 0, 0, 1],
        [0, 1, 0, 0],
    ];
    println!(
        "{}",
        shortest_path(field.iter().map(|x| x.to_vec()).collect(), 18)
    );
}
