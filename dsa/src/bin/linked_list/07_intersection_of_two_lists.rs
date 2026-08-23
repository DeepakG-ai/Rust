// LeetCode Problem 160: Intersection of Two Linked Lists
// Approaches:
//   1) Brute Force: Nested loop pointer comparison -> O(n * m) time | O(1) space
//   2) Better (HashSet of Addresses): Store nodes of list A, check with B -> O(n + m) time | O(n) space
//   3) Optimal (Two-Pointer Switcheroo): Both walk (lenA + lenB) total -> O(n + m) time | O(1) space
// Link: https://leetcode.com/problems/intersection-of-two-linked-lists/
//
// Example:
//   A: 4->1 -> 8->4->5      intersection at node 8
//   B: 5->6->1 -> 8->4->5

use std::cell::RefCell;
use std::collections::HashSet;
use std::rc::Rc;

type Link = Option<Rc<RefCell<ListNode>>>;

struct ListNode {
    val: i32,
    next: Link,
}

fn new_node(val: i32) -> Rc<RefCell<ListNode>> {
    Rc::new(RefCell::new(ListNode { val, next: None }))
}

fn build_intersecting(
    vals_a: &[i32],
    vals_b: &[i32],
    intersect_vals: &[i32],
) -> (Link, Link, Link) {
    let mut shared: Link = None;
    for &v in intersect_vals.iter().rev() {
        let n = new_node(v);
        n.borrow_mut().next = shared.take();
        shared = Some(n);
    }

    fn attach(prefix: &[i32], tail: &Link) -> Link {
        let mut head: Link = None;
        for &v in prefix.iter().rev() {
            let n = new_node(v);
            n.borrow_mut().next = head.take();
            head = Some(n);
        }
        match head {
            None => tail.clone(),
            Some(h) => {
                let mut cur = Rc::clone(&h);
                loop {
                    let next = cur.borrow().next.clone();
                    match next {
                        Some(n) => cur = n,
                        None => break,
                    }
                }
                cur.borrow_mut().next = tail.clone();
                Some(h)
            }
        }
    }

    let a = attach(vals_a, &shared);
    let b = attach(vals_b, &shared);
    (a, b, shared)
}

struct Solution;

impl Solution {
    /// 1. BRUTE FORCE: For every node in A, compare pointer equality against every node in B.
    /// Time: O(n * m) | Space: O(1)
    pub fn get_intersection_node_brute_force(a: Link, b: Link) -> Link {
        let mut cur_a = a;
        while let Some(node_a) = cur_a {
            let mut cur_b = b.clone();
            while let Some(node_b) = cur_b {
                if Rc::ptr_eq(&node_a, &node_b) {
                    return Some(node_a);
                }
                cur_b = node_b.borrow().next.clone();
            }
            cur_a = node_a.borrow().next.clone();
        }
        None
    }

    /// 2. BETTER (HashSet of Node Addresses):
    /// Store all node pointers of list A in a HashSet, then check nodes of list B.
    /// Time: O(n + m) | Space: O(n)
    pub fn get_intersection_node_hashset(a: Link, b: Link) -> Link {
        let mut seen: HashSet<usize> = HashSet::new();
        let mut cur = a;
        while let Some(node) = cur {
            seen.insert(Rc::as_ptr(&node) as usize);
            cur = node.borrow().next.clone();
        }

        let mut cur = b;
        while let Some(node) = cur {
            if seen.contains(&(Rc::as_ptr(&node) as usize)) {
                return Some(node);
            }
            cur = node.borrow().next.clone();
        }
        None
    }

    /// 3. OPTIMAL (Two Pointers Switcheroo):
    /// Pointer pa walks A then B; pb walks B then A.
    /// Both travel exactly (lenA + lenB) distance and align precisely at intersection or end (None).
    /// Time: O(n + m) | Space: O(1)
    pub fn get_intersection_node(a: Link, b: Link) -> Link {
        let (mut pa, mut pb) = (a.clone(), b.clone());

        loop {
            if pa.is_none() && pb.is_none() {
                return None;
            }
            if let (Some(x), Some(y)) = (&pa, &pb) {
                if Rc::ptr_eq(x, y) {
                    return pa.clone();
                }
            }
            pa = match &pa {
                Some(x) => x.borrow().next.clone(),
                None => b.clone(),
            };
            pb = match &pb {
                Some(y) => y.borrow().next.clone(),
                None => a.clone(),
            };
        }
    }
}

fn main() {
    // Test 1: intersect at 8
    let (a1, b1, shared1) = build_intersecting(&[4, 1], &[5, 6, 1], &[8, 4, 5]);
    let ans_val = Solution::get_intersection_node(a1.clone(), b1.clone()).map(|n| n.borrow().val);
    let ans_val_bf = Solution::get_intersection_node_brute_force(a1.clone(), b1.clone()).map(|n| n.borrow().val);
    let ans_val_hs = Solution::get_intersection_node_hashset(a1, b1).map(|n| n.borrow().val);
    let expect_val = shared1.map(|s| s.borrow().val);
    assert_eq!(ans_val, expect_val);
    assert_eq!(ans_val_bf, expect_val);
    assert_eq!(ans_val_hs, expect_val);
    assert_eq!(ans_val, Some(8));

    // Test 2: no intersection
    let (a2, b2, _) = build_intersecting(&[2, 6, 4], &[1, 5], &[]);
    assert!(Solution::get_intersection_node(a2.clone(), b2.clone()).is_none());
    assert!(Solution::get_intersection_node_brute_force(a2.clone(), b2.clone()).is_none());
    assert!(Solution::get_intersection_node_hashset(a2, b2).is_none());

    // Test 3: identical lists
    let (a3, b3, _) = build_intersecting(&[], &[], &[1, 2, 3]);
    assert_eq!(
        Solution::get_intersection_node(a3.clone(), b3.clone()).map(|n| n.borrow().val),
        Some(1)
    );
    assert_eq!(
        Solution::get_intersection_node_brute_force(a3.clone(), b3.clone()).map(|n| n.borrow().val),
        Some(1)
    );
    assert_eq!(
        Solution::get_intersection_node_hashset(a3, b3).map(|n| n.borrow().val),
        Some(1)
    );

    println!("All test cases passed for Intersection of Two Lists (Brute Force, HashSet, Two Pointers)!");
}
