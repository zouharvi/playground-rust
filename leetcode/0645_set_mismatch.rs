struct Solution {

}

impl Solution {
    pub fn find_error_nums(nums: Vec<i32>) -> Vec<i32> {
        use std::collections::HashSet;
        // let nums_set: HashSet<&i32> = HashSet::from_iter(nums.iter());
        // the above version is cleaner but doesn't work in leetcode
        let mut nums_set: HashSet<i32> = HashSet::with_capacity(nums.len());
        for num in &nums {
            nums_set.insert(*num);
        }
        let sum_unique: i32 = nums_set.iter().sum();
        let sum_all: i32 = nums.iter().sum();
        let num_duplicated = sum_all - sum_unique;
        let n = nums.iter().max().unwrap();
        let sum_theoretical = n * (n + 1) / 2;

        let num_missing = if sum_theoretical == sum_unique {
            n + 1
        } else {
            sum_theoretical - sum_unique
        };

        [num_duplicated, num_missing].to_vec()
    }
}

pub fn main() {
    println!("{:?}", Solution::find_error_nums([1, 2, 2, 4].to_vec()));
    println!("{:?}", Solution::find_error_nums([1, 1].to_vec()));
}
