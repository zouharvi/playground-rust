use std::cmp::Ordering;

pub fn remove_digit(number: String, digit: char) -> String {
    let indices = number
        .chars()
        .enumerate()
        .filter_map(|(c_i, c)| if c == digit { Some(c_i) } else { None });
    // we don't need 0000.. because the number is 1..9
    let mut largest_str= "0".to_string();
    for index in indices {
        let left_part = &number[..index];
        let right_part = &number[index + 1..];
        let new_str = left_part.to_string() + right_part;
        let mut larger = false;
        for (a, b) in largest_str.chars().zip(new_str.chars()) {
            match a.cmp(&b) {
                Ordering::Less => {larger = true; break; },
                Ordering::Greater => {larger = false; break; },
                Ordering::Equal => {},
            }
        }
        if larger {
            largest_str = new_str
        }
    }
    largest_str
}

pub fn main() {
    println!("{}", remove_digit("1231".to_string(), '1'));
    println!("{}", remove_digit("551".to_string(), '5'));
    println!("{}", remove_digit("2998589353917872714814599237991174513476623756395992135212546127959342974628712329595771672911914471".to_string(), '5'));
}
