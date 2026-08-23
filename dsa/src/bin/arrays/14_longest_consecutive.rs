// LeetCode Problem 128: Longest Consecutive Sequence
// Approaches:
//   1) Brute Force: For every element x, search for x+1, x+2, ... linearly -> O(n^2) time | O(1) space
//   2) Better (Sorting): Sort and find longest adjacent difference == 1 run -> O(n log n) time | O(1) space
//   3) Optimal (HashSet Sequence-Start): Look up consecutive sequence only if (x - 1) not in set -> O(n) time | O(n) space
// Link: https://leetcode.com/problems/longest-consecutive-sequence/
//
// Examples:
//   [100,4,200,1,3,2]          -> 4   ([1,2,3,4])
//   [0,3,7,2,5,8,4,6,0,1]      -> 9

use std::collections::HashSet;

struct Solution;

impl Solution {
    /// 1. BRUTE FORCE: For each number x, linearly search the entire array for x + 1, x + 2, etc.
    /// Time: O(n^2) | Space: O(1)
    pub fn longest_consecutive_brute_force(nums: Vec<i32>) -> i32 {
        let mut max_len = 0;

        for &x in &nums {
            let mut current_num = x;
            let mut current_streak = 1;

            while nums.contains(&(current_num + 1)) {
                current_num += 1;
                current_streak += 1;
            }

            max_len = max_len.max(current_streak);
        }
        max_len
    }

    /// 2. BETTER (Sorting):
    /// Sort array, eliminate duplicates, and measure longest consecutive run.
    /// Time: O(n log n) | Space: O(1) or O(n) depending on sort
    pub fn longest_consecutive_sorting(mut nums: Vec<i32>) -> i32 {
        if nums.is_empty() {
            return 0;
        }
        nums.sort_unstable();
        nums.dedup();

        let mut longest = 1;
        let mut current_length = 1;

        for i in 1..nums.len() {
            if nums[i] == nums[i - 1] + 1 {
                current_length += 1;
            } else {
                longest = longest.max(current_length);
                current_length = 1;
            }
        }
        longest.max(current_length)
    }

    /// 3. OPTIMAL (HashSet Starting Point Check):
    /// Insert all elements into a HashSet. Only start counting consecutive sequences
    /// from numbers that are the beginning of a sequence (i.e. x - 1 does not exist in set).
    /// Each element is visited at most twice.
    /// Time: O(n) | Space: O(n)
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        let num_set: HashSet<i32> = nums.into_iter().collect();
        let mut max_len = 0;

        for &n in &num_set {
            if !num_set.contains(&(n - 1)) {
                let mut current_num = n;
                let mut length = 1;

                while num_set.contains(&(current_num + 1)) {
                    current_num += 1;
                    length += 1;
                }
                max_len = max_len.max(length);
            }
        }
        max_len
    }
}

fn main() {
    let test_cases = vec![
        (vec![100, 4, 200, 1, 3, 2], 4),
        (vec![0, 3, 7, 2, 5, 8, 4, 6, 0, 1], 9),
        (vec![], 0),
        (vec![5], 1),
        (vec![1, 1, 1, 1], 1),
        (vec![-2, -1, 0, 1], 4),
        (vec![9, 1, 4, 7, 3, -1, 0, 5, 8, -1, 6], 7),
    ];

    for (nums, expected) in test_cases {
        assert_eq!(Solution::longest_consecutive_brute_force(nums.clone()), expected);
        assert_eq!(Solution::longest_consecutive_sorting(nums.clone()), expected);
        assert_eq!(Solution::longest_consecutive(nums), expected);
    }

    println!("All test cases passed for Longest Consecutive Sequence (Brute Force, Sorting, HashSet)!");
}
