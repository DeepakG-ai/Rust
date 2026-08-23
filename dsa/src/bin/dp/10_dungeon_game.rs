// LeetCode Problem 174: Dungeon Game
// Approaches:
//   1) Brute Force (Recursion): -> O(2^(m+n)) time | O(m+n) call stack
//   2) Better (2D Bottom-Up Tabulation DP): -> O(m * n) time | O(m * n) space
//   3) Optimal (1D Space-Optimized DP): -> O(m * n) time | O(n) space
// Link: https://leetcode.com/problems/dungeon-game/
//
// Examples:
//   dungeon = [[-2,-3,3],[-5,-10,1],[10,30,-5]] -> 7

struct Solution;

impl Solution {
    /// 1. BRUTE FORCE (Recursion starting from Bottom-Right princess room):
    /// min_health(r, c) = max(1, min(min_health(r+1, c), min_health(r, c+1)) - dungeon[r][c])
    /// Time: O(2^(m+n)) | Space: O(m+n)
    pub fn calculate_minimum_hp_recursive(dungeon: Vec<Vec<i32>>) -> i32 {
        let (m, n) = (dungeon.len(), dungeon[0].len());

        fn solve(r: usize, c: usize, m: usize, n: usize, d: &[Vec<i32>]) -> i32 {
            if r == m - 1 && c == n - 1 {
                return (1 - d[r][c]).max(1);
            }
            if r >= m || c >= n {
                return i32::MAX;
            }
            let next_step = solve(r + 1, c, m, n, d).min(solve(r, c + 1, m, n, d));
            (next_step - d[r][c]).max(1)
        }

        solve(0, 0, m, n, &dungeon)
    }

    /// 2. TABULATION (2D Bottom-Up starting from princess room):
    /// Time: O(m * n) | Space: O(m * n)
    pub fn calculate_minimum_hp_dp(dungeon: Vec<Vec<i32>>) -> i32 {
        let (m, n) = (dungeon.len(), dungeon[0].len());
        let mut dp = vec![vec![i32::MAX; n + 1]; m + 1];

        // Base case: to exit princess room with at least 1 HP
        dp[m][n - 1] = 1;
        dp[m - 1][n] = 1;

        for r in (0..m).rev() {
            for c in (0..n).rev() {
                let min_exit_hp = dp[r + 1][c].min(dp[r][c + 1]);
                dp[r][c] = (min_exit_hp - dungeon[r][c]).max(1);
            }
        }
        dp[0][0]
    }

    /// 3. OPTIMAL (1D Space-Optimized DP):
    /// Time: O(m * n) | Space: O(n)
    pub fn calculate_minimum_hp(dungeon: Vec<Vec<i32>>) -> i32 {
        let (m, n) = (dungeon.len(), dungeon[0].len());
        let mut dp = vec![i32::MAX; n + 1];
        dp[n - 1] = 1;

        for r in (0..m).rev() {
            for c in (0..n).rev() {
                let min_exit_hp = dp[c].min(dp[c + 1]);
                dp[c] = (min_exit_hp - dungeon[r][c]).max(1);
            }
            dp[n] = i32::MAX; // boundary condition for right edge
        }
        dp[0]
    }
}

fn main() {
    let test_cases = vec![
        (
            vec![
                vec![-2, -3, 3],
                vec![-5, -10, 1],
                vec![10, 30, -5],
            ],
            7,
        ),
        (vec![vec![0]], 1),
        (vec![vec![100]], 1),
        (vec![vec![-5]], 6),
        (vec![vec![1, -3, 3], vec![0, -2, 0], vec![-3, -3, -3]], 3),
    ];

    for (dungeon, expected) in test_cases {
        assert_eq!(
            Solution::calculate_minimum_hp_recursive(dungeon.clone()),
            expected
        );
        assert_eq!(
            Solution::calculate_minimum_hp_dp(dungeon.clone()),
            expected
        );
        assert_eq!(
            Solution::calculate_minimum_hp(dungeon),
            expected
        );
    }

    println!("All test cases passed for Dungeon Game (Recursion, 2D DP, 1D DP)!");
}
