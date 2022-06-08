// impl Solution {
pub fn max_profit(prices: Vec<i32>) -> i32 {
    let mut min = prices[0];
    let mut max = 0;

    for &price in prices.iter().skip(1) {
        if price < min {
            min = price;
        }

        let profit = price - min;
        if profit > max {
            max = profit;
        }
    }

    max
}
// }
