// Count of All Subarrays with Sum Less Than K
// Approaches:
//   1) Brute Force: Iterate over all possible subarrays -> O(n^2) time | O(1) space
//   2) Optimal: Sliding Window -> O(n) time | O(1) space
// (Assuming non-negative / positive elements)
//
// Examples:
//   nums=[1,2,3], k=6 -> 5   ([1],[2],[3],[1,2],[2,3])
//   nums=[2,4,6], k=10 -> 4  ([2],[4],[6],[2,4])

struct Solution;

impl Solution {
    /// 1. BRUTE FORCE: For every subarray starting at i and ending at j, compute sum and count if < k.
    /// Time: O(n^2) | Space: O(1)
    pub fn count_subarrays_sum_less_than_k_brute_force(nums: Vec<i64>, k: i64) -> i64 {
        let n = nums.len();
        let mut count = 0i64;

        for i in 0..n {
            let mut sum = 0;
            for j in i..n {
                sum += nums[j];
                if sum < k {
                    count += 1;
                } else {
                    break;
                }
            }
        }
        count
    }

    /// 2. OPTIMAL (Sliding Window):
    /// Maintain a valid window [left..=right] where sum < k.
    /// The number of valid subarrays ending at `right` is exactly (right - left + 1).
    /// Time: O(n) | Space: O(1)
    pub fn count_subarrays_sum_less_than_k(nums: Vec<i64>, k: i64) -> i64 {
        let mut count = 0i64;
        let mut left = 0usize;
        let mut cur_sum = 0i64;

        for right in 0..nums.len() {
            cur_sum += nums[right];

            while cur_sum >= k && left <= right {
                cur_sum -= nums[left];
                left += 1;
            }

            if cur_sum < k && left <= right {
                count += (right - left + 1) as i64;
            }
        }
        count
    }
}

fn main() {
    let test_cases = vec![
        (vec![1, 2, 3], 6, 5),
        (vec![2, 4, 6], 10, 4),
        (vec![1, 1, 1], 3, 5),
        (vec![5], 5, 0),
        (vec![5], 6, 1),
        (vec![1, 2, 3, 4], 0, 0),
        (vec![], 10, 0),
    ];

    for (nums, k, expected) in test_cases {
        assert_eq!(
            Solution::count_subarrays_sum_less_than_k_brute_force(nums.clone(), k),
            expected
        );
        assert_eq!(
            Solution::count_subarrays_sum_less_than_k(nums, k),
            expected
        );
    }

    println!("All test cases passed for Count Subarrays Sum < K (Brute Force, Sliding Window)!");
}
