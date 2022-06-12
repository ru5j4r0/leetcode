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

#[cfg(test)]
mod test_best_time_to_buy_and_sell_stock {
    fn test(prices: Vec<i32>, res: i32) {
        assert_eq!(super::max_profit(prices), res);
    }

    #[test]
    fn case1() {
        test(vec![7, 1, 5, 3, 6, 4], 5);
    }

    #[test]
    fn case2() {
        test(vec![7, 6, 4, 3, 1], 0);
    }
}
