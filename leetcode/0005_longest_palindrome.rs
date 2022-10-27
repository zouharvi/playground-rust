// impl Solution {
// }
pub fn longest_palindrome(s: String) -> String {
    let s = s.chars().collect::<Vec<char>>();
    // single palindromes
    let palindromes_0 = s
        .iter()
        .enumerate()
        .map(|(c_i, _c)| (c_i as i32, c_i as i32));
    // phantom palindromes
    let palindromes_1 = s
        .iter()
        .enumerate()
        .map(|(c_i, _c)| (c_i as i32, c_i as i32 - 1));

    let mut palindromes: Vec<(i32, i32)> = palindromes_0.chain(palindromes_1).collect();
    let mut next_palindromes: Vec<(i32, i32)> = Vec::new();
    for _possible_length in 0..(s.len() / 2) {
        for (p_i, p_j) in &palindromes {
            // attempt to extend it
            if (p_i > &0 && p_j < &((s.len() - 1) as i32))
                && (s[(p_i - 1) as usize] == s[(p_j + 1) as usize])
            {
                next_palindromes.push((p_i - 1, p_j + 1));
            }
        }
        if next_palindromes.is_empty() {
            break;
        } else {
            palindromes = next_palindromes;
            next_palindromes = Vec::new();
        }
    }

    // any member of palindromes at the end is the maximum one
    let max_palindrome = palindromes[0];
    s[max_palindrome.0 as usize..(max_palindrome.1 + 1) as usize]
        .iter()
        .collect::<String>()
}

pub fn main() {
    println!("{}", longest_palindrome("bdcacdax".to_string()));
    println!("{}", longest_palindrome("bb".to_string()));
    println!(
        "{}",
        longest_palindrome("zeusnilemacaronimaisanitratetartinasiaminoracamelinsuez".to_string())
    );
}
