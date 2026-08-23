// LeetCode Problem 131: Palindrome Partitioning
// Approaches:
//   1) Backtracking with On-the-Fly Palindrome Check: -> O(N * 2^N) time | O(N) space
//   2) Optimal (Backtracking with 2D DP Palindrome Lookup): -> O(2^N) time | O(N^2) space
// Link: https://leetcode.com/problems/palindrome-partitioning/
//
// Examples:
//   s = "aab" -> [["a","a","b"],["aa","b"]]
//   s = "a"   -> [["a"]]

struct Solution;

impl Solution {
    /// 1. BACKTRACKING WITH ON-THE-FLY PALINDROME CHECK:
    /// Time: O(N * 2^N) | Space: O(N)
    pub fn partition(s: String) -> Vec<Vec<String>> {
        let mut result = Vec::new();
        let mut current = Vec::new();
        let bytes = s.as_bytes();

        fn is_palindrome(bytes: &[u8], mut l: usize, mut r: usize) -> bool {
            while l < r {
                if bytes[l] != bytes[r] {
                    return false;
                }
                l += 1;
                r -= 1;
            }
            true
        }

        fn backtrack(
            start: usize,
            bytes: &[u8],
            current: &mut Vec<String>,
            result: &mut Vec<Vec<String>>,
        ) {
            if start == bytes.len() {
                result.push(current.clone());
                return;
            }

            for end in start..bytes.len() {
                if is_palindrome(bytes, start, end) {
                    let substring = String::from_utf8(bytes[start..=end].to_vec()).unwrap();
                    current.push(substring);
                    backtrack(end + 1, bytes, current, result);
                    current.pop();
                }
            }
        }

        backtrack(0, bytes, &mut current, &mut result);
        result
    }

    /// 2. OPTIMAL (Backtracking with 2D DP Palindrome Table):
    /// Precompute is_pal[i][j] in O(N^2) time to eliminate redundant substring validation.
    /// Time: O(2^N) | Space: O(N^2)
    pub fn partition_dp(s: String) -> Vec<Vec<String>> {
        let bytes = s.as_bytes();
        let n = bytes.len();
        if n == 0 {
            return vec![];
        }

        // Precompute palindrome table
        let mut is_pal = vec![vec![false; n]; n];
        for i in (0..n).rev() {
            for j in i..n {
                if bytes[i] == bytes[j] && (j - i <= 2 || is_pal[i + 1][j - 1]) {
                    is_pal[i][j] = true;
                }
            }
        }

        let mut result = Vec::new();
        let mut current = Vec::new();

        fn backtrack(
            start: usize,
            n: usize,
            bytes: &[u8],
            is_pal: &[Vec<bool>],
            current: &mut Vec<String>,
            result: &mut Vec<Vec<String>>,
        ) {
            if start == n {
                result.push(current.clone());
                return;
            }

            for end in start..n {
                if is_pal[start][end] {
                    let substring = String::from_utf8(bytes[start..=end].to_vec()).unwrap();
                    current.push(substring);
                    backtrack(end + 1, n, bytes, is_pal, current, result);
                    current.pop();
                }
            }
        }

        backtrack(0, n, bytes, &is_pal, &mut current, &mut result);
        result
    }
}

fn main() {
    let test_cases = vec![
        ("aab", vec![vec!["a", "a", "b"], vec!["aa", "b"]]),
        ("a", vec![vec!["a"]]),
        ("racecar", vec![
            vec!["r", "a", "c", "e", "c", "a", "r"],
            vec!["r", "a", "cec", "a", "r"],
            vec!["r", "aceca", "r"],
            vec!["racecar"],
        ]),
    ];

    for (s, expected) in test_cases {
        let mut ans1 = Solution::partition(s.to_string());
        ans1.sort_unstable();
        let mut ans2 = Solution::partition_dp(s.to_string());
        ans2.sort_unstable();

        let mut exp_sorted: Vec<Vec<String>> = expected
            .into_iter()
            .map(|v| v.into_iter().map(|str_| str_.to_string()).collect())
            .collect();
        exp_sorted.sort_unstable();

        assert_eq!(ans1, exp_sorted);
        assert_eq!(ans2, exp_sorted);
    }

    println!("All test cases passed for Palindrome Partitioning (Backtracking, DP Lookup Table)!");
}
