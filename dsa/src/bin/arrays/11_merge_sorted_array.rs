// LeetCode Problem 88: Merge Sorted Array
// Approaches:
//   1) Brute Force: Copy nums2 to nums1[m..] and sort -> O((m+n) log(m+n)) time | O(1) space
//   2) Better (Auxiliary Buffer): Copy real elements of nums1, merge into nums1 -> O(m+n) time | O(m) space
//   3) Optimal (Three Pointers Backwards): Merge from the back in-place -> O(m+n) time | O(1) space
// Link: https://leetcode.com/problems/merge-sorted-array/
//
// Examples:
//   [1,2,3,0,0,0], m=3, [2,5,6], n=3 -> [1,2,2,3,5,6]
//   [1],           m=1, [],      n=0 -> [1]
//   [0],           m=0, [1],     n=1 -> [1]

struct Solution;

impl Solution {
    /// 1. BRUTE FORCE: Append nums2 directly into the vacant tail slots of nums1, then sort.
    /// Time: O((m + n) log(m + n)) | Space: O(1)
    pub fn merge_brute_force(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        for i in 0..n as usize {
            nums1[m as usize + i] = nums2[i];
        }
        nums1.sort_unstable();
    }

    /// 2. BETTER (Auxiliary Copy):
    /// Clone the first m elements of nums1, then use two pointers to merge nums1_copy and nums2 into nums1.
    /// Time: O(m + n) | Space: O(m)
    pub fn merge_extra_space(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        let nums1_copy = nums1[0..m as usize].to_vec();
        let (mut p1, mut p2, mut p) = (0usize, 0usize, 0usize);

        while p1 < m as usize && p2 < n as usize {
            if nums1_copy[p1] <= nums2[p2] {
                nums1[p] = nums1_copy[p1];
                p1 += 1;
            } else {
                nums1[p] = nums2[p2];
                p2 += 1;
            }
            p += 1;
        }

        while p1 < m as usize {
            nums1[p] = nums1_copy[p1];
            p1 += 1;
            p += 1;
        }

        while p2 < n as usize {
            nums1[p] = nums2[p2];
            p2 += 1;
            p += 1;
        }
    }

    /// 3. OPTIMAL (Three Pointers Backwards In-Place):
    /// Place the largest available element at the end of nums1 (index p = m + n - 1).
    /// Prevents overwriting elements in nums1 that haven't been read yet.
    /// Time: O(m + n) | Space: O(1)
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        let mut p1 = m - 1;
        let mut p2 = n - 1;
        let mut p = m + n - 1;

        while p1 >= 0 && p2 >= 0 {
            if nums1[p1 as usize] > nums2[p2 as usize] {
                nums1[p as usize] = nums1[p1 as usize];
                p1 -= 1;
            } else {
                nums1[p as usize] = nums2[p2 as usize];
                p2 -= 1;
            }
            p -= 1;
        }

        while p2 >= 0 {
            nums1[p as usize] = nums2[p2 as usize];
            p2 -= 1;
            p -= 1;
        }
    }
}

fn main() {
    let test_cases = vec![
        (vec![1, 2, 3, 0, 0, 0], 3, vec![2, 5, 6], 3, vec![1, 2, 2, 3, 5, 6]),
        (vec![1], 1, vec![], 0, vec![1]),
        (vec![0], 0, vec![1], 1, vec![1]),
        (vec![4, 5, 6, 0, 0, 0], 3, vec![1, 2, 3], 3, vec![1, 2, 3, 4, 5, 6]),
        (vec![2, 0], 1, vec![1], 1, vec![1, 2]),
    ];

    for (nums1, m, nums2, n, expected) in test_cases {
        let mut a = nums1.clone();
        let mut b = nums2.clone();
        Solution::merge_brute_force(&mut a, m, &mut b, n);
        assert_eq!(a, expected);

        let mut c = nums1.clone();
        let mut d = nums2.clone();
        Solution::merge_extra_space(&mut c, m, &mut d, n);
        assert_eq!(c, expected);

        let mut e = nums1.clone();
        let mut f = nums2.clone();
        Solution::merge(&mut e, m, &mut f, n);
        assert_eq!(e, expected);
    }

    println!("All test cases passed for Merge Sorted Array (Brute Force, Auxiliary Buffer, In-Place Backwards)!");
}
