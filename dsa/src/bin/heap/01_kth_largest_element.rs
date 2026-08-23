// LeetCode Problem 215: Kth Largest Element in an Array
// Approaches:
//   1) Better (Sorting): -> O(n log n) time | O(1) auxiliary space
//   2) Better (Min-Heap of size k): -> O(n log k) time | O(k) space
//   3) Optimal (Quickselect / Hoare's Selection): -> O(n) average, O(n^2) worst case time | O(1) space
// Link: https://leetcode.com/problems/kth-largest-element-in-an-array/
//
// Examples:
//   nums = [3,2,1,5,6,4], k = 2 -> 5
//   nums = [3,2,3,1,2,4,5,5,6], k = 4 -> 4

use std::cmp::Reverse;
use std::collections::BinaryHeap;

struct Solution;

impl Solution {
    /// 1. SORTING:
    /// Sort descending and return index k - 1.
    /// Time: O(n log n) | Space: O(1)
    pub fn find_kth_largest_sort(mut nums: Vec<i32>, k: i32) -> i32 {
        nums.sort_unstable_by(|a, b| b.cmp(a));
        nums[(k - 1) as usize]
    }

    /// 2. MIN-HEAP OF SIZE K:
    /// Maintain a min-heap with k elements. Smallest among the top k is at the top.
    /// Time: O(n log k) | Space: O(k)
    pub fn find_kth_largest_heap(nums: Vec<i32>, k: i32) -> i32 {
        let k = k as usize;
        let mut min_heap = BinaryHeap::with_capacity(k);

        for num in nums {
            min_heap.push(Reverse(num));
            if min_heap.len() > k {
                min_heap.pop();
            }
        }

        min_heap.peek().unwrap().0
    }

    /// 3. OPTIMAL (Quickselect in-place):
    /// Find the (n - k)-th smallest element.
    /// Time: O(n) average | Space: O(1)
    pub fn find_kth_largest(mut nums: Vec<i32>, k: i32) -> i32 {
        let target_idx = nums.len() - k as usize;
        let (mut left, mut right) = (0, nums.len() - 1);

        while left < right {
            let pivot_idx = Self::partition(&mut nums, left, right);
            if pivot_idx == target_idx {
                return nums[pivot_idx];
            } else if pivot_idx < target_idx {
                left = pivot_idx + 1;
            } else {
                right = pivot_idx.saturating_sub(1);
            }
        }
        nums[left]
    }

    fn partition(nums: &mut [i32], left: usize, right: usize) -> usize {
        let pivot = nums[right];
        let mut i = left;

        for j in left..right {
            if nums[j] <= pivot {
                nums.swap(i, j);
                i += 1;
            }
        }
        nums.swap(i, right);
        i
    }
}

fn main() {
    let test_cases = vec![
        (vec![3, 2, 1, 5, 6, 4], 2, 5),
        (vec![3, 2, 3, 1, 2, 4, 5, 5, 6], 4, 4),
        (vec![1], 1, 1),
        (vec![7, 6, 5, 4, 3, 2, 1], 1, 7),
        (vec![7, 6, 5, 4, 3, 2, 1], 7, 1),
    ];

    for (nums, k, expected) in test_cases {
        assert_eq!(
            Solution::find_kth_largest_sort(nums.clone(), k),
            expected
        );
        assert_eq!(
            Solution::find_kth_largest_heap(nums.clone(), k),
            expected
        );
        assert_eq!(
            Solution::find_kth_largest(nums, k),
            expected
        );
    }

    println!("All test cases passed for Kth Largest Element (Sort, Min-Heap O(n log k), Quickselect O(n))!");
}
