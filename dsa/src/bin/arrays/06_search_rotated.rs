// LeetCode Problem 33: Search in Rotated Sorted Array
// Approaches:
//   1) Brute Force: Linear scan -> O(n) time | O(1) space
//   2) Optimal: Modified Binary Search -> O(log n) time | O(1) space
// Link: https://leetcode.com/problems/search-in-rotated-sorted-array/
//
// Examples:
//   [4,5,6,7,0,1,2], target=0 -> 4
//   [4,5,6,7,0,1,2], target=3 -> -1

struct Solution;

impl Solution {
    /// 1. BRUTE FORCE: Linear search through the array.
    /// Time: O(n) | Space: O(1)
    pub fn search_linear_brute_force(nums: Vec<i32>, target: i32) -> i32 {
        for (i, &x) in nums.iter().enumerate() {
            if x == target {
                return i as i32;
            }
        }
        -1
    }

    /// 2. OPTIMAL (Modified Binary Search):
    /// In any rotated sorted array, at least one half [lo..mid] or [mid..hi] is strictly sorted.
    /// Determine which half is sorted, then check if target is bounded within that sorted half.
    /// Time: O(log n) | Space: O(1)
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        if nums.is_empty() {
            return -1;
        }
        let mut lo = 0i32;
        let mut hi = (nums.len() - 1) as i32;

        while lo <= hi {
            let mid = lo + (hi - lo) / 2;
            let mid_u = mid as usize;
            if nums[mid_u] == target {
                return mid;
            }

            let lo_u = lo as usize;
            let hi_u = hi as usize;

            if nums[lo_u] <= nums[mid_u] {
                // Left half [lo..mid] is sorted
                if nums[lo_u] <= target && target < nums[mid_u] {
                    hi = mid - 1; // target is inside left half
                } else {
                    lo = mid + 1; // search right half
                }
            } else {
                // Right half [mid..hi] is sorted
                if nums[mid_u] < target && target <= nums[hi_u] {
                    lo = mid + 1; // target is inside right half
                } else {
                    hi = mid - 1; // search left half
                }
            }
        }
        -1
    }
}

fn main() {
    let test_cases = vec![
        (vec![4, 5, 6, 7, 0, 1, 2], 0, 4),
        (vec![4, 5, 6, 7, 0, 1, 2], 3, -1),
        (vec![1], 0, -1),
        (vec![1], 1, 0),
        (vec![5, 1, 3], 5, 0),
        (vec![3, 1], 1, 1),
        (vec![1, 3], 3, 1),
        (vec![6, 7, 1, 2, 3, 4, 5], 6, 0),
    ];

    for (nums, target, expected) in test_cases {
        assert_eq!(Solution::search_linear_brute_force(nums.clone(), target), expected);
        assert_eq!(Solution::search(nums, target), expected);
    }

    println!("All test cases passed for Search in Rotated Sorted Array (Brute Force, Binary Search)!");
}
