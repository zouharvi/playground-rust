pub fn max_profit_simple(prices: &[i32]) -> i32 {
    let mut min = prices[0];
    let mut max_profit = 0;
    for price in &prices[1..] {
        min = min.min(*price);
        max_profit = max_profit.max(price - min);
    }
    max_profit
}

pub fn max_profit(prices: Vec<i32>) -> i32 {
    let mut max_profit = 0;
    for i in 1..prices.len() {
        let profit1 = max_profit_simple(&prices[0..i]);
        let profit2 = max_profit_simple(&prices[i-1..]);
        max_profit = max_profit.max(profit1 + profit2);
    }
    let profit_simple = max_profit_simple(&prices);
    max_profit = max_profit.max(profit_simple);
    max_profit
}

pub fn main() {
    println!("{}", max_profit([7, 1, 5, 3, 6, 4].to_vec())); // 7
    println!("{}", max_profit([1, 2, 3, 4, 5].to_vec())); // 4
    println!("{}", max_profit([7, 6, 4, 3, 1].to_vec())); // 0
    println!("{}", max_profit([3, 3, 5, 0, 0, 3, 1, 4].to_vec())); // 8
}
