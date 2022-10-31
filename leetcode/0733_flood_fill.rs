pub fn flood_fill(mut image: Vec<Vec<i32>>, sr: i32, sc: i32, color: i32) -> Vec<Vec<i32>> {
    let orig_color = image[sr as usize][sc as usize];
    fn is_viable(i: i32, j: i32, image: &Vec<Vec<i32>>, orig_color: i32) -> bool {
        if i < 0 || j < 0 {
            return false;
        }
        if i >= image.len() as i32 || j >= image[0].len() as i32 {
            return false;
        }
        if image[i as usize][j as usize] != orig_color {
            return false;
        }
        return true;
    }

    let mut stack = vec![(sr, sc)];
    while let Some((i, j)) = stack.pop() {
        if is_viable(i + 1, j, &image, orig_color) {
            stack.push((i + 1, j));
        }
        if is_viable(i - 1, j, &image, orig_color) {
            stack.push((i - 1, j));
        }
        if is_viable(i, j + 1, &image, orig_color) {
            stack.push((i, j + 1));
        }
        if is_viable(i, j - 1, &image, orig_color) {
            stack.push((i, j - 1));
        }
        // mark visited with negatives -1
        image[i as usize][j as usize] = -color - 1;
    }
    image.iter_mut().for_each(|row| {
        row.iter_mut()
            .for_each(|x| *x = if *x < 0 { -*x - 1 } else { *x })
    });
    image
}

pub fn main() {
    let field = [[1, 1, 1], [1, 1, 0], [1, 0, 1]]
        .map(|x| x.to_vec())
        .to_vec();
    println!("{:?}", flood_fill(field, 1, 1, 2));
    let field = [[0, 0, 0], [0, 0, 0]].map(|x| x.to_vec()).to_vec();
    println!("{:?}", flood_fill(field, 0, 0, 3));
    let field = [[0, 0, 0], [0, 1, 0]].map(|x| x.to_vec()).to_vec();
    println!("{:?}", flood_fill(field, 0, 0, 3));
    let field = [[0, 0, 0], [0, 0, 0]].map(|x| x.to_vec()).to_vec();
    println!("{:?}", flood_fill(field, 0, 0, 0));
}
