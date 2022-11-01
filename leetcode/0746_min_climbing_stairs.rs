pub fn min_cost_climbing_stairs(cost: Vec<i32>) -> i32 {
    let mut min_costs : Vec<i32> = cost.iter().map(|_| i32::MAX).collect();
    min_costs.push(i32::MAX);
    min_costs[0] = 0;
    min_costs[1] = 0;
    for i in 2..(cost.len()+1) {
        min_costs[i] = (min_costs[i - 1] + cost[i - 1]).min(min_costs[i - 2] + cost[i - 2]);
    }

    min_costs[min_costs.len()-1]
}

pub fn main() {
    println!("{}", min_cost_climbing_stairs([10, 15, 20].to_vec()));
    println!("{}", min_cost_climbing_stairs([1,100,1,1,1,100,1,1,100,1].to_vec()));
}
