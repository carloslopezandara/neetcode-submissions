impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut most_profit = 0;
        let mut min_buy = prices[0];

        for i in 0..prices.len() {
            min_buy = min_buy.min(prices[i]);
            most_profit = most_profit.max(prices[i] - min_buy);
        }
        most_profit
    }
}
