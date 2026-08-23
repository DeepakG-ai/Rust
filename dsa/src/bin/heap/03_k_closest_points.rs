// LeetCode Problem 973: K Closest Points to Origin
// Approaches:
//   1) Better (Sorting by Euclidean Distance): -> O(n log n) time | O(1) space
//   2) Better (Max-Heap of Size K): -> O(n log k) time | O(k) space
//   3) Optimal (Quickselect): -> O(n) average time | O(1) space
// Link: https://leetcode.com/problems/k-closest-points-to-origin/
//
// Examples:
//   points = [[1,3],[-2,2]], k = 1 -> [[-2,2]]
//   points = [[3,3],[5,-1],[-2,4]], k = 2 -> [[3,3],[-2,4]]

use std::collections::BinaryHeap;

struct Solution;

impl Solution {
    /// 1. SORTING:
    /// Time: O(n log n) | Space: O(1)
    pub fn k_closest_sort(mut points: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        points.sort_unstable_by_key(|p| p[0] * p[0] + p[1] * p[1]);
        points.truncate(k as usize);
        points
    }

    /// 2. MAX-HEAP OF SIZE K:
    /// Store tuples of (distance, point) in a Max-Heap. When size > k, pop furthest point.
    /// Time: O(n log k) | Space: O(k)
    pub fn k_closest_heap(points: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        let k = k as usize;
        let mut max_heap: BinaryHeap<(i32, Vec<i32>)> = BinaryHeap::with_capacity(k);

        for p in points {
            let dist = p[0] * p[0] + p[1] * p[1];
            max_heap.push((dist, p));
            if max_heap.len() > k {
                max_heap.pop();
            }
        }

        max_heap.into_iter().map(|(_, p)| p).collect()
    }

    /// 3. OPTIMAL (Quickselect):
    /// In-place partition until k-th element is in place.
    /// Time: O(n) average | Space: O(1)
    pub fn k_closest(mut points: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        let k = k as usize;
        let n = points.len();
        if n <= k {
            return points;
        }

        let (mut left, mut right) = (0, n - 1);
        while left < right {
            let pivot_idx = Self::partition(&mut points, left, right);
            if pivot_idx == k {
                break;
            } else if pivot_idx < k {
                left = pivot_idx + 1;
            } else {
                right = pivot_idx.saturating_sub(1);
            }
        }

        points.truncate(k);
        points
    }

    fn partition(points: &mut [Vec<i32>], left: usize, right: usize) -> usize {
        let pivot_dist = points[right][0] * points[right][0] + points[right][1] * points[right][1];
        let mut i = left;

        for j in left..right {
            let dist = points[j][0] * points[j][0] + points[j][1] * points[j][1];
            if dist <= pivot_dist {
                points.swap(i, j);
                i += 1;
            }
        }
        points.swap(i, right);
        i
    }
}

fn main() {
    let test_cases = vec![
        (vec![vec![1, 3], vec![-2, 2]], 1, vec![vec![-2, 2]]),
        (
            vec![vec![3, 3], vec![5, -1], vec![-2, 4]],
            2,
            vec![vec![3, 3], vec![-2, 4]],
        ),
        (vec![vec![0, 1], vec![1, 0]], 2, vec![vec![0, 1], vec![1, 0]]),
    ];

    for (points, k, expected) in test_cases {
        let mut ans1 = Solution::k_closest_sort(points.clone(), k);
        ans1.sort_unstable();
        let mut ans2 = Solution::k_closest_heap(points.clone(), k);
        ans2.sort_unstable();
        let mut ans3 = Solution::k_closest(points, k);
        ans3.sort_unstable();

        let mut exp_sorted = expected;
        exp_sorted.sort_unstable();

        assert_eq!(ans1, exp_sorted);
        assert_eq!(ans2, exp_sorted);
        assert_eq!(ans3, exp_sorted);
    }

    println!("All test cases passed for K Closest Points to Origin (Sort, Max-Heap, Quickselect O(n))!");
}
