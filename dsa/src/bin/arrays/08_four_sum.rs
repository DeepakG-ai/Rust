// LeetCode Problem 18: 4Sum
// Approaches:
//   1) Brute Force: 4 nested loops with HashSet -> O(n^4) time | O(k) space
//   2) Better (3 Loops + HashSet): -> O(n^3) time | O(n) space
//   3) Optimal (Sort + Two Pointers): In-place duplicate skipping with i64 promotion -> O(n^3) time | O(1) extra space
// Link: https://leetcode.com/problems/4sum/
//
// Examples:
//   [1,0,-1,0,-2,2], target=0 -> [[-2,-1,1,2],[-2,0,0,2],[-1,0,0,1]]
//   [2,2,2,2,2],     target=8 -> [[2,2,2,2]]

use std::collections::HashSet;

struct Solution;

impl Solution {
    /// 1. BRUTE FORCE: 4 nested loops with HashSet for uniqueness.
    /// Time: O(n^4) | Space: O(k)
    pub fn four_sum_brute_force(nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let n = nums.len();
        let mut set = HashSet::new();
        let target64 = target as i64;

        for i in 0..n {
            for j in (i + 1)..n {
                for k in (j + 1)..n {
                    for l in (k + 1)..n {
                        let sum = nums[i] as i64 + nums[j] as i64 + nums[k] as i64 + nums[l] as i64;
                        if sum == target64 {
                            let mut quad = vec![nums[i], nums[j], nums[k], nums[l]];
                            quad.sort_unstable();
                            set.insert(quad);
                        }
                    }
                }
            }
        }
        let mut res: Vec<Vec<i32>> = set.into_iter().collect();
        res.sort_unstable();
        res
    }

    /// 2. BETTER (3 Loops + HashSet):
    /// Fix i and j, use HashSet for remaining pairs (k, l).
    /// Time: O(n^3) | Space: O(n)
    pub fn four_sum_better_hashset(nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let n = nums.len();
        let mut set = HashSet::new();
        let target64 = target as i64;

        for i in 0..n {
            for j in (i + 1)..n {
                let mut seen = HashSet::new();
                for k in (j + 1)..n {
                    let needed = target64 - (nums[i] as i64 + nums[j] as i64 + nums[k] as i64);
                    if needed >= i32::MIN as i64 && needed <= i32::MAX as i64 && seen.contains(&(needed as i32)) {
                        let mut quad = vec![nums[i], nums[j], nums[k], needed as i32];
                        quad.sort_unstable();
                        set.insert(quad);
                    }
                    seen.insert(nums[k]);
                }
            }
        }
        let mut res: Vec<Vec<i32>> = set.into_iter().collect();
        res.sort_unstable();
        res
    }

    /// 3. OPTIMAL (Sort + Two Pointers with Duplicate Skipping):
    /// Sort array. Fix indices i and j, then run two pointers on remaining range [k, l].
    /// Promote to i64 to prevent integer overflow on extreme values.
    /// Time: O(n^3) | Space: O(1) extra
    pub fn four_sum(mut nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        nums.sort_unstable();
        let n = nums.len();
        let mut result: Vec<Vec<i32>> = Vec::new();
        let target64 = target as i64;

        for i in 0..n {
            if i > 0 && nums[i] == nums[i - 1] {
                continue; // skip duplicate i
            }

            for j in (i + 1)..n {
                if j > i + 1 && nums[j] == nums[j - 1] {
                    continue; // skip duplicate j
                }

                let mut k = j + 1;
                let mut l = n.saturating_sub(1);

                while k < l {
                    let total = nums[i] as i64 + nums[j] as i64 + nums[k] as i64 + nums[l] as i64;

                    if total == target64 {
                        result.push(vec![nums[i], nums[j], nums[k], nums[l]]);
                        k += 1;
                        l = l.saturating_sub(1);

                        while k < l && nums[k] == nums[k - 1] {
                            k += 1;
                        }
                        while k < l && nums[l] == nums[l + 1] {
                            l = l.saturating_sub(1);
                        }
                    } else if total < target64 {
                        k += 1;
                    } else {
                        l = l.saturating_sub(1);
                    }
                }
            }
        }
        result
    }
}

fn main() {
    let test_cases = vec![
        (
            vec![1, 0, -1, 0, -2, 2],
            0,
            vec![vec![-2, -1, 1, 2], vec![-2, 0, 0, 2], vec![-1, 0, 0, 1]],
        ),
        (vec![2, 2, 2, 2, 2], 8, vec![vec![2, 2, 2, 2]]),
        (vec![1, 2, 3, 4], 10, vec![vec![1, 2, 3, 4]]),
        (vec![1000000000, 1000000000, 1000000000, 1000000000], -294967296, Vec::<Vec<i32>>::new()),
    ];

    for (nums, target, expected) in test_cases {
        let mut r1 = Solution::four_sum_brute_force(nums.clone(), target);
        let mut r2 = Solution::four_sum_better_hashset(nums.clone(), target);
        let mut r3 = Solution::four_sum(nums, target);

        r1.sort_unstable();
        r2.sort_unstable();
        r3.sort_unstable();

        assert_eq!(r1, expected);
        assert_eq!(r2, expected);
        assert_eq!(r3, expected);
    }

    println!("All test cases passed for 4Sum (Brute Force, HashSet, Sort + Two Pointers)!");
}
