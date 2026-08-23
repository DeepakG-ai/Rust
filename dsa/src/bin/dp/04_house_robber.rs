// LeetCode Problem 198: House Robber
// Approaches:
//   1) Brute Force (Recursion): -> O(2^n) time | O(n) call stack
//   2) Better (Memoization / Top-Down): -> O(n) time | O(n) space
//   3) Better (Tabulation / Bottom-Up): -> O(n) time | O(n) space
//   4) Optimal (Space-Optimized DP): -> O(n) time | O(1) space
// Link: https://leetcode.com/problems/house-robber/
//
// Examples:
//   nums = [1,2,3,1] -> 4 (rob house 1 and 3: 1 + 3 = 4)
//   nums = [2,7,9,3,1] -> 12 (rob house 1, 3, 5: 2 + 9 + 1 = 12)

struct Solution;

impl Solution {
    /// 1. BRUTE FORCE (Recursion):
    /// rob(i) = max(nums[i] + rob(i-2), rob(i-1))
    /// Time: O(2^n) | Space: O(n)
    pub fn rob_recursive(nums: Vec<i32>) -> i32 {
        fn solve(i: usize, nums: &[i32]) -> i32 {
            if i >= nums.len() {
                return 0;
            }
            let rob_curr = nums[i] + solve(i + 2, nums);
            let skip_curr = solve(i + 1, nums);
            rob_curr.max(skip_curr)
        }
        solve(0, &nums)
    }

    /// 2. MEMOIZATION (Top-Down):
    /// Time: O(n) | Space: O(n)
    pub fn rob_memo(nums: Vec<i32>) -> i32 {
        fn solve(i: usize, nums: &[i32], memo: &mut Vec<i32>) -> i32 {
            if i >= nums.len() {
                return 0;
            }
            if memo[i] != -1 {
                return memo[i];
            }
            let rob_curr = nums[i] + solve(i + 2, nums, memo);
            let skip_curr = solve(i + 1, nums, memo);
            memo[i] = rob_curr.max(skip_curr);
            memo[i]
        }

        let n = nums.len();
        let mut memo = vec![-1; n];
        solve(0, &nums, &mut memo)
    }

    /// 3. TABULATION (Bottom-Up):
    /// dp[i] = max money robbed from first i houses.
    /// dp[i] = max(dp[i-1], dp[i-2] + nums[i])
    /// Time: O(n) | Space: O(n)
    pub fn rob_tabulation(nums: Vec<i32>) -> i32 {
        if nums.is_empty() {
            return 0;
        }
        if nums.len() == 1 {
            return nums[0];
        }
        let n = nums.len();
        let mut dp = vec![0; n];
        dp[0] = nums[0];
        dp[1] = nums[0].max(nums[1]);

        for i in 2..n {
            dp[i] = dp[i - 1].max(dp[i - 2] + nums[i]);
        }
        dp[n - 1]
    }

    /// 4. OPTIMAL (Space-Optimized DP):
    /// Track only prev1 and prev2 values.
    /// Time: O(n) | Space: O(1)
    pub fn rob(nums: Vec<i32>) -> i32 {
        let (mut prev2, mut prev1) = (0, 0);

        for num in nums {
            let cur = prev1.max(prev2 + num);
            prev2 = prev1;
            prev1 = cur;
        }
        prev1
    }
}

fn main() {
    let test_cases = vec![
        (vec![1, 2, 3, 1], 4),
        (vec![2, 7, 9, 3, 1], 12),
        (vec![2, 1, 1, 2], 4),
        (vec![5], 5),
        (vec![], 0),
    ];

    for (nums, expected) in test_cases {
        assert_eq!(Solution::rob_recursive(nums.clone()), expected);
        assert_eq!(Solution::rob_memo(nums.clone()), expected);
        assert_eq!(Solution::rob_tabulation(nums.clone()), expected);
        assert_eq!(Solution::rob(nums), expected);
    }

    println!("All test cases passed for House Robber (Recursion, Memoization, Tabulation, Space O(1))!");
}
