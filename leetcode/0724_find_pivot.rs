pub fn pivot_index(nums: Vec<i32>) -> i32 {
    // compute prefix sum
    let mut prefix_sum: Vec<i32> = Vec::with_capacity(nums.len());
    let mut sum: i32 = 0;
    for x in &nums {
        sum += x;
        prefix_sum.push(sum);
    }
    let total_sum = prefix_sum[nums.len()-1]; 

    for (n_i, _n) in nums.iter().enumerate() {
        let left_sum = if n_i == 0 { 0 } else { prefix_sum[n_i - 1] };
        let right_sum = if n_i == nums.len()-1 { 0 } else { total_sum - prefix_sum[n_i] };
        if left_sum == right_sum {
            return n_i as i32;
        }
    }
    -1
}

pub fn main() {
    println!("{}", pivot_index([1,7,3,6,5,6].to_vec()));
}
