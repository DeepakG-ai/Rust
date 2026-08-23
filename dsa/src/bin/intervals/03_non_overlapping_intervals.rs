// LeetCode Problem 435: Non-overlapping Intervals
// Approaches:
//   1) Better (DP LIS-Style): Find maximum non-overlapping set -> O(n^2) time | O(n) space
//   2) Optimal (Greedy Interval Scheduling by End Time): Keep interval that finishes earliest -> O(n log n) time | O(1) space
// Link: https://leetcode.com/problems/non-overlapping-intervals/
//
// Examples:
//   intervals = [[1,2],[2,3],[3,4],[1,3]] -> 1 (remove [1,3])
//   intervals = [[1,2],[1,2],[1,2]]       -> 2
//   intervals = [[1,2],[2,3]]             -> 0

struct Solution;

impl Solution {
    /// 1. BETTER (DP LIS-Style):
    /// Sort intervals by start time. dp[i] = max non-overlapping intervals ending at i.
    /// Returns total length - max_non_overlapping.
    /// Time: O(n^2) | Space: O(n)
    pub fn erase_overlap_intervals_dp(mut intervals: Vec<Vec<i32>>) -> i32 {
        if intervals.is_empty() {
            return 0;
        }
        intervals.sort_unstable_by_key(|iv| iv[0]);
        let n = intervals.len();
        let mut dp = vec![1; n];
        let mut max_non_overlap = 1;

        for i in 1..n {
            for j in 0..i {
                if intervals[j][1] <= intervals[i][0] {
                    dp[i] = dp[i].max(dp[j] + 1);
                }
            }
            max_non_overlap = max_non_overlap.max(dp[i]);
        }
        (n - max_non_overlap) as i32
    }

    /// 2. OPTIMAL (Greedy Interval Scheduling):
    /// Sort by end time. Always pick the interval that finishes earliest to leave max room for subsequent intervals.
    /// Time: O(n log n) | Space: O(1)
    pub fn erase_overlap_intervals(mut intervals: Vec<Vec<i32>>) -> i32 {
        if intervals.is_empty() {
            return 0;
        }

        intervals.sort_unstable_by_key(|iv| iv[1]);
        let mut removed = 0;
        let mut prev_end = intervals[0][1];

        for iv in intervals.iter().skip(1) {
            if iv[0] < prev_end {
                // Overlaps with previous retained interval, must remove this one
                removed += 1;
            } else {
                // Non-overlapping, retain it and update finish time
                prev_end = iv[1];
            }
        }
        removed
    }
}

fn main() {
    let test_cases = vec![
        (vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![1, 3]], 1),
        (vec![vec![1, 2], vec![1, 2], vec![1, 2]], 2),
        (vec![vec![1, 2], vec![2, 3]], 0),
        (vec![vec![1, 100], vec![11, 22], vec![1, 11], vec![2, 12]], 2),
        (vec![], 0),
        (vec![vec![1, 2]], 0),
    ];

    for (intervals, expected) in test_cases {
        assert_eq!(
            Solution::erase_overlap_intervals_dp(intervals.clone()),
            expected
        );
        assert_eq!(
            Solution::erase_overlap_intervals(intervals),
            expected
        );
    }

    println!("All test cases passed for Non-overlapping Intervals (DP O(n^2), Greedy O(n log n))!");
}
