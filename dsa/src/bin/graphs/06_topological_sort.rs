// Topological Sort - GFG
// Method: 1) DFS + stack   2) Kahn's BFS (in-degree)
// Time: O(V + E) | Space: O(V)
//
// Linear ordering where every directed edge u -> v has u BEFORE v.
// Only exists for DAGs (cycles make it impossible).
//
// Real-life analogy: getting dressed - socks BEFORE shoes, etc.
//
// Example:
//     5 -> 0 <- 4        valid orders include [5,4,0,2,3,1]
//     |         |
//     v         v
//     2 -> 3 -> 1

use std::collections::VecDeque;

/// APPROACH 1: DFS. Push node onto stack AFTER all its neighbors finish,
/// then reverse the stack. Dependencies end up first.
pub fn topological_sort_dfs(v: usize, adj: &[Vec<usize>]) -> Vec<usize> {
    fn go(node: usize, adj: &[Vec<usize>], vis: &mut [bool], stack: &mut Vec<usize>) {
        vis[node] = true;
        for &nb in &adj[node] {
            if !vis[nb] {
                go(nb, adj, vis, stack);
            }
        }
        stack.push(node); // after ALL neighbors are done
    }

    let mut vis = vec![false; v];
    let mut stack = Vec::new();
    for node in 0..v {
        if !vis[node] {
            go(node, adj, &mut vis, &mut stack);
        }
    }
    stack.reverse(); // reversed = topological order
    stack
}

/// APPROACH 2: Kahn's algorithm. Nodes with in-degree 0 have no pending
/// dependencies -> process them, decrement neighbors' in-degree.
pub fn topological_sort_bfs(v: usize, adj: &[Vec<usize>]) -> Vec<usize> {
    // Step 1: compute in-degrees
    let mut in_degree = vec![0usize; v];
    for node in 0..v {
        for &nb in &adj[node] {
            in_degree[nb] += 1;
        }
    }

    // Step 2: seed queue with all "no dependency" nodes
    let mut q: VecDeque<usize> = in_degree
        .iter()
        .enumerate()
        .filter(|(_, &d)| d == 0)
        .map(|(i, _)| i)
        .collect();

    let mut result = Vec::with_capacity(v);

    // Step 3: peel nodes off front, relaxing their edges
    while let Some(node) = q.pop_front() {
        result.push(node);
        for &nb in &adj[node] {
            in_degree[nb] -= 1;
            if in_degree[nb] == 0 {
                q.push_back(nb);
            }
        }
    }

    // Step 4: not all nodes processed => cycle => no topo order
    if result.len() != v {
        return Vec::new();
    }
    result
}

fn build_directed(v: usize, edges: &[(usize, usize)]) -> Vec<Vec<usize>> {
    let mut adj = vec![Vec::new(); v];
    for &(u, w) in edges {
        adj[u].push(w); // u -> w ONLY (directed)
    }
    adj
}

fn verify_topo(order: &[usize], edges: &[(usize, usize)]) -> bool {
    let pos: Vec<usize> = {
        let mut p = vec![0; order.len()];
        for (i, &n) in order.iter().enumerate() { p[n] = i; }
        p
    };
    edges.iter().all(|&(u, v)| pos[u] < pos[v])
}

fn main() {
    // TEST 1: classic example
    let e1 = [(5usize, 0usize), (5, 2), (4, 0), (4, 1), (2, 3), (3, 1)];
    let adj1 = build_directed(6, &e1);
    let dfs_order = topological_sort_dfs(6, &adj1);
    let bfs_order = topological_sort_bfs(6, &adj1);
    assert_eq!(dfs_order.len(), 6);
    assert!(verify_topo(&dfs_order, &e1));
    assert_eq!(bfs_order, vec![4, 5, 0, 2, 3, 1]);
    assert!(verify_topo(&bfs_order, &e1));
    println!("DFS order: {:?}", dfs_order);

    // TEST 2: simple chain 0->1->2->3
    let adj2 = build_directed(4, &[(0, 1), (1, 2), (2, 3)]);
    assert_eq!(topological_sort_bfs(4, &adj2), vec![0, 1, 2, 3]);

    // TEST 3: single node / no edges
    assert_eq!(topological_sort_bfs(1, &[vec![]]), vec![0]);
    assert_eq!(topological_sort_bfs(4, &[vec![], vec![], vec![], vec![]]), vec![0, 1, 2, 3]);

    // TEST 4: cycle detection bonus - topo sort impossible
    let cyclic = build_directed(3, &[(0, 1), (1, 2), (2, 0)]);
    assert!(topological_sort_bfs(3, &cyclic).is_empty());

    println!("ALL TESTS PASSED!");
}
