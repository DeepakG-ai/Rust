// Rotting Oranges - LeetCode 994
// Method: MULTI-SOURCE BFS
// Time: O(n*m) | Space: O(n*m)
//
// Grid: 0 empty, 1 fresh, 2 rotten. Rot spreads 4-directionally per minute.
//
// Multi-source trick: enqueue EVERY rotten orange at t=0; each BFS layer
// is one minute. Track remaining fresh count to detect impossibility.
//
// Example:
//   [[2,1,1],
//    [1,1,0],     -> 4 minutes
//    [0,1,1]]

use std::collections::VecDeque;

struct Solution;

impl Solution {
    pub fn oranges_rotting(mut grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        if n == 0 {
            return 0;
        }
        let m = grid[0].len();
        // UP, DOWN, LEFT, RIGHT
        const DIRS: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

        let mut fresh = 0i32;
        let mut q: VecDeque<(usize, usize, i32)> = VecDeque::new(); // (row, col, time)

        // Step 1: seed queue with ALL rotten oranges, count fresh ones
        for r in 0..n {
            for c in 0..m {
                match grid[r][c] {
                    2 => q.push_back((r, c, 0)), // every source starts at t=0
                    1 => fresh += 1,
                    _ => {}
                }
            }
        }

        let mut minutes = 0i32;

        // Step 2: BFS wave by wave
        while let Some((r, c, t)) = q.pop_front() {
            minutes = minutes.max(t); // last processed time = total elapsed
            for (dr, dc) in DIRS {
                let (nr, nc) = (r as isize + dr, c as isize + dc);
                if nr >= 0 && nc >= 0 && (nr as usize) < n && (nc as usize) < m {
                    let (ur, uc) = (nr as usize, nc as usize);
                    if grid[ur][uc] == 1 {
                        grid[ur][uc] = 2; // just rotted
                        fresh -= 1;
                        q.push_back((ur, uc, t + 1));
                    }
                }
            }
        }

        if fresh > 0 { -1 } else { minutes } // leftover fresh -> impossible
    }
}

fn main() {
    assert_eq!(Solution::oranges_rotting(vec![vec![2, 1, 1], vec![1, 1, 0], vec![0, 1, 1]]), 4);
    // unreachable corner orange -> -1
    assert_eq!(Solution::oranges_rotting(vec![vec![2, 1, 1], vec![0, 1, 1], vec![1, 0, 1]]), -1);
    assert_eq!(Solution::oranges_rotting(vec![vec![0, 2]]), 0); // nothing fresh

    println!("All test cases passed!");
}
