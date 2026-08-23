// LeetCode Problem 98: Validate Binary Search Tree
// Method: DFS with (min,max) range / Inorder traversal
// Time: O(n) | Space: O(h)
//
// Range idea: root starts (-inf, +inf). LEFT child inherits max = parent,
// RIGHT child inherits min = parent. Violation anywhere -> false.
//
// Examples:
//     2          valid        5        invalid (4 < 5 in right subtree!)
//    / \                     / \
//   1   3                   1   4

use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

type Link = Option<Rc<RefCell<TreeNode>>>;

struct TreeNode {
    val: i32,
    left: Link,
    right: Link,
}

// i64 values so we can use +/- infinity bounds safely
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
            node.borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode {
                val: arr[i].unwrap(), left: None, right: None,
            })));
            if let Some(c) = &node.borrow().left { queue.push_back(c.clone()); }
        }
        i += 1;
        if i < arr.len() && arr[i].is_some() {
            node.borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode {
                val: arr[i].unwrap(), left: None, right: None,
            })));
            if let Some(c) = &node.borrow().right { queue.push_back(c.clone()); }
        }
        i += 1;
    }
    Some(root)
}

struct Solution;

impl Solution {
    /// OPTIMAL: DFS carrying an open interval (min_val, max_val).
    pub fn is_valid_bst(root: Link) -> bool {
        Self::validate(&root, i64::MIN as i128, i64::MAX as i128) // (-inf, +inf) start
    }

    fn validate(node: &Link, min_val: i128, max_val: i128) -> bool {
        match node {
            None => true, // empty subtree is valid
            Some(n) => {
                let b = n.borrow();
                // must be STRICTLY inside the range
                if (b.val as i128) <= min_val || (b.val as i128) >= max_val {
                    return false;
                }
                // left gets new MAX = current; right gets new MIN = current
                Self::validate(&b.left, min_val, b.val as i128)
                    && Self::validate(&b.right, b.val as i128, max_val)
            }
        }
    }

    /// BRUTE FORCE: inorder of a valid BST is strictly increasing.
    pub fn is_valid_bst_inorder(root: Link) -> bool {
        fn inorder(node: &Link, out: &mut Vec<i32>) {
            if let Some(n) = node {
                let b = n.borrow();
                inorder(&b.left, out);
                out.push(b.val);
                inorder(&b.right, out);
            }
        }
        let mut vals = Vec::new();
        inorder(&root, &mut vals);
        // check strictly increasing
        vals.windows(2).all(|w| w[0] < w[1])
    }
}

fn main() {
    // Test 1: valid BST [2,1,3]
    assert!(Solution::is_valid_bst(array_to_tree(&[Some(2), Some(1), Some(3)])));

    // Test 2: invalid [5,1,4,null,null,3,6] - 4 < 5 sits in right subtree
    assert!(!Solution::is_valid_bst(array_to_tree(&[
        Some(5), Some(1), Some(4), None, None, Some(3), Some(6)
    ])));

    // Test 3: valid deeper BST
    assert!(Solution::is_valid_bst(array_to_tree(&[
        Some(5), Some(3), Some(7), Some(1), Some(4)
    ])));

    // Test 4: tricky - looks valid but isn't (3 < 5 stuck in right side)
    assert!(!Solution::is_valid_bst(array_to_tree(&[
        Some(5), Some(4), Some(6), None, None, Some(3), Some(7)
    ])));

    // inorder variant spot-checks
    assert!(Solution::is_valid_bst_inorder(array_to_tree(&[Some(2), Some(1), Some(3)])));
    assert!(!Solution::is_valid_bst_inorder(array_to_tree(&[
        Some(5), Some(1), Some(4), None, None, Some(3), Some(6)
    ])));

    println!("All test cases passed!");
}
