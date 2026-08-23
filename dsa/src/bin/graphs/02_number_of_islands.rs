// Number of Islands - LeetCode 200
// Approaches:
//   1) BFS Flood Fill (Visited Matrix): -> O(n * m) time | O(n * m) space
//   2) DFS In-Place Flood Fill (Sink Land): -> O(n * m) time | O(n * m) call stack
//   3) Disjoint Set Union (Union-Find): -> O(n * m * alpha(n*m)) time | O(n * m) space
// Link: https://leetcode.com/problems/number-of-islands/
//
// Examples:
//   1 1 0 0 0        -> 3 islands
//   1 1 0 0 0
//   0 0 1 0 0
//   0 0 0 1 1

use std::collections::VecDeque;

struct Dsu {
    parent: Vec<usize>,
    rank: Vec<usize>,
    count: usize,
}

impl Dsu {
    fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            rank: vec![0; n],
            count: 0,
        }
    }

    fn find(&mut self, i: usize) -> usize {
        if self.parent[i] != i {
            let p = self.parent[i];
            self.parent[i] = self.find(p);
        }
        self.parent[i]
    }

    fn union(&mut self, i: usize, j: usize) {
        let root_i = self.find(i);
        let root_j = self.find(j);
        if root_i != root_j {
            if self.rank[root_i] < self.rank[root_j] {
                self.parent[root_i] = root_j;
            } else if self.rank[root_i] > self.rank[root_j] {
                self.parent[root_j] = root_i;
            } else {
                self.parent[root_j] = root_i;
                self.rank[root_i] += 1;
            }
            self.count -= 1;
        }
    }
}

struct Solution;

impl Solution {
    /// 1. BFS version with visited matrix
    pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
        let n = grid.len();
        if n == 0 {
            return 0;
        }
        let m = grid[0].len();
        let mut vis = vec![vec![false; m]; n];
        const DIRS: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];
        let mut count = 0;

        for r in 0..n {
            for c in 0..m {
                if grid[r][c] == '1' && !vis[r][c] {
                    count += 1;
                    let mut q = VecDeque::from([(r, c)]);
                    vis[r][c] = true;
                    while let Some((cr, cc)) = q.pop_front() {
                        for (dr, dc) in DIRS {
                            let (nr, nc) = (cr as isize + dr, cc as isize + dc);
                            if nr >= 0 && nc >= 0
                                && (nr as usize) < n && (nc as usize) < m
                                && grid[nr as usize][nc as usize] == '1'
                                && !vis[nr as usize][nc as usize]
                            {
                                vis[nr as usize][nc as usize] = true;
                                q.push_back((nr as usize, nc as usize));
                            }
                        }
                    }
                }
            }
        }
        count
    }

    /// 2. DFS "sink" version: overwrite visited land with '0' in-place.
    pub fn num_islands_sink(mut grid: Vec<Vec<char>>) -> i32 {
        fn dfs(g: &mut Vec<Vec<char>>, r: usize, c: usize) {
            g[r][c] = '0';
            let (n, m) = (g.len(), g[0].len());
            if r > 0 && g[r - 1][c] == '1' { dfs(g, r - 1, c); }
            if r + 1 < n && g[r + 1][c] == '1' { dfs(g, r + 1, c); }
            if c > 0 && g[r][c - 1] == '1' { dfs(g, r, c - 1); }
            if c + 1 < m && g[r][c + 1] == '1' { dfs(g, r, c + 1); }
        }

        let mut count = 0;
        for r in 0..grid.len() {
            for c in 0..grid[0].len() {
                if grid[r][c] == '1' {
                    count += 1;
                    dfs(&mut grid, r, c);
                }
            }
        }
        count
    }

    /// 3. DISJOINT SET UNION (Union-Find):
    /// Connect adjacent '1' cells (down and right).
    pub fn num_islands_union_find(grid: Vec<Vec<char>>) -> i32 {
        let n = grid.len();
        if n == 0 {
            return 0;
        }
        let m = grid[0].len();
        let mut dsu = Dsu::new(n * m);

        for r in 0..n {
            for c in 0..m {
                if grid[r][c] == '1' {
                    dsu.count += 1;
                }
            }
        }

        for r in 0..n {
            for c in 0..m {
                if grid[r][c] == '1' {
                    let idx = r * m + c;
                    // Union right neighbor
                    if c + 1 < m && grid[r][c + 1] == '1' {
                        dsu.union(idx, r * m + (c + 1));
                    }
                    // Union down neighbor
                    if r + 1 < n && grid[r + 1][c] == '1' {
                        dsu.union(idx, (r + 1) * m + c);
                    }
                }
            }
        }
        dsu.count as i32
    }
}

fn main() {
    let g1 = vec![
        vec!['1', '1', '1', '1', '0'],
        vec!['1', '1', '0', '1', '0'],
        vec!['1', '1', '0', '0', '0'],
        vec!['0', '0', '0', '0', '0'],
    ];
    assert_eq!(Solution::num_islands(g1.clone()), 1);
    assert_eq!(Solution::num_islands_sink(g1.clone()), 1);
    assert_eq!(Solution::num_islands_union_find(g1), 1);

    let g2 = vec![
        vec!['1', '1', '0', '0', '0'],
        vec!['1', '1', '0', '0', '0'],
        vec!['0', '0', '1', '0', '0'],
        vec!['0', '0', '0', '1', '1'],
    ];
    assert_eq!(Solution::num_islands(g2.clone()), 3);
    assert_eq!(Solution::num_islands_sink(g2.clone()), 3);
    assert_eq!(Solution::num_islands_union_find(g2), 3);

    assert_eq!(Solution::num_islands(vec![vec!['0', '0'], vec!['0', '0']]), 0);
    assert_eq!(Solution::num_islands(vec![vec!['1']]), 1);
    assert_eq!(Solution::num_islands(vec![vec!['1', '0'], vec!['0', '1']]), 2);

    println!("All test cases passed for Number of Islands (BFS, DFS Sink, Union-Find)!");
}
