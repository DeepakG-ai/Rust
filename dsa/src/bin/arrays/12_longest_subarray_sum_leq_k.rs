// Length of Longest Subarray with Sum Less Than or Equal to K
// Approaches:
//   1) Brute Force: Evaluate all subarrays -> O(n^2) time | O(1) space
//   2) Optimal: Sliding Window (Two Pointers) -> O(n) time | O(1) space
// (Assuming non-negative / positive elements)
//
// Examples:
//   nums=[3,1,2,1], k=4  -> 3   ([1,2,1])
//   nums=[5,1,1,1], k=5  -> 3   ([1,1,1])
//   nums=[1,2,3],   k=100-> 3   (whole array)

struct Solution;

impl Solution {
    /// 1. BRUTE FORCE: Check sum of every subarray [i..=j].
    /// Time: O(n^2) | Space: O(1)
    pub fn longest_subarray_sum_leq_k_brute_force(nums: Vec<i64>, k: i64) -> usize {
        let n = nums.len();
        let mut max_len = 0;

        for i in 0..n {
            let mut sum = 0;
            for j in i..n {
                sum += nums[j];
                if sum <= k {
                    max_len = max_len.max(j - i + 1);
                } else {
                    break; // elements are positive, adding more will only increase sum
                }
            }
        }
        max_len
    }

    /// 2. OPTIMAL (Sliding Window):
    /// Expand right pointer adding to cur_sum. Shrink left pointer whenever sum exceeds k.
    /// Time: O(n) | Space: O(1)
    pub fn longest_subarray_sum_leq_k(nums: Vec<i64>, k: i64) -> usize {
        let mut longest = 0usize;
        let mut left = 0usize;
        let mut cur_sum = 0i64;

        for right in 0..nums.len() {
            cur_sum += nums[right];

            while cur_sum > k && left <= right {
                cur_sum -= nums[left];
                left += 1;
            }

            if cur_sum <= k {
                longest = longest.max((right + 1).saturating_sub(left));
            }
        }
        longest
    }
}

fn main() {
    let test_cases = vec![
        (vec![3, 1, 2, 1], 4, 3),
        (vec![5, 1, 1, 1], 5, 3),
        (vec![1, 2, 3], 100, 3),
        (vec![10, 20], 5, 0),
        (vec![2, 2, 2], 6, 3),
        (vec![1, 2, 1, 0, 1, 1, 0], 4, 5),
        (vec![], 5, 0),
    ];

    for (nums, k, expected) in test_cases {
        assert_eq!(Solution::longest_subarray_sum_leq_k_brute_force(nums.clone(), k), expected);
        assert_eq!(Solution::longest_subarray_sum_leq_k(nums, k), expected);
    }

    println!("All test cases passed for Longest Subarray Sum <= K (Brute Force, Sliding Window)!");
}