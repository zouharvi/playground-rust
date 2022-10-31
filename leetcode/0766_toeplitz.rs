pub fn is_toeplitz_matrix(matrix: Vec<Vec<i32>>) -> bool {
    fn check_diagonal(matrix: &Vec<Vec<i32>>, start_i: usize, start_j: usize) -> bool {
        let element = matrix[start_i][start_j];
        let mut i = start_i + 1;
        let mut j = start_j + 1;
        loop {
            if i == matrix.len() || j == matrix[0].len() {
                break;
            }
            if matrix[i][j] != element {
                return false;
            }
            i += 1;
            j += 1;
        }
        true
    }

    // left edge
    for i in 0..matrix.len() {
        if !check_diagonal(&matrix, i, 0) {
            return false;
        }
    }
    // top edge
    for j in 1..matrix[0].len() {
        if !check_diagonal(&matrix, 0, j) {
            return false;
        }
    }
    true
}

pub fn main() {
    let matrix = [[1, 2, 3, 4], [5, 1, 2, 3], [9, 5, 1, 2]]
        .map(|x| x.to_vec())
        .to_vec();
    println!("{}", is_toeplitz_matrix(matrix));
    let matrix = [[1, 2], [2, 2]].map(|x| x.to_vec()).to_vec();
    println!("{}", is_toeplitz_matrix(matrix));
}
