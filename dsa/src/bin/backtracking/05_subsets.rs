// LeetCode Problem 78: Subsets
// Approaches:
//   1) Iterative (Cascading / Power Set Build): -> O(n * 2^n) time | O(n * 2^n) space
//   2) Backtracking (Pick or Skip): -> O(n * 2^n) time | O(n) recursion depth
//   3) Bitmask Enumeration: -> O(n * 2^n) time | O(1) extra space
// Link: https://leetcode.com/problems/subsets/
//
// Examples:
//   nums = [1,2,3] -> [[],[1],[2],[1,2],[3],[1,3],[2,3],[1,2,3]]
//   nums = [0]     -> [[],[0]]

struct Solution;

impl Solution {
    /// 1. ITERATIVE (Cascading):
    /// Start with [[]]. For each num, copy all existing subsets and append num.
    /// Time: O(n * 2^n) | Space: O(n * 2^n)
    pub fn subsets_iterative(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result: Vec<Vec<i32>> = vec![vec![]];

        for num in nums {
            let mut new_subsets = Vec::new();
            for existing in &result {
                let mut clone = existing.clone();
                clone.push(num);
                new_subsets.push(clone);
            }
            result.extend(new_subsets);
        }
        result
    }

    /// 2. BACKTRACKING (Pick or Skip):
    /// Time: O(n * 2^n) | Space: O(n)
    pub fn subsets_backtrack(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result = Vec::new();
        let mut current = Vec::new();

        fn backtrack(
            start: usize,
            nums: &[i32],
            current: &mut Vec<i32>,
            result: &mut Vec<Vec<i32>>,
        ) {
            result.push(current.clone());
            for i in start..nums.len() {
                current.push(nums[i]);
                backtrack(i + 1, nums, current, result);
                current.pop();
            }
        }

        backtrack(0, &nums, &mut current, &mut result);
        result
    }

    /// 3. BITMASK ENUMERATION:
    /// For n elements, enumerate all 2^n bitmasks 0..2^n. Bit i set means include nums[i].
    /// Time: O(n * 2^n) | Space: O(1) extra
    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let n = nums.len();
        let total = 1 << n;
        let mut result = Vec::with_capacity(total);

        for mask in 0..total {
            let mut subset = Vec::new();
            for i in 0..n {
                if mask & (1 << i) != 0 {
                    subset.push(nums[i]);
                }
            }
            result.push(subset);
        }
        result
    }
}

fn main() {
    let test_cases = vec![
        (vec![1, 2, 3], 8),  // 2^3 = 8 subsets
        (vec![0], 2),         // 2^1 = 2 subsets
        (vec![1, 2], 4),      // 2^2 = 4 subsets
    ];

    for (nums, expected_count) in test_cases {
        let mut r1 = Solution::subsets_iterative(nums.clone());
        let mut r2 = Solution::subsets_backtrack(nums.clone());
        let mut r3 = Solution::subsets(nums);

        assert_eq!(r1.len(), expected_count);
        assert_eq!(r2.len(), expected_count);
        assert_eq!(r3.len(), expected_count);

        // Sort for deterministic comparison
        r1.iter_mut().for_each(|s| s.sort_unstable());
        r1.sort_unstable();
        r2.iter_mut().for_each(|s| s.sort_unstable());
        r2.sort_unstable();
        r3.iter_mut().for_each(|s| s.sort_unstable());
        r3.sort_unstable();

        assert_eq!(r1, r2);
        assert_eq!(r2, r3);
    }

    println!("All test cases passed for Subsets (Cascading, Backtracking, Bitmask Enumeration)!");
}
