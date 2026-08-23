// LeetCode Problem 141: Linked List Cycle
// Method: Floyd's Tortoise & Hare (+ HashSet brute force)
// Time: O(n) | Space: O(1) floyd, O(n) hashset
//
// Slow moves 1 step, fast moves 2. On a circular track the fast runner
// must eventually LAP the slow one -> they meet.
//
// Examples:
//   3->2->0->-4 with tail pointing back to index 1 -> true
//   1->2->3 (no cycle)                             -> false

use std::cell::RefCell;
use std::collections::HashSet;
use std::rc::Rc;

#[allow(dead_code)]
struct ListNode {
    val: i32,
    next: Option<Rc<RefCell<ListNode>>>,
}

/// Build list of `vals`; if pos = Some(p), tail points back to node p (cycle).
fn build_cycle_list(vals: &[i32], pos: Option<usize>) -> Option<Rc<RefCell<ListNode>>> {
    if vals.is_empty() {
        return None;
    }
    let nodes: Vec<Rc<RefCell<ListNode>>> = vals
        .iter()
        .map(|&v| Rc::new(RefCell::new(ListNode { val: v, next: None })))
        .collect();
    for w in nodes.windows(2) {
        w[0].borrow_mut().next = Some(Rc::clone(&w[1]));
    }
    if let Some(p) = pos {
        let last = Rc::clone(nodes.last().unwrap());
        last.borrow_mut().next = Some(Rc::clone(&nodes[p])); // create the cycle
    }
    Some(Rc::clone(&nodes[0]))
}

struct Solution;

impl Solution {
    /// OPTIMAL: tortoise & hare. O(n) time, O(1) space.
    /// Mirrors the Python: `while fast and fast.next:`
    pub fn has_cycle(head: Option<Rc<RefCell<ListNode>>>) -> bool {
        let mut slow = head.clone();
        let mut fast = head;

        // loop only while fast and fast.next are alive
        while fast.is_some() && fast.as_ref().unwrap().borrow().next.is_some() {
            slow = slow.unwrap().borrow().next.clone(); // +1 step

            let f = fast.clone().unwrap(); // +2 steps
            fast = f.borrow().next.clone().and_then(|n| n.borrow().next.clone());

            if let (Some(s), Some(f)) = (slow.as_ref(), fast.as_ref()) {
                if Rc::ptr_eq(s, f) {
                    return true; // they met -> cycle exists
                }
            }
        }
        false // fast reached NULL -> no cycle
    }

    /// BRUTE FORCE: remember every visited node by memory address.
    pub fn has_cycle_hashset(head: Option<Rc<RefCell<ListNode>>>) -> bool {
        let mut visited: HashSet<usize> = HashSet::new();
        let mut cur = head;
        while let Some(node) = cur {
            let addr = Rc::as_ptr(&node) as usize;
            if !visited.insert(addr) {
                return true; // same node seen twice -> cycle
            }
            cur = node.borrow().next.clone();
        }
        false
    }
}

fn main() {
    // Test 1: cycle at pos 1: 3->2->0->-4->(back to node "2")
    assert!(Solution::has_cycle(build_cycle_list(&[3, 2, 0, -4], Some(1))));

    // Test 2: no cycle (odd length)
    assert!(!Solution::has_cycle(build_cycle_list(&[1, 2, 3], None)));

    // Test 3: no cycle (even length - previously caused unwrap panic!)
    assert!(!Solution::has_cycle(build_cycle_list(&[1, 2], None)));
    assert!(!Solution::has_cycle(build_cycle_list(&[1, 2, 3, 4], None)));

    // Test 4: single node without cycle
    assert!(!Solution::has_cycle(build_cycle_list(&[7], None)));

    // hashset variant
    assert!(Solution::has_cycle_hashset(build_cycle_list(&[3, 2, 0, -4], Some(1))));
    assert!(!Solution::has_cycle_hashset(build_cycle_list(&[1], None)));
    assert!(!Solution::has_cycle_hashset(build_cycle_list(&[1, 2], None)));

    println!("All test cases passed!");
}
