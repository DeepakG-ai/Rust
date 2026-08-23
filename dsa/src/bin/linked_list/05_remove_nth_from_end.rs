// LeetCode Problem 19: Remove Nth Node From End of List
// Approaches:
//   1) Better (Two-Pass): Count total length L, advance to L - n -> O(n) time | O(1) space
//   2) Optimal (One-Pass Two Pointers with Gap n): Fast moves n steps ahead, then walk together -> O(n) time | O(1) space
// Link: https://leetcode.com/problems/remove-nth-node-from-end-of-list/
//
// Examples:
//   1->2->3->4->5, n=2 -> 1->2->3->5
//   [1], n=1           -> []

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
    /// 1. BETTER (Two-Pass):
    /// Pass 1: Count total nodes (length).
    /// Pass 2: Advance to index (length - n - 1) and skip the target node.
    /// Time: O(n) (2 passes) | Space: O(1)
    pub fn remove_nth_from_end_two_pass(head: Link, n: i32) -> Link {
        let mut length = 0i32;
        let mut cur = head.clone();
        while let Some(node) = cur {
            length += 1;
            cur = node.borrow().next.clone();
        }

        if length == n {
            return head.and_then(|h| h.borrow().next.clone());
        }

        let cur = match &head {
            Some(h) => Rc::clone(h),
            None => return None,
        };

        let mut walker = cur;
        for _ in 0..(length - n - 1) {
            let next = walker.borrow().next.clone().expect("bad index");
            walker = next;
        }

        let target = walker.borrow().next.clone();
        if let Some(t) = target {
            walker.borrow_mut().next = t.borrow().next.clone();
        }

        head
    }

    /// 2. OPTIMAL (One-Pass with Dummy Node and Two Pointers):
    /// Place a dummy node before head. Advance right pointer n + 1 steps ahead of left.
    /// When right falls off the end, left is sitting immediately before the node to remove.
    /// Time: O(n) (1 pass) | Space: O(1)
    pub fn remove_nth_from_end(head: Link, n: i32) -> Link {
        let dummy = new_node(0);
        dummy.borrow_mut().next = head.clone();

        let mut left = Rc::clone(&dummy);
        let mut right = Some(Rc::clone(&dummy));

        // Advance right pointer n + 1 steps
        for _ in 0..=n {
            right = right.and_then(|r| r.borrow().next.clone());
        }

        // Move both until right is None
        while right.is_some() {
            let l_next = left.borrow().next.clone().unwrap();
            left = l_next;
            right = right.unwrap().borrow().next.clone();
        }

        // Unlink target node
        let target = left.borrow().next.clone();
        if let Some(t) = target {
            left.borrow_mut().next = t.borrow().next.clone();
        }

        let result = dummy.borrow().next.clone();
        result
    }
}

fn main() {
    let test_cases = vec![
        (vec![1, 2, 3, 4, 5], 2, vec![1, 2, 3, 5]),
        (vec![1], 1, vec![]),
        (vec![1, 2], 1, vec![1]),
        (vec![1, 2], 2, vec![2]),
        (vec![1, 2, 3], 3, vec![2, 3]),
    ];

    for (vals, n, expected) in test_cases {
        assert_eq!(
            to_vec(&Solution::remove_nth_from_end_two_pass(build_list(&vals), n)),
            expected
        );
        assert_eq!(
            to_vec(&Solution::remove_nth_from_end(build_list(&vals), n)),
            expected
        );
    }

    println!("All test cases passed for Remove Nth Node From End (Two-Pass, One-Pass Fast/Slow)!");
}
