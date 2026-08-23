// GeeksforGeeks: Chocolate Distribution Problem
// Approaches:
//   1) Brute Force (Check all m-element combinations): -> O(C(n, m)) time
//   2) Optimal (Sort & Sliding Window of size m): -> O(n log n) time | O(1) space
//
// Description:
//   Given an array of n packets with chocolate counts and m students.
//   Distribute 1 packet to each student such that (max chocolates - min chocolates) in distribution is minimized.
//
// Examples:
//   arr = [7, 3, 2, 4, 9, 12, 56], m = 3 -> 2 (pick [2, 3, 4] -> 4 - 2 = 2)
//   arr = [3, 4, 1, 9, 56, 7, 9, 12], m = 5 -> 6 (pick [3, 4, 7, 9, 9] -> 9 - 3 = 6)

struct Solution;

impl Solution {
    /// 1. BRUTE FORCE (Combinations):
    /// Try all C(n, m) subsets of size m and find minimal (max - min).
    /// Time: O(C(n, m)) | Space: O(m)
    pub fn find_min_diff_brute(arr: Vec<i32>, m: usize) -> i32 {
        let n = arr.len();
        if n < m || m == 0 {
            return 0;
        }

        fn backtrack(
            start: usize,
            m: usize,
            current: &mut Vec<i32>,
            arr: &[i32],
            min_diff: &mut i32,
        ) {
            if current.len() == m {
                let min_val = *current.iter().min().unwrap();
                let max_val = *current.iter().max().unwrap();
                *min_diff = (*min_diff).min(max_val - min_val);
                return;
            }
            for i in start..arr.len() {
                current.push(arr[i]);
                backtrack(i + 1, m, current, arr, min_diff);
                current.pop();
            }
        }

        let mut min_diff = i32::MAX;
        let mut cur = Vec::new();
        backtrack(0, m, &mut cur, &arr, &mut min_diff);
        min_diff
    }

    /// 2. OPTIMAL (Sort & Sliding Window of size m):
    /// Sort array. For every window of size m [i..i+m-1], the difference between max and min is arr[i+m-1] - arr[i].
    /// Time: O(n log n) | Space: O(1)
    pub fn find_min_diff(mut arr: Vec<i32>, m: usize) -> i32 {
        let n = arr.len();
        if n < m || m == 0 {
            return 0;
        }

        arr.sort_unstable();
        let mut min_diff = i32::MAX;

        for i in 0..=(n - m) {
            let diff = arr[i + m - 1] - arr[i];
            min_diff = min_diff.min(diff);
        }

        min_diff
    }
}

fn main() {
    let test_cases = vec![
        (vec![7, 3, 2, 4, 9, 12, 56], 3, 2),
        (vec![3, 4, 1, 9, 56, 7, 9, 12], 5, 6),
        (vec![12, 4, 7, 9, 2, 23, 25, 41, 30, 40, 28, 42, 30, 44, 48, 43, 50], 7, 10),
        (vec![1, 2, 3], 3, 2),
        (vec![5, 5, 5, 5], 2, 0),
    ];

    for (arr, m, expected) in test_cases {
        if arr.len() <= 10 {
            assert_eq!(Solution::find_min_diff_brute(arr.clone(), m), expected);
        }
        assert_eq!(Solution::find_min_diff(arr, m), expected);
    }

    println!("All test cases passed for Chocolate Distribution Problem (Brute Force Combinations, Sort & Window O(n log n))!");
}
