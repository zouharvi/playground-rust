type OutputType = u8;
// we know it's ascii so we can speed it up
type CharType = u8;

pub fn max_length(arr: Vec<String>) -> i32 {
    use std::collections::HashSet;
    let arr_s = arr
        .iter()
        // leetcode's compiler doesn't know HashSet::from_iter :'(
        // .map(|s| HashSet::from_iter(s.chars()))
        .map(|s| {
            let mut h = HashSet::with_capacity(s.len());
            for c in s.chars() {
                h.insert(c as CharType);
            }
            h
        })
        .zip(arr.iter())
        .filter_map(|(a_s, a)| {
            if a_s.len() == a.len() {
                Some(a_s)
            } else {
                None
            }
        })
        .collect::<Vec<HashSet<CharType>>>();

    let mut viables: Vec<HashSet<CharType>> = [HashSet::<CharType>::new()].to_vec();
    let mut largest: OutputType = 0;
    for a_s in arr_s {
        let mut viables_next: Vec<HashSet<CharType>> = Vec::new();
        let mut updated = false;
        for v in viables.iter() {
            if !a_s.is_disjoint(v) {
                continue;
            }
            updated = true;
            let v_new = a_s
                .union(v)
                .into_iter()
                .copied()
                .collect::<HashSet<CharType>>();
            largest = largest.max(v_new.len() as OutputType);
            viables_next.push(v_new);
        }

        // exit early
        if !updated {
            break;
        }

        // viables = viables_next;
        viables.append(&mut viables_next);
    }
    largest as i32
}

pub fn main() {
    println!(
        "{}",
        max_length(["un".to_string(), "iq".to_string(), "ue".to_string()].to_vec())
    );
    println!(
        "{}",
        max_length(
            ["a", "abc", "d", "de", "def"]
                .iter()
                .map(|x| x.to_string())
                .collect()
        )
    );
}

// def maxLength(arr: List[str]) -> int:

//     for a_s in arr_s:
//         for v in viable:
//             if len(v & a_s) != 0:
//                 continue
//             v_new = v | a_s
//             viable.append(v_new)
//             largest = max(largest, len(v_new))

//     return largest

// print(maxLength(["un", "iq", "ue"]))
// print(maxLength(["cha", "r", "act", "ers"]))
// print(maxLength(["abcdefghijklmnopqrstuvwxyz"]))
// print(maxLength(["aa", "bb"]))
// print(maxLength(["a", "abc", "d", "de", "def"]))
// print(maxLength([
//     "a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", "n", "o", "p"
// ]))
