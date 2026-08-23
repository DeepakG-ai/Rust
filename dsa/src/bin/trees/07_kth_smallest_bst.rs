// LeetCode Problem 230: Kth Smallest Element in a BST
// Method: Inorder traversal with early stop (+ full collect + iterative stack)
// Time: O(H + k) | Space: O(H)
//
// BST inorder = sorted order, so just count k nodes during traversal.
//
// Tree:        5
//             / \
//            3   6
//           / \
//          2   4
//         /
//        1
// Inorder: [1,2,3,4,5,6];  k=3 -> 3

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
    /// OPTIMAL: recursive inorder with a counter; stops after k-th visit.
    pub fn kth_smallest(root: Link, k: i32) -> i32 {
        // Cell lets the recursion share counter & result without a struct field
        let state = std::cell::Cell::new((k, -1)); // (remaining_k, answer)
        Self::inorder(&root, &state);
        state.get().1
    }

    fn inorder(node: &Link, state: &std::cell::Cell<(i32, i32)>) {
        if node.is_none() {
            return;
        }
        let n = node.as_ref().unwrap().clone();
        let b = n.borrow();

        Self::inorder(&b.left, state); // smaller elements first

        let (k_left, ans) = state.get();
        if ans != -1 {
            return; // early stop: already found
        }
        let new_k = k_left - 1;
        if new_k == 0 {
            state.set((0, b.val)); // k-th smallest found!
        } else {
            state.set((new_k, -1));
        }

        if state.get().1 == -1 {
            Self::inorder(&b.right, state); // larger elements
        }
    }

    /// ITERATIVE with explicit stack (classic controlled descent).
    pub fn kth_smallest_iterative(root: Link, mut k: i32) -> i32 {
        let mut stack: Vec<Rc<RefCell<TreeNode>>> = Vec::new();
        let mut current = root;

        while !stack.is_empty() || current.is_some() {
            // slide all the way left
            while let Some(cur) = current.clone() {
                stack.push(cur.clone());
                current = cur.borrow().left.clone();
            }
            // visit the smallest unvisited node
            let node = stack.pop().unwrap();
            k -= 1;
            if k == 0 {
                return node.borrow().val;
            }
            current = node.borrow().right.clone(); // then go right
        }
        -1 // unreachable for valid input
    }

    /// SIMPLE: collect everything, index in. O(n)
    pub fn kth_smallest_collect(root: Link, k: i32) -> i32 {
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
        vals[(k - 1) as usize]
    }
}

fn main() {
    // [3,1,4,null,2], k=1 -> 1
    let t1 = array_to_tree(&[Some(3), Some(1), Some(4), None, Some(2)]);
    assert_eq!(Solution::kth_smallest(t1.clone(), 1), 1);
    assert_eq!(Solution::kth_smallest_collect(t1.clone(), 1), 1);

    // bigger tree, k=3 -> 3
    let t2 = array_to_tree(&[
        Some(5), Some(3), Some(6), Some(2), Some(4), None, None, Some(1),
    ]);
    assert_eq!(Solution::kth_smallest(t2.clone(), 3), 3);
    assert_eq!(Solution::kth_smallest_iterative(t2.clone(), 4), 4);
    assert_eq!(Solution::kth_smallest(t1, 4), 4); // largest element

    println!("All test cases passed!");
}
