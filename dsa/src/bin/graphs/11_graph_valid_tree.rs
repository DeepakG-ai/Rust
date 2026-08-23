// Graph Valid Tree - LeetCode 261
// Method: edge count + BFS connectivity (or Union-Find)
// Time: O(V + E) | Space: O(V + E)
//
// A valid tree needs BOTH:
//   1. exactly n - 1 edges  (fewer = disconnected, more = cycle guaranteed)
//   2. full connectivity from any single start node
//
// Examples:
//   n=5, edges=[[0,1],[0,2],[0,3],[1,4]] -> true
//   n=5, edges=[[0,1],[1,2],[2,3],[1,3],[1,4]] -> false (cycle)

use std::collections::VecDeque;

struct Solution;

impl Solution {
    /// BFS connectivity check with the n-1 shortcut
    pub fn valid_tree(n: usize, edges: Vec<Vec<i32>>) -> bool {
        // Rule 1: a tree ALWAYS has exactly n-1 edges
        // (guard n==0 first so `n - 1` can't underflow)
        if n == 0 || edges.len() != n - 1 {
            return false;
        }

        let mut adj = vec![Vec::new(); n];
        for e in &edges {
            let (u, v) = (e[0] as usize, e[1] as usize);
            adj[u].push(v); // undirected -> both ways
            adj[v].push(u);
        }

        // Rule 2: BFS from node 0 must reach ALL n nodes.
        // (with n-1 edges and no cycle possible when connected)
        let mut vis = vec![false; n];
        let mut q = VecDeque::from([0usize]);
        vis[0] = true;
        while let Some(node) = q.pop_front() {
            for &nb in &adj[node] {
                if !vis[nb] {
                    vis[nb] = true;
                    q.push_back(nb);
                }
            }
        }
        vis.iter().all(|&v| v) // every node reached?
    }

    /// UNION-FIND variant: adding an edge between two nodes already in the
    /// same set would create a cycle; at the end there must be 1 component.
    pub fn valid_tree_union_find(n: usize, edges: Vec<Vec<i32>>) -> bool {
        if edges.len() != n - 1 {
            return false;
        }
        let mut parent: Vec<usize> = (0..n).collect();
        fn find(p: &mut [usize], x: usize) -> usize {
            if p[x] != x {
                p[x] = find(p, p[x]); // path compression
            }
            p[x]
        }

        for e in &edges {
            let (ru, rv) = (find(&mut parent, e[0] as usize), find(&mut parent, e[1] as usize));
            if ru == rv {
                return false; // already connected -> this edge makes a cycle
            }
            parent[ru] = rv; // union the sets
        }
        true // n-1 successful unions => fully connected
    }
}

fn main() {
    let ok_edges = vec![vec![0, 1], vec![0, 2], vec![0, 3], vec![1, 4]];
    assert!(Solution::valid_tree(5, ok_edges.clone()));
    assert!(Solution::valid_tree_union_find(5, ok_edges));

    // cycle present
    let cyc = vec![vec![0, 1], vec![1, 2], vec![2, 3], vec![1, 3], vec![1, 4]];
    assert!(!Solution::valid_tree(5, cyc.clone()));
    assert!(!Solution::valid_tree_union_find(5, cyc));

    // disconnected: two components
    assert!(!Solution::valid_tree(6, vec![vec![0, 1], vec![2, 3]])); // wrong edge count

    // single node is a tree; zero nodes is not a valid tree input
    assert!(Solution::valid_tree(1, vec![]));
    assert!(!Solution::valid_tree(0, vec![]));

    println!("All test cases passed!");
}
