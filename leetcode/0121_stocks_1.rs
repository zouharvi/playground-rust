pub fn max_profit(prices: Vec<i32>) -> i32 {
    let mut min = prices[0];
    let mut max_profit = 0;
    for price in &prices[1..] {
        min = min.min(*price);
        max_profit = max_profit.max(price-min);
    }
    max_profit
}

pub fn main() {
    println!("{}", max_profit([7, 1, 5, 3, 6, 4].to_vec()));
    println!("{}", max_profit([7,6,4,3,1].to_vec()));
    println!("{}", max_profit([7].to_vec()));
    println!("{}", max_profit([0, 7, 10].to_vec()));
    println!("{}", max_profit([4, 7, 10].to_vec()));
    println!("{}", max_profit([4, 7, 10, 0, 7].to_vec()));
}
