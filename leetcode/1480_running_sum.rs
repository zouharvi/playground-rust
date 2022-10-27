pub fn running_sum(nums: Vec<i32>) -> Vec<i32> {
    let mut output : Vec<i32> = Vec::with_capacity(nums.len());
    let mut sum:i32 = 0;
    for x in nums {
        sum += x;
        output.push(sum);
    }
    output
}

pub fn main() {
    println!("{:?}", running_sum([1,2,3,4].to_vec()));
}
