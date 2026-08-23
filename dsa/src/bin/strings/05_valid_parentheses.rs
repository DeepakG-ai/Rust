// LeetCode Problem 20: Valid Parentheses
// Approaches:
//   1) Brute Force: Iteratively eliminate matching pairs `()`, `{}`, `[]` -> O(n^2) time | O(n) space
//   2) Optimal (Stack): Push opening, pop & match closing -> O(n) time | O(n) space
// Link: https://leetcode.com/problems/valid-parentheses/
//
// Examples:
//   "()"     -> true
//   "()[]{}" -> true
//   "(]"     -> false

struct Solution;

impl Solution {
    /// 1. BRUTE FORCE: Repeatedly replace adjacent valid pairs until string is empty or cannot be reduced.
    /// Time: O(n^2) | Space: O(n)
    pub fn is_valid_brute_force(mut s: String) -> bool {
        loop {
            let prev_len = s.len();
            s = s.replace("()", "").replace("{}", "").replace("[]", "");
            if s.is_empty() {
                return true;
            }
            if s.len() == prev_len {
                return false;
            }
        }
    }

    /// 2. OPTIMAL (Stack):
    /// Push open brackets onto stack; on closing bracket, pop top and verify match.
    /// Time: O(n) | Space: O(n)
    pub fn is_valid(s: String) -> bool {
        let mut stack: Vec<char> = Vec::with_capacity(s.len());

        for ch in s.chars() {
            match ch {
                '(' => stack.push(')'),
                '{' => stack.push('}'),
                '[' => stack.push(']'),
                ')' | '}' | ']' => {
                    if stack.pop() != Some(ch) {
                        return false;
                    }
                }
                _ => {}
            }
        }

        stack.is_empty()
    }
}

fn main() {
    let test_cases = vec![
        ("()".to_string(), true),
        ("()[]{}".to_string(), true),
        ("(]".to_string(), false),
        ("([)]".to_string(), false),
        ("{[]}".to_string(), true),
        ("".to_string(), true),
        ("[".to_string(), false),
        ("]".to_string(), false),
        ("((((({{{[[[()]]]}}})))))".to_string(), true),
    ];

    for (s, expected) in test_cases {
        assert_eq!(Solution::is_valid_brute_force(s.clone()), expected);
        assert_eq!(Solution::is_valid(s), expected);
    }

    println!("All test cases passed for Valid Parentheses (Brute Force, Stack)!");
}
