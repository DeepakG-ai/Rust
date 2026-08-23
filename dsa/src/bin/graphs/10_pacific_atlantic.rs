// Pacific Atlantic Water Flow - LeetCode 417
// Approaches:
//   1) Brute Force: Downhill DFS/BFS from every cell -> O((n*m)^2) time | O(n*m) space
//   2) Optimal: Multi-source BFS/DFS climbing UPHILL from ocean borders -> O(n*m) time | O(n*m) space
// Link: https://leetcode.com/problems/pacific-atlantic-water-flow/
//
// Examples:
//   1 2 2 3 5        answer: [[0,4],[1,3],[1,4],[2,2],[3,0],[3,1],[4,0]]
//   3 2 3 4 4
//   2 4 5 3 1
//   6 7 1 4 5
//   5 1 1 2 4

use std::collections::{HashSet, VecDeque};

struct Solution;

impl Solution {
    /// 1. BRUTE FORCE:
    /// Run DFS from every individual cell (r, c) downhill to check if both Pacific and Atlantic can be reached.
    /// Time: O((n * m)^2) | Space: O(n * m)
    pub fn pacific_atlantic_brute_force(heights: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let n = heights.len();
        if n == 0 {
            return vec![];
        }
        let m = heights[0].len();
        const DIRS: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

        fn can_reach(
            r: usize,
            c: usize,
            h: &[Vec<i32>],
            n: usize,
            m: usize,
            visited: &mut HashSet<(usize, usize)>,
        ) -> (bool, bool) {
            visited.insert((r, c));
            let mut reach_p = r == 0 || c == 0;
            let mut reach_a = r == n - 1 || c == m - 1;

            if reach_p && reach_a {
                return (true, true);
            }

            for (dr, dc) in DIRS {
                let (nr, nc) = (r as isize + dr, c as isize + dc);
                if nr >= 0 && nc >= 0 && (nr as usize) < n && (nc as usize) < m {
                    let (ur, uc) = (nr as usize, nc as usize);
                    if !visited.contains(&(ur, uc)) && h[ur][uc] <= h[r][c] {
                        let (p, a) = can_reach(ur, uc, h, n, m, visited);
                        reach_p = reach_p || p;
                        reach_a = reach_a || a;
                        if reach_p && reach_a {
                            break;
                        }
                    }
                }
            }
            (reach_p, reach_a)
        }

        let mut res = Vec::new();
        for r in 0..n {
            for c in 0..m {
                let mut visited = HashSet::new();
                let (reach_p, reach_a) = can_reach(r, c, &heights, n, m, &mut visited);
                if reach_p && reach_a {
                    res.push(vec![r as i32, c as i32]);
                }
            }
        }
        res.sort_unstable();
        res
    }

    /// 2. OPTIMAL (Multi-Source Uphill BFS from Ocean Borders):
    /// Climb uphill from Pacific (top & left) and Atlantic (bottom & right).
    /// Intersect the reachable cell sets.
    /// Time: O(n * m) | Space: O(n * m)
    pub fn pacific_atlantic(heights: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let n = heights.len();
        if n == 0 {
            return vec![];
        }
        let m = heights[0].len();
        const DIRS: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

        fn flood(
            starts: &[(usize, usize)],
            h: &[Vec<i32>],
            n: usize,
            m: usize,
        ) -> HashSet<(usize, usize)> {
            let mut seen: HashSet<(usize, usize)> = starts.iter().cloned().collect();
            let mut q = VecDeque::from(starts.to_vec());

            while let Some((r, c)) = q.pop_front() {
                for (dr, dc) in DIRS {
                    let (nr, nc) = (r as isize + dr, c as isize + dc);
                    if nr >= 0 && nc >= 0 && (nr as usize) < n && (nc as usize) < m {
                        let (ur, uc) = (nr as usize, nc as usize);
                        if !seen.contains(&(ur, uc)) && h[ur][uc] >= h[r][c] {
                            seen.insert((ur, uc));
                            q.push_back((ur, uc));
                        }
                    }
                }
            }
            seen
        }

        let mut pac_starts: Vec<(usize, usize)> =
            (0..m).map(|c| (0usize, c)).chain((0..n).map(|r| (r, 0usize))).collect();
        pac_starts.dedup();

        let mut atl_starts: Vec<(usize, usize)> =
            (0..m).map(|c| (n - 1, c)).chain((0..n).map(|r| (r, m - 1))).collect();
        atl_starts.dedup();

        let pacific = flood(&pac_starts, &heights, n, m);
        let atlantic = flood(&atl_starts, &heights, n, m);

        let mut both: Vec<Vec<i32>> = pacific
            .intersection(&atlantic)
            .map(|&(r, c)| vec![r as i32, c as i32])
            .collect();
        both.sort_unstable();
        both
    }
}

fn main() {
    let g = vec![
        vec![1, 2, 2, 3, 5],
        vec![3, 2, 3, 4, 4],
        vec![2, 4, 5, 3, 1],
        vec![6, 7, 1, 4, 5],
        vec![5, 1, 1, 2, 4],
    ];
    let expected = vec![
        vec![0, 4],
        vec![1, 3],
        vec![1, 4],
        vec![2, 2],
        vec![3, 0],
        vec![3, 1],
        vec![4, 0],
    ];

    assert_eq!(Solution::pacific_atlantic_brute_force(g.clone()), expected);
    assert_eq!(Solution::pacific_atlantic(g), expected);

    assert_eq!(Solution::pacific_atlantic_brute_force(vec![vec![1]]), vec![vec![0, 0]]);
    assert_eq!(Solution::pacific_atlantic(vec![vec![1]]), vec![vec![0, 0]]);

    println!("All test cases passed for Pacific Atlantic Water Flow (Brute Force, Multi-Source Uphill BFS)!");
}
