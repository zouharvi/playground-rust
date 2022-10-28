pub fn search(nums: Vec<i32>, target: i32) -> i32 {
    let mut i = 0;
    let mut j = nums.len()-1;

    if nums[i] == target {
        return i as i32;
    }
    if nums[j] == target {
        return j as i32;
    }

    loop {
        if (i == j) && nums[i] != target {
            return -1;
        }

        let mid = (i+j)/2;
        let mid_v  = nums[mid];
        if mid_v == target {
            return mid as i32;
        } else if mid_v > target {
            if j == mid {
                return -1;
            }
            j = mid;
        } else if mid_v < target {
            if i == mid {
                return -1;
            }
            i = mid;
        }
    }
}

pub fn main() {
    println!("{}", search([-1,0,3,5,9,12].to_vec(), 9));
    println!("{}", search([-1,0,3,5,5,5,9,12].to_vec(), 9));
    println!("{}", search([9].to_vec(), 9));
    println!("{}", search([-2].to_vec(), 9));
    println!("{}", search([-2,9].to_vec(), 9));
    println!("{}", search([-2,9].to_vec(), -2));
    println!("{}", search([-1,0,3,5,9,12].to_vec(), 2));
}
