// LeetCode Problem 621: Task Scheduler
// Approaches:
//   1) Better (Max-Heap Simulation with Cooldown Queue): -> O(N * 26) time | O(26) space
//   2) Optimal (Math Formula): -> O(N) time | O(1) space
// Link: https://leetcode.com/problems/task-scheduler/
//
// Description:
//   Given a char array `tasks` where each char is A-Z representing a task,
//   and a non-negative integer `n` representing the cooldown between two same tasks,
//   return the minimum number of intervals the CPU needs to finish all tasks.
//
// Examples:
//   tasks = ['A','A','A','B','B','B'], n = 2 -> 8 (A B idle A B idle A B)
//   tasks = ['A','A','A','B','B','B'], n = 0 -> 6
//   tasks = ['A','A','A','A','A','A','B','C','D','E','F','G'], n = 2 -> 16

use std::cmp::Reverse;
use std::collections::{BinaryHeap, VecDeque};

struct Solution;

impl Solution {
    /// 1. MAX-HEAP SIMULATION WITH COOLDOWN QUEUE:
    /// Greedily pick the most frequent task available. After execution, put it into
    /// a cooldown queue with the time it becomes available again.
    /// Time: O(N * 26) | Space: O(26) = O(1)
    pub fn least_interval_heap(tasks: Vec<char>, n: i32) -> i32 {
        let mut freq = [0i32; 26];
        for &t in &tasks {
            freq[(t as u8 - b'A') as usize] += 1;
        }

        let mut heap: BinaryHeap<i32> = BinaryHeap::new();
        for &f in &freq {
            if f > 0 {
                heap.push(f);
            }
        }

        let mut cooldown: VecDeque<(i32, i32)> = VecDeque::new(); // (remaining_count, available_at_time)
        let mut time = 0;

        while !heap.is_empty() || !cooldown.is_empty() {
            time += 1;

            // Check if any task in cooldown is ready
            if let Some(&(count, avail)) = cooldown.front() {
                if avail == time {
                    cooldown.pop_front();
                    heap.push(count);
                }
            }

            if let Some(count) = heap.pop() {
                if count - 1 > 0 {
                    cooldown.push_back((count - 1, time + n + 1));
                }
            }
        }

        time
    }

    /// 2. OPTIMAL (Math Formula):
    /// The most frequent task(s) dictate the minimum time.
    /// Let f_max = max frequency, count_max = how many tasks have f_max.
    /// Result = max(total_tasks, (f_max - 1) * (n + 1) + count_max)
    ///
    /// Intuition: Arrange f_max slots with n gaps between them, fill in other tasks.
    /// Time: O(N) | Space: O(1)
    pub fn least_interval(tasks: Vec<char>, n: i32) -> i32 {
        let mut freq = [0i32; 26];
        for &t in &tasks {
            freq[(t as u8 - b'A') as usize] += 1;
        }

        let f_max = *freq.iter().max().unwrap();
        let count_max = freq.iter().filter(|&&f| f == f_max).count() as i32;

        let formula = (f_max - 1) * (n + 1) + count_max;
        formula.max(tasks.len() as i32)
    }
}

fn main() {
    let test_cases = vec![
        (vec!['A', 'A', 'A', 'B', 'B', 'B'], 2, 8),
        (vec!['A', 'A', 'A', 'B', 'B', 'B'], 0, 6),
        (
            vec!['A', 'A', 'A', 'A', 'A', 'A', 'B', 'C', 'D', 'E', 'F', 'G'],
            2,
            16,
        ),
        (vec!['A', 'B', 'C', 'D'], 2, 4),
        (vec!['A'], 0, 1),
    ];

    for (tasks, n, expected) in test_cases {
        assert_eq!(
            Solution::least_interval_heap(tasks.clone(), n),
            expected
        );
        assert_eq!(
            Solution::least_interval(tasks, n),
            expected
        );
    }

    println!("All test cases passed for Task Scheduler (Max-Heap Simulation, Math Formula O(N))!");
}
