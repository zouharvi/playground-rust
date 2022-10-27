use std::collections::HashMap;

pub fn check_subarray_sum(nums: Vec<i32>, k: i32) -> bool {
    let mut existing_mods: HashMap<i32, i32> = HashMap::new();
    let mut sum = 0;
    existing_mods.insert(0, -1);
    for (n_i,n) in nums.iter().enumerate() {
        // we can modulo the sum because at the end we're checking the difference modulo
        sum = (sum + n) % k;

        let prev_mod = existing_mods.get(&sum);
        if let Some(prev_mod) = prev_mod {
            if (n_i as i32) - *prev_mod >= 2 {
                return true;
            } 
        } else {
            existing_mods.insert(sum, n_i as i32);
        }
    }
    false
}


pub fn main() {
    println!("{}", check_subarray_sum([23, 2, 6, 4, 7].to_vec(), 6)); // true
    println!("{}", check_subarray_sum([23, 2, 6, 4, 7].to_vec(), 13)); // false
    println!("{}", check_subarray_sum([23, 6, 9].to_vec(), 6)); // false
    println!("{}", check_subarray_sum([0, 0].to_vec(), 1)); // true
    println!("{}", check_subarray_sum([5, 0, 0, 0].to_vec(), 3)); // true
}
