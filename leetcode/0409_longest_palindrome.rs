use std::collections::HashMap;

pub fn longest_palindrome(s: String) -> i32 {
    let mut map = HashMap::new();
    for c in s.chars() {
        *map.entry(c).or_insert(0) += 1;
    }
    let mut max_palindrome = 0;
    let mut extra_center = false;
    for (_c, count) in map.iter() {
        if count % 2 != 0 {
            extra_center = true;
        }
        max_palindrome += (count / 2)*2;
    }

    if extra_center {
        max_palindrome + 1
    } else {
        max_palindrome
    }
}

pub fn main() {
    println!("{}", longest_palindrome("abccccdd".to_string()));
    println!("{}", longest_palindrome("a".to_string()));
    println!("{}", longest_palindrome("abccccddxx".to_string()));
    println!("{}", longest_palindrome("abccccddxxz".to_string()));
}
