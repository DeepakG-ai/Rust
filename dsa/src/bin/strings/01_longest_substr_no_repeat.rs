// LeetCode Problem 3: Longest Substring Without Repeating Characters
// Approaches:
//   1) Brute Force: Check all substrings -> O(n^2) time | O(min(n, charset)) space
//   2) Better (Sliding Window with HashSet): Step-by-step shrink -> O(2n) time | O(min(n, charset)) space
//   3) Optimal (Direct Jump with Last-Seen Map): -> O(n) time | O(min(n, charset)) space
// Link: https://leetcode.com/problems/longest-substring-without-repeating-characters/
//
// Examples:
//   "abcabcbb" -> 3  ("abc")
//   "bbbbb"    -> 1  ("b")
//   "pwwkew"   -> 3  ("wke")

use std::collections::{HashMap, HashSet};

struct Solution;

impl Solution {
    /// 1. BRUTE FORCE:
    /// For every starting index i, expand j until a duplicate character is seen.
    /// Time: O(n^2) | Space: O(min(n, charset))
    pub fn length_of_longest_substring_brute(s: String) -> i32 {
        let chars: Vec<char> = s.chars().collect();
        let n = chars.len();
        let mut longest = 0;

        for i in 0..n {
            let mut seen = HashSet::new();
            for j in i..n {
                if !seen.insert(chars[j]) {
                    break;
                }
                longest = longest.max(j - i + 1);
            }
        }
        longest as i32
    }

    /// 2. BETTER (Sliding Window with HashSet):
    /// When chars[right] is already in the window set, increment left and remove chars[left] one by one.
    /// Each character is visited at most twice (once by left, once by right).
    /// Time: O(2n) | Space: O(min(n, charset))
    pub fn length_of_longest_substring_window_set(s: String) -> i32 {
        let chars: Vec<char> = s.chars().collect();
        let mut left = 0usize;
        let mut longest = 0usize;
        let mut window = HashSet::new();

        for right in 0..chars.len() {
            while window.contains(&chars[right]) {
                window.remove(&chars[left]);
                left += 1;
            }
            window.insert(chars[right]);
            longest = longest.max(right - left + 1);
        }
        longest as i32
    }

    /// 3. OPTIMAL (Sliding Window with Direct Index Jumping):
    /// Map stores character -> last seen index + 1.
    /// When duplicate is found, left jumps directly to max(left, last_seen_index + 1), avoiding step-by-step shrinking.
    /// Time: O(n) | Space: O(min(n, charset))
    pub fn length_of_longest_substring(s: String) -> i32 {
        let chars: Vec<char> = s.chars().collect();
        let mut last_seen: HashMap<char, usize> = HashMap::new();
        let mut left = 0usize;
        let mut longest = 0usize;

        for right in 0..chars.len() {
            if let Some(&prev_idx) = last_seen.get(&chars[right]) {
                left = left.max(prev_idx + 1);
            }
            last_seen.insert(chars[right], right);
            longest = longest.max(right - left + 1);
        }
        longest as i32
    }
}

fn main() {
    let test_cases = vec![
        ("abcabcbb".to_string(), 3),
        ("bbbbb".to_string(), 1),
        ("pwwkew".to_string(), 3),
        ("".to_string(), 0),
        ("au".to_string(), 2),
        ("abba".to_string(), 2),
        ("dvdf".to_string(), 3),
    ];

    for (s, expected) in test_cases {
        assert_eq!(Solution::length_of_longest_substring_brute(s.clone()), expected);
        assert_eq!(Solution::length_of_longest_substring_window_set(s.clone()), expected);
        assert_eq!(Solution::length_of_longest_substring(s), expected);
    }

    println!("All test cases passed for Longest Substring Without Repeating Characters (Brute Force, Window Set, Index Jump)!");
}
