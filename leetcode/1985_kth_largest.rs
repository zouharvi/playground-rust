pub fn kth_largest_number(mut nums: Vec<String>, k: i32) -> String {
    use std::cmp::Ordering;
    nums.sort_by(|a, b| {
        let result = a.len().cmp(&b.len());
        if result != Ordering::Equal {
            return result;
        }

        for (a, b) in a.chars().zip(b.chars()) {
            let result = a.cmp(&b);
            if result != Ordering::Equal {
                return result;
            }
        }

        Ordering::Equal
    });
    println!("{:?}", nums);
    nums[nums.len() - k as usize].clone()
}

pub fn main() {
    println!(
        "{:?}",
        kth_largest_number(
            [
                "3".to_string(),
                "6".to_string(),
                "7".to_string(),
                "10".to_string()
            ]
            .to_vec(),
            4
        )
    );
    println!(
        "{:?}",
        kth_largest_number(
            [
                "2".to_string(),
                "21".to_string(),
                "12".to_string(),
                "1".to_string()
            ]
            .to_vec(),
            3
        )
    );
}
