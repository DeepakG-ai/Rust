// LeetCode Problem 62: Unique Paths
// Approaches:
//   1) Brute Force (Recursion): -> O(2^(m+n)) time | O(m+n) call stack
//   2) Better (Memoization / 2D DP): -> O(m * n) time | O(m * n) space
//   3) Better (1D Space-Optimized DP): -> O(m * n) time | O(n) space
//   4) Optimal (Combinatorics / Math): -> O(min(m, n)) time | O(1) space
// Link: https://leetcode.com/problems/unique-paths/
//
// Examples:
//   m = 3, n = 7 -> 28
//   m = 3, n = 2 -> 3

struct Solution;

impl Solution {
    /// 1. BRUTE FORCE (Recursion):
    /// paths(r, c) = paths(r+1, c) + paths(r, c+1)
    /// Time: O(2^(m+n)) | Space: O(m+n)
    pub fn unique_paths_recursive(m: i32, n: i32) -> i32 {
        fn solve(r: i32, c: i32, m: i32, n: i32) -> i32 {
            if r == m - 1 && c == n - 1 {
                return 1;
            }
            if r >= m || c >= n {
                return 0;
            }
            solve(r + 1, c, m, n) + solve(r, c + 1, m, n)
        }
        solve(0, 0, m, n)
    }

    /// 2. MEMOIZATION (Top-Down):
    /// Time: O(m * n) | Space: O(m * n)
    pub fn unique_paths_memo(m: i32, n: i32) -> i32 {
        fn solve(r: usize, c: usize, m: usize, n: usize, memo: &mut Vec<Vec<i32>>) -> i32 {
            if r == m - 1 && c == n - 1 {
                return 1;
            }
            if r >= m || c >= n {
                return 0;
            }
            if memo[r][c] != -1 {
                return memo[r][c];
            }
            memo[r][c] = solve(r + 1, c, m, n, memo) + solve(r, c + 1, m, n, memo);
            memo[r][c]
        }

        let mut memo = vec![vec![-1; n as usize]; m as usize];
        solve(0, 0, m as usize, n as usize, &mut memo)
    }

    /// 3. TABULATION (1D Space-Optimized Bottom-Up):
    /// dp[c] = dp[c] (from above) + dp[c-1] (from left)
    /// Time: O(m * n) | Space: O(n)
    pub fn unique_paths_dp(m: i32, n: i32) -> i32 {
        let (m, n) = (m as usize, n as usize);
        let mut dp = vec![1; n];

        for _ in 1..m {
            for c in 1..n {
                dp[c] += dp[c - 1];
            }
        }
        dp[n - 1]
    }

    /// 4. OPTIMAL (Combinatorics):
    /// Total steps = (m-1) downs + (n-1) rights = (m+n-2).
    /// Choose (m-1) downs out of (m+n-2) steps: C(m+n-2, m-1).
    /// Time: O(min(m, n)) | Space: O(1)
    pub fn unique_paths(m: i32, n: i32) -> i32 {
        let total_steps = (m + n - 2) as i64;
        let k = ((m - 1).min(n - 1)) as i64;
        let mut ans: i64 = 1;

        for i in 1..=k {
            ans = ans * (total_steps - k + i) / i;
        }
        ans as i32
    }
}

fn main() {
    let test_cases = vec![
        (3, 7, 28),
        (3, 2, 3),
        (7, 3, 28),
        (3, 3, 6),
        (1, 1, 1),
        (10, 10, 48620),
    ];

    for (m, n, expected) in test_cases {
        assert_eq!(Solution::unique_paths_recursive(m, n), expected);
        assert_eq!(Solution::unique_paths_memo(m, n), expected);
        assert_eq!(Solution::unique_paths_dp(m, n), expected);
        assert_eq!(Solution::unique_paths(m, n), expected);
    }

    println!("All test cases passed for Unique Paths (Recursion, Memoization, 1D DP, Combinatorics)!");
}
