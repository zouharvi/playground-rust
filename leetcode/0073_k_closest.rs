pub fn k_closest(mut points: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
    points.sort_by(|a, b| {
        let a = a[0] * a[0] + a[1] * a[1];
        let b = b[0] * b[0] + b[1] * b[1];
        a.cmp(&b)
    });
    points[..k as usize].to_vec()
}

pub fn main() {
    println!(
        "{:?}",
        k_closest([[1, 3].to_vec(), [-2, 2].to_vec()].to_vec(), 1)
    );
}
