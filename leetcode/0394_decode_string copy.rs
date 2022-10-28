pub fn decode_string_slice(s: &[char]) -> (String, usize) {
    let mut s_out = Vec::<char>::new();
    let mut i = 0;
    let mut replicate_count = 0; 

    loop {
        if i >= s.len() || s[i] == ']' {
            i += 1;
            break;
        }
        if s[i] == '[' {
            // duplicate child
            // TODO: > 9 numbers
            let (child_result, child_i) = decode_string_slice(&s[i + 1..]);
            
            // println!("{} {} {:?} {}", replicate_count, child_result, s_out, child_i+i);
            for _ in 0..replicate_count {
                for c in child_result.chars() {
                    s_out.push(c);
                }
            }
            i += child_i+1;
            
            // reset buffer
            replicate_count = 0;
            continue;
        }
        if s[i].is_alphabetic() {
            s_out.push(s[i]);
            i += 1;
        } else {
            // is numeric
            let replicate_count_local = String::from(s[i]).parse::<u32>().unwrap();
            replicate_count = 10*replicate_count + replicate_count_local;
            i += 1;
        }
    }
    let mut s_out_real = String::with_capacity(s_out.len());
    for c in s_out {
        s_out_real.push(c);
    }
    // String::from_iter is more elegant but leetcode doesn't support it
    return (s_out_real, i);
}

pub fn decode_string(s: String) -> String {
    let s = s.chars().collect::<Vec<char>>();
    decode_string_slice(&s).0
}

pub fn main() {
    println!("{}", decode_string("3[a]2[bc]".to_string()));
    println!("aaabcbc");
    println!("{}", decode_string("3[a2[c]]".to_string()));
    println!("accaccacc");
    println!("{}", decode_string("2[abc]3[cd]ef".to_string()));
    println!("abcabccdcdcdef");
}
