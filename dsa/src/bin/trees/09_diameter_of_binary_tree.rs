// LeetCode Problem 543: Diameter of Binary Tree
// Approaches:
//   1) Brute Force: Compute depth from every node and check left+right -> O(n^2) time
//   2) Optimal (Single DFS): Track diameter as side-effect of depth computation -> O(n) time | O(h) space
// Link: https://leetcode.com/problems/diameter-of-binary-tree/
//
// Examples:
//   [1,2,3,4,5] -> 3 (path 4 -> 2 -> 1 -> 3 or 5 -> 2 -> 1 -> 3)
//   [1,2]       -> 1

use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
struct TreeNode {
    val: i32,
    left: Option<Rc<RefCell<TreeNode>>>,
    right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

type Node = Option<Rc<RefCell<TreeNode>>>;

fn build_tree(vals: &[Option<i32>]) -> Node {
    if vals.is_empty() || vals[0].is_none() {
        return None;
    }
    let root = Rc::new(RefCell::new(TreeNode::new(vals[0].unwrap())));
    let mut queue = std::collections::VecDeque::new();
    queue.push_back(Rc::clone(&root));
    let mut i = 1;
    while i < vals.len() {
        if let Some(node) = queue.pop_front() {
            if i < vals.len() {
                if let Some(v) = vals[i] {
                    let left = Rc::new(RefCell::new(TreeNode::new(v)));
                    node.borrow_mut().left = Some(Rc::clone(&left));
                    queue.push_back(left);
                }
                i += 1;
            }
            if i < vals.len() {
                if let Some(v) = vals[i] {
                    let right = Rc::new(RefCell::new(TreeNode::new(v)));
                    node.borrow_mut().right = Some(Rc::clone(&right));
                    queue.push_back(right);
                }
                i += 1;
            }
        }
    }
    Some(root)
}

struct Solution;

impl Solution {
    /// 1. BRUTE FORCE:
    /// For every node, compute depth of left subtree + depth of right subtree.
    /// The diameter is the maximum of all such sums.
    /// Time: O(n^2) | Space: O(h)
    pub fn diameter_brute(root: &Node) -> i32 {
        fn depth(node: &Node) -> i32 {
            match node {
                None => 0,
                Some(n) => {
                    let b = n.borrow();
                    1 + depth(&b.left).max(depth(&b.right))
                }
            }
        }

        fn solve(node: &Node) -> i32 {
            match node {
                None => 0,
                Some(n) => {
                    let b = n.borrow();
                    let through_here = depth(&b.left) + depth(&b.right);
                    let left_dia = solve(&b.left);
                    let right_dia = solve(&b.right);
                    through_here.max(left_dia).max(right_dia)
                }
            }
        }
        solve(root)
    }

    /// 2. OPTIMAL (Single DFS):
    /// Compute depth recursively. At each node, update a global max with left_depth + right_depth.
    /// Time: O(n) | Space: O(h)
    pub fn diameter_of_binary_tree(root: &Node) -> i32 {
        fn dfs(node: &Node, max_diameter: &mut i32) -> i32 {
            match node {
                None => 0,
                Some(n) => {
                    let b = n.borrow();
                    let left_depth = dfs(&b.left, max_diameter);
                    let right_depth = dfs(&b.right, max_diameter);
                    *max_diameter = (*max_diameter).max(left_depth + right_depth);
                    1 + left_depth.max(right_depth)
                }
            }
        }

        let mut diameter = 0;
        dfs(root, &mut diameter);
        diameter
    }
}

fn main() {
    // Test 1: [1,2,3,4,5] -> 3
    let tree1 = build_tree(&[Some(1), Some(2), Some(3), Some(4), Some(5)]);
    assert_eq!(Solution::diameter_brute(&tree1), 3);
    assert_eq!(Solution::diameter_of_binary_tree(&tree1), 3);

    // Test 2: [1,2] -> 1
    let tree2 = build_tree(&[Some(1), Some(2)]);
    assert_eq!(Solution::diameter_brute(&tree2), 1);
    assert_eq!(Solution::diameter_of_binary_tree(&tree2), 1);

    // Test 3: Single node -> 0
    let tree3 = build_tree(&[Some(1)]);
    assert_eq!(Solution::diameter_brute(&tree3), 0);
    assert_eq!(Solution::diameter_of_binary_tree(&tree3), 0);

    // Test 4: Skewed left chain [1,2,None,3,None,4] -> 3
    let mut n4 = TreeNode::new(4);
    let mut n3 = TreeNode::new(3);
    n3.left = Some(Rc::new(RefCell::new(n4)));
    let mut n2 = TreeNode::new(2);
    n2.left = Some(Rc::new(RefCell::new(n3)));
    let n1 = TreeNode::new(1);
    let mut root4 = TreeNode::new(0);
    root4.left = Some(Rc::new(RefCell::new(n2)));
    root4.right = Some(Rc::new(RefCell::new(n1)));
    let tree4: Node = Some(Rc::new(RefCell::new(root4)));
    assert_eq!(Solution::diameter_brute(&tree4), 4);
    assert_eq!(Solution::diameter_of_binary_tree(&tree4), 4);

    println!("All test cases passed for Diameter of Binary Tree (Brute Force O(n^2), DFS O(n))!");
}
