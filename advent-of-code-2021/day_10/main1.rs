use std::collections::VecDeque;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(PartialEq)]
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

fn bracket_score(bracket: BracketType) -> u32 {
    match bracket {
        BracketType::Round => 3,
        BracketType::Square => 57,
        BracketType::Curly => 1197,
        BracketType::Sharp => 25137,
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


    let mut score = 0;
    for line in data {
        let mut queue = VecDeque::<BracketType>::new();
        for (b_type, b_open) in line {
            if b_open {
                queue.push_back(b_type);
            } else {
                let b_expected = queue.pop_back();
                if (b_expected.is_none()) || (b_type != b_expected.unwrap()) {
                    score += bracket_score(b_type);
                    break;
                }
            }
        }
    }
    println!("{}", score);
}
