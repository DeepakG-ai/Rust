// LeetCode Problem 42: Trapping Rain Water
// Approaches:
//   1) Brute Force: Scan left and right max for each bar -> O(n^2) time | O(1) space
//   2) Better (Prefix & Suffix Max Arrays): Precompute maximums -> O(n) time | O(n) space
//   3) Optimal (Two Pointers): Inward scan with left_max and right_max -> O(n) time | O(1) space
// Link: https://leetcode.com/problems/trapping-rain-water/
//
// Example:
//   [0,1,0,2,1,0,1,3,2,1,2,1] -> 6

struct Solution;

impl Solution {
    /// 1. BRUTE FORCE:
    /// For every bar i, find tallest bar to its left and tallest to its right.
    /// Trapped water at i = max(0, min(max_left, max_right) - height[i]).
    /// Time: O(n^2) | Space: O(1)
    pub fn trap_brute_force(height: Vec<i32>) -> i32 {
        let n = height.len();
        if n < 3 {
            return 0;
        }
        let mut total = 0;

        for i in 1..n - 1 {
            let left_max = *height[..=i].iter().max().unwrap();
            let right_max = *height[i..].iter().max().unwrap();
            let water = left_max.min(right_max) - height[i];
            if water > 0 {
                total += water;
            }
        }
        total
    }

    /// 2. BETTER (Prefix and Suffix Max Arrays):
    /// Precompute left_max[i] and right_max[i] in two linear passes.
    /// Time: O(n) | Space: O(n)
    pub fn trap_prefix_suffix_arrays(height: Vec<i32>) -> i32 {
        let n = height.len();
        if n < 3 {
            return 0;
        }

        let mut left_max = vec![0; n];
        let mut right_max = vec![0; n];

        left_max[0] = height[0];
        for i in 1..n {
            left_max[i] = left_max[i - 1].max(height[i]);
        }

        right_max[n - 1] = height[n - 1];
        for i in (0..n - 1).rev() {
            right_max[i] = right_max[i + 1].max(height[i]);
        }

        let mut total = 0;
        for i in 0..n {
            let water = left_max[i].min(right_max[i]) - height[i];
            if water > 0 {
                total += water;
            }
        }
        total
    }

    /// 3. OPTIMAL (Two Pointers):
    /// Keep left_max and right_max. Move smaller pointer inward since that side is the bottleneck.
    /// Time: O(n) | Space: O(1)
    pub fn trap(height: Vec<i32>) -> i32 {
        if height.len() < 3 {
            return 0;
        }
        let mut left = 0;
        let mut right = height.len() - 1;
        let mut left_max = 0;
        let mut right_max = 0;
        let mut total = 0;

        while left < right {
            if height[left] <= height[right] {
                if height[left] >= left_max {
                    left_max = height[left];
                } else {
                    total += left_max - height[left];
                }
                left += 1;
            } else {
                if height[right] >= right_max {
                    right_max = height[right];
                } else {
                    total += right_max - height[right];
                }
                right -= 1;
            }
        }
        total
    }
}

fn main() {
    let test_cases = vec![
        (vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1], 6),
        (vec![4, 2, 0, 3, 2, 5], 9),
        (vec![3, 3, 3], 0),
        (vec![1, 2], 0),
        (vec![], 0),
        (vec![5, 4, 1, 2], 1),
    ];

    for (heights, expected) in test_cases {
        assert_eq!(Solution::trap_brute_force(heights.clone()), expected);
        assert_eq!(Solution::trap_prefix_suffix_arrays(heights.clone()), expected);
        assert_eq!(Solution::trap(heights), expected);
    }

    println!("All test cases passed for Trapping Rain Water (Brute Force, Prefix/Suffix, Two Pointers)!");
}
