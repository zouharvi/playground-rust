use std::collections::{HashMap,HashSet};

pub fn find_anagrams_naive(s: &str, p: &str) -> HashSet<i32> {
    let mut remaining = HashMap::<char,usize>::new();
    let mut original = HashMap::<char,usize>::new();
    for c in p.chars(){
        *original.entry(c).or_insert(0) += 1;
    }

    remaining = original.clone();
    let mut out = HashSet::new();
    for (c_i, c) in s.chars().enumerate() {
        println!("{c} {:?}", remaining);
        let remaining_entry = remaining.get(&c);
        if remaining_entry.is_some() && *remaining_entry.unwrap() > 0 {
            // subtract the used one
            *remaining.get_mut(&c).unwrap() -= 1;

            // no remaining characters
            if remaining.values().sum::<usize>() == 0 {
                out.insert((c_i+1-p.len()) as i32);
                remaining = original.clone();
            }
        } else {
            // reset
            remaining = original.clone();
        }
    }
    out
}

pub fn find_anagrams(s: &str, p: &str) -> Vec<i32> {
    
}


pub fn main() {
    println!("{:?}", find_anagrams("cbaebabacd".to_string(), "abc".to_string()));
    // println!("{:?}", find_anagrams("abab".to_string(), "ab".to_string()));
}
