pub fn backspace_compare(s: String, t: String) -> bool {
    let mut s_true = Vec::<char>::new();
    let mut t_true = Vec::<char>::new();
    for s in s.chars() {
        if s == '#' {
            if !s_true.is_empty() {
                s_true.pop();
            }
        } else {
            s_true.push(s);
        }
    }
    for t in t.chars() {
        if t == '#' {
            if !t_true.is_empty() {
                t_true.pop();
            }
        } else {
            t_true.push(t);
        }
    }
    let s = s_true.into_iter().collect::<String>();
    let t = t_true.into_iter().collect::<String>();
    println!("\n |{s}| |{t}|");
    s == t
}

pub fn main() {
    println!(
        "{}",
        backspace_compare("ab#c".to_string(), "ad#c".to_string())
    );
    println!(
        "{}",
        backspace_compare("ab##".to_string(), "c#d#".to_string())
    );
    println!("{}", backspace_compare("a#c".to_string(), "b".to_string()));
    println!(
        "{}",
        backspace_compare("y#fo##".to_string(), "y#f#o##f".to_string())
    );
    println!(
        "{}",
        backspace_compare("fo##f".to_string(), "f#o##f".to_string())
    );
}
