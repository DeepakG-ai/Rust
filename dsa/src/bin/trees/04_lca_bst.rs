// LeetCode Problem 235: Lowest Common Ancestor of a BST
// Approaches:
//   1) Brute Force (Path Tracing): Trace root-to-node path for both nodes and find last common node -> O(n) time | O(n) space
//   2) Better (Recursive BST Property): -> O(h) time | O(h) call stack
//   3) Optimal (Iterative BST Property): -> O(h) time | O(1) space
// Link: https://leetcode.com/problems/lowest-common-ancestor-of-a-binary-search-tree/
//
// Example tree:        6
//                     / \
//                    2   8
//                   / \ / \
//                  0  4 7  9
//                    / \
//                   3   5

use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

type Link = Option<Rc<RefCell<TreeNode>>>;

struct TreeNode {
    val: i32,
    left: Link,
    right: Link,
}

fn array_to_tree(arr: &[Option<i32>]) -> Link {
    if arr.is_empty() || arr[0].is_none() {
        return None;
    }
    let root = Rc::new(RefCell::new(TreeNode { val: arr[0].unwrap(), left: None, right: None }));
    let mut queue = VecDeque::from([root.clone()]);
    let mut i = 1;
    while !queue.is_empty() && i < arr.len() {
        let node = queue.pop_front().unwrap();
        if i < arr.len() && arr[i].is_some() {
            let c = Rc::new(RefCell::new(TreeNode { val: arr[i].unwrap(), left: None, right: None }));
            node.borrow_mut().left = Some(c.clone());
            queue.push_back(c);
        }
        i += 1;
        if i < arr.len() && arr[i].is_some() {
            let c = Rc::new(RefCell::new(TreeNode { val: arr[i].unwrap(), left: None, right: None }));
            node.borrow_mut().right = Some(c.clone());
            queue.push_back(c);
        }
        i += 1;
    }
    Some(root)
}

struct Solution;

impl Solution {
    /// 1. BRUTE FORCE (Path Tracing):
    /// Find paths from root to p and root to q, then return the last common node.
    /// Time: O(n) | Space: O(n)
    pub fn lca_path_trace(root: Link, p_val: i32, q_val: i32) -> Link {
        fn find_path(node: &Link, target: i32, path: &mut Vec<Rc<RefCell<TreeNode>>>) -> bool {
            if let Some(n) = node {
                path.push(n.clone());
                let val = n.borrow().val;
                if val == target {
                    return true;
                }
                if (val > target && find_path(&n.borrow().left, target, path))
                    || (val < target && find_path(&n.borrow().right, target, path))
                {
                    return true;
                }
                path.pop();
            }
            false
        }

        let mut path_p = Vec::new();
        let mut path_q = Vec::new();
        find_path(&root, p_val, &mut path_p);
        find_path(&root, q_val, &mut path_q);

        let mut lca = None;
        for (np, nq) in path_p.iter().zip(path_q.iter()) {
            if Rc::ptr_eq(np, nq) {
                lca = Some(np.clone());
            } else {
                break;
            }
        }
        lca
    }

    /// 2. BETTER (Recursive BST Navigation):
    /// Time: O(h) | Space: O(h) call stack
    pub fn lca_recursive(node: &Link, p_val: i32, q_val: i32) -> Link {
        match node {
            None => None,
            Some(n) => {
                let v = n.borrow().val;
                if p_val < v && q_val < v {
                    Self::lca_recursive(&n.borrow().left, p_val, q_val)
                } else if p_val > v && q_val > v {
                    Self::lca_recursive(&n.borrow().right, p_val, q_val)
                } else {
                    Some(Rc::clone(n))
                }
            }
        }
    }

    /// 3. OPTIMAL (Iterative BST Navigation):
    /// Walk down the tree without recursion. Split point is the LCA.
    /// Time: O(h) | Space: O(1)
    pub fn lowest_common_ancestor(root: Link, p: Link, q: Link) -> Link {
        let p_val = p.as_ref().map(|n| n.borrow().val).unwrap_or(0);
        let q_val = q.as_ref().map(|n| n.borrow().val).unwrap_or(0);

        let mut current = root;
        while let Some(node) = current.clone() {
            let cv = node.borrow().val;
            if p_val < cv && q_val < cv {
                current = node.borrow().left.clone();
            } else if p_val > cv && q_val > cv {
                current = node.borrow().right.clone();
            } else {
                return current;
            }
        }
        None
    }
}

fn find_node(root: &Link, val: i32) -> Link {
    let mut cur = root.clone();
    while let Some(n) = cur {
        let node_val = n.borrow().val;
        let next = match val.cmp(&node_val) {
            std::cmp::Ordering::Less => n.borrow().left.clone(),
            std::cmp::Ordering::Greater => n.borrow().right.clone(),
            std::cmp::Ordering::Equal => return Some(n),
        };
        cur = next;
    }
    None
}

fn main() {
    let root = array_to_tree(&[
        Some(6), Some(2), Some(8), Some(0), Some(4), Some(7), Some(9), None, None, Some(3), Some(5),
    ]);

    // Test 1: LCA(2, 8) = 6
    let p = find_node(&root, 2);
    let q = find_node(&root, 8);
    assert_eq!(
        Solution::lowest_common_ancestor(root.clone(), p, q).map(|n| n.borrow().val),
        Some(6)
    );
    assert_eq!(
        Solution::lca_path_trace(root.clone(), 2, 8).map(|n| n.borrow().val),
        Some(6)
    );
    assert_eq!(
        Solution::lca_recursive(&root, 2, 8).map(|n| n.borrow().val),
        Some(6)
    );

    // Test 2: LCA(2, 4) = 2
    let p = find_node(&root, 2);
    let q = find_node(&root, 4);
    assert_eq!(
        Solution::lowest_common_ancestor(root.clone(), p, q).map(|n| n.borrow().val),
        Some(2)
    );
    assert_eq!(
        Solution::lca_path_trace(root.clone(), 2, 4).map(|n| n.borrow().val),
        Some(2)
    );
    assert_eq!(
        Solution::lca_recursive(&root, 2, 4).map(|n| n.borrow().val),
        Some(2)
    );

    // Test 3: LCA(3, 5) = 4
    assert_eq!(
        Solution::lca_recursive(&root, 3, 5).map(|n| n.borrow().val),
        Some(4)
    );

    println!("All test cases passed for LCA of BST (Path Trace, Recursive, Iterative)!");
}
