pub fn array_strings_are_equal(word1: Vec<String>, word2: Vec<String>) -> bool {
    let l1 = word1.iter().fold(0, |s, a| s+a.len());
    let l2 = word2.iter().fold(0, |s, a| s+a.len());
    if l1 != l2 {
        return false;
    }
    let c1 = word1.iter().flat_map(|x| x.chars());
    let c2 = word2.iter().flat_map(|x| x.chars());
    for (c1, c2) in c1.zip(c2) {
        if c1 != c2 {
            return false;
        }
    }
    true
}

pub fn main() {
    println!(
        "{}",
        array_strings_are_equal(
            ["ab".to_string(), "c".to_string()].to_vec(),
            ["a".to_string(), "bc".to_string()].to_vec()
        )
    );
}
