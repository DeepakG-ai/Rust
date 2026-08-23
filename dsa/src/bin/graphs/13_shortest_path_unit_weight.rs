// Shortest Path in Undirected Graph with Unit Weights - GFG
// Method: BFS (level = distance)
// Time: O(V + E) | Space: O(V + E)
//
// All edges weigh 1, so BFS layers ARE shortest distances - the first time
// we reach a node is via the fewest edges. No Dijkstra needed!
//
// Unreachable nodes report -1.

use std::collections::VecDeque;

pub fn shortest_path_unit(v: usize, edges: &[(usize, usize)], src: usize) -> Vec<i64> {
    // build undirected adjacency
    let mut adj = vec![Vec::new(); v];
    for &(u, w) in edges {
        adj[u].push(w);
        adj[w].push(u);
    }

    let mut dist = vec![-1i64; v]; // -1 = unvisited/unreachable
    dist[src] = 0;
    let mut q = VecDeque::from([src]);

    while let Some(node) = q.pop_front() {
        for &nb in &adj[node] {
            if dist[nb] == -1 {
                // first visit = shortest possible in unit-weight graph
                dist[nb] = dist[node] + 1;
                q.push_back(nb);
            }
        }
    }
    dist
}

fn main() {
    //   0 - 1 - 2      distances from 0: [0,1,2,2]
    //        \|
    //         3
    assert_eq!(
        shortest_path_unit(4, &[(0, 1), (1, 2), (1, 3)], 0),
        vec![0, 1, 2, 2]
    );

    // disconnected component -> -1 for the far island
    let d = [(0usize, 1usize), (1, 2), (3, 4)];
    assert_eq!(shortest_path_unit(5, &d, 0), vec![0, 1, 2, -1, -1]);

    // cycle graph: still fine, BFS marks each node once
    let cyc = [(0usize, 1usize), (1, 2), (2, 0)];
    assert_eq!(shortest_path_unit(3, &cyc, 1), vec![1, 0, 1]);

    println!("All test cases passed!");
}
