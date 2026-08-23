// Shortest Path in DAG (source = 0) - GFG
// Method: Topological sort + edge relaxation
// Time: O(V + E) | Space: O(V + E)  (faster than Dijkstra's O((V+E)logV))
//
// WHY topo sort? In topo order, every edge u->v has u processed BEFORE v,
// so when we relax u's edges, dist[u] is already final. One pass suffices.
//
// Unreachable nodes report -1.
//
// Example:
//   0 -> 1 (2), 0 -> 2 (1), 2 -> 3 (3), 1 -> 3 (4)
//   dist from 0: [0, 2, 1, 4]  (0->2->3 beats 0->1->3)

use std::cell::Cell;
use std::rc::Rc;

pub fn shortest_path_dag(v: usize, adj: &[Vec<(usize, i64)>]) -> Vec<i64> {
    // Step 1: topological order via DFS + stack
    fn topo(node: usize, adj: &[Vec<(usize, i64)>], vis: &mut [bool], stack: &mut Vec<usize>) {
        vis[node] = true;
        for &(nb, _) in &adj[node] {
            if !vis[nb] {
                topo(nb, adj, vis, stack);
            }
        }
        stack.push(node); // after all descendants
    }

    let mut vis = vec![false; v];
    let mut stack = Vec::new();
    for node in 0..v {
        if !vis[node] {
            topo(node, adj, &mut vis, &mut stack);
        }
    }
    // reversed stack = topological order; source 0 comes first if reachable
    stack.reverse();

    // Step 2: relax edges in topo order
    const INF: i64 = i64::MAX;
    let mut dist = vec![INF; v];
    dist[0] = 0;

    for node in stack {
        // skip nodes we cannot reach yet
        if dist[node] == INF {
            continue;
        }
        for &(nb, w) in &adj[node] {
            if dist[node] + w < dist[nb] {
                dist[nb] = dist[node] + w; // RELAX!
            }
        }
    }

    // Step 3: convert INF -> -1 per problem statement
    dist.into_iter().map(|d| if d == INF { -1 } else { d }).collect()
}

fn main() {
    let _marker = Rc::new(Cell::new(())); // keep imports honest

    // Graph: 0->1(2), 0->2(1), 2->3(3), 1->3(4), plus isolated-ish node 4
    let adj = vec![
        vec![(1, 2), (2, 1)],
        vec![(3, 4)],
        vec![(3, 3)],
        vec![],
        vec![], // unreachable from 0
    ];
    assert_eq!(shortest_path_dag(5, &adj), vec![0, 2, 1, 4, -1]);

    // linear chain with weights
    let chain = vec![vec![(1, 5)], vec![(2, 3)], vec![]];
    assert_eq!(shortest_path_dag(3, &chain), vec![0, 5, 8]);

    println!("All test cases passed!");
}
