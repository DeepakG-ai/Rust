// LeetCode Problem 102: Binary Tree Level Order Traversal
// Approaches:
//   1) Iterative BFS: Queue snapshot per level -> O(n) time | O(w) space
//   2) Recursive DFS: Pass level index down -> O(n) time | O(h) call stack
//   3) Level Order Bottom-Up (LeetCode 107): -> O(n) time | O(w) space
// Link: https://leetcode.com/problems/binary-tree-level-order-traversal/
//
// Examples:
//       3
//      / \
//     9  20        -> [[3], [9,20], [15,7]]
//        / \
//      15   7

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
    /// 1. ITERATIVE BFS (Queue):
    /// Process exactly queue.len() elements per iteration.
    /// Time: O(n) | Space: O(w)
    pub fn level_order(root: Link) -> Vec<Vec<i32>> {
        let mut result: Vec<Vec<i32>> = Vec::new();
        let mut queue = VecDeque::new();
        if root.is_some() {
            queue.push_back(root);
        }

        while !queue.is_empty() {
            let mut level = Vec::with_capacity(queue.len());
            for _ in 0..queue.len() {
                if let Some(Some(node)) = queue.pop_front() {
                    level.push(node.borrow().val);
                    let (l, r) = (node.borrow().left.clone(), node.borrow().right.clone());
                    if l.is_some() { queue.push_back(l); }
                    if r.is_some() { queue.push_back(r); }
                }
            }
            result.push(level);
        }
        result
    }

    /// 2. RECURSIVE DFS:
    /// Traverse tree passing depth level index.
    /// Time: O(n) | Space: O(h) call stack
    pub fn level_order_dfs(root: Link) -> Vec<Vec<i32>> {
        fn dfs(node: &Link, level: usize, res: &mut Vec<Vec<i32>>) {
            if let Some(n) = node {
                if level == res.len() {
                    res.push(Vec::new());
                }
                res[level].push(n.borrow().val);
                dfs(&n.borrow().left, level + 1, res);
                dfs(&n.borrow().right, level + 1, res);
            }
        }

        let mut result = Vec::new();
        dfs(&root, 0, &mut result);
        result
    }

    /// 3. BOTTOM-UP LEVEL ORDER (LeetCode 107)
    pub fn level_order_bottom_up(root: Link) -> Vec<Vec<i32>> {
        let mut res = Self::level_order(root);
        res.reverse();
        res
    }
}

fn main() {
    let t = array_to_tree(&[Some(3), Some(9), Some(20), None, None, Some(15), Some(7)]);
    assert_eq!(
        Solution::level_order(t.clone()),
        vec![vec![3], vec![9, 20], vec![15, 7]]
    );
    assert_eq!(
        Solution::level_order_dfs(t.clone()),
        vec![vec![3], vec![9, 20], vec![15, 7]]
    );
    assert_eq!(
        Solution::level_order_bottom_up(t.clone()),
        vec![vec![15, 7], vec![9, 20], vec![3]]
    );

    assert_eq!(Solution::level_order(None), Vec::<Vec<i32>>::new());
    assert_eq!(Solution::level_order_dfs(None), Vec::<Vec<i32>>::new());
    assert_eq!(Solution::level_order(array_to_tree(&[Some(1)])), vec![vec![1]]);

    println!("All test cases passed for Binary Tree Level Order Traversal (BFS, DFS, Bottom-Up)!");
}
