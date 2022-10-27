// impl Solution {
// }
pub fn letter_combinations(digits: String) -> Vec<String> {
    fn map (c: char) -> String {
        match c {
        '2' => "abc".to_string(),
        '3' => "def".to_string(),
        '4' => "ghi".to_string(),
        '5' => "jkl".to_string(),
        '6' => "mno".to_string(),
        '7' => "pqrs".to_string(),
        '8' => "tuv".to_string(),
        '9' => "wxyz".to_string(),
        _ => "".to_string()
        }
    }

    let mut output = Vec::<String>::new();
    for c in digits.chars() {
        let letters = map(c);
        let mut output_new = Vec::<String>::new();
        for letter in letters.chars() {
            if output.is_empty() {
                output_new.push(letter.to_string());
            }
            for solution in &output {
                let mut solution_new = solution.clone();
                solution_new.push(letter);
                output_new.push(solution_new);
            }
        }
        output = output_new;
    }
    output
}

fn main() {
    println!("{:?}", letter_combinations("".to_string()));
    println!("{:?}", letter_combinations("2".to_string()));
    println!("{:?}", letter_combinations("23".to_string()));
}
