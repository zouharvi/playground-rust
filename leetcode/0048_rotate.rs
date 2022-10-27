pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
    let n = matrix.len();
    let n_effective_u = ((n as f32) / 2.0).ceil() as i32;
    let n_effective_d = ((n as f32) / 2.0).floor() as i32;

    // columns
    for a_i in 0..n_effective_u {
        // rows
        for a_j in 0..n_effective_d {
            let a_j = a_j as usize;
            let a_i = a_i as usize;
            let a = matrix[a_j][a_i];
            // a will go here
            let b_i = n - a_j - 1;
            let b_j = a_i;
            let b = matrix[b_j][b_i];
            // b will go here
            let c_i = n - b_j - 1;
            let c_j = b_i;
            let c = matrix[c_j][c_i];
            // c will go here
            let d_i = n - c_j - 1;
            let d_j = c_i;
            let d = matrix[d_j][d_i];

            // set b to a
            matrix[b_j][b_i] = a;
            // set c to b
            matrix[c_j][c_i] = b;
            // set d to c
            matrix[d_j][d_i] = c;
            // set a to d
            matrix[a_j][a_i] = d;
        }
    }
}

pub fn main() {
    println!(
        "{:?}",
        rotate(&mut [[1, 2, 3].to_vec(), [4, 5, 6].to_vec(), [7, 8, 9].to_vec()].to_vec())
    );
}
