pub fn reverse_num(num: u32) -> u32 {
    num.to_string()
        .chars()
        .rev()
        .collect::<String>()
        .parse()
        .unwrap()
}

pub fn sum_of_number_and_reverse(num: i32) -> bool {
    // special case for 0
    if num == 0 {
        return true;
    }
    let num = num as u32;

    // TODO: better upper bound?
    let upper_bound = if num > 100 { num - 9 } else { num };
    for i in 0..upper_bound {
        let i_r = reverse_num(i);
        if i_r + i == num {
            return true;
        }
    }
    false
}

pub fn main() {
    println!("{}", sum_of_number_and_reverse(0));
    println!("{}", sum_of_number_and_reverse(443));
    println!("{}", sum_of_number_and_reverse(63));
    println!("{}", sum_of_number_and_reverse(181));
}
