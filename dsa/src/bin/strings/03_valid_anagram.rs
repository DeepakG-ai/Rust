// LeetCode Problem 242: Valid Anagram
// Approaches:
//   1) Better (Sorting): Sort both strings and compare -> O(n log n) time | O(n) space
//   2) Better (HashMap): Count frequency of each character -> O(n) time | O(k) space
//   3) Optimal (Fixed 26-Element Array): Stack-allocated array counter -> O(n) time | O(1) space
// Link: https://leetcode.com/problems/valid-anagram/
//
// Examples:
//   "anagram" vs "nagaram" -> true
//   "rat"     vs "car"     -> false

use std::collections::HashMap;

struct Solution;

impl Solution {
    /// 1. SORTING APPROACH:
    /// Two strings are anagrams if and only if their sorted character sequences match.
    /// Time: O(n log n) | Space: O(n)
    pub fn is_anagram_sorting(s: String, t: String) -> bool {
        if s.len() != t.len() {
            return false;
        }
        let mut a: Vec<u8> = s.bytes().collect();
        let mut b: Vec<u8> = t.bytes().collect();
        a.sort_unstable();
        b.sort_unstable();
        a == b
    }

    /// 2. HASHMAP APPROACH (handles arbitrary Unicode characters):
    /// Increment count for characters in s, decrement for t.
    /// Time: O(n) | Space: O(k) unique characters
    pub fn is_anagram_hashmap(s: String, t: String) -> bool {
        if s.len() != t.len() {
            return false;
        }
        let mut counts: HashMap<char, i32> = HashMap::new();
        for c in s.chars() {
            *counts.entry(c).or_insert(0) += 1;
        }
        for c in t.chars() {
            match counts.get_mut(&c) {
                Some(v) if *v > 0 => *v -= 1,
                _ => return false,
            }
        }
        true
    }

    /// 3. OPTIMAL (Fixed 26-Element Frequency Array):
    /// For standard lowercase English letters, track frequencies in a stack-allocated array.
    /// Time: O(n) | Space: O(1)
    pub fn is_anagram(s: String, t: String) -> bool {
        if s.len() != t.len() {
            return false;
        }
        let mut counts = [0i32; 26];
        let sb = s.as_bytes();
        let tb = t.as_bytes();

        for i in 0..sb.len() {
            counts[(sb[i] - b'a') as usize] += 1;
            counts[(tb[i] - b'a') as usize] -= 1;
        }

        counts.iter().all(|&c| c == 0)
    }
}

fn main() {
    let test_cases = vec![
        ("anagram", "nagaram", true),
        ("rat", "car", false),
        ("a", "ab", false),
        ("listen", "silent", true),
        ("triangle", "integral", true),
        ("apple", "pale", false),
        ("", "", true),
    ];

    for (s, t, expected) in test_cases {
        assert_eq!(
            Solution::is_anagram_sorting(s.to_string(), t.to_string()),
            expected
        );
        assert_eq!(
            Solution::is_anagram_hashmap(s.to_string(), t.to_string()),
            expected
        );
        assert_eq!(
            Solution::is_anagram(s.to_string(), t.to_string()),
            expected
        );
    }

    println!("All test cases passed for Valid Anagram (Sorting, HashMap, 26-Array)!");
}
