// LeetCode Problem 11: Container With Most Water
// Approaches:
//   1) Brute Force: Check all pairs (i, j) -> O(n^2) time | O(1) space
//   2) Optimal: Two Pointers inward scan -> O(n) time | O(1) space
// Link: https://leetcode.com/problems/container-with-most-water/
//
// Examples:
//   [1,8,6,2,5,4,8,3,7] -> 49
//   [1,1]               -> 1

struct Solution;

impl Solution {
    /// 1. BRUTE FORCE:
    /// Calculate area for every combination of left line i and right line j.
    /// area = min(height[i], height[j]) * (j - i)
    /// Time: O(n^2) | Space: O(1)
    pub fn max_area_brute_force(height: Vec<i32>) -> i32 {
        let n = height.len();
        let mut max_water = 0;

        for i in 0..n {
            for j in (i + 1)..n {
                let water = height[i].min(height[j]) * (j - i) as i32;
                max_water = max_water.max(water);
            }
        }
        max_water
    }

    /// 2. OPTIMAL (Two Pointers):
    /// Start with widest container (left = 0, right = n - 1).
    /// To potentially find a taller container, always move the pointer pointing to the shorter line inward.
    /// Time: O(n) | Space: O(1)
    pub fn max_area(height: Vec<i32>) -> i32 {
        if height.len() < 2 {
            return 0;
        }
        let mut left = 0;
        let mut right = height.len() - 1;
        let mut max_water = 0;

        while left < right {
            let width = (right - left) as i32;
            let current_water = height[left].min(height[right]) * width;
            max_water = max_water.max(current_water);

            if height[left] < height[right] {
                left += 1;
            } else {
                right -= 1;
            }
        }
        max_water
    }
}

fn main() {
    let test_cases = vec![
        (vec![1, 8, 6, 2, 5, 4, 8, 3, 7], 49),
        (vec![1, 1], 1),
        (vec![4, 3, 2, 1, 4], 16),
        (vec![1, 2, 1], 2),
        (vec![2], 0),
        (vec![], 0),
    ];

    for (height, expected) in test_cases {
        assert_eq!(Solution::max_area_brute_force(height.clone()), expected);
        assert_eq!(Solution::max_area(height), expected);
    }

    println!("All test cases passed for Container With Most Water (Brute Force, Two Pointers)!");
}
