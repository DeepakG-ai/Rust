// BFS (Breadth-First Search) Traversal
// Time: O(V + E) | Space: O(V)
//
// 1. Mark start visited, push to queue.
// 2. Pop from FRONT, record node, mark & enqueue unvisited neighbors
//    IMMEDIATELY (marking at enqueue-time prevents duplicates).
//
// Example:     0 --- 1        adj = [[1,3],[0,2],[1,3],[0,2]]
//              |     |
//              3 --- 2        BFS from 0 = [0,1,3,2]

use std::collections::VecDeque;

fn bfs_of_graph(v: usize, adj: &[Vec<usize>], start: usize) -> Vec<usize> {
    let mut vis = vec![false; v];
    vis[start] = true; // mark BEFORE enqueue
    let mut q = VecDeque::from([start]);
    let mut order = Vec::new();

    while let Some(node) = q.pop_front() {
        order.push(node); // visit
        for &nb in &adj[node] {
            if !vis[nb] {
                vis[nb] = true;
                q.push_back(nb);
            }
        }
    }
    order
}

fn main() {
    // TEST 1: tree      0
    //                  / \
    //                 1   2
    //                 |   |
    //                 3   4
    let adj1 = [vec![1, 2], vec![0, 3], vec![0, 4], vec![1], vec![2]];
    assert_eq!(bfs_of_graph(5, &adj1, 0), vec![0, 1, 2, 3, 4]);
    assert_eq!(bfs_of_graph(5, &adj1, 3), vec![3, 1, 0, 2, 4]);

    // TEST 3: cycle graph
    let adj2 = [vec![1, 3], vec![0, 2], vec![1, 3], vec![0, 2]];
    assert_eq!(bfs_of_graph(4, &adj2, 0), vec![0, 1, 3, 2]);
    assert_eq!(bfs_of_graph(4, &adj2, 2), vec![2, 1, 3, 0]);

    // TEST 5: linear chain 0-1-2-3-4
    let adj3 = [vec![1], vec![0, 2], vec![1, 3], vec![2, 4], vec![3]];
    assert_eq!(bfs_of_graph(5, &adj3, 2), vec![2, 1, 3, 0, 4]);

    // TEST 7: star graph centered at 0
    let adj4 = [vec![1, 2, 3, 4], vec![0], vec![0], vec![0], vec![0]];
    assert_eq!(bfs_of_graph(5, &adj4, 0), vec![0, 1, 2, 3, 4]);

    // TEST 9: single node / two nodes
    assert_eq!(bfs_of_graph(1, &[vec![]], 0), vec![0]);
    assert_eq!(bfs_of_graph(2, &[vec![1], vec![0]], 1), vec![1, 0]);

    println!("ALL TESTS PASSED!");
}
