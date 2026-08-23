// LeetCode Problem 2: Add Two Numbers
// Method: Linked list traversal + carry
// Time: O(max(m, n)) | Space: O(max(m, n)) for output
//
// Numbers are stored in REVERSE order (least significant digit first).
//
// Example:
//   2 -> 4 -> 3  (342) +  5 -> 6 -> 4  (465)  =  7 -> 0 -> 8  (807)

use std::cell::RefCell;
use std::rc::Rc;

type Link = Option<Rc<RefCell<ListNode>>>;

#[derive(Debug)]
struct ListNode {
    val: i32,
    next: Link,
}

impl ListNode {
    fn new(val: i32) -> Rc<RefCell<ListNode>> {
        Rc::new(RefCell::new(ListNode { val, next: None }))
    }
}

/// Build list from digits (helper for tests, like Python's create_list)
fn build_list(vals: &[i32]) -> Link {
    let mut head: Link = None;
    for &v in vals.iter().rev() {
        // insert at front -> keeps order
        let node = ListNode::new(v);
        node.borrow_mut().next = head.take();
        head = Some(node);
    }
    head
}

/// Convert list back to Vec for easy printing/asserting
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
    pub fn add_two_numbers(
        l1: Option<Rc<RefCell<ListNode>>>,
        l2: Option<Rc<RefCell<ListNode>>>,
    ) -> Option<Rc<RefCell<ListNode>>> {
        // Dummy node simplifies edge cases (same trick as Python)
        let dummy = ListNode::new(0);
        let mut current = dummy.clone();
        let mut p1 = l1;
        let mut p2 = l2;
        let mut carry = 0i32;

        while p1.is_some() || p2.is_some() || carry != 0 {
            let v1 = p1.as_ref().map_or(0, |n| n.borrow().val);
            let v2 = p2.as_ref().map_or(0, |n| n.borrow().val);
            let total = v1 + v2 + carry;

            carry = total / 10;
            let new_node = ListNode::new(total % 10); // new digit node
            current.borrow_mut().next = Some(new_node.clone());
            current = new_node;

            p1 = p1.and_then(|n| n.borrow().next.clone());
            p2 = p2.and_then(|n| n.borrow().next.clone());
        }

        let result = dummy.borrow().next.clone(); // copy out before dummy drops
        result
    }
}

fn main() {
    // 342 + 465 = 807
    let a = build_list(&[2, 4, 3]);
    let b = build_list(&[5, 6, 4]);
    assert_eq!(to_vec(&Solution::add_two_numbers(a, b)), vec![7, 0, 8]);

    // 0 + 0 = 0
    assert_eq!(
        to_vec(&Solution::add_two_numbers(build_list(&[0]), build_list(&[0]))),
        vec![0]
    );

    // 9999999 + 9999 = 10009998
    let c = Solution::add_two_numbers(build_list(&[9, 9, 9, 9, 9, 9, 9]), build_list(&[9, 9, 9, 9]));
    assert_eq!(to_vec(&c), vec![8, 9, 9, 9, 0, 0, 0, 1]);

    println!("All test cases passed!");
}

// NOTE: LeetCode's Rust template for PLAIN linked lists (this problem) hands
// you Option<Box<ListNode>>. We use Option<Rc<RefCell<ListNode>>> here so
// the same node type also works for cycle/intersection problems and local
// test builders. The algorithm itself is identical - swap the wrapper types
// when pasting into LeetCode.
