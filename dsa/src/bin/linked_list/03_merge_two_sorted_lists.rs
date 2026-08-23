// LeetCode Problem 21: Merge Two Sorted Lists
// Approaches:
//   1) Brute Force: Collect all values to vector, sort, and reconstruct -> O((n+m) log(n+m)) time | O(n+m) space
//   2) Better (Recursive): Merge recursively -> O(n+m) time | O(n+m) call stack
//   3) Optimal (Iterative with Dummy Node): Splice smaller head each step -> O(n+m) time | O(1) space
// Link: https://leetcode.com/problems/merge-two-sorted-lists/
//
// Example:
//   1->2->4 + 1->3->4 = 1->1->2->3->4->4

use std::cell::RefCell;
use std::rc::Rc;

type Link = Option<Rc<RefCell<ListNode>>>;

struct ListNode {
    val: i32,
    next: Link,
}

fn new_node(val: i32) -> Rc<RefCell<ListNode>> {
    Rc::new(RefCell::new(ListNode { val, next: None }))
}

fn build_list(vals: &[i32]) -> Link {
    let mut head: Link = None;
    for &v in vals.iter().rev() {
        let n = new_node(v);
        n.borrow_mut().next = head.take();
        head = Some(n);
    }
    head
}

fn to_vec(head: &Link) -> Vec<i32> {
    let mut out = Vec::new();
    let mut cur = head.clone();
    while let Some(node) = cur {
        let b = node.borrow();
        out.push(b.val);
        cur = b.next.clone();
    }
    out
}

struct Solution;

impl Solution {
    /// 1. BRUTE FORCE:
    /// Collect all values from both lists into a vector, sort, and reconstruct a new list.
    /// Time: O((n + m) log(n + m)) | Space: O(n + m)
    pub fn merge_two_lists_brute_force(l1: Link, l2: Link) -> Link {
        let mut vals = to_vec(&l1);
        vals.extend(to_vec(&l2));
        vals.sort_unstable();
        build_list(&vals)
    }

    /// 2. BETTER (Recursive):
    /// Match smaller head, and recursively link the rest.
    /// Time: O(n + m) | Space: O(n + m) call stack
    pub fn merge_two_lists_rec(l1: Link, l2: Link) -> Link {
        match (l1, l2) {
            (None, b) => b,
            (a, None) => a,
            (Some(x), Some(y)) => {
                if x.borrow().val <= y.borrow().val {
                    let rest = x.borrow_mut().next.take();
                    x.borrow_mut().next = Self::merge_two_lists_rec(rest, Some(y));
                    Some(x)
                } else {
                    let rest = y.borrow_mut().next.take();
                    y.borrow_mut().next = Self::merge_two_lists_rec(Some(x), rest);
                    Some(y)
                }
            }
        }
    }

    /// 3. OPTIMAL (Iterative with Dummy Node):
    /// Walk both lists, splicing the smaller head each step without node re-allocations.
    /// Time: O(n + m) | Space: O(1)
    pub fn merge_two_lists(l1: Link, l2: Link) -> Link {
        let dummy = new_node(0);
        let mut tail = dummy.clone();

        let mut a = l1;
        let mut b = l2;
        while let (Some(x), Some(y)) = (a.clone(), b.clone()) {
            if x.borrow().val <= y.borrow().val {
                let next = x.borrow_mut().next.take();
                tail.borrow_mut().next = Some(x.clone());
                tail = x;
                a = next;
            } else {
                let next = y.borrow_mut().next.take();
                tail.borrow_mut().next = Some(y.clone());
                tail = y;
                b = next;
            }
        }

        let rest = if a.is_some() { a } else { b };
        tail.borrow_mut().next = rest;

        let result = dummy.borrow().next.clone();
        result
    }
}

fn main() {
    let test_cases = vec![
        (vec![1, 2, 4], vec![1, 3, 4], vec![1, 1, 2, 3, 4, 4]),
        (vec![], vec![], vec![]),
        (vec![], vec![0], vec![0]),
        (vec![1, 3, 5, 7], vec![2, 4], vec![1, 2, 3, 4, 5, 7]),
        (vec![5], vec![1, 2, 3], vec![1, 2, 3, 5]),
    ];

    for (l1_vals, l2_vals, expected) in test_cases {
        assert_eq!(
            to_vec(&Solution::merge_two_lists_brute_force(build_list(&l1_vals), build_list(&l2_vals))),
            expected
        );
        assert_eq!(
            to_vec(&Solution::merge_two_lists_rec(build_list(&l1_vals), build_list(&l2_vals))),
            expected
        );
        assert_eq!(
            to_vec(&Solution::merge_two_lists(build_list(&l1_vals), build_list(&l2_vals))),
            expected
        );
    }

    println!("All test cases passed for Merge Two Sorted Lists (Brute Force, Recursive, Iterative Splice)!");
}
