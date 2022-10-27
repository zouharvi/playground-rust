use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

type SetChar = HashSet<char>;
type Mapping = HashMap<char, char>;

fn hashset_from_str(line: &str) -> SetChar {
    let set: SetChar = line.chars().collect();
    set
}

fn find_by_count_single(signal: &Vec<SetChar>, size: usize) -> SetChar {
    // assume that this will be non-empty
    // and takes the first element
    signal
        .into_iter()
        .filter_map(|digit| {
            if digit.len() == size {
                Some(digit.clone())
            } else {
                None
            }
        })
        .next()
        .unwrap()
}

fn find_by_count(signal: &Vec<SetChar>, size: usize) -> Vec<SetChar> {
    signal
        .into_iter()
        .filter_map(|digit| {
            if digit.len() == size {
                Some(digit.clone())
            } else {
                None
            }
        })
        .collect::<Vec<SetChar>>()
}

fn filter_by_present_wire(signal: &Vec<SetChar>, wire: char) -> Vec<SetChar> {
    signal
        .into_iter()
        .filter_map(|digit| {
            if digit.contains(&wire) {
                Some(digit.clone())
            } else {
                None
            }
        })
        .collect::<Vec<SetChar>>()
}

fn filter_by_present_wires(signal: &Vec<SetChar>, wires: &SetChar) -> Vec<SetChar> {
    signal
        .into_iter()
        .filter_map(|digit| {
            if wires.iter().all(|wire| digit.contains(&wire)) {
                Some(digit.clone())
            } else {
                None
            }
        })
        .collect::<Vec<SetChar>>()
}

fn filter_by_missing_wire(signal: &Vec<SetChar>, wire: char) -> Vec<SetChar> {
    signal
        .into_iter()
        .filter_map(|digit| {
            if !digit.contains(&wire) {
                Some(digit.clone())
            } else {
                None
            }
        })
        .collect::<Vec<SetChar>>()
}

fn filter_by_missing_wires(signal: &Vec<SetChar>, wires: &SetChar) -> Vec<SetChar> {
    signal
        .into_iter()
        .filter_map(|digit| {
            if !wires.iter().all(|wire| digit.contains(&wire)) {
                Some(digit.clone())
            } else {
                None
            }
        })
        .collect::<Vec<SetChar>>()
}

fn wire_to_digit(wire: char) -> SetChar {
    let a = SetChar::from([wire]);
    a
}

fn is_subset_fix(digit: &SetChar, check: &str) -> bool {
    check.len() == digit.len() && check.chars().into_iter().all(|c| digit.contains(&c))
}

fn map_digits(mapping: &Mapping, digit: &SetChar) -> u32 {
    // transform
    let digit = digit
        .iter()
        .map(|x| *mapping.get(x).unwrap())
        .collect::<SetChar>();

    // map
    if is_subset_fix(&digit, "abcefg") {
        0
    } else if is_subset_fix(&digit, "cf") {
        1
    } else if is_subset_fix(&digit, "acdeg") {
        2
    } else if is_subset_fix(&digit, "acdfg") {
        3
    } else if is_subset_fix(&digit, "bcdf") {
        4
    } else if is_subset_fix(&digit, "abdfg") {
        5
    } else if is_subset_fix(&digit, "abdefg") {
        6
    } else if is_subset_fix(&digit, "acf") {
        7
    } else if is_subset_fix(&digit, "abcdefg") {
        8
    } else if is_subset_fix(&digit, "abcdfg") {
        9
    } else {
        panic!("Unknown digit");
    }
}

fn main() {
    let file = File::open("day_08/input2").unwrap();
    let reader = BufReader::new(file);
    // load and parse data
    let data = reader
        .lines()
        .map(|l| {
            let l = l.unwrap();
            let m = l.split(" | ").collect::<Vec<&str>>();
            (
                m[0].split(" ").map(|x| hashset_from_str(x)).collect(),
                m[1].split(" ").map(|x| hashset_from_str(x)).collect(),
            )
        })
        .collect::<Vec<(Vec<SetChar>, Vec<SetChar>)>>();

    let total = data
        .iter()
        .map(|(signal, output)| {
            let mut mapping = Mapping::new();
            let digit_1 = find_by_count_single(signal, 2);
            let digit_4 = find_by_count_single(signal, 4);
            let digit_7 = find_by_count_single(signal, 3);
            let digit_8 = find_by_count_single(signal, 7);
            let wire_a = digit_7.difference(&digit_1).into_iter().next().unwrap();
            mapping.insert(*wire_a, 'a');

            let digit_069 = find_by_count(signal, 6);
            let digit_9 = &filter_by_present_wires(&digit_069, &digit_4)[0];
            let wire_e = digit_8.difference(digit_9).into_iter().next().unwrap();
            mapping.insert(*wire_e, 'e');
            let digit_06 = filter_by_missing_wires(&digit_069, &digit_4);
            let digit_6 = &filter_by_missing_wires(&digit_06, &digit_1)[0];
            let digit_0 = &filter_by_present_wires(&digit_06, &digit_1)[0];
            let wire_c = digit_1.difference(digit_6).into_iter().next().unwrap();
            mapping.insert(*wire_c, 'c');
            let tmp = SetChar::from([*wire_c]);
            let wire_f = digit_1.difference(&tmp).into_iter().next().unwrap();
            mapping.insert(*wire_f, 'f');
            let tmp = SetChar::from([*wire_a, *wire_c, *wire_e, *wire_f]);
            let tmp = digit_0.difference(&tmp).map(|x| *x).collect::<SetChar>();
            let wire_g = tmp.difference(&digit_4).into_iter().next().unwrap();
            mapping.insert(*wire_g, 'g');
            let tmp = SetChar::from([*wire_a, *wire_c, *wire_e, *wire_f, *wire_g]);
            let wire_b = digit_0.difference(&tmp).into_iter().next().unwrap();
            mapping.insert(*wire_b, 'b');
            let wire_d = digit_8.difference(&digit_0).into_iter().next().unwrap();
            mapping.insert(*wire_d, 'd');

            let total = map_digits(&mapping, &output[0]) * 1000
                + map_digits(&mapping, &output[1]) * 100
                + map_digits(&mapping, &output[2]) * 10
                + map_digits(&mapping, &output[3]) * 1;
            total
        })
        .sum::<u32>();
    println!("{}", total);
}
