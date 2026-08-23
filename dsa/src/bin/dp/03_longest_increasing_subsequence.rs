// LeetCode Problem 300: Longest Increasing Subsequence (LIS)
// Approaches:
//   1) Brute Force (Recursion): -> O(2^n) time | O(n) call stack
//   2) Better (Tabulation / Bottom-Up DP): -> O(n^2) time | O(n) space
//   3) Optimal (Binary Search / Patience Sorting): -> O(n log n) time | O(n) space
// Link: https://leetcode.com/problems/longest-increasing-subsequence/
//
// Examples:
//   nums = [10,9,2,5,3,7,101,18] -> 4 ([2,3,7,101] or [2,5,7,18])
//   nums = [0,1,0,3,2,3]         -> 4 ([0,1,2,3])
//   nums = [7,7,7,7,7,7,7]       -> 1

struct Solution;

impl Solution {
    /// 1. BRUTE FORCE (Recursion):
    /// At each index, either include current element (if > prev) or skip it.
    /// Time: O(2^n) | Space: O(n)
    pub fn length_of_lis_recursive(nums: Vec<i32>) -> i32 {
        fn solve(idx: usize, prev_idx: isize, nums: &[i32]) -> i32 {
            if idx == nums.len() {
                return 0;
            }
            // Option 1: skip nums[idx]
            let mut take = 0;
            let skip = solve(idx + 1, prev_idx, nums);

            // Option 2: take nums[idx] if valid
            if prev_idx == -1 || nums[idx] > nums[prev_idx as usize] {
                take = 1 + solve(idx + 1, idx as isize, nums);
            }
            take.max(skip)
        }
        solve(0, -1, &nums)
    }

    /// 2. TABULATION (Bottom-Up DP):
    /// dp[i] = length of LIS ending at index i.
    /// dp[i] = 1 + max(dp[j]) for all j < i where nums[j] < nums[i].
    /// Time: O(n^2) | Space: O(n)
    pub fn length_of_lis_dp(nums: Vec<i32>) -> i32 {
        if nums.is_empty() {
            return 0;
        }
        let n = nums.len();
        let mut dp = vec![1; n];
        let mut max_lis = 1;

        for i in 1..n {
            for j in 0..i {
                if nums[j] < nums[i] {
                    dp[i] = dp[i].max(dp[j] + 1);
                }
            }
            max_lis = max_lis.max(dp[i]);
        }
        max_lis
    }

    /// 3. OPTIMAL (Patience Sorting with Binary Search):
    /// Maintain an array `tails` where tails[i] stores the smallest tail of all increasing subsequences of length i + 1.
    /// For each num, binary search for the first element >= num.
    /// Time: O(n log n) | Space: O(n)
    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        if nums.is_empty() {
            return 0;
        }
        let mut tails = Vec::new();

        for num in nums {
            match tails.binary_search(&num) {
                Ok(_) => {
                    // Element already exists, nothing changes the minimal tail
                }
                Err(idx) => {
                    if idx == tails.len() {
                        tails.push(num);
                    } else {
                        tails[idx] = num;
                    }
                }
            }
        }
        tails.len() as i32
    }
}

fn main() {
    let test_cases = vec![
        (vec![10, 9, 2, 5, 3, 7, 101, 18], 4),
        (vec![0, 1, 0, 3, 2, 3], 4),
        (vec![7, 7, 7, 7, 7, 7, 7], 1),
        (vec![4, 10, 4, 3, 8, 9], 3),
        (vec![1], 1),
        (vec![], 0),
    ];

    for (nums, expected) in test_cases {
        assert_eq!(
            Solution::length_of_lis_recursive(nums.clone()),
            expected
        );
        assert_eq!(
            Solution::length_of_lis_dp(nums.clone()),
            expected
        );
        assert_eq!(
            Solution::length_of_lis(nums),
            expected
        );
    }

    println!("All test cases passed for Longest Increasing Subsequence (Recursion, O(n^2) DP, O(n log n) Patience Sort)!");
}
