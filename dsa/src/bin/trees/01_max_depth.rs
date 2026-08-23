// LeetCode Problem 104: Maximum Depth of Binary Tree
// Method: DFS recursive / BFS / DFS iterative
// Time: O(n) | Space: O(h) recursion, O(w) BFS
//
// max_depth(node) = 1 + max(depth(left), depth(right))
// Base case: None -> 0
//
// Example:
//       3
//      / \
//     9  20
//        / \
//      15   7      -> depth = 3

use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

type Link = Option<Rc<RefCell<TreeNode>>>;

#[allow(dead_code)]
struct TreeNode {
    val: i32,
    left: Link,
    right: Link,
}

/// Build a tree from a LeetCode-style level order array (None = missing node)
fn array_to_tree(arr: &[Option<i32>]) -> Link {
    if arr.is_empty() || arr[0].is_none() {
        return None;
    }
    let root = Rc::new(RefCell::new(TreeNode {
        val: arr[0].unwrap(),
        left: None,
        right: None,
    }));
    let mut queue = VecDeque::from([root.clone()]);
    let mut i = 1;

    while !queue.is_empty() && i < arr.len() {
        let node = queue.pop_front().unwrap();
        // left child
        if i < arr.len() && arr[i].is_some() {
            let child = Rc::new(RefCell::new(TreeNode { val: arr[i].unwrap(), left: None, right: None }));
            node.borrow_mut().left = Some(child.clone());
            queue.push_back(child);
        }
        i += 1;
        // right child
        if i < arr.len() && arr[i].is_some() {
            let child = Rc::new(RefCell::new(TreeNode { val: arr[i].unwrap(), left: None, right: None }));
            node.borrow_mut().right = Some(child.clone());
            queue.push_back(child);
        }
        i += 1;
    }
    Some(root)
}

struct Solution;

impl Solution {
    /// DFS recursive (most intuitive)
    pub fn max_depth(root: Link) -> i32 {
        match root {
            None => 0,
            Some(node) => {
                let b = node.borrow();
                let l = Self::max_depth(b.left.clone());
                let r = Self::max_depth(b.right.clone());
                1 + l.max(r) // my depth = me + deeper child
            }
        }
    }

    /// BFS: count levels. Each processed level adds +1 depth.
    pub fn max_depth_bfs(root: Link) -> i32 {
        let mut depth = 0;
        let mut queue = VecDeque::new();
        if root.is_some() {
            queue.push_back(root);
        }

        while !queue.is_empty() {
            depth += 1;
            for _ in 0..queue.len() {
                // process whole current level
                if let Some(Some(node)) = queue.pop_front() {
                    let b = node.borrow();
                    if b.left.is_some() {
                        queue.push_back(b.left.clone());
                    }
                    if b.right.is_some() {
                        queue.push_back(b.right.clone());
                    }
                }
            }
        }
        depth
    }

    /// DFS ITERATIVE with explicit stack of (node, depth)
    pub fn max_depth_iterative(root: Link) -> i32 {
        let mut stack: Vec<(Link, i32)> = vec![(root, 1)];
        let mut max_depth = 0;

        while let Some((node, depth)) = stack.pop() {
            if let Some(n) = node {
                max_depth = max_depth.max(depth);
                let b = n.borrow();
                stack.push((b.left.clone(), depth + 1));
                stack.push((b.right.clone(), depth + 1));
            }
        }
        max_depth
    }
}

fn main() {
    let t = array_to_tree(&[Some(3), Some(9), Some(20), None, None, Some(15), Some(7)]);
    assert_eq!(Solution::max_depth(t.clone()), 3);
    assert_eq!(Solution::max_depth_bfs(t.clone()), 3);
    assert_eq!(Solution::max_depth_iterative(t), 3);

    assert_eq!(Solution::max_depth(array_to_tree(&[Some(1), None, Some(2)])), 2);
    assert_eq!(Solution::max_depth(None), 0);

    println!("All test cases passed! (DFS/BFS/iterative)");
}
