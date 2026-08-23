// LeetCode Problem 322: Coin Change
// Approaches:
//   1) Brute Force (Recursion): -> O(S^n) time | O(amount) stack
//   2) Better (Memoization / Top-Down): -> O(amount * n) time | O(amount) space
//   3) Optimal (Tabulation / Bottom-Up): -> O(amount * n) time | O(amount) space
// Link: https://leetcode.com/problems/coin-change/
//
// Examples:
//   coins = [1,2,5], amount = 11 -> 3 (5 + 5 + 1)
//   coins = [2], amount = 3 -> -1
//   coins = [1], amount = 0 -> 0

struct Solution;

impl Solution {
    /// 1. BRUTE FORCE (Recursion):
    /// Try every coin and take the minimum of recursive subproblems.
    /// Time: O(len(coins)^amount) | Space: O(amount)
    pub fn coin_change_recursive(coins: Vec<i32>, amount: i32) -> i32 {
        fn solve(coins: &[i32], rem: i32) -> i32 {
            if rem == 0 {
                return 0;
            }
            if rem < 0 {
                return -1;
            }
            let mut min_coins = i32::MAX;
            for &c in coins {
                let res = solve(coins, rem - c);
                if res >= 0 && res < min_coins {
                    min_coins = res + 1;
                }
            }
            if min_coins == i32::MAX { -1 } else { min_coins }
        }
        solve(&coins, amount)
    }

    /// 2. MEMOIZATION (Top-Down):
    /// Time: O(amount * n) | Space: O(amount)
    pub fn coin_change_memo(coins: Vec<i32>, amount: i32) -> i32 {
        fn solve(coins: &[i32], rem: i32, memo: &mut Vec<i32>) -> i32 {
            if rem == 0 {
                return 0;
            }
            if rem < 0 {
                return -1;
            }
            if memo[rem as usize] != -2 {
                return memo[rem as usize];
            }
            let mut min_coins = i32::MAX;
            for &c in coins {
                let res = solve(coins, rem - c, memo);
                if res >= 0 && res < min_coins {
                    min_coins = res + 1;
                }
            }
            memo[rem as usize] = if min_coins == i32::MAX { -1 } else { min_coins };
            memo[rem as usize]
        }

        if amount == 0 {
            return 0;
        }
        let mut memo = vec![-2; (amount as usize) + 1];
        solve(&coins, amount, &mut memo)
    }

    /// 3. OPTIMAL (Tabulation / Bottom-Up DP):
    /// dp[i] = min coins to make amount i.
    /// dp[i] = min(dp[i], dp[i - coin] + 1)
    /// Time: O(amount * n) | Space: O(amount)
    pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
        if amount == 0 {
            return 0;
        }
        let n = amount as usize;
        let mut dp = vec![amount + 1; n + 1];
        dp[0] = 0;

        for i in 1..=n {
            for &coin in &coins {
                if coin <= i as i32 {
                    dp[i] = dp[i].min(dp[i - coin as usize] + 1);
                }
            }
        }

        if dp[n] > amount { -1 } else { dp[n] }
    }
}

fn main() {
    let test_cases = vec![
        (vec![1, 2, 5], 11, 3),
        (vec![2], 3, -1),
        (vec![1], 0, 0),
        (vec![1, 5, 6, 9], 11, 2), // 5 + 6
        (vec![2, 5, 10, 1], 27, 4), // 10 + 10 + 5 + 2
    ];

    for (coins, amount, expected) in test_cases {
        assert_eq!(
            Solution::coin_change_recursive(coins.clone(), amount),
            expected
        );
        assert_eq!(
            Solution::coin_change_memo(coins.clone(), amount),
            expected
        );
        assert_eq!(
            Solution::coin_change(coins, amount),
            expected
        );
    }

    println!("All test cases passed for Coin Change (Recursion, Memoization, Bottom-Up DP)!");
}
