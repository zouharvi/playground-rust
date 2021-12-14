use std::collections::HashMap;

type Element = char;

fn main() {
    let mut rules = HashMap::<(Element, Element), Element>::new();
    let mut formula = HashMap::<(Element, Element), usize>::new();

    for line in include_str!("input2").lines() {
        if line.contains(" -> ") {
            let line = line.split_once(" -> ").unwrap();
            let mut left = line.0.chars();
            let right = line.1.chars().nth(0).unwrap();
            rules.insert((left.nth(0).unwrap(), left.nth(0).unwrap()), right);
        } else if line.len() > 0 {
            let line = line.chars().collect::<Vec<char>>();
            for i in 0..(line.len() - 1) {
                *formula.entry((line[i], line[i + 1])).or_insert(0) += 1;
            }
        }
    }

    for _step in 0..40 {
        let mut new_formula = HashMap::<(Element, Element), usize>::new();

        // compute rules application
        for (left, val) in formula.iter() {
            match rules.get(left) {
                // here we're adding things that are not even covered by the rules but maybe that's a good thing
                Some(new_el) => {
                    *new_formula.entry((left.0, *new_el)).or_insert(0) += val;
                    *new_formula.entry((*new_el, left.1)).or_insert(0) += val;
                }
                None => {}
            };
        }
        formula = new_formula;
    }

    let mut freqs = HashMap::<Element, usize>::new();
    for (el, val) in formula.iter() {
        *freqs.entry(el.0).or_insert(0) += val;
        *freqs.entry(el.1).or_insert(0) += val;
    }
    let mut freqs = freqs.iter().collect::<Vec<(&char, &usize)>>();
    freqs.sort_by(|a, b| a.1.partial_cmp(b.1).unwrap());
    // println!("{:?}", freqs);
    let out = match (freqs[freqs.len() - 1].1 % 2, freqs[0].1 % 2) {
        (0, 0) => freqs[freqs.len() - 1].1 / 2 - freqs[0].1 / 2,
        (1, 0) => freqs[freqs.len() - 1].1 / 2 - freqs[0].1 / 2 + 1,
        (0, 1) => panic!("Unknown!"),
        (1, 1) => freqs[freqs.len() - 1].1 / 2 - freqs[0].1 / 2,
        _ => unreachable!(),
    };
    println!("{}", out);
}
