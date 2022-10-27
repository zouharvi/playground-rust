pub fn my_pow(x: f64, n: i32) -> f64 {
    x.powf(n as f64)
}

pub fn main() {
    println!("{}", my_pow(10.0, 2));
}