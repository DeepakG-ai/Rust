// Detect Cycle in Undirected Graph - GFG
// Approaches:
//   1) BFS with Parent Tracking: -> O(V + E) time | O(V) space
//   2) DFS with Parent Tracking: -> O(V + E) time | O(V) call stack
//   3) Disjoint Set Union (Union-Find): Find if two vertices of an edge are already in same component -> O(E * alpha(V)) time | O(V) space
//
// Examples:
//   0 - 1 - 2        1-2-3-1 forms a triangle -> cycle
//       |
//       3

use std::collections::VecDeque;

struct Dsu {
    parent: Vec<usize>,
}

impl Dsu {
    fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
        }
    }

    fn find(&mut self, i: usize) -> usize {
        if self.parent[i] != i {
            let p = self.parent[i];
            self.parent[i] = self.find(p);
        }
        self.parent[i]
    }

    fn union(&mut self, i: usize, j: usize) -> bool {
        let root_i = self.find(i);
        let root_j = self.find(j);
        if root_i == root_j {
            return false; // Cycle detected!
        }
        self.parent[root_i] = root_j;
        true
    }
}

/// 1. BFS with parent tracking
pub fn is_cycle_undirected(v: usize, adj: &[Vec<usize>]) -> bool {
    let mut vis = vec![false; v];

    for start in 0..v {
        if vis[start] {
            continue;
        }
        let mut q = VecDeque::from([(start, usize::MAX)]);
        vis[start] = true;

        while let Some((node, parent)) = q.pop_front() {
            for &nb in &adj[node] {
                if !vis[nb] {
                    vis[nb] = true;
                    q.push_back((nb, node));
                } else if nb != parent {
                    return true;
                }
            }
        }
    }
    false
}

/// 2. DFS variant with parent tracking
pub fn is_cycle_undirected_dfs(v: usize, adj: &[Vec<usize>]) -> bool {
    fn go(node: usize, parent: usize, adj: &[Vec<usize>], vis: &mut [bool]) -> bool {
        vis[node] = true;
        for &nb in &adj[node] {
            if !vis[nb] {
                if go(nb, node, adj, vis) {
                    return true;
                }
            } else if nb != parent {
                return true;
            }
        }
        false
    }

    let mut vis = vec![false; v];
    (0..v).any(|n| !vis[n] && go(n, usize::MAX, adj, &mut vis))
}

/// 3. UNION-FIND:
/// Process all unique undirected edges (u < v). If u and v have the same root, an edge creates a cycle.
pub fn is_cycle_undirected_union_find(v: usize, adj: &[Vec<usize>]) -> bool {
    let mut dsu = Dsu::new(v);

    for u in 0..v {
        for &nb in &adj[u] {
            if u < nb {
                if !dsu.union(u, nb) {
                    return true;
                }
            }
        }
    }
    false
}

fn main() {
    // TEST 1: triangle 0-1-2-0 -> cycle
    let cyc = [vec![1, 2], vec![0, 2], vec![0, 1]];
    assert!(is_cycle_undirected(3, &cyc));
    assert!(is_cycle_undirected_dfs(3, &cyc));
    assert!(is_cycle_undirected_union_find(3, &cyc));

    // TEST 2: tree (no cycle)
    let tree = [vec![1], vec![0, 2], vec![1, 3], vec![2]];
    assert!(!is_cycle_undirected(4, &tree));
    assert!(!is_cycle_undirected_dfs(4, &tree));
    assert!(!is_cycle_undirected_union_find(4, &tree));

    // TEST 3: square 0-1-2-3-0 -> cycle
    let sq = [vec![1, 3], vec![0, 2], vec![1, 3], vec![2, 0]];
    assert!(is_cycle_undirected(4, &sq));
    assert!(is_cycle_undirected_dfs(4, &sq));
    assert!(is_cycle_undirected_union_find(4, &sq));

    // TEST 4: disconnected, only one component cyclic
    let disc = [vec![1], vec![0], vec![3, 4], vec![2, 4], vec![2, 3]];
    assert!(is_cycle_undirected(5, &disc));
    assert!(is_cycle_undirected_dfs(5, &disc));
    assert!(is_cycle_undirected_union_find(5, &disc));

    println!("All test cases passed for Undirected Cycle Detection (BFS, DFS, Union-Find)!");
}
