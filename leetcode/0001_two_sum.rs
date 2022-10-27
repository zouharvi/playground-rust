use std::collections::HashMap;

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    // this may be an overkill
    let mut map = HashMap::with_capacity(nums.len());
    for (n_i, n) in nums.iter().enumerate() {
        if map.contains_key(&(target-n)) {
            return [*map.get(&(target-n)).unwrap(), n_i as i32].to_vec();
        }
        map.insert(*n, n_i as i32);
    }
    todo!()
}

pub fn main() {
    println!("{:?}", two_sum([2,7,11,15].to_vec(), 9));
    println!("{:?}", two_sum([3,2,4].to_vec(), 6));
    println!("{:?}", two_sum([3,3].to_vec(), 6));
}
