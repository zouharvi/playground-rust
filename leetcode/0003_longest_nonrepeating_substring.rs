use std::collections::HashMap;

pub fn length_of_longest_substring(s: String) -> i32 {
    let mut existing = HashMap::new();
    let mut max_len = 0;
    let mut last_good_c = 0;

    for (c_i, c) in s.chars().enumerate() {
        let prev_c_index = existing.get(&c);
        // we already have the c_i
        if let Some(prev_c_index) = prev_c_index {
            if *prev_c_index >= last_good_c {
                last_good_c = prev_c_index + 1;
            }
        }
        let new_len = c_i + 1 - last_good_c;
        if new_len > max_len {
            max_len = new_len;
        }
        existing.insert(c, c_i);
    }
    max_len as i32
}
pub fn main() {
    println!("{:?}", length_of_longest_substring("abcabcbb".to_string())); // 3
    println!("{:?}", length_of_longest_substring("bbbbb".to_string())); // 1
    println!("{:?}", length_of_longest_substring("pwwkew".to_string())); // 3
    println!("{:?}", length_of_longest_substring("pwwkewp".to_string())); // 4
    println!("{:?}", length_of_longest_substring("".to_string())); // 0
    println!("{:?}", length_of_longest_substring("tmmzuxt".to_string())); // 5
    println!(
        "{:?}",
        length_of_longest_substring("ggububgvfk".to_string())
    ); // 6
    println!("{:?}", length_of_longest_substring("bubgvfk".to_string())); // 6
}
