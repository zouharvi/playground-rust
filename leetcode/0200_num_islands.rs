type Map = Vec<Vec<(bool, Option<usize>)>>;

pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
    let mut grid: Map = grid
        .into_iter()
        .map(|row| row.iter().map(|x| (*x == '1', None)).collect())
        .collect();

    fn is_viable(i: i32, j: i32, grid: &mut Map) -> bool {
        if i < 0 || j < 0 {
            return false;
        }
        if i >= grid.len() as i32 || j >= grid[0].len() as i32 {
            return false;
        }
        // is water
        if !grid[i as usize][j as usize].0 {
            return false;
        }
        if grid[i as usize][j as usize].1.is_some() {
            return false;
        }
        true
    }
    let mut segments = 0;

    // add all
    let mut queue: Vec<(usize, usize)> = (0..grid.len())
        .flat_map(|i| (0..grid[0].len()).map(move |j| (i, j)))
        .collect();

    while let Some((i, j)) = queue.pop() {
        let cell = grid[i][j];
        // is water
        if !cell.0 {
            continue;
        }
        // is already assigned cluster
        if cell.1.is_some() {
            continue;
        }

        segments += 1;

        // flood-fill
        let mut queue_small = vec![(i as i32, j as i32)];
        while let Some((a, b)) = queue_small.pop() {
            // set the corresponding island num
            grid[a as usize][b as usize].1 = Some(segments);

            // bfs
            if is_viable(a + 1, b, &mut grid) {
                queue_small.push((a + 1, b));
            }
            if is_viable(a - 1, b, &mut grid) {
                queue_small.push((a - 1, b));
            }
            if is_viable(a, b + 1, &mut grid) {
                queue_small.push((a, b + 1));
            }
            if is_viable(a, b - 1, &mut grid) {
                queue_small.push((a, b - 1));
            }
        }
    }
    segments as i32
}

pub fn main() {
    let grid = [
        ["1", "1", "1", "1", "0"],
        ["1", "1", "0", "1", "0"],
        ["1", "1", "0", "0", "0"],
        ["0", "0", "0", "0", "0"],
    ]
    .iter()
    .map(|row| {
        row.iter()
            .map(|x| if *x == "1" { '1' } else { '0' })
            .collect()
    })
    .collect();
    println!("{}", num_islands(grid));
    let grid = [
        ["1", "1", "0", "0", "0"],
        ["1", "1", "0", "0", "0"],
        ["0", "0", "1", "0", "0"],
        ["0", "0", "0", "1", "1"],
    ]
    .iter()
    .map(|row| {
        row.iter()
            .map(|x| if *x == "1" { '1' } else { '0' })
            .collect()
    })
    .collect();
    println!("{}", num_islands(grid));
}
