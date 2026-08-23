// LeetCode Problem 310: Minimum Height Trees
// Approaches:
//   1) Brute Force: Run BFS from every single node to find max tree height -> O(V * (V + E)) = O(n^2) time
//   2) Optimal: Topological Leaf Trimming (trim outer degree-1 leaves layer by layer until <= 2 centroids remain) -> O(V + E) = O(n) time | O(n) space
// Link: https://leetcode.com/problems/minimum-height-trees/
//
// Examples:
//   n = 4, edges = [[1,0],[1,2],[1,3]] -> [1]
//   n = 6, edges = [[3,0],[3,1],[3,2],[3,4],[5,4]] -> [3, 4]

use std::collections::{HashSet, VecDeque};

struct Solution;

impl Solution {
    /// 1. BRUTE FORCE:
    /// Calculate tree height rooted at every possible node 0..n via BFS.
    /// Return the nodes that produce the minimum height.
    /// Time: O(n^2) | Space: O(n)
    pub fn find_min_height_trees_brute(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
        let n = n as usize;
        if n <= 2 {
            return (0..n as i32).collect();
        }

        let mut adj = vec![Vec::new(); n];
        for e in &edges {
            let (u, v) = (e[0] as usize, e[1] as usize);
            adj[u].push(v);
            adj[v].push(u);
        }

        let get_height = |start: usize| -> i32 {
            let mut vis = vec![false; n];
            let mut q = VecDeque::from([(start, 0i32)]);
            vis[start] = true;
            let mut max_depth = 0;

            while let Some((node, depth)) = q.pop_front() {
                max_depth = max_depth.max(depth);
                for &nb in &adj[node] {
                    if !vis[nb] {
                        vis[nb] = true;
                        q.push_back((nb, depth + 1));
                    }
                }
            }
            max_depth
        };

        let mut heights = vec![0; n];
        let mut min_h = i32::MAX;
        for i in 0..n {
            heights[i] = get_height(i);
            min_h = min_h.min(heights[i]);
        }

        let mut res = Vec::new();
        for i in 0..n {
            if heights[i] == min_h {
                res.push(i as i32);
            }
        }
        res
    }

    /// 2. OPTIMAL (Leaf Trimming / Centroid Finding):
    /// A tree has at most 2 centroid nodes that minimize max depth.
    /// Repeatedly remove all degree 1 leaves until <= 2 nodes remain.
    /// Time: O(n) | Space: O(n)
    pub fn find_min_height_trees(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
        let n = n as usize;
        if n <= 2 {
            return (0..n as i32).collect();
        }

        let mut adj: Vec<HashSet<usize>> = vec![HashSet::new(); n];
        for e in &edges {
            let (u, v) = (e[0] as usize, e[1] as usize);
            adj[u].insert(v);
            adj[v].insert(u);
        }

        let mut leaves: Vec<usize> = (0..n).filter(|&i| adj[i].len() == 1).collect();
        let mut remaining_nodes = n;

        while remaining_nodes > 2 {
            remaining_nodes -= leaves.len();
            let mut new_leaves = Vec::new();

            for leaf in leaves {
                if let Some(&neighbor) = adj[leaf].iter().next() {
                    adj[neighbor].remove(&leaf);
                    if adj[neighbor].len() == 1 {
                        new_leaves.push(neighbor);
                    }
                }
            }
            leaves = new_leaves;
        }

        let mut res: Vec<i32> = leaves.into_iter().map(|x| x as i32).collect();
        res.sort_unstable();
        res
    }
}

fn main() {
    let test_cases = vec![
        (4, vec![vec![1, 0], vec![1, 2], vec![1, 3]], vec![1]),
        (
            6,
            vec![vec![3, 0], vec![3, 1], vec![3, 2], vec![3, 4], vec![5, 4]],
            vec![3, 4],
        ),
        (1, vec![], vec![0]),
        (2, vec![vec![0, 1]], vec![0, 1]),
    ];

    for (n, edges, expected) in test_cases {
        let mut ans_brute = Solution::find_min_height_trees_brute(n, edges.clone());
        ans_brute.sort_unstable();
        let ans_opt = Solution::find_min_height_trees(n, edges);

        assert_eq!(ans_brute, expected);
        assert_eq!(ans_opt, expected);
    }

    println!("All test cases passed for Minimum Height Trees (Brute Force BFS, Leaf Trimming)!");
}
