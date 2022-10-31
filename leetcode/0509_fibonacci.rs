pub fn fib(n: i32) -> i32 {
    match n {
        0 => 0,
        1 => 1,
        _ => {
            let mut s = 0;
            let mut a = 0;
            let mut b = 1;
            for _ in 1..n {
                s = a+b;
                a = b;
                b = s;
            }

            s as i32
        }
    }
}

pub fn main() {
    println!("{}", fib(2));
    println!("{}", fib(3));
    println!("{}", fib(4));
    println!("{}", fib(5));
    println!("{}", fib(10));
}
