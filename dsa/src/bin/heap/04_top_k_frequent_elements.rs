// LeetCode Problem 347: Top K Frequent Elements
// Approaches:
//   1) Better (HashMap Frequency + Sort): -> O(n log n) time | O(n) space
//   2) Better (Min-Heap of Size K): -> O(n log k) time | O(n + k) space
//   3) Optimal (Bucket Sort / Frequency Indexing): -> O(n) time | O(n) space
// Link: https://leetcode.com/problems/top-k-frequent-elements/
//
// Examples:
//   nums = [1,1,1,2,2,3], k = 2 -> [1,2]
//   nums = [1], k = 1           -> [1]

use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};

struct Solution;

impl Solution {
    /// 1. HASHMAP + SORTING:
    /// Time: O(n log n) | Space: O(n)
    pub fn top_k_frequent_sort(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut counts = HashMap::new();
        for num in nums {
            *counts.entry(num).or_insert(0) += 1;
        }

        let mut freq_vec: Vec<(i32, usize)> = counts.into_iter().collect();
        freq_vec.sort_unstable_by(|a, b| b.1.cmp(&a.1));
        freq_vec.into_iter().take(k as usize).map(|(num, _)| num).collect()
    }

    /// 2. MIN-HEAP OF SIZE K:
    /// Store tuples of (count, num) in min-heap.
    /// Time: O(n log k) | Space: O(n + k)
    pub fn top_k_frequent_heap(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut counts = HashMap::new();
        for num in nums {
            *counts.entry(num).or_insert(0) += 1;
        }

        let k = k as usize;
        let mut min_heap: BinaryHeap<Reverse<(usize, i32)>> = BinaryHeap::with_capacity(k);

        for (num, count) in counts {
            min_heap.push(Reverse((count, num)));
            if min_heap.len() > k {
                min_heap.pop();
            }
        }

        min_heap.into_iter().map(|Reverse((_, num))| num).collect()
    }

    /// 3. OPTIMAL (Bucket Sort):
    /// Array of buckets where index is the frequency (0..=n).
    /// Walk backwards from bucket n to 1 and collect k elements.
    /// Time: O(n) | Space: O(n)
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let n = nums.len();
        let mut counts = HashMap::new();
        for &num in &nums {
            *counts.entry(num).or_insert(0) += 1;
        }

        let mut buckets: Vec<Vec<i32>> = vec![Vec::new(); n + 1];
        for (num, count) in counts {
            buckets[count].push(num);
        }

        let mut result = Vec::new();
        for freq in (0..=n).rev() {
            for &num in &buckets[freq] {
                result.push(num);
                if result.len() == k as usize {
                    return result;
                }
            }
        }
        result
    }
}

fn main() {
    let test_cases = vec![
        (vec![1, 1, 1, 2, 2, 3], 2, vec![1, 2]),
        (vec![1], 1, vec![1]),
        (vec![4, 1, -1, 2, -1, 2, 3], 2, vec![-1, 2]),
    ];

    for (nums, k, expected) in test_cases {
        let mut ans1 = Solution::top_k_frequent_sort(nums.clone(), k);
        ans1.sort_unstable();
        let mut ans2 = Solution::top_k_frequent_heap(nums.clone(), k);
        ans2.sort_unstable();
        let mut ans3 = Solution::top_k_frequent(nums, k);
        ans3.sort_unstable();

        let mut exp_sorted = expected;
        exp_sorted.sort_unstable();

        assert_eq!(ans1, exp_sorted);
        assert_eq!(ans2, exp_sorted);
        assert_eq!(ans3, exp_sorted);
    }

    println!("All test cases passed for Top K Frequent Elements (Sort, Min-Heap, Bucket Sort O(n))!");
}
