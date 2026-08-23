// Shortest Path in Weighted Undirected Graph (with path!) - GFG
// Method: Dijkstra + parent tracking for path reconstruction
// Time: O((V + E) log V) | Space: O(V + E)
//
// Two parts:
//   1. Dijkstra from node 1; whenever an edge relaxes, record parent[v] = u.
//   2. Walk parents backwards from destination n, then reverse.
//
// Nodes are 1-INDEXED. No path -> [-1].
//
// Example: n=5
//   edges: 1-2(2), 2-5(5), 1-3(3), 3-4(1), 4-5(4)
//   path 1->5: [1,2,5] cost 7

use std::cmp::Reverse;
use std::collections::BinaryHeap;

pub fn shortest_path_with_path(n: usize, m: usize, edges: &[Vec<i64>]) -> Vec<i64> {
    // build undirected adjacency (nodes 1..=n)
    let mut adj: Vec<Vec<(usize, i64)>> = vec![vec![]; n + 1];
    for e in edges.iter().take(m) {
        let (u, v, w) = (e[0] as usize, e[1] as usize, e[2]);
        adj[u].push((v, w));
        adj[v].push((u, w));
    }

    const INF: i64 = i64::MAX;
    let mut dist = vec![INF; n + 1];
    let mut parent = vec![0usize; n + 1]; // parent[1] stays 0 as sentinel
    dist[1] = 0;

    let mut pq: BinaryHeap<Reverse<(i64, usize)>> = BinaryHeap::new();
    pq.push(Reverse((0, 1)));

    while let Some(Reverse((d, node))) = pq.pop() {
        if d > dist[node] {
            continue; // stale entry
        }
        for &(nb, w) in &adj[node] {
            if d + w < dist[nb] {
                dist[nb] = d + w;
                parent[nb] = node; // came from `node`!
                pq.push(Reverse((dist[nb], nb)));
            }
        }
    }

    if dist[n] == INF {
        return vec![-1]; // destination unreachable
    }

    // reconstruct by walking parents backwards from n
    let mut path = vec![n as i64];
    let mut cur = n;
    while cur != 1 {
        cur = parent[cur];
        path.push(cur as i64);
    }
    path.reverse(); // now starts at source
    path
}

fn main() {
    // Graph:      1 --2-- 2 --5-- 5
    //             |               |
    //             +--3-- 3 --1-- 4 ----+
    //             |____________________|
    // best 1->5: via 2 (cost 7) vs via 3-4-5? 3+1+4=8 vs direct 1->5 none
    let e1 = vec![vec![1, 2, 2], vec![2, 5, 5], vec![1, 3, 3], vec![3, 4, 1], vec![4, 5, 4]];
    assert_eq!(shortest_path_with_path(5, 5, &e1), vec![1, 2, 5]); // cost 2+5=7

    // unreachable destination
    let e2 = vec![vec![1, 2, 1], vec![3, 4, 1]];
    assert_eq!(shortest_path_with_path(4, 2, &e2), vec![-1]);

    println!("All test cases passed!");
}
