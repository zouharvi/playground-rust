pub fn combine_nums(nums: Vec<i32>, k: i32) -> Vec<Vec<i32>> {
    if k == 1 {
        return nums.iter().map(|x| [*x].to_vec()).collect();
    }

    let mut output: Vec<Vec<i32>> = Vec::new();
    for (a_i, a) in nums.iter().enumerate() {
        let new_nums: Vec<i32> = nums[a_i+1..].to_vec();
        let mut new_combinations = combine_nums(new_nums, k - 1);
        for v in new_combinations.iter_mut() {
            v.push(*a);
        }
        output.append(&mut new_combinations);
    }
    output
}

pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
    let nums: Vec<i32> = (1..=n).collect();
    combine_nums(nums, k)
}

pub fn main() {
    println!("{:?}", combine(4, 2));
}
