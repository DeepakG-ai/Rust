// LeetCode Problem 15: 3Sum
// Approaches:
//   1) Brute Force (Three Loops + HashSet): -> O(n^3) time | O(k) space
//   2) Better (Two Loops + HashSet Lookup): -> O(n^2) time | O(n) space
//   3) Optimal (Sort + Two Pointers): In-place duplicate skipping -> O(n^2) time | O(1) extra space
// Link: https://leetcode.com/problems/3sum/
//
// Examples:
//   [-1,0,1,2,-1,-4] -> [[-1,-1,2],[-1,0,1]]
//   [0,0,0]          -> [[0,0,0]]

use std::collections::HashSet;

struct Solution;

impl Solution {
    /// 1. BRUTE FORCE: 3 nested loops + HashSet to eliminate duplicate triplets.
    /// Time: O(n^3) | Space: O(k) for unique triplets
    pub fn three_sum_brute_force(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let n = nums.len();
        let mut set = HashSet::new();

        for i in 0..n {
            for j in (i + 1)..n {
                for k in (j + 1)..n {
                    if nums[i] + nums[j] + nums[k] == 0 {
                        let mut triplet = vec![nums[i], nums[j], nums[k]];
                        triplet.sort_unstable();
                        set.insert(triplet);
                    }
                }
            }
        }
        let mut res: Vec<Vec<i32>> = set.into_iter().collect();
        res.sort_unstable();
        res
    }

    /// 2. BETTER (Hash Set Lookup):
    /// Fix index i, and for j > i, check if -(nums[i] + nums[j]) was seen in the current inner pass.
    /// Time: O(n^2) | Space: O(n)
    pub fn three_sum_better_hashset(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let n = nums.len();
        let mut unique_triplets = HashSet::new();

        for i in 0..n {
            let mut seen = HashSet::new();
            for j in (i + 1)..n {
                let complement = -(nums[i] + nums[j]);
                if seen.contains(&complement) {
                    let mut triplet = vec![nums[i], nums[j], complement];
                    triplet.sort_unstable();
                    unique_triplets.insert(triplet);
                }
                seen.insert(nums[j]);
            }
        }
        let mut res: Vec<Vec<i32>> = unique_triplets.into_iter().collect();
        res.sort_unstable();
        res
    }

    /// 3. OPTIMAL (Sort + Two Pointers):
    /// Sort array. Fix index i; use two pointers (left = i+1, right = n-1) to find sum == -nums[i].
    /// Skip duplicate elements at i, left, and right to avoid duplicate triplets without HashSet overhead.
    /// Time: O(n^2) | Space: O(1) extra (ignoring output)
    pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        nums.sort_unstable();
        let n = nums.len();
        let mut res: Vec<Vec<i32>> = Vec::new();

        for i in 0..n {
            if i > 0 && nums[i] == nums[i - 1] {
                continue; // skip duplicate anchor values
            }
            if nums[i] > 0 {
                break; // smallest number > 0 -> sum cannot be 0
            }

            let mut left = i + 1;
            let mut right = n.saturating_sub(1);

            while left < right {
                let total = nums[i] + nums[left] + nums[right];

                if total < 0 {
                    left += 1;
                } else if total > 0 {
                    right -= 1;
                } else {
                    res.push(vec![nums[i], nums[left], nums[right]]);
                    left += 1;
                    right = right.saturating_sub(1);

                    // Skip duplicates on both pointers
                    while left < right && nums[left] == nums[left - 1] {
                        left += 1;
                    }
                    while left < right && nums[right] == nums[right + 1] {
                        right = right.saturating_sub(1);
                    }
                }
            }
        }
        res
    }
}

fn main() {
    let test_cases = vec![
        (
            vec![-1, 0, 1, 2, -1, -4],
            vec![vec![-1, -1, 2], vec![-1, 0, 1]],
        ),
        (vec![0, 1, 1], Vec::<Vec<i32>>::new()),
        (vec![0, 0, 0], vec![vec![0, 0, 0]]),
        (vec![-2, 0, 1, 1, 2], vec![vec![-2, 0, 2], vec![-2, 1, 1]]),
        (vec![], Vec::<Vec<i32>>::new()),
    ];

    for (nums, expected) in test_cases {
        let mut r1 = Solution::three_sum_brute_force(nums.clone());
        let mut r2 = Solution::three_sum_better_hashset(nums.clone());
        let mut r3 = Solution::three_sum(nums);

        r1.sort_unstable();
        r2.sort_unstable();
        r3.sort_unstable();

        assert_eq!(r1, expected);
        assert_eq!(r2, expected);
        assert_eq!(r3, expected);
    }

    println!("All test cases passed for 3Sum (Brute Force, HashSet, Sort + Two Pointers)!");
}
