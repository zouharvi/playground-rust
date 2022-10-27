pub fn rob_simple(mut nums: Vec<i32>) -> i32 {
    // needed for special case
    if nums.len() == 1 {
        return nums[0];
    }
    // boostrap the nums
    nums[1] = nums[1].max(nums[0]);

    // reuse nums to hold maximum attainable vals up to a specific house
    for i in 2..nums.len() {
        // either we don't rob the specific house or we rob but but then we
        // must use the max of prev prev
        nums[i] = nums[i - 1].max(nums[i - 2] + nums[i]);
    }
    nums[nums.len() - 1]
}

pub fn rob(nums: Vec<i32>) -> i32 {
    if nums.len() == 1 {
        return nums[0];
    }
    // implicit clone
    // no first house
    let r1 = rob_simple(nums[1..].to_vec());
    // no last house
    let r2 = rob_simple(nums[..nums.len()-1].to_vec());
    r1.max(r2)
}

pub fn main() {
    println!("{}", rob([1, 2, 3, 1].to_vec()));
    // println!("{}", rob([2, 7, 9, 3, 1].to_vec()));
    // println!("{}", rob([2, 1, 1, 2].to_vec()));
}
