use std::collections::VecDeque;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(PartialEq, Clone)]
enum BracketType {
    Round,
    Square,
    Curly,
    Sharp,
}

type Bracket = (BracketType, bool);

fn match_bracket_open(c: char) -> Bracket {
    match c {
        '(' => (BracketType::Round, true),
        '[' => (BracketType::Square, true),
        '{' => (BracketType::Curly, true),
        '<' => (BracketType::Sharp, true),
        ')' => (BracketType::Round, false),
        ']' => (BracketType::Square, false),
        '}' => (BracketType::Curly, false),
        '>' => (BracketType::Sharp, false),
        _ => panic!("Unknown char {}", c),
    }
}

fn bracket_score(bracket: BracketType) -> u64 {
    match bracket {
        BracketType::Round => 1,
        BracketType::Square => 2,
        BracketType::Curly => 3,
        BracketType::Sharp => 4,
    }
}

fn main() {
    let file = File::open("day_10/input2").unwrap();
    let reader = BufReader::new(file);
    // load and parse data
    let data: Vec<Vec<Bracket>> = reader
        .lines()
        .map(|line| {
            line.unwrap()
                .chars()
                .map(|c| match_bracket_open(c))
                .collect()
        })
        .collect();

    let mut scores = data
        .into_iter()
        .map(|line| {
            let mut queue = VecDeque::<BracketType>::new();
            for (b_type, b_open) in line {
                if b_open {
                    queue.push_back(b_type.clone());
                } else {
                    let b_expected = queue.pop_back();

                    // corrupted
                    if b_expected.is_none() {
                        return 0;
                    }

                    // corrupted
                    if b_type != b_expected.unwrap() {
                        return 0;
                    }
                }
            }

            // incomplete (TODO: may be corrupted later)
            // compute score
            let mut score : u64 = 0;
            for b_leftover in queue.into_iter().rev() {
                score = score * 5 + bracket_score(b_leftover);
            }

            return score;
        })
        .filter(|x| *x != 0)
        .collect::<Vec<u64>>();

    // sort and select median
    scores.sort();
    println!("{:?}", scores);
    println!("{}", scores[scores.len() / 2]);
}
