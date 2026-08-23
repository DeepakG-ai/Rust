// LeetCode Problem 124: Binary Tree Maximum Path Sum (HARD)
// Method: DFS with global max tracking
// Time: O(n) | Space: O(h) recursion
//
// At each node compute:
//   local_max = val + left_gain + right_gain  (path THROUGH node, both arms)
//   return    = val + max(left_gain, right_gain, 0) (one arm only, to parent)
// max(0, gain): ignore negative contributions entirely.
//
// Examples:
//   [1,2,3]                    -> 6   (2->1->3)
//   [-10,9,20,null,null,15,7]  -> 42  (15->20->7)

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
    pub fn max_path_sum(root: Link) -> i32 {
        // "global" maximum lives in a Cell so recursion can update it
        let max_sum = std::cell::Cell::new(i32::MIN);
        Self::dfs(&root, &max_sum);
        max_sum.get()
    }

    /// Returns the max GAIN this subtree gives its parent (one direction),
    /// updating `max_sum` with the best path found anywhere.
    fn dfs(node: &Link, max_sum: &std::cell::Cell<i32>) -> i32 {
        match node {
            None => 0,
            Some(n) => {
                let b = n.borrow();
                // ignore negative gains with max(0, ...)
                let left_gain = Self::dfs(&b.left, max_sum).max(0);
                let right_gain = Self::dfs(&b.right, max_sum).max(0);

                // path THROUGH this node uses BOTH children
                let local_max = b.val + left_gain + right_gain;
                max_sum.set(max_sum.get().max(local_max));

                // to parent we can only extend ONE arm
                b.val + left_gain.max(right_gain)
            }
        }
    }
}

fn main() {
    assert_eq!(Solution::max_path_sum(array_to_tree(&[Some(1), Some(2), Some(3)])), 6);

    // -10 tree -> best is 15->20->7 = 42
    assert_eq!(
        Solution::max_path_sum(array_to_tree(&[
            Some(-10), Some(9), Some(20), None, None, Some(15), Some(7)
        ])),
        42
    );

    // single negative node -> answer can be negative!
    assert_eq!(Solution::max_path_sum(array_to_tree(&[Some(-3)])), -3);

    // all negative -> pick the least negative single node
    assert_eq!(Solution::max_path_sum(array_to_tree(&[Some(-1), Some(-2), Some(-3)])), -1);

    println!("All test cases passed!");
}
