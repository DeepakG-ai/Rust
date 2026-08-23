// LeetCode Problem 206: Reverse Linked List
// Approaches:
//   1) Brute Force (Auxiliary Stack): Collect node values onto stack -> O(n) time | O(n) space
//   2) Better (Recursive): Reversal on recursion unwind -> O(n) time | O(n) stack space
//   3) Optimal (Iterative 3-Pointer): In-place pointer redirection -> O(n) time | O(1) space
// Link: https://leetcode.com/problems/reverse-linked-list/
//
// Examples:
//   [1,2,3,4,5] -> [5,4,3,2,1]
//   []          -> []

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
    /// 1. BRUTE FORCE (Auxiliary Stack):
    /// Push values onto a stack, then reconstruct a new reversed list.
    /// Time: O(n) | Space: O(n)
    pub fn reverse_list_stack(head: Link) -> Link {
        let mut values = Vec::new();
        let mut cur = head;
        while let Some(node) = cur {
            values.push(node.borrow().val);
            cur = node.borrow().next.clone();
        }

        let mut dummy = None;
        for v in values {
            let n = new_node(v);
            n.borrow_mut().next = dummy.take();
            dummy = Some(n);
        }
        dummy
    }

    /// 2. BETTER (Recursive):
    /// Recurse to end of list; on unwind, point next node back at us.
    /// Time: O(n) | Space: O(n) call stack
    pub fn reverse_list_recursive(head: Link) -> Link {
        match head {
            None => None,
            Some(node) => Self::rev(Some(node)).0,
        }
    }

    fn rev(head: Link) -> (Link, Link) {
        if let Some(node) = &head {
            let next = node.borrow_mut().next.take();
            if next.is_none() {
                return (Some(Rc::clone(node)), Some(Rc::clone(node)));
            }
            let (new_head, tail) = Self::rev(next);
            tail.as_ref().unwrap().borrow_mut().next = Some(Rc::clone(node));
            return (new_head, Some(Rc::clone(node)));
        }
        (None, None)
    }

    /// 3. OPTIMAL (Iterative 3-Pointer):
    /// Save next -> redirect current arrow to prev -> advance prev & curr.
    /// Time: O(n) | Space: O(1)
    pub fn reverse_list(mut head: Link) -> Link {
        let mut prev: Link = None;

        while let Some(curr) = head.take() {
            let next = curr.borrow_mut().next.take();
            curr.borrow_mut().next = prev.take();
            prev = Some(curr);
            head = next;
        }
        prev
    }
}

fn main() {
    let test_cases = vec![
        (vec![1, 2, 3, 4, 5], vec![5, 4, 3, 2, 1]),
        (vec![1, 2], vec![2, 1]),
        (vec![1], vec![1]),
        (vec![], vec![]),
    ];

    for (input, expected) in test_cases {
        assert_eq!(to_vec(&Solution::reverse_list_stack(build_list(&input))), expected);
        assert_eq!(to_vec(&Solution::reverse_list_recursive(build_list(&input))), expected);
        assert_eq!(to_vec(&Solution::reverse_list(build_list(&input))), expected);
    }

    println!("All test cases passed for Reverse Linked List (Stack, Recursive, Iterative 3-Pointer)!");
}
