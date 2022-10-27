type Matrix = Vec<Vec<bool>>;

pub fn is_zero(mat: &Matrix) -> bool {
    mat.iter().all(|row| row.iter().all(|x| !x))
}

pub fn flip(position: i32, mat: &mut Matrix) {
    let x = (position / (mat.len() as i32)) as usize;
    let y = (position % (mat.len() as i32)) as usize;
    mat[y][x] = !mat[y][x];
    if y >= 1 {
        mat[y - 1][x] = !mat[y - 1][x];
    }
    if y < mat.len() - 1 {
        mat[y + 1][x] = !mat[y + 1][x];
    }
    if x >= 1 {
        mat[y][x - 1] = !mat[y][x - 1];
    }
    if x < mat[0].len() - 1 {
        mat[y][x + 1] = !mat[y][x + 1];
    }
}

pub fn _min_flips(position: i32, mat: &mut Matrix) -> Option<i32> {
    // edge case
    if position as usize == mat.len() * mat[0].len() {
        if is_zero(mat) {
            return Some(0);
        } else {
            return None;
        }
    }

    // don't flip this one
    let result_0 = _min_flips(position + 1, mat);
    // flip the position
    flip(position, mat);
    let result_1 = _min_flips(position + 1, mat);
    // flip back
    flip(position, mat);

    if let (Some(result_0), Some(result_1)) = (result_0, result_1){
        return Some(result_0.min(1+result_1));
    } else if result_0.is_some() {
        return result_0;
    } else if let Some(result_1) = result_1 {
        return Some(1+result_1);
    }

    None
}

pub fn min_flips(mat: Vec<Vec<i32>>) -> i32 {
    let mat = &mut mat
        .iter()
        .map(|row| row.iter().map(|x| *x == 1).collect::<Vec<bool>>())
        .collect::<Vec<Vec<bool>>>();

    if let Some(x) = _min_flips(0, mat) {
        x
    } else {
        -1
    }
}

pub fn main() {
    println!("{}", min_flips([[0, 0].to_vec(), [0, 1].to_vec()].to_vec()));
    println!("{}", min_flips([[0].to_vec()].to_vec()));
    println!("{}", min_flips([[1, 0, 0].to_vec(), [1, 0, 0].to_vec()].to_vec()));
}
