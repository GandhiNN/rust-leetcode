/* You are given an integer array `prices` where `prices[i]`
is the price of a given stock on the `ith` day. You want to
maximize your profit by choosing a single day to buy one stock
and choosing a different day in the future to sell that stock.

Return the maximum profit you can achieve from this transaction.
If you cannot achieve any profit, return 0.
Note that you cannot buy stocks at day (n+1) and sell on day n
because you must buy before you sell.

Example 1:
Input: prices = [7, 1, 5, 3, 6, 4]
Output: 5
Explanation: Buy on day 2 (price = 1) and sell on day 5 (price = 6), profit = 6-1 = 5

Example 2:
Input: prices = [7, 6, 4, 3, 1]
Output = 0
Explanation: In this case, no transactions are done and the max profit = 0
*/
pub fn max_profit(prices: Vec<i32>) -> i32 {
    let mut min_price = -1; // sentinel value, stock price cannot be negative
    let mut max_profit = 0;
    for price in prices.iter() {
        if min_price < 0 {
            // trick: set the first element of the prices as the min price
            min_price = *price;
        }
        if *price < min_price {
            min_price = *price;
        } else {
            let profit = price - min_price;
            if profit > max_profit {
                max_profit = profit;
            }
        }
    }
    max_profit
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_profit_1() {
        let prices = vec![7, 1, 5, 3, 6, 4];
        let expected = 5;
        assert_eq!(max_profit(prices), expected);
    }

    #[test]
    fn test_max_profit_2() {
        let prices = vec![7, 6, 4, 3, 1];
        let expected = 0;
        assert_eq!(max_profit(prices), expected);
    }
}
