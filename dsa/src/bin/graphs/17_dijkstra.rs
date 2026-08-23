// Dijkstra's Algorithm - GFG
// Method: Min-heap with LAZY DELETION
// Time: O((V + E) log V) | Space: O(V)
//
// Greedy: pop the closest unsettled node, relax its edges, push improved
// entries. Outdated heap entries are SKIPPED via `d > dist[node]` (lazy
// deletion) because Rust's BinaryHeap can't remove arbitrary items.
//
// Works only with NON-NEGATIVE weights.
//
// Example graph:
//       1
//   0 ----- 1
//   |       |
//  4|       |2          dist from 0 = [0, 1, 4, 3]
//   2 ----- 3
//       3

use std::cmp::Reverse;
use std::collections::BinaryHeap;

pub type Adj = Vec<Vec<(usize, i64)>>; // adj[u] = [(neighbor, weight), ...]

/// Dijkstra returning shortest distances from `src`
/// (i64::MAX means unreachable).
pub fn dijkstra(v: usize, adj: &Adj, src: usize) -> Vec<i64> {
    let mut dist = vec![i64::MAX; v];
    dist[src] = 0;

    // min-heap of (distance, node): Reverse makes BinaryHeap a MIN-heap
    let mut pq: BinaryHeap<Reverse<(i64, usize)>> = BinaryHeap::new();
    pq.push(Reverse((0, src)));

    while let Some(Reverse((d, node))) = pq.pop() {
        // LAZY DELETION: this entry is stale if we already found shorter
        if d > dist[node] {
            continue;
        }

        for &(nb, w) in &adj[node] {
            let nd = d + w;
            if nd < dist[nb] {
                dist[nb] = nd;
                pq.push(Reverse((nd, nb)));
            }
        }
    }
    dist
}

fn main() {
    // TEST 1: square graph from the Python file
    let adj1: Adj = vec![
        vec![(1, 1), (2, 4)],
        vec![(0, 1), (3, 2)],
        vec![(0, 4), (3, 3)],
        vec![(1, 2), (2, 3)],
    ];
    assert_eq!(dijkstra(4, &adj1, 0), vec![0, 1, 4, 3]);
    // different source!
    assert_eq!(dijkstra(4, &adj1, 2), vec![4, 5, 0, 3]);

    // TEST 2: Striver's example exercising lazy deletion on node 5
    // (0->2->5 direct = 10 gets superseded by 0->2->4->5 = 8)
    let mut adj2: Adj = vec![vec![]; 6];
    let add_edge = |adj: &mut Adj, u: usize, v: usize, w: i64| {
        adj[u].push((v, w));
        adj[v].push((u, w)); // undirected: both directions
    };
    add_edge(&mut adj2, 0, 1, 4);
    add_edge(&mut adj2, 0, 2, 4);
    add_edge(&mut adj2, 1, 2, 2);
    add_edge(&mut adj2, 2, 3, 3);
    add_edge(&mut adj2, 2, 4, 1);
    add_edge(&mut adj2, 2, 5, 6);
    add_edge(&mut adj2, 3, 5, 2);
    add_edge(&mut adj2, 4, 5, 3);
    assert_eq!(dijkstra(6, &adj2, 0), vec![0, 4, 4, 7, 5, 8]);

    // TEST 3: disconnected -> INF stays for the other island
    let disc: Adj = vec![vec![(1, 1)], vec![(0, 1)], vec![(3, 1)], vec![(2, 1)]];
    assert_eq!(dijkstra(4, &disc, 0), vec![0, 1, i64::MAX, i64::MAX]);

    println!("ALL TESTS PASSED!");
}
