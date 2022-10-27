pub fn max_area(heights: Vec<i32>) -> i32 {
    let mut i = 0;
    let mut j = heights.len() - 1;
    let mut max_area = 0;
    loop {
        if i == j {
            break;
        }
        let h1 = heights[i];
        let h2 = heights[j];
        let area = ((j - i) as i32) * h1.min(h2);
        max_area = max_area.max(area);
        if h2 > h1 {
            i += 1;
        } else {
            j -= 1;
        }
    }
    max_area as i32
}

pub fn main() {
    println!("{}", max_area([1, 8, 6, 2, 5, 4, 8, 3, 7].to_vec()));
    println!("{}", max_area([1, 1].to_vec()));
    println!("{}", max_area([2, 1, 2].to_vec()));
    println!("{}", max_area([1, 2, 2].to_vec()));
    println!("{}", max_area([2, 3, 4, 5, 18, 17, 6].to_vec())); // 17
}
