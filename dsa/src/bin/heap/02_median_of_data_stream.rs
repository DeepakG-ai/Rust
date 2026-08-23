// LeetCode Problem 295: Find Median from Data Stream
// Approaches:
//   1) Better (Sorted Vector via Binary Insertion): -> O(n) insert | O(1) findMedian | O(n) space
//   2) Optimal (Two Heaps - Max-Heap for lower half, Min-Heap for upper half): -> O(log n) insert | O(1) findMedian | O(n) space
// Link: https://leetcode.com/problems/find-median-from-data-stream/
//
// Examples:
//   addNum(1), addNum(2) -> findMedian() = 1.5
//   addNum(3)            -> findMedian() = 2.0

use std::cmp::Reverse;
use std::collections::BinaryHeap;

/// 1. VECTOR WITH BINARY SEARCH INSERTION:
#[derive(Default)]
pub struct MedianFinderVec {
    nums: Vec<i32>,
}

impl MedianFinderVec {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_num(&mut self, num: i32) {
        let idx = match self.nums.binary_search(&num) {
            Ok(i) => i,
            Err(i) => i,
        };
        self.nums.insert(idx, num);
    }

    pub fn find_median(&self) -> f64 {
        let n = self.nums.len();
        if n % 2 == 1 {
            self.nums[n / 2] as f64
        } else {
            (self.nums[n / 2 - 1] as f64 + self.nums[n / 2] as f64) / 2.0
        }
    }
}

/// 2. OPTIMAL (Two Heaps):
/// small: Max-Heap storing smaller half (holds n/2 or n/2 + 1 elements)
/// large: Min-Heap storing larger half (holds n/2 elements)
#[derive(Default)]
pub struct MedianFinder {
    small: BinaryHeap<i32>,          // max-heap
    large: BinaryHeap<Reverse<i32>>, // min-heap
}

impl MedianFinder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_num(&mut self, num: i32) {
        // Step 1: Add to small max-heap
        self.small.push(num);

        // Step 2: Ensure all elements in small <= all in large
        if let (Some(&max_small), Some(&Reverse(min_large))) = (self.small.peek(), self.large.peek()) {
            if max_small > min_large {
                let val = self.small.pop().unwrap();
                self.large.push(Reverse(val));
            }
        }

        // Step 3: Balance sizes (small.len() == large.len() OR small.len() == large.len() + 1)
        if self.small.len() > self.large.len() + 1 {
            let val = self.small.pop().unwrap();
            self.large.push(Reverse(val));
        } else if self.large.len() > self.small.len() {
            let Reverse(val) = self.large.pop().unwrap();
            self.small.push(val);
        }
    }

    pub fn find_median(&self) -> f64 {
        if self.small.len() > self.large.len() {
            *self.small.peek().unwrap() as f64
        } else {
            let small_max = *self.small.peek().unwrap() as f64;
            let large_min = self.large.peek().unwrap().0 as f64;
            (small_max + large_min) / 2.0
        }
    }
}

fn main() {
    let mut mf_vec = MedianFinderVec::new();
    let mut mf = MedianFinder::new();

    let operations = vec![1, 2, 3, 4, 5, 6];
    let expected_medians = vec![1.0, 1.5, 2.0, 2.5, 3.0, 3.5];

    for (num, expected) in operations.into_iter().zip(expected_medians.into_iter()) {
        mf_vec.add_num(num);
        mf.add_num(num);

        assert!((mf_vec.find_median() - expected).abs() < 1e-5);
        assert!((mf.find_median() - expected).abs() < 1e-5);
    }

    // Test with mixed positive & negative numbers
    let mut mf2 = MedianFinder::new();
    mf2.add_num(-1);
    assert_eq!(mf2.find_median(), -1.0);
    mf2.add_num(-2);
    assert_eq!(mf2.find_median(), -1.5);
    mf2.add_num(-3);
    assert_eq!(mf2.find_median(), -2.0);
    mf2.add_num(-4);
    assert_eq!(mf2.find_median(), -2.5);
    mf2.add_num(-5);
    assert_eq!(mf2.find_median(), -3.0);

    println!("All test cases passed for Median of Data Stream (Sorted Vector, Two Heaps O(log n))!");
}
