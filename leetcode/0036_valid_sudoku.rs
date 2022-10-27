pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
    use std::collections::HashSet;
    const N : usize = 9;


    // check rows
    for row in board.iter() {
        let mut set = HashSet::with_capacity(N);
        for c in row {
            if c != &'.' && set.contains(&c) {
                return false;
            }
            set.insert(c);
        }
    }

    // check cols
    for col_i in 0..N {
        let mut set = HashSet::with_capacity(N);
        for row_i in 0..N {
            let c = board[row_i][col_i];
            if c != '.' && set.contains(&c) {
                return false;
            }
            set.insert(c);
        }
    }

    // check squares
    for square_i in 0..3 {
        for square_j in 0..3 {
            let mut set = HashSet::with_capacity(N);
            for i in square_i * 3..(square_i + 1) * 3 {
                for j in square_j * 3..(square_j + 1) * 3 {
                    let c = board[i][j];
                    if c != '.' && set.contains(&c) {
                        return false;
                    }
                    set.insert(c);
                }
            }
        }
    }
    
    true
}

pub fn main() {
    let board = [
        ["5", "3", ".", ".", "7", ".", ".", ".", "."],
        ["6", ".", ".", "1", "9", "5", ".", ".", "."],
        [".", "9", "8", ".", ".", ".", ".", "6", "."],
        ["8", ".", ".", ".", "6", ".", ".", ".", "3"],
        ["4", ".", ".", "8", ".", "3", ".", ".", "1"],
        ["7", ".", ".", ".", "2", ".", ".", ".", "6"],
        [".", "6", ".", ".", ".", ".", "2", "8", "."],
        [".", ".", ".", "4", "1", "9", ".", ".", "5"],
        [".", ".", ".", ".", "8", ".", ".", "7", "9"],
    ]
    .iter()
    .map(|x| {
        x.map(|c| c.chars().next().unwrap())
        .to_vec()
    })
    .collect();
    println!("{}", is_valid_sudoku(board));
    let board = [
        ["8", "3", ".", ".", "7", ".", ".", ".", "."],
        ["6", ".", ".", "1", "9", "5", ".", ".", "."],
        [".", "9", "8", ".", ".", ".", ".", "6", "."],
        ["8", ".", ".", ".", "6", ".", ".", ".", "3"],
        ["4", ".", ".", "8", ".", "3", ".", ".", "1"],
        ["7", ".", ".", ".", "2", ".", ".", ".", "6"],
        [".", "6", ".", ".", ".", ".", "2", "8", "."],
        [".", ".", ".", "4", "1", "9", ".", ".", "5"],
        [".", ".", ".", ".", "8", ".", ".", "7", "9"],
    ]
    .iter()
    .map(|x| {
        x.map(|c| c.chars().next().unwrap()).to_vec()
    })
    .collect();
    println!("{}", is_valid_sudoku(board));
}
