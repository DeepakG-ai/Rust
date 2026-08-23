// LeetCode Problem 55: Jump Game
// Approaches:
//   1) Brute Force (Recursion): -> O(2^n) time | O(n) call stack
//   2) Better (Bottom-Up Tabulation DP): -> O(n^2) time | O(n) space
//   3) Optimal (Greedy Maximum Reachable Index): -> O(n) time | O(1) space
// Link: https://leetcode.com/problems/jump-game/
//
// Examples:
//   nums = [2,3,1,1,4] -> true
//   nums = [3,2,1,0,4] -> false

struct Solution;

impl Solution {
    /// 1. BRUTE FORCE (Recursion):
    /// From index pos, try all jumps 1..=nums[pos].
    /// Time: O(2^n) | Space: O(n)
    pub fn can_jump_recursive(nums: Vec<i32>) -> bool {
        fn solve(pos: usize, nums: &[i32]) -> bool {
            if pos >= nums.len() - 1 {
                return true;
            }
            let max_jump = nums[pos] as usize;
            for jump in (1..=max_jump).rev() {
                if solve(pos + jump, nums) {
                    return true;
                }
            }
            false
        }
        solve(0, &nums)
    }

    /// 2. TABULATION (Bottom-Up DP):
    /// dp[i] = can reach the last index from index i.
    /// Work backwards from n - 2 to 0.
    /// Time: O(n^2) | Space: O(n)
    pub fn can_jump_dp(nums: Vec<i32>) -> bool {
        let n = nums.len();
        if n <= 1 {
            return true;
        }

        let mut dp = vec![false; n];
        dp[n - 1] = true;

        for i in (0..n - 1).rev() {
            let furthest = (i + nums[i] as usize).min(n - 1);
            for j in (i + 1)..=furthest {
                if dp[j] {
                    dp[i] = true;
                    break;
                }
            }
        }
        dp[0]
    }

    /// 3. OPTIMAL (Greedy Max Reachable Index):
    /// Maintain max_reachable index. If at any point current index > max_reachable, return false.
    /// Time: O(n) | Space: O(1)
    pub fn can_jump(nums: Vec<i32>) -> bool {
        let mut max_reachable = 0usize;

        for (i, &num) in nums.iter().enumerate() {
            if i > max_reachable {
                return false;
            }
            max_reachable = max_reachable.max(i + num as usize);
            if max_reachable >= nums.len() - 1 {
                return true;
            }
        }
        true
    }
}

fn main() {
    let test_cases = vec![
        (vec![2, 3, 1, 1, 4], true),
        (vec![3, 2, 1, 0, 4], false),
        (vec![0], true),
        (vec![2, 0, 0], true),
        (vec![1, 0, 1, 0], false),
        (vec![2, 5, 0, 0], true),
    ];

    for (nums, expected) in test_cases {
        assert_eq!(
            Solution::can_jump_recursive(nums.clone()),
            expected
        );
        assert_eq!(
            Solution::can_jump_dp(nums.clone()),
            expected
        );
        assert_eq!(
            Solution::can_jump(nums),
            expected
        );
    }

    println!("All test cases passed for Jump Game (Recursion, O(n^2) DP, Greedy O(n))!");
}
