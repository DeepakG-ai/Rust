// Number of Provinces (Connected Components) - LeetCode 547
// Approaches:
//   1) BFS Flood Fill: Queue-based component traversal -> O(n^2) time | O(n) space
//   2) DFS Flood Fill: Recursive component traversal -> O(n^2) time | O(n) call stack
//   3) Disjoint Set Union (Union-Find with Path Compression & Rank): -> O(n^2 * alpha(n)) time | O(n) space
// Link: https://leetcode.com/problems/number-of-provinces/
//
// Examples:
//   [[1,1,0],
//    [1,1,0],     -> 2 provinces: {0,1} and {2}
//    [0,0,1]]

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
            count: n,
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
    /// 1. BFS flood fill per unvisited node
    pub fn find_circle_num(is_connected: Vec<Vec<i32>>) -> i32 {
        let n = is_connected.len();
        let mut vis = vec![false; n];
        let mut count = 0;

        for start in 0..n {
            if vis[start] {
                continue;
            }
            count += 1;
            let mut q = VecDeque::from([start]);
            vis[start] = true;
            while let Some(node) = q.pop_front() {
                for nb in 0..n {
                    if is_connected[node][nb] == 1 && !vis[nb] {
                        vis[nb] = true;
                        q.push_back(nb);
                    }
                }
            }
        }
        count
    }

    /// 2. DFS recursive flood fill
    pub fn find_circle_num_dfs(is_connected: Vec<Vec<i32>>) -> i32 {
        fn dfs(mat: &[Vec<i32>], node: usize, vis: &mut [bool]) {
            vis[node] = true;
            for nb in 0..mat.len() {
                if mat[node][nb] == 1 && !vis[nb] {
                    dfs(mat, nb, vis);
                }
            }
        }

        let n = is_connected.len();
        let mut vis = vec![false; n];
        let mut count = 0;
        for start in 0..n {
            if !vis[start] {
                count += 1;
                dfs(&is_connected, start, &mut vis);
            }
        }
        count
    }

    /// 3. DISJOINT SET UNION (Union-Find):
    /// Start with N disjoint components. Union any two connected cities.
    /// Remaining root count is the number of provinces.
    pub fn find_circle_num_union_find(is_connected: Vec<Vec<i32>>) -> i32 {
        let n = is_connected.len();
        let mut dsu = Dsu::new(n);

        for i in 0..n {
            for j in (i + 1)..n {
                if is_connected[i][j] == 1 {
                    dsu.union(i, j);
                }
            }
        }
        dsu.count as i32
    }
}

fn main() {
    let test_cases = vec![
        (vec![vec![1, 1, 0], vec![1, 1, 0], vec![0, 0, 1]], 2),
        (vec![vec![1, 0, 0], vec![0, 1, 0], vec![0, 0, 1]], 3),
        (vec![vec![1, 1, 1], vec![1, 1, 1], vec![1, 1, 1]], 1),
        (vec![vec![1]], 1),
        (
            vec![
                vec![1, 1, 0, 0],
                vec![1, 1, 1, 0],
                vec![0, 1, 1, 1],
                vec![0, 0, 1, 1],
            ],
            1,
        ),
    ];

    for (mat, expected) in test_cases {
        assert_eq!(Solution::find_circle_num(mat.clone()), expected);
        assert_eq!(Solution::find_circle_num_dfs(mat.clone()), expected);
        assert_eq!(Solution::find_circle_num_union_find(mat), expected);
    }

    println!("All test cases passed for Number of Provinces (BFS, DFS, Union-Find)!");
}
