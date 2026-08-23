// LeetCode Problem 1: Two Sum
// Approaches:
//   1) Brute Force: Check all pairs -> O(n^2) time | O(1) space
//   2) Better (Sorting + Two Pointers): Pair with index, sort & scan -> O(n log n) time | O(n) space
//   3) Optimal (Hash Map): One-pass complement lookup -> O(n) time | O(n) space
// Link: https://leetcode.com/problems/two-sum/
//
// Examples:
//   [2,7,11,15], target=9 -> [0,1]
//   [3,2,4],     target=6 -> [1,2]
//   [3,3],       target=6 -> [0,1]

use std::collections::HashMap;

struct Solution;

impl Solution {
    /// 1. BRUTE FORCE: Nested loops checking every possible pair (i, j).
    /// Time: O(n^2) | Space: O(1)
    pub fn two_sum_brute_force(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let n = nums.len();
        for i in 0..n {
            for j in (i + 1)..n {
                if nums[i] + nums[j] == target {
                    return vec![i as i32, j as i32];
                }
            }
        }
        Vec::new()
    }

    /// 2. BETTER (Sorting + Two Pointers):
    /// Maintain original indices, sort by value, and use two pointers.
    /// Time: O(n log n) | Space: O(n)
    pub fn two_sum_better_sort(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut indexed: Vec<(i32, usize)> = nums.into_iter().enumerate().map(|(i, v)| (v, i)).collect();
        indexed.sort_unstable_by_key(|&(v, _)| v);

        let mut left = 0;
        let mut right = indexed.len().saturating_sub(1);

        while left < right {
            let sum = indexed[left].0 + indexed[right].0;
            if sum == target {
                let mut res = vec![indexed[left].1 as i32, indexed[right].1 as i32];
                res.sort_unstable(); // normalize index order
                return res;
            } else if sum < target {
                left += 1;
            } else {
                right -= 1;
            }
        }
        Vec::new()
    }

    /// 3. OPTIMAL (Hash Map):
    /// Single pass: lookup complement in hash map; if missing, record current number and index.
    /// Time: O(n) | Space: O(n)
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut seen: HashMap<i32, i32> = HashMap::with_capacity(nums.len());

        for (i, &num) in nums.iter().enumerate() {
            let diff = target - num;
            if let Some(&j) = seen.get(&diff) {
                return vec![j, i as i32];
            }
            seen.insert(num, i as i32);
        }
        Vec::new()
    }
}

fn main() {
    let test_cases = vec![
        (vec![2, 7, 11, 15], 9, vec![0, 1]),
        (vec![3, 2, 4], 6, vec![1, 2]),
        (vec![3, 3], 6, vec![0, 1]),
        (vec![-1, -2, -3, -4, -5], -8, vec![2, 4]),
        (vec![0, 4, 3, 0], 0, vec![0, 3]),
    ];

    for (nums, target, expected) in test_cases {
        assert_eq!(Solution::two_sum_brute_force(nums.clone(), target), expected);
        assert_eq!(Solution::two_sum_better_sort(nums.clone(), target), expected);
        assert_eq!(Solution::two_sum(nums, target), expected);
    }

    println!("All test cases passed for Two Sum (Brute Force, Better, Optimal)!");
}
