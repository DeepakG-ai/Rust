// LeetCode Problem 39: Combination Sum
// Approach: Backtracking (Pick or Not-Pick with sorting optimization)
// Time: O(2^target) | Space: O(target) recursion stack
// Link: https://leetcode.com/problems/combination-sum/
//
// Examples:
//   candidates = [2,3,6,7], target = 7 -> [[2,2,3],[7]]
//   candidates = [2,3,5], target = 8   -> [[2,2,2,2],[2,3,3],[3,5]]
//   candidates = [2], target = 1       -> []

struct Solution;

impl Solution {
    /// Backtracking with early branch pruning (via sorted candidates).
    pub fn combination_sum(mut candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        candidates.sort_unstable();
        let mut result = Vec::new();
        let mut current = Vec::new();

        fn backtrack(
            start: usize,
            remain: i32,
            candidates: &[i32],
            current: &mut Vec<i32>,
            result: &mut Vec<Vec<i32>>,
        ) {
            if remain == 0 {
                result.push(current.clone());
                return;
            }
            for i in start..candidates.len() {
                if candidates[i] > remain {
                    break; // Prune branch since array is sorted
                }
                current.push(candidates[i]);
                // We pass `i` (not `i + 1`) because we can reuse the same element unbounded times
                backtrack(i, remain - candidates[i], candidates, current, result);
                current.pop(); // backtrack
            }
        }

        backtrack(0, target, &candidates, &mut current, &mut result);
        result
    }
}

fn main() {
    let test_cases = vec![
        (vec![2, 3, 6, 7], 7, vec![vec![2, 2, 3], vec![7]]),
        (
            vec![2, 3, 5],
            8,
            vec![vec![2, 2, 2, 2], vec![2, 3, 3], vec![3, 5]],
        ),
        (vec![2], 1, vec![]),
        (vec![1], 3, vec![vec![1, 1, 1]]),
    ];

    for (candidates, target, expected) in test_cases {
        let mut ans = Solution::combination_sum(candidates, target);
        ans.sort_unstable();
        let mut exp = expected;
        exp.sort_unstable();
        assert_eq!(ans, exp);
    }

    println!("All test cases passed for Combination Sum (Backtracking with Pruning)!");
}
