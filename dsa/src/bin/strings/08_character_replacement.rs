// LeetCode Problem 424: Longest Repeating Character Replacement
// Approaches:
//   1) Brute Force: Check all substrings -> O(n^2) time | O(26) space
//   2) Optimal: Sliding Window with max frequency tracker -> O(n) time | O(26) space
// Link: https://leetcode.com/problems/longest-repeating-character-replacement/
//
// Example:
//   s="ABAB",   k=2 -> 4  (replace both A's or both B's)
//   s="AABABBA",k=1 -> 4  ("AABA" -> replace one B)

use std::collections::HashMap;

struct Solution;

impl Solution {
    /// 1. BRUTE FORCE:
    /// Check every possible substring s[i..=j].
    /// A substring is valid if: (length - count of most frequent char in substring) <= k.
    /// Time: O(n^2) | Space: O(26)
    pub fn character_replacement_brute_force(s: String, k: i32) -> i32 {
        let chars: Vec<char> = s.chars().collect();
        let n = chars.len();
        let mut max_len = 0;

        for i in 0..n {
            let mut counts = HashMap::new();
            let mut max_freq = 0;
            for j in i..n {
                let entry = counts.entry(chars[j]).or_insert(0);
                *entry += 1;
                max_freq = max_freq.max(*entry);

                let window_len = (j - i + 1) as i32;
                if window_len - max_freq <= k {
                    max_len = max_len.max(window_len);
                }
            }
        }
        max_len
    }

    /// 2. OPTIMAL (Sliding Window):
    /// Maintain window [l..=r].
    /// Expand r and update max_freq. If (window_len - max_freq > k), shrink from l.
    /// Time: O(n) | Space: O(26)
    pub fn character_replacement(s: String, k: i32) -> i32 {
        let chars: Vec<char> = s.chars().collect();
        let mut count = [0i32; 26];
        let (mut l, mut max_freq, mut max_len) = (0usize, 0i32, 0i32);

        for r in 0..chars.len() {
            let idx = (chars[r] as u8 - b'A') as usize;
            count[idx] += 1;
            max_freq = max_freq.max(count[idx]);

            while ((r - l + 1) as i32) - max_freq > k {
                let left_idx = (chars[l] as u8 - b'A') as usize;
                count[left_idx] -= 1;
                l += 1;
            }

            max_len = max_len.max((r - l + 1) as i32);
        }
        max_len
    }
}

fn main() {
    let test_cases = vec![
        ("ABAB", 2, 4),
        ("AABABBA", 1, 4),
        ("AAAA", 0, 4),
        ("ABCDE", 1, 2),
        ("BAAA", 0, 3),
        ("", 2, 0),
    ];

    for (s, k, expected) in test_cases {
        assert_eq!(
            Solution::character_replacement_brute_force(s.to_string(), k),
            expected
        );
        assert_eq!(
            Solution::character_replacement(s.to_string(), k),
            expected
        );
    }

    println!("All test cases passed for Longest Repeating Character Replacement (Brute Force, Sliding Window)!");
}
