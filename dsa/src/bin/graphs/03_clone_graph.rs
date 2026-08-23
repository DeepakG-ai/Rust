// Clone Graph - LeetCode 133
// Method: DFS with HashMap (old node -> clone) / BFS
// Time: O(V + E) | Space: O(V)
//
// Graphs can contain CYCLES, so a plain recursion would loop forever.
// The map breaks cycles: "already cloned? return the existing clone."
//
// Example (adjacency, 1-indexed):
//   [[2,4],[1,3],[2,4],[1,3]]  = square 1-2-3-4-1
//   -> deep copy must be a NEW set of nodes with same structure

use std::cell::RefCell;
use std::collections::{HashMap, HashSet, VecDeque};
use std::rc::Rc;

#[derive(Debug)]
struct GraphNode {
    val: i32,
    neighbors: RefCell<Vec<Rc<GraphNode>>>,
}

/// Build graph from adjacency list; returns node with val=1
fn build_graph(adj: &[Vec<i32>]) -> Option<Rc<GraphNode>> {
    if adj.is_empty() {
        return None;
    }
    let nodes: Vec<Rc<GraphNode>> = (1..=adj.len() as i32)
        .map(|v| Rc::new(GraphNode { val: v, neighbors: RefCell::new(vec![]) }))
        .collect();
    for (i, nbs) in adj.iter().enumerate() {
        for &n_idx in nbs {
            nodes[i].neighbors.borrow_mut().push(Rc::clone(&nodes[(n_idx - 1) as usize]));
        }
    }
    Some(Rc::clone(&nodes[0]))
}

/// Convert cloned graph back to adjacency list for verification
fn to_adj_list(start: &Rc<GraphNode>) -> Vec<Vec<i32>> {
    let mut result: HashMap<i32, Vec<i32>> = HashMap::new();
    let mut visited: HashSet<usize> = HashSet::new();
    let mut q = VecDeque::from([start.clone()]);
    visited.insert(Rc::as_ptr(start) as usize);

    while let Some(cur) = q.pop_front() {
        let nbs: Vec<i32> = cur.neighbors.borrow().iter().map(|n| n.val).collect();
        result.insert(cur.val, nbs);
        for nb in cur.neighbors.borrow().iter() {
            if visited.insert(Rc::as_ptr(nb) as usize) {
                q.push_back(nb.clone());
            }
        }
    }

    // output rows 1..=max_val like LeetCode format
    let max_val = *result.keys().max().unwrap_or(&0);
    (1..=max_val).map(|i| result.get(&i).cloned().unwrap_or_default()).collect()
}

struct Solution;

impl Solution {
    /// DFS recursive clone. The HashMap is what stops infinite loops on cycles.
    pub fn clone_graph(node: Option<Rc<GraphNode>>) -> Option<Rc<GraphNode>> {
        fn dfs(
            curr: &Rc<GraphNode>,
            old_to_new: &mut HashMap<usize, Rc<GraphNode>>,
        ) -> Rc<GraphNode> {
            let addr = Rc::as_ptr(curr) as usize;
            if let Some(clone) = old_to_new.get(&addr) {
                return Rc::clone(clone); // already cloned -> reuse (cycle guard!)
            }
            let copy = Rc::new(GraphNode { val: curr.val, neighbors: RefCell::new(vec![]) });
            old_to_new.insert(addr, Rc::clone(&copy)); // register BEFORE recursing

            for nb in curr.neighbors.borrow().iter() {
                copy.neighbors.borrow_mut().push(dfs(nb, old_to_new));
            }
            copy
        }

        node.map(|n| dfs(&n, &mut HashMap::new()))
    }
}

fn main() {
    // TEST 1: simple cycle/square [[2,4],[1,3],[2,4],[1,3]]
    let original = build_graph(&[vec![2, 4], vec![1, 3], vec![2, 4], vec![1, 3]]).unwrap();
    let cloned = Solution::clone_graph(Some(Rc::clone(&original))).unwrap();

    assert_eq!(to_adj_list(&cloned), vec![vec![2, 4], vec![1, 3], vec![2, 4], vec![1, 3]]);
    // must be a DIFFERENT object in memory (true deep copy!)
    assert!(!std::ptr::eq(Rc::as_ptr(&original), Rc::as_ptr(&cloned)));

    // TEST 2: single isolated node
    let single = build_graph(&[vec![]]).unwrap();
    let c2 = Solution::clone_graph(Some(single)).unwrap();
    assert_eq!(to_adj_list(&c2), vec![vec![]]);

    println!("ALL TESTS PASSED!");
}
