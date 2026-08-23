// LeetCode Problem 152: Maximum Product Subarray
// Approaches:
//   1) Brute Force: Check all subarray products -> O(n^2) time | O(1) space
//   2) Better (Prefix & Suffix Sweep): Max of left-to-right and right-to-left prefix products -> O(n) time | O(1) space
//   3) Optimal (Min/Max Tracking DP): Swap min and max on negative multipliers -> O(n) time | O(1) space
// Link: https://leetcode.com/problems/maximum-product-subarray/
//
// Examples:
//   [2,3,-2,4] -> 6
//   [-2,0,-1]  -> 0

struct Solution;

impl Solution {
    /// 1. BRUTE FORCE: For every start index i, compute cumulative products ending at j >= i.
    /// Time: O(n^2) | Space: O(1)
    pub fn max_product_brute_force(nums: Vec<i32>) -> i32 {
        if nums.is_empty() {
            return 0;
        }
        let n = nums.len();
        let mut max_prod = nums[0];

        for i in 0..n {
            let mut prod = 1;
            for j in i..n {
                prod *= nums[j];
                max_prod = max_prod.max(prod);
            }
        }
        max_prod
    }

    /// 2. BETTER (Prefix & Suffix Sweep):
    /// An optimal subarray either starts from the left or right, or is bounded by zeros.
    /// Sweeping forwards and backwards while resetting at 0 captures the maximum product.
    /// Time: O(n) | Space: O(1)
    pub fn max_product_prefix_suffix(nums: Vec<i32>) -> i32 {
        if nums.is_empty() {
            return 0;
        }
        let n = nums.len();
        let mut max_prod = nums[0];
        let mut prefix = 1;
        let mut suffix = 1;

        for i in 0..n {
            prefix = if prefix == 0 { nums[i] } else { prefix * nums[i] };
            suffix = if suffix == 0 { nums[n - 1 - i] } else { suffix * nums[n - 1 - i] };
            max_prod = max_prod.max(prefix.max(suffix));
        }
        max_prod
    }

    /// 3. OPTIMAL (DP with Min & Max Tracking):
    /// Maintain current max and current min ending at current element.
    /// A negative number inverts min <-> max.
    /// Time: O(n) | Space: O(1)
    pub fn max_product(nums: Vec<i32>) -> i32 {
        if nums.is_empty() {
            return 0;
        }

        let mut max_prod = nums[0];
        let mut cur_max = nums[0];
        let mut cur_min = nums[0];

        for &n in &nums[1..] {
            if n < 0 {
                std::mem::swap(&mut cur_max, &mut cur_min);
            }
            cur_max = (cur_max * n).max(n);
            cur_min = (cur_min * n).min(n);

            max_prod = max_prod.max(cur_max);
        }
        max_prod
    }
}

fn main() {
    let test_cases = vec![
        (vec![2, 3, -2, 4], 6),
        (vec![-2, 0, -1], 0),
        (vec![-2, -3, -4], 12),
        (vec![-2, 3, -4, 0, 5, -2], 24),
        (vec![5], 5),
        (vec![-4, -3], 12),
        (vec![1, -2, 3, -4, 5, -6], 360),
        (vec![0, 2], 2),
        (vec![-2], -2),
    ];

    for (nums, expected) in test_cases {
        assert_eq!(Solution::max_product_brute_force(nums.clone()), expected);
        assert_eq!(Solution::max_product_prefix_suffix(nums.clone()), expected);
        assert_eq!(Solution::max_product(nums), expected);
    }

    println!("All test cases passed for Maximum Product Subarray (Brute Force, Prefix/Suffix, Min/Max DP)!");
}
