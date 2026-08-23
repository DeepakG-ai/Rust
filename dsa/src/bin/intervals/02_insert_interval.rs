// LeetCode Problem 57: Insert Interval
// Approaches:
//   1) Better (Append & Merge Intervals): -> O(n log n) time | O(n) space
//   2) Optimal (Single Pass Linear Insertion): -> O(n) time | O(n) space
// Link: https://leetcode.com/problems/insert-interval/
//
// Examples:
//   intervals = [[1,3],[6,9]], newInterval = [2,5] -> [[1,5],[6,9]]
//   intervals = [[1,2],[3,5],[6,7],[8,10],[12,16]], newInterval = [4,8] -> [[1,2],[3,10],[12,16]]

struct Solution;

impl Solution {
    /// 1. BETTER (Append & Merge):
    /// Append new_interval to intervals, sort, and merge overlapping intervals.
    /// Time: O(n log n) | Space: O(n)
    pub fn insert_append_merge(
        mut intervals: Vec<Vec<i32>>,
        new_interval: Vec<i32>,
    ) -> Vec<Vec<i32>> {
        intervals.push(new_interval);
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

    /// 2. OPTIMAL (Single Pass Linear Scan):
    /// 3 phases:
    /// 1) Add all intervals that end before new_interval starts.
    /// 2) Merge all intervals that overlap with new_interval.
    /// 3) Add new_interval, then add all remaining intervals.
    /// Time: O(n) | Space: O(n)
    pub fn insert(intervals: Vec<Vec<i32>>, mut new_interval: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result = Vec::new();
        let mut i = 0;
        let n = intervals.len();

        // 1. Add intervals ending before new_interval
        while i < n && intervals[i][1] < new_interval[0] {
            result.push(intervals[i].clone());
            i += 1;
        }

        // 2. Merge overlapping intervals
        while i < n && intervals[i][0] <= new_interval[1] {
            new_interval[0] = new_interval[0].min(intervals[i][0]);
            new_interval[1] = new_interval[1].max(intervals[i][1]);
            i += 1;
        }
        result.push(new_interval);

        // 3. Add remaining intervals
        while i < n {
            result.push(intervals[i].clone());
            i += 1;
        }

        result
    }
}

fn main() {
    let test_cases = vec![
        (
            vec![vec![1, 3], vec![6, 9]],
            vec![2, 5],
            vec![vec![1, 5], vec![6, 9]],
        ),
        (
            vec![
                vec![1, 2],
                vec![3, 5],
                vec![6, 7],
                vec![8, 10],
                vec![12, 16],
            ],
            vec![4, 8],
            vec![vec![1, 2], vec![3, 10], vec![12, 16]],
        ),
        (vec![], vec![5, 7], vec![vec![5, 7]]),
        (vec![vec![1, 5]], vec![2, 3], vec![vec![1, 5]]),
        (vec![vec![1, 5]], vec![2, 7], vec![vec![1, 7]]),
    ];

    for (intervals, new_interval, expected) in test_cases {
        assert_eq!(
            Solution::insert_append_merge(intervals.clone(), new_interval.clone()),
            expected
        );
        assert_eq!(
            Solution::insert(intervals, new_interval),
            expected
        );
    }

    println!("All test cases passed for Insert Interval (Append & Merge, Single-Pass O(n))!");
}
