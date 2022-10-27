pub fn is_subsequence(s: String, t: String) -> bool {
    if s.is_empty() {
        return true;
    }
    let mut s_i : usize = 0;
    let mut s_chars = s.chars();
    let mut cur_char = s_chars.next().unwrap();
    for t in t.chars() {
        if cur_char == t {
            // we reached the end
            if s_i == s.len()-1 {
                return true;
            }
            s_i += 1;
            cur_char = s_chars.next().unwrap();
        }
    }
    false
}

pub fn main() {
    println!("{}", is_subsequence("abc".to_string(), "ahbgdc".to_string()));
    println!("{}", is_subsequence("axc".to_string(), "ahbgdc".to_string()));
}
