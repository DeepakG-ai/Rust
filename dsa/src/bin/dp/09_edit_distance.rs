// LeetCode Problem 72: Edit Distance
// Approaches:
//   1) Brute Force (Recursion): -> O(3^(m+n)) time | O(m+n) call stack
//   2) Better (Memoization / Top-Down): -> O(m * n) time | O(m * n) space
//   3) Better (2D Tabulation DP): -> O(m * n) time | O(m * n) space
//   4) Optimal (1D Space-Optimized DP): -> O(m * n) time | O(n) space
// Link: https://leetcode.com/problems/edit-distance/
//
// Examples:
//   word1 = "horse", word2 = "ros" -> 3 (horse -> rorse -> rose -> ros)
//   word1 = "intention", word2 = "execution" -> 5

struct Solution;

impl Solution {
    /// 1. BRUTE FORCE (Recursion):
    /// At (i, j): If s1[i] == s2[j], solve(i+1, j+1).
    /// Else 1 + min(insert: solve(i, j+1), delete: solve(i+1, j), replace: solve(i+1, j+1)).
    /// Time: O(3^(m+n)) | Space: O(m+n)
    pub fn min_distance_recursive(word1: String, word2: String) -> i32 {
        fn solve(i: usize, j: usize, s1: &[u8], s2: &[u8]) -> i32 {
            if i == s1.len() {
                return (s2.len() - j) as i32;
            }
            if j == s2.len() {
                return (s1.len() - i) as i32;
            }
            if s1[i] == s2[j] {
                solve(i + 1, j + 1, s1, s2)
            } else {
                let insert = solve(i, j + 1, s1, s2);
                let delete = solve(i + 1, j, s1, s2);
                let replace = solve(i + 1, j + 1, s1, s2);
                1 + insert.min(delete).min(replace)
            }
        }
        solve(0, 0, word1.as_bytes(), word2.as_bytes())
    }

    /// 2. MEMOIZATION (Top-Down):
    /// Time: O(m * n) | Space: O(m * n)
    pub fn min_distance_memo(word1: String, word2: String) -> i32 {
        fn solve(i: usize, j: usize, s1: &[u8], s2: &[u8], memo: &mut Vec<Vec<i32>>) -> i32 {
            if i == s1.len() {
                return (s2.len() - j) as i32;
            }
            if j == s2.len() {
                return (s1.len() - i) as i32;
            }
            if memo[i][j] != -1 {
                return memo[i][j];
            }
            if s1[i] == s2[j] {
                memo[i][j] = solve(i + 1, j + 1, s1, s2, memo);
            } else {
                let insert = solve(i, j + 1, s1, s2, memo);
                let delete = solve(i + 1, j, s1, s2, memo);
                let replace = solve(i + 1, j + 1, s1, s2, memo);
                memo[i][j] = 1 + insert.min(delete).min(replace);
            }
            memo[i][j]
        }

        let (s1, s2) = (word1.as_bytes(), word2.as_bytes());
        let mut memo = vec![vec![-1; s2.len()]; s1.len()];
        solve(0, 0, s1, s2, &mut memo)
    }

    /// 3. TABULATION (2D Bottom-Up):
    /// dp[i][j] = min operations to convert s1[0..i] to s2[0..j].
    /// Time: O(m * n) | Space: O(m * n)
    pub fn min_distance_tabulation(word1: String, word2: String) -> i32 {
        let (s1, s2) = (word1.as_bytes(), word2.as_bytes());
        let (m, n) = (s1.len(), s2.len());
        let mut dp = vec![vec![0; n + 1]; m + 1];

        for i in 0..=m {
            dp[i][0] = i as i32;
        }
        for j in 0..=n {
            dp[0][j] = j as i32;
        }

        for i in 1..=m {
            for j in 1..=n {
                if s1[i - 1] == s2[j - 1] {
                    dp[i][j] = dp[i - 1][j - 1];
                } else {
                    dp[i][j] = 1 + dp[i - 1][j].min(dp[i][j - 1]).min(dp[i - 1][j - 1]);
                }
            }
        }
        dp[m][n]
    }

    /// 4. OPTIMAL (1D Space-Optimized DP):
    /// Time: O(m * n) | Space: O(n)
    pub fn min_distance(word1: String, word2: String) -> i32 {
        let (s1, s2) = (word1.as_bytes(), word2.as_bytes());
        let (m, n) = (s1.len(), s2.len());
        let mut dp: Vec<i32> = (0..=n as i32).collect();

        for i in 1..=m {
            let mut prev_diag = dp[0];
            dp[0] = i as i32;
            for j in 1..=n {
                let temp = dp[j];
                if s1[i - 1] == s2[j - 1] {
                    dp[j] = prev_diag;
                } else {
                    dp[j] = 1 + dp[j].min(dp[j - 1]).min(prev_diag);
                }
                prev_diag = temp;
            }
        }
        dp[n]
    }
}

fn main() {
    let test_cases = vec![
        ("horse", "ros", 3),
        ("intention", "execution", 5),
        ("", "", 0),
        ("a", "", 1),
        ("", "abc", 3),
        ("kitten", "sitting", 3),
    ];

    for &(w1, w2, expected) in &test_cases {
        if w1.len() <= 5 && w2.len() <= 5 {
            assert_eq!(
                Solution::min_distance_recursive(w1.to_string(), w2.to_string()),
                expected
            );
        }
        assert_eq!(
            Solution::min_distance_memo(w1.to_string(), w2.to_string()),
            expected
        );
        assert_eq!(
            Solution::min_distance_tabulation(w1.to_string(), w2.to_string()),
            expected
        );
        assert_eq!(
            Solution::min_distance(w1.to_string(), w2.to_string()),
            expected
        );
    }

    println!("All test cases passed for Edit Distance (Recursion, Memoization, 2D DP, 1D DP)!");
}
