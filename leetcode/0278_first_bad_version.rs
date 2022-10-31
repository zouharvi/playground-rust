#[allow(non_snake_case)]
pub fn isBadVersion(v: i32) -> bool {
    // [false, false, false, true, true][(v-1) as usize]
    [true, true][(v-1) as usize]
}

pub fn first_bad_version(n: i32) -> i32 {
    // i is ok
    let mut i = 1;
    // j is bad
    let mut j = n;

    while i < j {
        let mid = (j-i)/2 + i;

        if !isBadVersion(mid) {
            if isBadVersion(mid+1) {
                return mid+1;
            }

            i = mid+1;
        }
        else {
            j = mid;
        }
    }
    i
}

pub fn main() {
    println!("{}", first_bad_version(2));
    // println!("{}", first_bad_version(5));
}
