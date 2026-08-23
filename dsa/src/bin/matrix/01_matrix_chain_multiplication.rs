// Matrix Chain Multiplication (MCM) - Classic Dynamic Programming
// Approaches:
//   1) Brute Force (Recursion): -> O(2^n) time | O(n) call stack
//   2) Better (Memoization / Top-Down): -> O(n^3) time | O(n^2) space
//   3) Optimal (Tabulation / Bottom-Up Interval DP): -> O(n^3) time | O(n^2) space
//
// Description:
//   Given array p[] of dimension sizes where matrix A_i has dimension p[i-1] x p[i].
//   Find the minimum number of scalar multiplications needed to multiply the chain.
//
// Examples:
//   p = [40, 20, 30, 10, 30] -> 26000
//   p = [10, 20, 30, 40, 30] -> 30000
//   p = [10, 20, 30]         -> 6000

struct Solution;

impl Solution {
    /// 1. BRUTE FORCE (Recursion):
    /// mcm(i, j) = min_{i <= k < j} (mcm(i, k) + mcm(k+1, j) + p[i-1]*p[k]*p[j])
    /// Time: O(2^n) | Space: O(n)
    pub fn matrix_chain_order_recursive(p: &[i32]) -> i32 {
        fn solve(i: usize, j: usize, p: &[i32]) -> i32 {
            if i >= j {
                return 0;
            }
            let mut min_ops = i32::MAX;
            for k in i..j {
                let cost = solve(i, k, p) + solve(k + 1, j, p) + p[i - 1] * p[k] * p[j];
                min_ops = min_ops.min(cost);
            }
            min_ops
        }
        if p.len() <= 2 {
            return 0;
        }
        solve(1, p.len() - 1, p)
    }

    /// 2. MEMOIZATION (Top-Down):
    /// Time: O(n^3) | Space: O(n^2)
    pub fn matrix_chain_order_memo(p: &[i32]) -> i32 {
        let n = p.len();
        if n <= 2 {
            return 0;
        }

        fn solve(i: usize, j: usize, p: &[i32], memo: &mut Vec<Vec<i32>>) -> i32 {
            if i >= j {
                return 0;
            }
            if memo[i][j] != -1 {
                return memo[i][j];
            }
            let mut min_ops = i32::MAX;
            for k in i..j {
                let cost = solve(i, k, p, memo) + solve(k + 1, j, p, memo) + p[i - 1] * p[k] * p[j];
                min_ops = min_ops.min(cost);
            }
            memo[i][j] = min_ops;
            memo[i][j]
        }

        let mut memo = vec![vec![-1; n]; n];
        solve(1, n - 1, p, &mut memo)
    }

    /// 3. TABULATION (Bottom-Up Interval DP):
    /// Iterate over chain length len from 2 to n - 1.
    /// Time: O(n^3) | Space: O(n^2)
    pub fn matrix_chain_order(p: &[i32]) -> i32 {
        let n = p.len();
        if n <= 2 {
            return 0;
        }

        // dp[i][j] stores minimum cost of multiplying A_i .. A_j
        let mut dp = vec![vec![0; n]; n];

        for len in 2..n {
            for i in 1..(n - len + 1) {
                let j = i + len - 1;
                dp[i][j] = i32::MAX;
                for k in i..j {
                    let cost = dp[i][k] + dp[k + 1][j] + p[i - 1] * p[k] * p[j];
                    dp[i][j] = dp[i][j].min(cost);
                }
            }
        }
        dp[1][n - 1]
    }
}

fn main() {
    let test_cases = vec![
        (vec![40, 20, 30, 10, 30], 26000),
        (vec![10, 20, 30, 40, 30], 30000),
        (vec![10, 20, 30], 6000),
        (vec![10, 30, 5, 60], 4500),
    ];

    for (p, expected) in test_cases {
        assert_eq!(
            Solution::matrix_chain_order_recursive(&p),
            expected
        );
        assert_eq!(
            Solution::matrix_chain_order_memo(&p),
            expected
        );
        assert_eq!(
            Solution::matrix_chain_order(&p),
            expected
        );
    }

    println!("All test cases passed for Matrix Chain Multiplication (Recursion, Memoization, Interval DP O(n^3))!");
}
