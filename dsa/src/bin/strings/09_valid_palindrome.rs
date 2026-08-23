// LeetCode Problem 125: Valid Palindrome
// Approaches:
//   1) Better (Filter & Reverse String): -> O(n) time | O(n) space
//   2) Optimal (Two Pointers In-Place): -> O(n) time | O(1) space
// Link: https://leetcode.com/problems/valid-palindrome/
//
// Examples:
//   "A man, a plan, a canal: Panama" -> true
//   "race a car"                     -> false

struct Solution;

impl Solution {
    /// 1. FILTER & REVERSE:
    /// Filter out non-alphanumeric characters, convert to lowercase, and check if equals reversed copy.
    /// Time: O(n) | Space: O(n)
    pub fn is_palindrome_filter_reverse(s: String) -> bool {
        let filtered: Vec<char> = s
            .chars()
            .filter(|c| c.is_ascii_alphanumeric())
            .map(|c| c.to_ascii_lowercase())
            .collect();

        let reversed: Vec<char> = filtered.iter().rev().cloned().collect();
        filtered == reversed
    }

    /// 2. OPTIMAL (Two Pointers In-Place):
    /// Walk from left and right inward, skipping non-alphanumerics without allocating an auxiliary string.
    /// Time: O(n) | Space: O(1)
    pub fn is_palindrome(s: String) -> bool {
        let bytes = s.as_bytes();
        if bytes.is_empty() {
            return true;
        }

        let mut left = 0;
        let mut right = bytes.len() - 1;

        while left < right {
            while left < right && !bytes[left].is_ascii_alphanumeric() {
                left += 1;
            }
            while left < right && !bytes[right].is_ascii_alphanumeric() {
                right -= 1;
            }

            if left < right {
                if bytes[left].to_ascii_lowercase() != bytes[right].to_ascii_lowercase() {
                    return false;
                }
                left += 1;
                right -= 1;
            }
        }
        true
    }
}

fn main() {
    let test_cases = vec![
        ("A man, a plan, a canal: Panama", true),
        ("race a car", false),
        (" ", true),
        ("0P", false),
        ("madam", true),
        ("ab_a", true),
        (".,", true),
        ("", true),
    ];

    for (s, expected) in test_cases {
        assert_eq!(
            Solution::is_palindrome_filter_reverse(s.to_string()),
            expected
        );
        assert_eq!(
            Solution::is_palindrome(s.to_string()),
            expected
        );
    }

    println!("All test cases passed for Valid Palindrome (Filter/Reverse, Two Pointers In-Place)!");
}
