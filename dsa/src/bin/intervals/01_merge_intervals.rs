// LeetCode Problem 56: Merge Intervals
// Approaches:
//   1) Brute Force (Graph Connected Components): -> O(n^2) time | O(n^2) space
//   2) Optimal (Sort by Start Time & Single-Pass Merge): -> O(n log n) time | O(n) space
// Link: https://leetcode.com/problems/merge-intervals/
//
// Examples:
//   intervals = [[1,3],[2,6],[8,10],[15,18]] -> [[1,6],[8,10],[15,18]]
//   intervals = [[1,4],[4,5]]                 -> [[1,5]]

struct Solution;

impl Solution {
    /// 1. BRUTE FORCE:
    /// Check all pairs, repeatedly merge overlapping pairs until no further merges can occur.
    /// Time: O(n^2) | Space: O(n)
    pub fn merge_brute(mut intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        if intervals.is_empty() {
            return vec![];
        }

        let mut changed = true;
        while changed {
            changed = false;
            let mut merged: Vec<Vec<i32>> = Vec::new();
            let mut visited = vec![false; intervals.len()];

            for i in 0..intervals.len() {
                if visited[i] {
                    continue;
                }
                let mut cur = intervals[i].clone();
                for j in (i + 1)..intervals.len() {
                    if !visited[j] {
                        // Check overlap
                        if cur[0].max(intervals[j][0]) <= cur[1].min(intervals[j][1]) {
                            cur[0] = cur[0].min(intervals[j][0]);
                            cur[1] = cur[1].max(intervals[j][1]);
                            visited[j] = true;
                            changed = true;
                        }
                    }
                }
                visited[i] = true;
                merged.push(cur);
            }
            intervals = merged;
        }
        intervals.sort_unstable_by_key(|iv| iv[0]);
        intervals
    }

    /// 2. OPTIMAL (Sort & Single Pass):
    /// Sort intervals by start time. Iterate through: if interval starts <= last end, merge. Else push.
    /// Time: O(n log n) | Space: O(n)
    pub fn merge(mut intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        if intervals.is_empty() {
            return vec![];
        }

        intervals.sort_unstable_by_key(|iv| iv[0]);
        let mut merged: Vec<Vec<i32>> = Vec::new();

        for iv in intervals {
            match merged.last_mut() {
                Some(last) if iv[0] <= last[1] => {
                    last[1] = last[1].max(iv[1]);
                }
                _ => merged.push(iv),
            }
        }
        merged
    }
}

fn main() {
    let test_cases = vec![
        (
            vec![vec![1, 3], vec![2, 6], vec![8, 10], vec![15, 18]],
            vec![vec![1, 6], vec![8, 10], vec![15, 18]],
        ),
        (vec![vec![1, 4], vec![4, 5]], vec![vec![1, 5]]),
        (vec![vec![1, 4], vec![2, 3]], vec![vec![1, 4]]),
        (vec![vec![1, 4], vec![0, 4]], vec![vec![0, 4]]),
        (vec![vec![1, 4], vec![0, 0]], vec![vec![0, 0], vec![1, 4]]),
        (vec![], vec![]),
    ];

    for (intervals, expected) in test_cases {
        assert_eq!(Solution::merge_brute(intervals.clone()), expected);
        assert_eq!(Solution::merge(intervals), expected);
    }

    println!("All test cases passed for Merge Intervals (Brute Force, Sort & Single-Pass O(n log n))!");
}
