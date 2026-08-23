// LeetCode Problem 253: Meeting Rooms II
// Approaches:
//   1) Better (Min-Heap of Active End Times): -> O(n log n) time | O(n) space
//   2) Optimal (Chronological Sweep-Line / Two Pointers): -> O(n log n) time | O(n) space
// Link: https://leetcode.com/problems/meeting-rooms-ii/
//
// Examples:
//   intervals = [[0,30],[5,10],[15,20]] -> 2
//   intervals = [[7,10],[2,4]]           -> 1

use std::cmp::Reverse;
use std::collections::BinaryHeap;

struct Solution;

impl Solution {
    /// 1. MIN-HEAP OF END TIMES:
    /// Sort intervals by start time. Store end times of active meetings in a Min-Heap.
    /// If next meeting starts >= earliest ending meeting, reuse that room (pop heap).
    /// Always push current meeting's end time. Heap size is max rooms needed.
    /// Time: O(n log n) | Space: O(n)
    pub fn min_meeting_rooms_heap(mut intervals: Vec<Vec<i32>>) -> i32 {
        if intervals.is_empty() {
            return 0;
        }

        intervals.sort_unstable_by_key(|iv| iv[0]);
        let mut min_heap: BinaryHeap<Reverse<i32>> = BinaryHeap::new();

        for iv in intervals {
            let start = iv[0];
            let end = iv[1];

            if let Some(&Reverse(earliest_end)) = min_heap.peek() {
                if start >= earliest_end {
                    min_heap.pop(); // Room freed up
                }
            }
            min_heap.push(Reverse(end));
        }

        min_heap.len() as i32
    }

    /// 2. OPTIMAL (Chronological Sweep-Line / Two Pointers):
    /// Separate start times and end times into two sorted arrays.
    /// Walk with two pointers: if start < end, room needed++ and advance start.
    /// Else room freed-- and advance end.
    /// Time: O(n log n) | Space: O(n)
    pub fn min_meeting_rooms(intervals: Vec<Vec<i32>>) -> i32 {
        if intervals.is_empty() {
            return 0;
        }

        let mut starts: Vec<i32> = intervals.iter().map(|iv| iv[0]).collect();
        let mut ends: Vec<i32> = intervals.iter().map(|iv| iv[1]).collect();

        starts.sort_unstable();
        ends.sort_unstable();

        let (mut start_ptr, mut end_ptr) = (0, 0);
        let (mut rooms, mut max_rooms) = (0, 0);

        while start_ptr < starts.len() {
            if starts[start_ptr] < ends[end_ptr] {
                rooms += 1;
                max_rooms = max_rooms.max(rooms);
                start_ptr += 1;
            } else {
                rooms -= 1;
                end_ptr += 1;
            }
        }

        max_rooms
    }
}

fn main() {
    let test_cases = vec![
        (vec![vec![0, 30], vec![5, 10], vec![15, 20]], 2),
        (vec![vec![7, 10], vec![2, 4]], 1),
        (vec![vec![1, 5], vec![8, 9], vec![8, 9]], 2),
        (vec![vec![1, 4], vec![2, 5], vec![7, 9]], 2),
        (vec![], 0),
        (vec![vec![13, 15], vec![1, 13]], 1),
    ];

    for (intervals, expected) in test_cases {
        assert_eq!(
            Solution::min_meeting_rooms_heap(intervals.clone()),
            expected
        );
        assert_eq!(
            Solution::min_meeting_rooms(intervals),
            expected
        );
    }

    println!("All test cases passed for Meeting Rooms II (Min-Heap, Chronological Sweep-Line)!");
}
