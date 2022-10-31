pub fn climb_stairs(n: i32) -> i32 {
    let n = n + 1;
    match n {
        0 => 0,
        1 => 1,
        _ => {
            let mut s = 0;
            let mut a = 0;
            let mut b = 1;
            for _ in 1..n {
                s = a + b;
                a = b;
                b = s;
            }

            s as i32
        }
    }
}

pub fn main() {
    println!("{}", climb_stairs(2));
    println!("{}", climb_stairs(3));
}
