// LeetCode Problem 46: Permutations
// Approaches:
//   1) Backtracking with Visited Array: -> O(n * n!) time | O(n) space
//   2) Backtracking with In-Place Swap: -> O(n * n!) time | O(n) recursion depth
// Link: https://leetcode.com/problems/permutations/
//
// Examples:
//   nums = [1,2,3] -> [[1,2,3],[1,3,2],[2,1,3],[2,3,1],[3,1,2],[3,2,1]]
//   nums = [0,1]   -> [[0,1],[1,0]]
//   nums = [1]     -> [[1]]

struct Solution;

impl Solution {
    /// 1. BACKTRACKING WITH VISITED ARRAY:
    /// Build each permutation element-by-element, tracking which indices are used.
    /// Time: O(n * n!) | Space: O(n)
    pub fn permute_visited(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let n = nums.len();
        let mut result = Vec::new();
        let mut current = Vec::with_capacity(n);
        let mut used = vec![false; n];

        fn backtrack(
            nums: &[i32],
            used: &mut Vec<bool>,
            current: &mut Vec<i32>,
            result: &mut Vec<Vec<i32>>,
        ) {
            if current.len() == nums.len() {
                result.push(current.clone());
                return;
            }
            for i in 0..nums.len() {
                if !used[i] {
                    used[i] = true;
                    current.push(nums[i]);
                    backtrack(nums, used, current, result);
                    current.pop();
                    used[i] = false;
                }
            }
        }

        backtrack(&nums, &mut used, &mut current, &mut result);
        result
    }

    /// 2. BACKTRACKING WITH IN-PLACE SWAP:
    /// Fix each element at position `start` by swapping, then recurse on remaining.
    /// Time: O(n * n!) | Space: O(n) recursion depth
    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result = Vec::new();
        let mut nums = nums;

        fn backtrack(start: usize, nums: &mut Vec<i32>, result: &mut Vec<Vec<i32>>) {
            if start == nums.len() {
                result.push(nums.clone());
                return;
            }
            for i in start..nums.len() {
                nums.swap(start, i);
                backtrack(start + 1, nums, result);
                nums.swap(start, i); // backtrack
            }
        }

        backtrack(0, &mut nums, &mut result);
        result
    }
}

fn main() {
    let test_cases = vec![
        (vec![1, 2, 3], 6),  // 3! = 6
        (vec![0, 1], 2),     // 2! = 2
        (vec![1], 1),        // 1! = 1
        (vec![1, 2, 3, 4], 24), // 4! = 24
    ];

    for (nums, expected_count) in test_cases {
        let mut r1 = Solution::permute_visited(nums.clone());
        let mut r2 = Solution::permute(nums);

        assert_eq!(r1.len(), expected_count);
        assert_eq!(r2.len(), expected_count);

        r1.sort_unstable();
        r2.sort_unstable();
        assert_eq!(r1, r2);
    }

    println!("All test cases passed for Permutations (Visited Array, In-Place Swap)!");
}
