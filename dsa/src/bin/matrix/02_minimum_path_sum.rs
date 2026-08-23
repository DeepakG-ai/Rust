// LeetCode Problem 64: Minimum Path Sum
// Approaches:
//   1) Brute Force (Recursion): -> O(2^(m+n)) time | O(m+n) call stack
//   2) Better (2D Tabulation DP): -> O(m * n) time | O(m * n) space
//   3) Optimal (1D Space-Optimized DP): -> O(m * n) time | O(n) space
// Link: https://leetcode.com/problems/minimum-path-sum/
//
// Examples:
//   grid = [[1,3,1],[1,5,1],[4,2,1]] -> 7 (1 -> 3 -> 1 -> 1 -> 1 = 7)
//   grid = [[1,2,3],[4,5,6]]         -> 12

struct Solution;

impl Solution {
    /// 1. BRUTE FORCE (Recursion):
    /// min_path(r, c) = grid[r][c] + min(min_path(r+1, c), min_path(r, c+1))
    /// Time: O(2^(m+n)) | Space: O(m+n)
    pub fn min_path_sum_recursive(grid: Vec<Vec<i32>>) -> i32 {
        let (m, n) = (grid.len(), grid[0].len());
        fn solve(r: usize, c: usize, m: usize, n: usize, grid: &[Vec<i32>]) -> i32 {
            if r == m - 1 && c == n - 1 {
                return grid[r][c];
            }
            if r >= m || c >= n {
                return i32::MAX;
            }
            grid[r][c] + solve(r + 1, c, m, n, grid).min(solve(r, c + 1, m, n, grid))
        }
        solve(0, 0, m, n, &grid)
    }

    /// 2. TABULATION (2D Bottom-Up):
    /// dp[r][c] = grid[r][c] + min(dp[r-1][c], dp[r][c-1])
    /// Time: O(m * n) | Space: O(m * n)
    pub fn min_path_sum_dp(grid: Vec<Vec<i32>>) -> i32 {
        let (m, n) = (grid.len(), grid[0].len());
        let mut dp = vec![vec![0; n]; m];
        dp[0][0] = grid[0][0];

        for c in 1..n {
            dp[0][c] = dp[0][c - 1] + grid[0][c];
        }
        for r in 1..m {
            dp[r][0] = dp[r - 1][0] + grid[r][0];
        }

        for r in 1..m {
            for c in 1..n {
                dp[r][c] = grid[r][c] + dp[r - 1][c].min(dp[r][c - 1]);
            }
        }
        dp[m - 1][n - 1]
    }

    /// 3. OPTIMAL (1D Space-Optimized DP):
    /// Time: O(m * n) | Space: O(n)
    pub fn min_path_sum(grid: Vec<Vec<i32>>) -> i32 {
        let (m, n) = (grid.len(), grid[0].len());
        let mut dp = vec![i32::MAX; n];
        dp[0] = 0;

        for r in 0..m {
            dp[0] += grid[r][0];
            for c in 1..n {
                dp[c] = grid[r][c] + dp[c].min(dp[c - 1]);
            }
        }
        dp[n - 1]
    }
}

fn main() {
    let test_cases = vec![
        (
            vec![
                vec![1, 3, 1],
                vec![1, 5, 1],
                vec![4, 2, 1],
            ],
            7,
        ),
        (
            vec![
                vec![1, 2, 3],
                vec![4, 5, 6],
            ],
            12,
        ),
        (vec![vec![5]], 5),
    ];

    for (grid, expected) in test_cases {
        assert_eq!(
            Solution::min_path_sum_recursive(grid.clone()),
            expected
        );
        assert_eq!(
            Solution::min_path_sum_dp(grid.clone()),
            expected
        );
        assert_eq!(
            Solution::min_path_sum(grid),
            expected
        );
    }

    println!("All test cases passed for Minimum Path Sum (Recursion, 2D DP, 1D DP O(n))!");
}
