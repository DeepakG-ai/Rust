// LeetCode Problem 53: Maximum Subarray
// Approaches:
//   1) Brute Force: Compute sums of all possible subarrays -> O(n^2) time | O(1) space
//   2) Better (DP Table): dp[i] = max(dp[i-1] + nums[i], nums[i]) -> O(n) time | O(n) space
//   3) Optimal (Kadane's Algorithm): Reset running sum when negative -> O(n) time | O(1) space
// Link: https://leetcode.com/problems/maximum-subarray/
//
// Examples:
//   [-2,1,-3,4,-1,2,1,-5,4] -> 6   ([4,-1,2,1])
//   [5,4,-1,7,8]            -> 23

struct Solution;

impl Solution {
    /// 1. BRUTE FORCE: Consider every starting index i and extend to j, maintaining the running sum.
    /// Time: O(n^2) | Space: O(1)
    pub fn max_sub_array_brute_force(nums: Vec<i32>) -> i32 {
        if nums.is_empty() {
            return 0;
        }
        let n = nums.len();
        let mut max_sum = nums[0];

        for i in 0..n {
            let mut current_sum = 0;
            for j in i..n {
                current_sum += nums[j];
                max_sum = max_sum.max(current_sum);
            }
        }
        max_sum
    }

    /// 2. BETTER (Dynamic Programming with Table):
    /// dp[i] represents the maximum subarray sum ending at index i.
    /// Recurrence: dp[i] = max(dp[i-1] + nums[i], nums[i])
    /// Time: O(n) | Space: O(n)
    pub fn max_sub_array_dp(nums: Vec<i32>) -> i32 {
        if nums.is_empty() {
            return 0;
        }
        let n = nums.len();
        let mut dp = vec![0i32; n];
        dp[0] = nums[0];
        let mut max_sum = dp[0];

        for i in 1..n {
            dp[i] = (dp[i - 1] + nums[i]).max(nums[i]);
            max_sum = max_sum.max(dp[i]);
        }
        max_sum
    }

    /// 3. OPTIMAL (Kadane's Algorithm):
    /// Track running sum; if it drops below 0, reset to 0 since negative prefix hurts future sums.
    /// Time: O(n) | Space: O(1)
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        if nums.is_empty() {
            return 0;
        }
        let mut max_sub = nums[0];
        let mut cur_sum = 0;

        for &x in &nums {
            if cur_sum < 0 {
                cur_sum = 0;
            }
            cur_sum += x;
            max_sub = max_sub.max(cur_sum);
        }
        max_sub
    }
}

fn main() {
    let test_cases = vec![
        (vec![-2, 1, -3, 4, -1, 2, 1, -5, 4], 6),
        (vec![1], 1),
        (vec![5, 4, -1, 7, 8], 23),
        (vec![-1], -1),
        (vec![-5, -2, -8, -1], -1),
    ];

    for (nums, expected) in test_cases {
        assert_eq!(Solution::max_sub_array_brute_force(nums.clone()), expected);
        assert_eq!(Solution::max_sub_array_dp(nums.clone()), expected);
        assert_eq!(Solution::max_sub_array(nums), expected);
    }

    println!("All test cases passed for Maximum Subarray (Brute Force, DP, Kadane)!");
}
