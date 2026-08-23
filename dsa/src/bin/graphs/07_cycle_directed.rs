// Detect Cycle in Directed Graph - GFG
// Approaches:
//   1) Kahn's BFS (Topological Sort Node Count): -> O(V + E) time | O(V) space
//   2) DFS with 3-Color / Recursion Stack Tracking: -> O(V + E) time | O(V) call stack
//
// Examples:
//   Cycle: A -> B -> C -> A
//   DAG:   A -> B -> C

use std::collections::VecDeque;

/// 1. KAHN'S BFS (In-Degree Tracking):
/// In a DAG, topological sort processes all V nodes. If a cycle exists, nodes in the cycle
/// never reach in-degree 0, so processed count < V.
pub fn is_cycle_directed(v: usize, adj: &[Vec<usize>]) -> bool {
    let mut in_degree = vec![0usize; v];
    for node in 0..v {
        for &nb in &adj[node] {
            in_degree[nb] += 1;
        }
    }

    let mut q: VecDeque<usize> = in_degree
        .iter()
        .enumerate()
        .filter(|(_, &d)| d == 0)
        .map(|(i, _)| i)
        .collect();

    let mut processed = 0usize;
    while let Some(node) = q.pop_front() {
        processed += 1;
        for &nb in &adj[node] {
            in_degree[nb] -= 1;
            if in_degree[nb] == 0 {
                q.push_back(nb);
            }
        }
    }

    processed != v
}

/// 2. DFS 3-COLOR MARKING:
/// 0 = unvisited (white)
/// 1 = in current recursion stack / visiting (gray)
/// 2 = fully visited / backtrack safe (black)
/// Encountering a gray node indicates a back-edge (cycle).
pub fn is_cycle_directed_dfs(v: usize, adj: &[Vec<usize>]) -> bool {
    fn go(node: usize, adj: &[Vec<usize>], color: &mut [u8]) -> bool {
        color[node] = 1; // gray: entering current stack
        for &nb in &adj[node] {
            match color[nb] {
                1 => return true, // back-edge found -> cycle
                0 => {
                    if go(nb, adj, color) {
                        return true;
                    }
                }
                _ => {} // 2 = safe
            }
        }
        color[node] = 2; // black: done
        false
    }

    let mut color = vec![0u8; v];
    (0..v).any(|n| color[n] == 0 && go(n, adj, &mut color))
}

fn main() {
    // TEST 1: cycle 0->1->2->0
    let cyc = [vec![1], vec![2], vec![0]];
    assert!(is_cycle_directed(3, &cyc));
    assert!(is_cycle_directed_dfs(3, &cyc));

    // TEST 2: chain 0->1->2 -> no cycle
    let chain = [vec![1], vec![2], vec![]];
    assert!(!is_cycle_directed(3, &chain));
    assert!(!is_cycle_directed_dfs(3, &chain));

    // TEST 3: self-loop
    let self_loop = [vec![0], vec![2], vec![]];
    assert!(is_cycle_directed(3, &self_loop));
    assert!(is_cycle_directed_dfs(3, &self_loop));

    // TEST 4: diamond (no cycle)
    let diamond = [vec![1, 2], vec![3], vec![3], vec![]];
    assert!(!is_cycle_directed(4, &diamond));
    assert!(!is_cycle_directed_dfs(4, &diamond));

    println!("All test cases passed for Directed Cycle Detection (Kahn's BFS, 3-Color DFS)!");
}
