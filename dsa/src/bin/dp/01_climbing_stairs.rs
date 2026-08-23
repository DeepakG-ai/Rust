// LeetCode Problem 70: Climbing Stairs
// Approaches:
//   1) Brute Force (Recursion): -> O(2^n) time | O(n) call stack
//   2) Better (Memoization / Top-Down): -> O(n) time | O(n) space
//   3) Better (Tabulation / Bottom-Up): -> O(n) time | O(n) space
//   4) Optimal (Space-Optimized DP): -> O(n) time | O(1) space
// Link: https://leetcode.com/problems/climbing-stairs/
//
// Examples:
//   n = 2 -> 2  (1+1, 2)
//   n = 3 -> 3  (1+1+1, 1+2, 2+1)
//   n = 4 -> 5

struct Solution;

impl Solution {
    /// 1. BRUTE FORCE (Pure Recursion):
    /// f(n) = f(n-1) + f(n-2)
    /// Time: O(2^n) | Space: O(n)
    pub fn climb_stairs_recursive(n: i32) -> i32 {
        if n <= 2 {
            return n;
        }
        Self::climb_stairs_recursive(n - 1) + Self::climb_stairs_recursive(n - 2)
    }

    /// 2. MEMOIZATION (Top-Down):
    /// Cache computed values in a memo vector.
    /// Time: O(n) | Space: O(n)
    pub fn climb_stairs_memo(n: i32) -> i32 {
        fn solve(i: usize, memo: &mut Vec<i32>) -> i32 {
            if i <= 2 {
                return i as i32;
            }
            if memo[i] != -1 {
                return memo[i];
            }
            memo[i] = solve(i - 1, memo) + solve(i - 2, memo);
            memo[i]
        }

        let mut memo = vec![-1; (n as usize) + 1];
        solve(n as usize, &mut memo)
    }

    /// 3. TABULATION (Bottom-Up DP):
    /// Iteratively build dp array from base cases dp[1]=1, dp[2]=2.
    /// Time: O(n) | Space: O(n)
    pub fn climb_stairs_tabulation(n: i32) -> i32 {
        if n <= 2 {
            return n;
        }
        let n = n as usize;
        let mut dp = vec![0; n + 1];
        dp[1] = 1;
        dp[2] = 2;
        for i in 3..=n {
            dp[i] = dp[i - 1] + dp[i - 2];
        }
        dp[n]
    }

    /// 4. OPTIMAL (Space-Optimized DP):
    /// Maintain only the last two step counts.
    /// Time: O(n) | Space: O(1)
    pub fn climb_stairs(n: i32) -> i32 {
        if n <= 2 {
            return n;
        }
        let mut prev2 = 1;
        let mut prev1 = 2;
        for _ in 3..=n {
            let cur = prev1 + prev2;
            prev2 = prev1;
            prev1 = cur;
        }
        prev1
    }
}

fn main() {
    let test_cases = vec![
        (1, 1),
        (2, 2),
        (3, 3),
        (4, 5),
        (5, 8),
        (10, 89),
    ];

    for (n, expected) in test_cases {
        assert_eq!(Solution::climb_stairs_recursive(n), expected);
        assert_eq!(Solution::climb_stairs_memo(n), expected);
        assert_eq!(Solution::climb_stairs_tabulation(n), expected);
        assert_eq!(Solution::climb_stairs(n), expected);
    }

    // Large n test for optimal space O(1)
    assert_eq!(Solution::climb_stairs(45), 1836311903);

    println!("All test cases passed for Climbing Stairs (Recursion, Memoization, Tabulation, Space O(1))!");
}
