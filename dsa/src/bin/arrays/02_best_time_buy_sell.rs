// LeetCode Problem 121: Best Time to Buy and Sell Stock
// Approaches:
//   1) Brute Force: Check all pairs (buy_day, sell_day) -> O(n^2) time | O(1) space
//   2) Optimal: Single-pass track minimum buying price so far -> O(n) time | O(1) space
// Link: https://leetcode.com/problems/best-time-to-buy-and-sell-stock/
//
// Examples:
//   [7,1,5,3,6,4] -> 5  (buy at 1, sell at 6)
//   [7,6,4,3,1]   -> 0  (prices only fall)

struct Solution;

impl Solution {
    /// 1. BRUTE FORCE:
    /// Try every possible buying day i and subsequent selling day j > i.
    /// Time: O(n^2) | Space: O(1)
    pub fn max_profit_brute_force(prices: Vec<i32>) -> i32 {
        let n = prices.len();
        let mut max_profit = 0;
        for i in 0..n {
            for j in (i + 1)..n {
                let profit = prices[j] - prices[i];
                if profit > max_profit {
                    max_profit = profit;
                }
            }
        }
        max_profit
    }

    /// 2. OPTIMAL (Single-pass Greedy):
    /// Maintain the lowest price seen so far. For each price, compute potential profit.
    /// Time: O(n) | Space: O(1)
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        if prices.is_empty() {
            return 0;
        }
        let mut min_price = prices[0];
        let mut max_profit = 0;

        for &price in &prices {
            if price < min_price {
                min_price = price;
            } else {
                max_profit = max_profit.max(price - min_price);
            }
        }
        max_profit
    }
}

fn main() {
    let test_cases = vec![
        (vec![7, 1, 5, 3, 6, 4], 5),
        (vec![7, 6, 4, 3, 1], 0),
        (vec![2, 4, 1], 2),
        (vec![1, 2], 1),
        (vec![3], 0),
        (vec![], 0),
    ];

    for (prices, expected) in test_cases {
        assert_eq!(Solution::max_profit_brute_force(prices.clone()), expected);
        assert_eq!(Solution::max_profit(prices), expected);
    }

    println!("All test cases passed for Best Time to Buy and Sell Stock!");
}
