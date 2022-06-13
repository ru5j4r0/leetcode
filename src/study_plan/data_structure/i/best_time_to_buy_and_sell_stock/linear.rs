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
mod test {
    fn test<const N: usize>(prices: [i32; N], res: i32) {
        assert_eq!(super::max_profit(Vec::from(prices)), res);
    }

    #[test]
    fn case1() {
        test([7, 1, 5, 3, 6, 4], 5);
    }

    #[test]
    fn case2() {
        test([7, 6, 4, 3, 1], 0);
    }
}
