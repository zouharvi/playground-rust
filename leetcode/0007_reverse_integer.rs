// impl Solution {
// }
pub fn reverse(x: i32) -> i32 {
    let mut x: i32 = x;
    let mut x_new: i32 = 0;
    while x != 0 {
        let leftover = x % 10;
        // we only need to check at the last position becase the input is also i32
        if x < 10 && x > -10 {
            if x > 0 {
                if x_new > i32::MAX / 10 || (leftover > 0 && x_new > i32::MAX - leftover) {
                    return 0;
                }
            } else if x_new < i32::MIN / 10 || (leftover < 0 && x_new < i32::MIN - leftover) {
                return 0;
            }
        }
        x_new = x_new * 10 + leftover;
        x /= 10;
    }

    x_new
}

pub fn main() {
    println!("{}", reverse(152));
    println!("{}", reverse(-123));
    println!("{}", reverse(120));
    println!("{}", reverse(2147483644));
    println!("{}", reverse(-2147483648));
}
