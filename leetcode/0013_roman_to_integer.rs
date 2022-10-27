// impl Solution {
// }
pub fn roman_to_int(s: String) -> i32 {
    // use u16 for speedup
    let mut seq: Vec<u16> = s
        .chars()
        .map(|c| match c {
            'I' => 1,
            'V' => 5,
            'X' => 10,
            'L' => 50,
            'C' => 100,
            'D' => 500,
            'M' => 1000,
            _ => 0,
        })
        .collect();

    let mut acc: u16 = 0;
    let length = seq.len();
    for a_i in 0..length {
        if a_i + 1 < length && seq[a_i] < seq[a_i + 1] {
            seq[a_i + 1] -= seq[a_i]
        } else {
            acc += seq[a_i]
        }
    }
    acc as i32
}

fn main() {
    println!("{}", roman_to_int("III".to_string()));
    println!("{}", roman_to_int("LVIII".to_string()));
    println!("{}", roman_to_int("MCMXCIV".to_string()));
}
