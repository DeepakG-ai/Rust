// LeetCode Problem 226: Invert Binary Tree
// Approaches:
//   1) Recursive DFS: Post-order child swap -> O(n) time | O(h) call stack
//   2) Iterative BFS: Queue-based level swap -> O(n) time | O(w) space
//   3) Iterative DFS: Stack-based swap -> O(n) time | O(h) space
// Link: https://leetcode.com/problems/invert-binary-tree/
//
// Examples:
//      4                4
//     / \              / \
//    2   7     ->     7   2
//   / \ / \          / \ / \
//  1  3 6  9        9  6 3  1

use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

type Link = Option<Rc<RefCell<TreeNode>>>;

struct TreeNode {
    val: i32,
    left: Link,
    right: Link,
}

fn new_node(val: i32) -> Rc<RefCell<TreeNode>> {
    Rc::new(RefCell::new(TreeNode { val, left: None, right: None }))
}

fn array_to_tree(arr: &[Option<i32>]) -> Link {
    if arr.is_empty() || arr[0].is_none() {
        return None;
    }
    let root = new_node(arr[0].unwrap());
    let mut queue = VecDeque::from([root.clone()]);
    let mut i = 1;
    while !queue.is_empty() && i < arr.len() {
        let node = queue.pop_front().unwrap();
        if i < arr.len() && arr[i].is_some() {
            let c = new_node(arr[i].unwrap());
            node.borrow_mut().left = Some(c.clone());
            queue.push_back(c);
        }
        i += 1;
        if i < arr.len() && arr[i].is_some() {
            let c = new_node(arr[i].unwrap());
            node.borrow_mut().right = Some(c.clone());
            queue.push_back(c);
        }
        i += 1;
    }
    Some(root)
}

fn level_order(root: &Link) -> Vec<i32> {
    let mut out = Vec::new();
    let mut queue = VecDeque::from([root.clone()]);
    while let Some(cur) = queue.pop_front() {
        if let Some(node) = cur {
            out.push(node.borrow().val);
            queue.push_back(node.borrow().left.clone());
            queue.push_back(node.borrow().right.clone());
        }
    }
    out
}

struct Solution;

impl Solution {
    /// 1. RECURSIVE DFS:
    /// Invert left and right subtrees recursively, then swap.
    /// Time: O(n) | Space: O(h) call stack
    pub fn invert_tree(root: Link) -> Link {
        if let Some(node) = &root {
            let l = Self::invert_tree(node.borrow().left.clone());
            let r = Self::invert_tree(node.borrow().right.clone());
            node.borrow_mut().left = r;
            node.borrow_mut().right = l;
        }
        root
    }

    /// 2. ITERATIVE BFS (Queue):
    /// Dequeue node, swap children, and enqueue child subtrees.
    /// Time: O(n) | Space: O(w)
    pub fn invert_tree_bfs(root: Link) -> Link {
        let mut queue = VecDeque::from([root.clone()]);
        while let Some(cur) = queue.pop_front() {
            if let Some(node) = cur {
                let l = node.borrow().left.clone();
                let r = node.borrow().right.clone();
                node.borrow_mut().left = r.clone();
                node.borrow_mut().right = l.clone();
                if r.is_some() { queue.push_back(r); }
                if l.is_some() { queue.push_back(l); }
            }
        }
        root
    }

    /// 3. ITERATIVE DFS (Stack):
    /// Pop node from stack, swap children, push child subtrees.
    /// Time: O(n) | Space: O(h)
    pub fn invert_tree_dfs_iterative(root: Link) -> Link {
        let mut stack = vec![root.clone()];
        while let Some(cur) = stack.pop() {
            if let Some(node) = cur {
                let l = node.borrow().left.clone();
                let r = node.borrow().right.clone();
                node.borrow_mut().left = r.clone();
                node.borrow_mut().right = l.clone();
                if r.is_some() { stack.push(r); }
                if l.is_some() { stack.push(l); }
            }
        }
        root
    }
}

fn main() {
    let t = array_to_tree(&[Some(4), Some(2), Some(7), Some(1), Some(3), Some(6), Some(9)]);
    let inv = Solution::invert_tree(t);
    assert_eq!(level_order(&inv), vec![4, 7, 2, 9, 6, 3, 1]);

    let t2 = array_to_tree(&[Some(4), Some(2), Some(7), Some(1), Some(3), Some(6), Some(9)]);
    let inv2 = Solution::invert_tree_bfs(t2);
    assert_eq!(level_order(&inv2), vec![4, 7, 2, 9, 6, 3, 1]);

    let t3 = array_to_tree(&[Some(4), Some(2), Some(7), Some(1), Some(3), Some(6), Some(9)]);
    let inv3 = Solution::invert_tree_dfs_iterative(t3);
    assert_eq!(level_order(&inv3), vec![4, 7, 2, 9, 6, 3, 1]);

    // Empty tree
    assert!(Solution::invert_tree(None).is_none());

    println!("All test cases passed for Invert Binary Tree (Recursive DFS, Iterative BFS, Iterative DFS)!");
}
