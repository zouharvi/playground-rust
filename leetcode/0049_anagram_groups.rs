use std::collections::HashMap;

pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
    let mut groups = HashMap::<String, Vec<usize>>::new();

    for (s_i, s_orig) in strs.iter().enumerate() {
        let s = s_orig.clone();
        let mut sig = s.chars().collect::<Vec<char>>();
        sig.sort();
        let sig = sig.iter().collect::<String>();

        if !groups.contains_key(&sig) {
            let v = Vec::<usize>::new();
            groups.insert(sig.clone(), v);
        }
        let group_entry = groups.get_mut(&sig).unwrap();
        group_entry.push(s_i);
    }

    groups
        .values()
        .map(|indicies| indicies.iter().map(|i| strs[*i].clone()).collect())
        .collect()
}

pub fn main() {
    println!(
        "{:?}",
        group_anagrams(
            ["eat", "tea", "tan", "ate", "nat", "bat"]
                .iter()
                .map(|x| x.to_string())
                .collect()
        )
    );

    println!(
        "{:?}",
        group_anagrams(
            [""]
                .iter()
                .map(|x| x.to_string())
                .collect()
        )
    );

    println!(
        "{:?}",
        group_anagrams(
            ["a"]
                .iter()
                .map(|x| x.to_string())
                .collect()
        )
    );
    println!(
        "{:?}",
        group_anagrams(
            ["a", "aab", "bba", "aba"]
                .iter()
                .map(|x| x.to_string())
                .collect()
        )
    );
}
