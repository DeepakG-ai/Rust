// LeetCode Problem 139: Word Break
// Approaches:
//   1) Brute Force (Recursion): Try all possible prefix splits -> O(2^n) time | O(n) stack
//   2) Better (Memoization / Top-Down): -> O(n^2 * m) time | O(n) space (m = avg word len)
//   3) Optimal (Bottom-Up Tabulation DP): -> O(n^2 * m) time | O(n) space
// Link: https://leetcode.com/problems/word-break/
//
// Examples:
//   s = "leetcode", wordDict = ["leet","code"] -> true
//   s = "applepenapple", wordDict = ["apple","pen"] -> true
//   s = "catsandog", wordDict = ["cats","dog","sand","and","cat"] -> false

use std::collections::HashSet;

struct Solution;

impl Solution {
    /// 1. BRUTE FORCE (Recursion):
    /// At position i, try every prefix s[i..j+1] that is in the dict. If it matches, recurse on j+1.
    /// Time: O(2^n) | Space: O(n)
    pub fn word_break_recursive(s: String, word_dict: Vec<String>) -> bool {
        let dict: HashSet<String> = word_dict.into_iter().collect();

        fn solve(start: usize, s: &str, dict: &HashSet<String>) -> bool {
            if start == s.len() {
                return true;
            }
            for end in (start + 1)..=s.len() {
                if dict.contains(&s[start..end]) && solve(end, s, dict) {
                    return true;
                }
            }
            false
        }
        solve(0, &s, &dict)
    }

    /// 2. MEMOIZATION (Top-Down):
    /// Time: O(n^2 * m) | Space: O(n)
    pub fn word_break_memo(s: String, word_dict: Vec<String>) -> bool {
        let dict: HashSet<String> = word_dict.into_iter().collect();
        let n = s.len();
        let mut memo = vec![None; n];

        fn solve(
            start: usize,
            s: &str,
            dict: &HashSet<String>,
            memo: &mut Vec<Option<bool>>,
        ) -> bool {
            if start == s.len() {
                return true;
            }
            if let Some(cached) = memo[start] {
                return cached;
            }
            let mut result = false;
            for end in (start + 1)..=s.len() {
                if dict.contains(&s[start..end]) && solve(end, s, dict, memo) {
                    result = true;
                    break;
                }
            }
            memo[start] = Some(result);
            result
        }

        solve(0, &s, &dict, &mut memo)
    }

    /// 3. OPTIMAL (Bottom-Up Tabulation DP):
    /// dp[i] = can we segment s[0..i] using dictionary words?
    /// dp[i] = true if there exists j < i such that dp[j] == true AND s[j..i] is in dict.
    /// Time: O(n^2 * m) | Space: O(n)
    pub fn word_break(s: String, word_dict: Vec<String>) -> bool {
        let dict: HashSet<String> = word_dict.into_iter().collect();
        let n = s.len();
        let mut dp = vec![false; n + 1];
        dp[0] = true; // empty prefix is always valid

        for i in 1..=n {
            for j in 0..i {
                if dp[j] && dict.contains(&s[j..i]) {
                    dp[i] = true;
                    break;
                }
            }
        }
        dp[n]
    }
}

fn main() {
    let test_cases = vec![
        ("leetcode", vec!["leet", "code"], true),
        ("applepenapple", vec!["apple", "pen"], true),
        ("catsandog", vec!["cats", "dog", "sand", "and", "cat"], false),
        ("a", vec!["a"], true),
        ("ab", vec!["a", "b"], true),
        ("", vec!["a"], true),
    ];

    for (s, dict, expected) in test_cases {
        let dict_strings: Vec<String> = dict.iter().map(|w| w.to_string()).collect();

        assert_eq!(
            Solution::word_break_recursive(s.to_string(), dict_strings.clone()),
            expected
        );
        assert_eq!(
            Solution::word_break_memo(s.to_string(), dict_strings.clone()),
            expected
        );
        assert_eq!(
            Solution::word_break(s.to_string(), dict_strings),
            expected
        );
    }

    println!("All test cases passed for Word Break (Recursion, Memoization, Bottom-Up DP)!");
}
