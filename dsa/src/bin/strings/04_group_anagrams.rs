// LeetCode Problem 49: Group Anagrams
// Approaches:
//   1) Better (Sorted String as Key): -> O(N * K log K) time | O(N * K) space
//   2) Optimal (26-Byte Character Count Tuple as Key): -> O(N * K) time | O(N * K) space
// Link: https://leetcode.com/problems/group-anagrams/
//
// Examples:
//   ["eat","tea","tan","ate","nat","bat"]
//   -> [["eat","tea","ate"], ["tan","nat"], ["bat"]]

use std::collections::HashMap;

struct Solution;

impl Solution {
    /// 1. SORTED STRING KEY:
    /// Sort each string to form the canonical key for the hash map.
    /// Time: O(N * K log K) where N = number of strings, K = max string length | Space: O(N * K)
    pub fn group_anagrams_sorting_key(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut groups: HashMap<String, Vec<String>> = HashMap::new();

        for s in strs {
            let mut key: Vec<char> = s.chars().collect();
            key.sort_unstable();
            let key_str: String = key.into_iter().collect();
            groups.entry(key_str).or_default().push(s);
        }
        groups.into_values().collect()
    }

    /// 2. OPTIMAL (26-Element Frequency Array as Key):
    /// Compute the 26-character count array in O(K) without sorting overhead.
    /// Time: O(N * K) | Space: O(N * K)
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut groups: HashMap<[u8; 26], Vec<String>> = HashMap::new();

        for s in strs {
            let mut count = [0u8; 26];
            for b in s.bytes() {
                count[(b - b'a') as usize] += 1;
            }
            groups.entry(count).or_default().push(s);
        }
        groups.into_values().collect()
    }
}

fn normalize_result(mut res: Vec<Vec<String>>) -> Vec<Vec<String>> {
    for group in &mut res {
        group.sort_unstable();
    }
    res.sort_unstable();
    res
}

fn main() {
    let test_cases = vec![
        vec![
            "eat".to_string(),
            "tea".to_string(),
            "tan".to_string(),
            "ate".to_string(),
            "nat".to_string(),
            "bat".to_string(),
        ],
        vec!["".to_string()],
        vec!["a".to_string()],
        vec!["cab".to_string(), "tin".to_string(), "pew".to_string(), "duh".to_string(), "may".to_string(), "ill".to_string(), "buy".to_string(), "bar".to_string(), "max".to_string(), "doc".to_string()],
    ];

    for strs in test_cases {
        let r1 = normalize_result(Solution::group_anagrams_sorting_key(strs.clone()));
        let r2 = normalize_result(Solution::group_anagrams(strs));
        assert_eq!(r1, r2);
    }

    println!("All test cases passed for Group Anagrams (Sorted Key, 26-Byte Count Key)!");
}
