// LeetCode Problem 23: Merge K Sorted Lists
// Approaches:
//   1) Brute Force: Collect all values, sort, and reconstruct -> O(N log N) time | O(N) space
//   2) Better (Sequential Pairwise Merge): Merge list after list -> O(k * N) time | O(1) space
//   3) Optimal (Divide & Conquer Merge): Pairwise merge in log k rounds -> O(N log k) time | O(1) space
// Link: https://leetcode.com/problems/merge-k-sorted-lists/
//
// Example:
//   [[1,4,5],[1,3,4],[2,6]] -> [1,1,2,3,4,4,5,6]

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

fn merge_two(l1: Link, l2: Link) -> Link {
    let dummy = new_node(0);
    let mut tail = dummy.clone();
    let (mut a, mut b) = (l1, l2);

    while a.is_some() && b.is_some() {
        let x = a.clone().unwrap();
        let y = b.clone().unwrap();
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
    tail.borrow_mut().next = if a.is_some() { a } else { b };
    let result = dummy.borrow().next.clone();
    result
}

struct Solution;

impl Solution {
    /// 1. BRUTE FORCE:
    /// Collect all values into a single Vec, sort, and reconstruct a new list.
    /// Time: O(N log N) where N is total number of nodes | Space: O(N)
    pub fn merge_k_lists_brute(lists: Vec<Link>) -> Link {
        let mut vals = Vec::new();
        for list in lists {
            vals.extend(to_vec(&list));
        }
        vals.sort_unstable();
        build_list(&vals)
    }

    /// 2. BETTER (Sequential Pairwise Merging):
    /// Start with first list and fold in each subsequent list.
    /// Time: O(k * N) | Space: O(1)
    pub fn merge_k_lists_sequential(lists: Vec<Link>) -> Link {
        let mut iter = lists.into_iter();
        let mut result = iter.next().unwrap_or(None);
        for l in iter {
            result = merge_two(result, l);
        }
        result
    }

    /// 3. OPTIMAL (Divide & Conquer):
    /// In each round, merge pairs (0, 1), (2, 3), etc. Total log k rounds.
    /// Time: O(N log k) | Space: O(1) auxiliary
    pub fn merge_k_lists(mut lists: Vec<Link>) -> Link {
        if lists.is_empty() {
            return None;
        }

        while lists.len() > 1 {
            let mut merged_round: Vec<Link> = Vec::new();
            for i in (0..lists.len()).step_by(2) {
                let l1 = lists[i].take();
                let l2 = if i + 1 < lists.len() { lists[i + 1].take() } else { None };
                merged_round.push(merge_two(l1, l2));
            }
            lists = merged_round;
        }
        lists.into_iter().next().flatten()
    }
}

fn main() {
    let test_cases = vec![
        (
            vec![vec![1, 4, 5], vec![1, 3, 4], vec![2, 6]],
            vec![1, 1, 2, 3, 4, 4, 5, 6],
        ),
        (vec![], vec![]),
        (vec![vec![]], vec![]),
        (vec![vec![1, 2], vec![3]], vec![1, 2, 3]),
        (vec![vec![2], vec![], vec![-1]], vec![-1, 2]),
    ];

    for (input, expected) in test_cases {
        let lists1: Vec<Link> = input.iter().map(|v| build_list(v)).collect();
        let lists2: Vec<Link> = input.iter().map(|v| build_list(v)).collect();
        let lists3: Vec<Link> = input.iter().map(|v| build_list(v)).collect();

        assert_eq!(to_vec(&Solution::merge_k_lists_brute(lists1)), expected);
        assert_eq!(to_vec(&Solution::merge_k_lists_sequential(lists2)), expected);
        assert_eq!(to_vec(&Solution::merge_k_lists(lists3)), expected);
    }

    println!("All test cases passed for Merge K Sorted Lists (Brute Force, Sequential, Divide & Conquer)!");
}
