// LeetCode Problem 91: Decode Ways
// Approaches:
//   1) Brute Force (Recursion): -> O(2^n) time | O(n) call stack
//   2) Better (Memoization / Top-Down): -> O(n) time | O(n) space
//   3) Better (Tabulation / Bottom-Up): -> O(n) time | O(n) space
//   4) Optimal (Space-Optimized DP): -> O(n) time | O(1) space
// Link: https://leetcode.com/problems/decode-ways/
//
// Examples:
//   "12" -> 2 ("AB" (1 2) or "L" (12))
//   "226" -> 3 ("BZ" (2 26), "VF" (22 6), or "BBF" (2 2 6))
//   "06" -> 0

struct Solution;

impl Solution {
    /// 1. BRUTE FORCE (Recursion):
    /// At each index i, we can take 1 digit (if != '0') or 2 digits (if between "10" and "26").
    /// Time: O(2^n) | Space: O(n)
    pub fn num_decodings_recursive(s: String) -> i32 {
        fn solve(i: usize, bytes: &[u8]) -> i32 {
            if i == bytes.len() {
                return 1;
            }
            if bytes[i] == b'0' {
                return 0;
            }
            let mut ways = solve(i + 1, bytes);
            if i + 1 < bytes.len() {
                let two_digit = (bytes[i] - b'0') * 10 + (bytes[i + 1] - b'0');
                if two_digit <= 26 {
                    ways += solve(i + 2, bytes);
                }
            }
            ways
        }
        solve(0, s.as_bytes())
    }

    /// 2. MEMOIZATION (Top-Down):
    /// Time: O(n) | Space: O(n)
    pub fn num_decodings_memo(s: String) -> i32 {
        fn solve(i: usize, bytes: &[u8], memo: &mut Vec<i32>) -> i32 {
            if i == bytes.len() {
                return 1;
            }
            if bytes[i] == b'0' {
                return 0;
            }
            if memo[i] != -1 {
                return memo[i];
            }
            let mut ways = solve(i + 1, bytes, memo);
            if i + 1 < bytes.len() {
                let two_digit = (bytes[i] - b'0') * 10 + (bytes[i + 1] - b'0');
                if two_digit <= 26 {
                    ways += solve(i + 2, bytes, memo);
                }
            }
            memo[i] = ways;
            memo[i]
        }

        let bytes = s.as_bytes();
        let mut memo = vec![-1; bytes.len()];
        solve(0, bytes, &mut memo)
    }

    /// 3. TABULATION (Bottom-Up):
    /// dp[i] represents number of decodings for prefix of length i.
    /// Time: O(n) | Space: O(n)
    pub fn num_decodings_tabulation(s: String) -> i32 {
        let bytes = s.as_bytes();
        let n = bytes.len();
        if n == 0 || bytes[0] == b'0' {
            return 0;
        }

        let mut dp = vec![0; n + 1];
        dp[0] = 1;
        dp[1] = 1;

        for i in 2..=n {
            // Single digit
            if bytes[i - 1] != b'0' {
                dp[i] += dp[i - 1];
            }
            // Two digits
            let two_digit = (bytes[i - 2] - b'0') * 10 + (bytes[i - 1] - b'0');
            if (10..=26).contains(&two_digit) {
                dp[i] += dp[i - 2];
            }
        }
        dp[n]
    }

    /// 4. OPTIMAL (Space-Optimized DP):
    /// Time: O(n) | Space: O(1)
    pub fn num_decodings(s: String) -> i32 {
        let bytes = s.as_bytes();
        let n = bytes.len();
        if n == 0 || bytes[0] == b'0' {
            return 0;
        }

        let mut prev2 = 1;
        let mut prev1 = 1;

        for i in 2..=n {
            let mut cur = 0;
            if bytes[i - 1] != b'0' {
                cur += prev1;
            }
            let two_digit = (bytes[i - 2] - b'0') * 10 + (bytes[i - 1] - b'0');
            if (10..=26).contains(&two_digit) {
                cur += prev2;
            }
            prev2 = prev1;
            prev1 = cur;
        }
        prev1
    }
}

fn main() {
    let test_cases = vec![
        ("12", 2),
        ("226", 3),
        ("06", 0),
        ("10", 1),
        ("27", 1),
        ("2101", 1),
        ("111111", 13),
    ];

    for (s, expected) in test_cases {
        assert_eq!(
            Solution::num_decodings_recursive(s.to_string()),
            expected
        );
        assert_eq!(
            Solution::num_decodings_memo(s.to_string()),
            expected
        );
        assert_eq!(
            Solution::num_decodings_tabulation(s.to_string()),
            expected
        );
        assert_eq!(
            Solution::num_decodings(s.to_string()),
            expected
        );
    }

    println!("All test cases passed for Decode Ways (Recursion, Memoization, Tabulation, Space O(1))!");
}
