pub fn is_isomorphic(a: String, b: String) -> bool {
    use std::collections::{HashMap,HashSet};
    let mut map: HashMap<char, char> = HashMap::new();
    let mut mapped : HashSet<char> = HashSet::new();
    for (a, b) in a.chars().zip(b.chars()) {
        let true_a = map.get(&a);
        if let Some(true_a) = true_a {
            if *true_a != b {
                return false;
            }
        } else {
            // there is no mapping value associated
            // println!("inserting {a}->{b}");
            // can't map two to the same
            if mapped.contains(&b) {
                return false;
            }
            map.insert(a, b);
            mapped.insert(b);
        }
    }
    true
}

pub fn main() {
    println!("{}", is_isomorphic("egg".to_string(), "add".to_string()));
    println!("{}", is_isomorphic("foo".to_string(), "bar".to_string()));
    println!("{}", is_isomorphic("paper".to_string(), "title".to_string()));
    println!("{}", is_isomorphic("badc".to_string(), "baba".to_string()));
}
