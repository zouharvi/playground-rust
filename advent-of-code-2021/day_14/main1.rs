use std::collections::HashMap;

type Element = char;

fn main() {
    let mut rules = HashMap::<(Element, Element), Element>::new();
    let mut formula = Vec::<Element>::new();

    for line in include_str!("input2").lines() {
        if line.contains(" -> ") {
            let line = line.split_once(" -> ").unwrap();
            let mut left = line.0.chars();
            let right = line.1.chars().nth(0).unwrap();
            rules.insert((left.nth(0).unwrap(), left.nth(0).unwrap()), right);
        } else if line.len() > 0 {
            formula = line.chars().collect();
        }
    }

    for step in 0..10 {
        let mut to_insert = Vec::<(usize, Element)>::new();

        // compute rules application
        let mut offset = 0;
        for i in 0..(formula.len() - 1) {
            let el1 = formula[i];
            let el2 = formula[i + 1];
            match rules.get(&(el1, el2)) {
                Some(val) => {
                    to_insert.push((i + offset + 1, *val));
                    offset += 1;
                }
                None => {}
            };
        }

        for (i, val) in to_insert.iter() {
            formula.insert(*i, *val);
        }
    }

    let mut freqs = HashMap::<Element, usize>::new();
    for el in formula.iter() {
        *freqs.entry(*el).or_insert(0) += 1;
    }
    let mut freqs = freqs.iter().collect::<Vec<(&char, &usize)>>(); 
    freqs.sort_by(|a, b| a.1.partial_cmp(b.1).unwrap());
    println!("{}", freqs[freqs.len()-1].1-freqs[0].1);
}
