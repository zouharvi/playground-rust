use rand::Rng;

fn rand7() -> i32 {
    let mut rng = rand::thread_rng();
    rng.gen_range(1..=7)
}

pub fn rand10() -> i32 {
    // both are 0..=6 so their sum is 0..=12
    let s1 = rand7()-1;
    let s2 = rand7()-1;
    // is 0..49
    let s_new = s1*7 + s2;
    // modulo may not be correct, need to reject that part
    if s_new >= 40 {
        // for leetcode use Solution::
        return rand10();
    }
    s_new % 10 + 1
}

pub fn main() {
    let mut counts = [0;10].to_vec();
    for _ in 0..10000 {
        let s = rand10();
        counts[(s-1) as usize] += 1;
    }
    for (s, count) in counts.iter().enumerate() {
        let s = s+1;
        println!("{s}: {count}");
    }
}
