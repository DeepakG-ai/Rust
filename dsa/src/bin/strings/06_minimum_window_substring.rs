// LeetCode Problem 76: Minimum Window Substring
// Approaches:
//   1) Brute Force: Check all substrings against target character frequencies -> O(n^2 * |Sigma|) time | O(|T|) space
//   2) Optimal: Sliding Window with have/need counter -> O(|S| + |T|) time | O(|T|) space
// Link: https://leetcode.com/problems/minimum-window-substring/
//
// Examples:
//   s="ADOBECODEBANC", t="ABC" -> "BANC"
//   s="a", t="a"               -> "a"
//   s="a", t="aa"              -> ""

use std::collections::HashMap;

struct Solution;

impl Solution {
    /// 1. BRUTE FORCE:
    /// Check every possible substring s[i..=j] and verify if it contains all characters of t with sufficient frequency.
    /// Time: O(n^2 * |Sigma|) | Space: O(|T|)
    pub fn min_window_brute_force(s: String, t: String) -> String {
        if s.is_empty() || t.is_empty() {
            return String::new();
        }

        let mut need = HashMap::new();
        for c in t.chars() {
            *need.entry(c).or_insert(0) += 1;
        }

        let chars: Vec<char> = s.chars().collect();
        let n = chars.len();
        let mut min_len = usize::MAX;
        let mut best = (0, 0);

        for i in 0..n {
            let mut cur_counts = HashMap::new();
            for j in i..n {
                *cur_counts.entry(chars[j]).or_insert(0) += 1;

                let valid = need.iter().all(|(&c, &required)| {
                    cur_counts.get(&c).unwrap_or(&0) >= &required
                });

                if valid {
                    let len = j - i + 1;
                    if len < min_len {
                        min_len = len;
                        best = (i, j);
                    }
                    break; // further expanding from i will only make window larger
                }
            }
        }

        if min_len == usize::MAX {
            String::new()
        } else {
            chars[best.0..=best.1].iter().collect()
        }
    }

    /// 2. OPTIMAL (Sliding Window with Have/Need Counts):
    /// Expand right pointer. Once all unique characters in t are satisfied (have == need_count),
    /// shrink left pointer to find the minimal window.
    /// Time: O(|S| + |T|) | Space: O(|T|)
    pub fn min_window(s: String, t: String) -> String {
        if s.is_empty() || t.is_empty() {
            return String::new();
        }

        let mut need: HashMap<char, i32> = HashMap::new();
        for c in t.chars() {
            *need.entry(c).or_insert(0) += 1;
        }

        let mut window: HashMap<char, i32> = HashMap::new();
        let need_count = need.len();
        let mut have = 0;
        let mut min_len = usize::MAX;
        let mut best = (0, 0);

        let chars: Vec<char> = s.chars().collect();
        let mut l = 0;

        for r in 0..chars.len() {
            let c = chars[r];
            if need.contains_key(&c) {
                *window.entry(c).or_insert(0) += 1;
                if window[&c] == need[&c] {
                    have += 1;
                }
            }

            while have == need_count {
                if r - l + 1 < min_len {
                    min_len = r - l + 1;
                    best = (l, r);
                }

                let left_char = chars[l];
                if need.contains_key(&left_char) {
                    *window.get_mut(&left_char).unwrap() -= 1;
                    if window[&left_char] < need[&left_char] {
                        have -= 1;
                    }
                }
                l += 1;
            }
        }

        if min_len == usize::MAX {
            String::new()
        } else {
            chars[best.0..=best.1].iter().collect()
        }
    }
}

fn main() {
    let test_cases = vec![
        ("ADOBECODEBANC", "ABC", "BANC"),
        ("a", "a", "a"),
        ("a", "aa", ""),
        ("ab", "b", "b"),
        ("cabwefgewcwaefgcf", "cae", "cwae"),
        ("", "a", ""),
    ];

    for (s, t, expected) in test_cases {
        assert_eq!(
            Solution::min_window_brute_force(s.to_string(), t.to_string()),
            expected
        );
        assert_eq!(
            Solution::min_window(s.to_string(), t.to_string()),
            expected
        );
    }

    println!("All test cases passed for Minimum Window Substring (Brute Force, Sliding Window)!");
}
