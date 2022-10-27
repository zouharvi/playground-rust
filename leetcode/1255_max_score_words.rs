use std::collections::HashMap;

// we can use more light-weight types because of the restrictions
type CounterMap = HashMap<char, u8>;

pub fn max_score_words_local(
    words: Vec<(CounterMap, i32)>,
    letters: &CounterMap,
    score: &CounterMap,
) -> i32 {
    let mut max_score:i32 = 0;
    for (w_i, (w, w_score)) in words.iter().enumerate() {
        let mut letters = letters.clone();
        // check if we can use all the characters
        let mut invalid = false;
        for (c, count_needed) in w.iter() {
            let letters_remaining = if letters.contains_key(c) {
                *letters.get(c).unwrap()
            } else {
                0
            };
            if &letters_remaining < count_needed {
                invalid = true;
                break;
            }

            // TODO: this could be nicer with entry interface
            letters.insert(*c, letters_remaining - count_needed);
        }

        // don't continue search if we don't satisfy the score
        if invalid {
            continue;
        }

        // create local copy
        // TODO: could be sped-up with some smarter structure
        let mut new_words = words.clone();
        new_words.remove(w_i);

        // recurse
        let w_recurse_score = w_score + max_score_words_local(new_words, &letters, score);

        max_score = max_score.max(w_recurse_score);
    }
    max_score
}

pub fn max_score_words(words: Vec<String>, letters: Vec<char>, scores: Vec<i32>) -> i32 {
    // precompute scores to map
    let mut scores_map = CounterMap::new();
    for i in 0..26 {
        let c = char::from_u32(('a' as u32) + i).unwrap();
        scores_map.insert(c, scores[i as usize] as u8);
    }

    // precompute words to (map, score)
    let words = words
        .iter()
        .map(|word| {
            let mut map = CounterMap::new();
            for w in word.chars() {
                *map.entry(w).or_insert(0) += 1;
            }

            // precompute word score
            let w_score = map
                .iter()
                .map(|(c, count)| count * scores_map.get(c).unwrap())
                .sum::<u8>() as i32;

            (map, w_score)
        })
        .collect::<Vec<(CounterMap, i32)>>();

    // precompute letters to map
    let mut letters_map = CounterMap::new();
    for letter in letters {
        *letters_map.entry(letter).or_insert(0) += 1;
    }

    max_score_words_local(words, &letters_map, &scores_map)
}

pub fn main() {
    println!(
        "{}",
        max_score_words(
            [
                "xxxz".to_string(),
                "ax".to_string(),
                "bx".to_string(),
                "cx".to_string()
            ]
            .to_vec(),
            ['z', 'a', 'b', 'c', 'x', 'x', 'x'].to_vec(),
            [4, 4, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 0, 10]
                .to_vec()
        )
    );
    println!(
        "{}",
        max_score_words(
            [
                "add".to_string(),
                // "dda".to_string(),
                "bb".to_string(),
                "ba".to_string(),
                "add".to_string()
            ]
            .to_vec(),
            ['a', 'a', 'a', 'a', 'b', 'b', 'b', 'b', 'd', 'd', 'd'].to_vec(),
            [3, 9, 8, 9, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0].to_vec()
        )
    );
    // should be 51
}
