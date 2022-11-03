use std::collections::HashMap;

pub fn longest_palindrome(words: Vec<String>) -> i32 {
    let mut words_nodup = HashMap::<(char, char), usize>::new();
    let mut words_dup = HashMap::<(char, char), usize>::new();
    for word in words.iter() {
        let mut chars = word.chars();
        let c1 = chars.next().unwrap();
        let c2 = chars.next().unwrap();

        *(if c1 == c2 {
            &mut words_dup
        } else {
            &mut words_nodup
        })
        .entry((c1, c2))
        .or_insert(0) += 1;
    }

    let mut max_length = 0;
    let mut single_added = false;
    for ((c1, c2), count) in words_nodup.iter() {
        // TODO: this is inefficient and can be sped up 2x
        if let Some(other_min) = words_nodup.get(&(*c2, *c1)) {
            let to_add = count.min(other_min);
            max_length += to_add;
        }
    }
    for ((_c1, _c2), count) in words_dup.iter() {
        max_length += (count / 2) * 2;
        if !single_added && count % 2 == 1 {
            max_length += 1;
            single_added = true;
        }
    }
    (max_length as i32) * 2
}

pub fn main() {
    // 8
    let words = ["lc", "cl", "gg"].iter().map(|x| x.to_string()).collect();
    println!("{:?}", longest_palindrome(words));
    
    // 6
    let words = ["ab", "ty", "yt", "lc", "cl", "ab"]
        .iter()
        .map(|x| x.to_string())
        .collect();
    println!("{:?}", longest_palindrome(words));
    
    // 2
    let words = ["cc", "ll", "xx"].iter().map(|x| x.to_string()).collect();
    println!("{:?}", longest_palindrome(words));

    // 2
    let words = ["lx", "xx"].iter().map(|x| x.to_string()).collect();
    println!("{:?}", longest_palindrome(words));

    // 0
    let words = ["lx", "kx"].iter().map(|x| x.to_string()).collect();
    println!("{:?}", longest_palindrome(words));

    // 22
    let words = [
        "dd", "aa", "bb", "dd", "aa", "dd", "bb", "dd", "aa", "cc", "bb", "cc", "dd", "cc",
    ]
    .iter()
    .map(|x| x.to_string())
    .collect();
    println!("{:?}", longest_palindrome(words));

    // 30
    let words = ["mm","mm","yb","by","bb","bm","ym","mb","yb","by","mb","mb","bb","yb","by","bb","yb","my","mb","ym"]
    .iter()
    .map(|x| x.to_string())
    .collect();
    println!("{:?}", longest_palindrome(words));

    // ["ym","mb","mb","yb","mb"]

    // ybybyb bm my mm bbbbbb mm ym mb bybyby
}
