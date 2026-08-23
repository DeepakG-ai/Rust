// LeetCode Problem 1143: Longest Common Subsequence (LCS)
// Approaches:
//   1) Brute Force (Recursion): -> O(2^(m+n)) time | O(m+n) call stack
//   2) Better (Memoization / Top-Down): -> O(m * n) time | O(m * n) space
//   3) Better (2D Tabulation DP): -> O(m * n) time | O(m * n) space
//   4) Optimal (1D Space-Optimized DP): -> O(m * n) time | O(n) space
// Link: https://leetcode.com/problems/longest-common-subsequence/
//
// Examples:
//   text1 = "abcde", text2 = "ace" -> 3 ("ace")
//   text1 = "abc", text2 = "abc"   -> 3 ("abc")
//   text1 = "abc", text2 = "def"   -> 0

struct Solution;

impl Solution {
    /// 1. BRUTE FORCE (Recursion):
    /// If text1[i] == text2[j], 1 + lcs(i+1, j+1), else max(lcs(i+1, j), lcs(i, j+1)).
    /// Time: O(2^(m+n)) | Space: O(m+n)
    pub fn lcs_recursive(text1: String, text2: String) -> i32 {
        fn solve(i: usize, j: usize, s1: &[u8], s2: &[u8]) -> i32 {
            if i == s1.len() || j == s2.len() {
                return 0;
            }
            if s1[i] == s2[j] {
                1 + solve(i + 1, j + 1, s1, s2)
            } else {
                solve(i + 1, j, s1, s2).max(solve(i, j + 1, s1, s2))
            }
        }
        solve(0, 0, text1.as_bytes(), text2.as_bytes())
    }

    /// 2. MEMOIZATION (Top-Down):
    /// Time: O(m * n) | Space: O(m * n)
    pub fn lcs_memo(text1: String, text2: String) -> i32 {
        fn solve(i: usize, j: usize, s1: &[u8], s2: &[u8], memo: &mut Vec<Vec<i32>>) -> i32 {
            if i == s1.len() || j == s2.len() {
                return 0;
            }
            if memo[i][j] != -1 {
                return memo[i][j];
            }
            if s1[i] == s2[j] {
                memo[i][j] = 1 + solve(i + 1, j + 1, s1, s2, memo);
            } else {
                memo[i][j] = solve(i + 1, j, s1, s2, memo).max(solve(i, j + 1, s1, s2, memo));
            }
            memo[i][j]
        }

        let (s1, s2) = (text1.as_bytes(), text2.as_bytes());
        let mut memo = vec![vec![-1; s2.len()]; s1.len()];
        solve(0, 0, s1, s2, &mut memo)
    }

    /// 3. TABULATION (2D Bottom-Up):
    /// dp[i][j] = LCS of s1[0..i] and s2[0..j].
    /// Time: O(m * n) | Space: O(m * n)
    pub fn lcs_tabulation(text1: String, text2: String) -> i32 {
        let (s1, s2) = (text1.as_bytes(), text2.as_bytes());
        let (m, n) = (s1.len(), s2.len());
        let mut dp = vec![vec![0; n + 1]; m + 1];

        for i in 1..=m {
            for j in 1..=n {
                if s1[i - 1] == s2[j - 1] {
                    dp[i][j] = dp[i - 1][j - 1] + 1;
                } else {
                    dp[i][j] = dp[i - 1][j].max(dp[i][j - 1]);
                }
            }
        }
        dp[m][n]
    }

    /// 4. OPTIMAL (1D Space-Optimized DP):
    /// Track previous row and current row.
    /// Time: O(m * n) | Space: O(min(m, n))
    pub fn longest_common_subsequence(text1: String, text2: String) -> i32 {
        let (s1, s2) = (text1.as_bytes(), text2.as_bytes());
        let (s1, s2) = if s1.len() < s2.len() { (s2, s1) } else { (s1, s2) };
        let (m, n) = (s1.len(), s2.len());

        let mut dp = vec![0; n + 1];

        for i in 1..=m {
            let mut prev_diag = 0;
            for j in 1..=n {
                let temp = dp[j];
                if s1[i - 1] == s2[j - 1] {
                    dp[j] = prev_diag + 1;
                } else {
                    dp[j] = dp[j].max(dp[j - 1]);
                }
                prev_diag = temp;
            }
        }
        dp[n]
    }
}

fn main() {
    let test_cases = vec![
        ("abcde", "ace", 3),
        ("abc", "abc", 3),
        ("abc", "def", 0),
        ("pmjghexybyrgzrcrmbtx", "hafcdqbgncrcbihkd", 6),
        ("", "abc", 0),
    ];

    for &(t1, t2, expected) in &test_cases {
        if t1.len() <= 5 && t2.len() <= 5 {
            assert_eq!(
                Solution::lcs_recursive(t1.to_string(), t2.to_string()),
                expected
            );
        }
        assert_eq!(
            Solution::lcs_memo(t1.to_string(), t2.to_string()),
            expected
        );
        assert_eq!(
            Solution::lcs_tabulation(t1.to_string(), t2.to_string()),
            expected
        );
        assert_eq!(
            Solution::longest_common_subsequence(t1.to_string(), t2.to_string()),
            expected
        );
    }

    println!("All test cases passed for Longest Common Subsequence (Recursion, Memo, 2D DP, 1D DP)!");
}
