pub fn earliest_full_bloom(plant_time: Vec<i32>, grow_time: Vec<i32>) -> i32 {
    let mut plants: Vec<(_, (_, _))> = plant_time
        .into_iter()
        .map(|x| x as i16)
        .zip(grow_time.into_iter().map(|x| x as i16))
        .enumerate()
        .collect();

    plants.sort_unstable_by_key(|(_a_i, (_a_p, a_g))| -a_g);

    let mut max_time: i32 = 0;
    // time to plant
    let mut cum_sum_p: i32 = 0;
    for (_p_i, (p_p, p_g)) in plants {
        cum_sum_p += p_p as i32;
        max_time = max_time.max(cum_sum_p + p_g as i32);
    }
    max_time
}

pub fn main() {
    println!(
        "{}",
        earliest_full_bloom([1, 4, 3].to_vec(), [2, 3, 1].to_vec())
    );
    println!(
        "{}",
        earliest_full_bloom([1, 2, 3, 2].to_vec(), [2, 1, 2, 1].to_vec())
    );
    println!("{}", earliest_full_bloom([1].to_vec(), [2].to_vec()));
    println!(
        "{}",
        earliest_full_bloom(
            [27, 5, 24, 17, 27, 4, 23, 16, 6, 26, 13, 17, 21, 3, 9, 10, 28, 26, 4, 10, 28, 2]
                .to_vec(),
            [26, 9, 14, 17, 6, 14, 23, 24, 11, 6, 27, 14, 13, 1, 15, 5, 12, 15, 23, 27, 28, 12]
                .to_vec()
        )
    );
}
