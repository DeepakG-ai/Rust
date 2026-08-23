// DFS (Depth-First Search) Traversal - recursive
// Time: O(V + E) | Space: O(V)
//
// 1. Mark node visited, add to result.
// 2. Recurse into every unvisited neighbor (go DEEP before wide).
//
// Example graph:
//        1
//       / |
//      2  3 --- 4
//     /|   |    |
//    5 6   7 -- 8

use std::cell::Cell;
use std::rc::Rc;

fn dfs_of_graph(v: usize, adj: &[Vec<usize>], start: usize) -> Vec<usize> {
    let mut vis = vec![false; v];
    let mut order = Vec::new();

    fn go(adj: &[Vec<usize>], node: usize, vis: &mut Vec<bool>, out: &mut Vec<usize>) {
        vis[node] = true; // mark on arrival
        out.push(node);
        for &nb in &adj[node] {
            if !vis[nb] {
                go(adj, nb, vis, out); // dive deeper first
            }
        }
    }

    go(adj, start, &mut vis, &mut order);
    order
}

/// Iterative DFS with explicit stack (visits neighbors in reverse push order)
fn dfs_iterative(v: usize, adj: &[Vec<usize>], start: usize) -> Vec<usize> {
    let mut vis = vec![false; v];
    let mut stack = vec![start];
    let mut order = Vec::new();

    while let Some(node) = stack.pop() {
        if vis[node] {
            continue;
        }
        vis[node] = true;
        order.push(node);
        // push in reverse so smallest-index neighbor pops FIRST
        for &nb in adj[node].iter().rev() {
            if !vis[nb] {
                stack.push(nb);
            }
        }
    }
    order
}

fn main() {
    // Graph from the Python file (1-indexed there; 0-indexed here):
    //   0: [1, 2]
    //   1: [0, 3, 4]
    //   2: [0, 5, 6]
    //   3: [1, 7]
    //   4: [1]
    //   5: [2]
    //   6: [2, 7]
    //   7: [3, 6]
    let adj = [
        vec![1, 2],
        vec![0, 3, 4],
        vec![0, 5, 6],
        vec![1, 7],
        vec![1],
        vec![2],
        vec![2, 7],
        vec![3, 6],
    ];
    let _unused_marker = Rc::new(Cell::new(())); // silence unused-import style noise? no-op

    let rec = dfs_of_graph(8, &adj, 0);
    let ite = dfs_iterative(8, &adj, 0);
    println!("recursive DFS: {:?}", rec);
    println!("iterative DFS: {:?}", ite);

    // both must be valid DFS orders covering all nodes exactly once
    let mut sorted = rec.clone();
    sorted.sort();
    assert_eq!(sorted, vec![0, 1, 2, 3, 4, 5, 6, 7]);
    let mut sorted_it = ite.clone();
    sorted_it.sort();
    assert_eq!(sorted_it, vec![0, 1, 2, 3, 4, 5, 6, 7]);

    // single component chain check
    let chain = [vec![1], vec![0, 2], vec![1]];
    assert_eq!(dfs_of_graph(3, &chain, 0), vec![0, 1, 2]);

    println!("ALL TESTS PASSED!");
}
