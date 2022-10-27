type ImgI = Vec<Vec<i32>>;
type Img = Vec<Vec<bool>>;

pub fn comp_overlap_up_left(img1: &Img, img2: &Img, off_x: usize, off_y: usize) -> i32 {
    // don't actually do any image translation but just change indicies
    let mut overlap = 0;
    let n = img1.len();
    for i in 0..off_y {
        for j in 0..off_x {
            if img1[n + i - off_y][n + j - off_x] && img2[i][j] {
                overlap += 1;
            }
        }
    }
    overlap
}

pub fn comp_overlap_up_right(img1: &Img, img2: &Img, off_x: usize, off_y: usize) -> i32 {
    // don't actually do any image translation but just change indicies
    let mut overlap = 0;
    let n = img1.len();
    for i in 0..off_y {
        for j in off_x..n {
            if img1[n + i - off_y][j - off_x] && img2[i][j] {
                overlap += 1;
            }
        }
    }
    overlap
}

pub fn comp_overlap_down_right(img1: &Img, img2: &Img, off_x: usize, off_y: usize) -> i32 {
    // don't actually do any image translation but just change indicies
    let mut overlap = 0;
    let n = img1.len();
    for i in off_y..n {
        for j in off_x..n {
            if img1[i - off_y][j - off_x] && img2[i][j] {
                overlap += 1;
            }
        }
    }
    overlap
}
pub fn comp_overlap_down_left(img1: &Img, img2: &Img, off_x: usize, off_y: usize) -> i32 {
    // don't actually do any image translation but just change indicies
    let mut overlap = 0;
    let n = img1.len();
    for i in off_y..n {
        for j in 0..off_x {
            if img1[i - off_y][n+j - off_x] && img2[i][j] {
                overlap += 1;
            }
        }
    }
    overlap
}
pub fn largest_overlap_bin(img1: Img, img2: Img) -> i32 {
    let mut max_overlap = 0;
    let n = img1.len();
    for i in 0..(n) {
        for j in 0..(n) {
            // estimate the maximum remaining overlap and if it's less than what we currently have, return
            max_overlap = max_overlap.max(comp_overlap_down_right(&img1, &img2, i, j));
            max_overlap = max_overlap.max(comp_overlap_down_left(&img1, &img2, i, j));
            max_overlap = max_overlap.max(comp_overlap_up_right(&img1, &img2, i, j));
            max_overlap = max_overlap.max(comp_overlap_up_left(&img1, &img2, i, j));
        }
    }
    max_overlap
}

pub fn largest_overlap(img1: ImgI, img2: ImgI) -> i32 {
    let img1 = img1
        .iter()
        .map(|row| row.iter().map(|x| *x == 1).collect())
        .collect();
    let img2 = img2
        .iter()
        .map(|row| row.iter().map(|x| *x == 1).collect())
        .collect();
    largest_overlap_bin(img1, img2)
}

pub fn main() {
    println!(
        "{}",
        largest_overlap(
            [[1, 1, 0].to_vec(), [0, 1, 0].to_vec(), [0, 1, 0].to_vec()].to_vec(),
            [[0, 0, 0].to_vec(), [0, 1, 1].to_vec(), [0, 0, 1].to_vec()].to_vec(),
        )
    );
    println!(
        "{}",
        largest_overlap([[1].to_vec()].to_vec(), [[1].to_vec()].to_vec(),)
    );
    println!(
        "{}",
        largest_overlap(
            [
                [0, 0, 0, 0, 1].to_vec(),
                [0, 0, 0, 0, 0].to_vec(),
                [0, 0, 0, 0, 0].to_vec(),
                [0, 0, 0, 0, 0].to_vec(),
                [0, 0, 0, 0, 0].to_vec()
            ]
            .to_vec(),
            [
                [0, 0, 0, 0, 0].to_vec(),
                [0, 0, 0, 0, 0].to_vec(),
                [0, 0, 0, 0, 0].to_vec(),
                [0, 0, 0, 0, 0].to_vec(),
                [1, 0, 0, 0, 0].to_vec()
            ]
            .to_vec(),
        )
    );
}
