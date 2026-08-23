// LeetCode Problem 74: Search a 2D Matrix
// Approaches:
//   1) Brute Force (Linear Scan): -> O(m * n) time | O(1) space
//   2) Better (Staircase Search): -> O(m + n) time | O(1) space
//   3) Optimal (Single Binary Search on Flattened Matrix): -> O(log(m * n)) time | O(1) space
// Link: https://leetcode.com/problems/search-a-2d-matrix/
//
// Examples:
//   matrix = [[1,3,5,7],[10,11,16,20],[23,30,34,60]], target = 3 -> true
//   matrix = [[1,3,5,7],[10,11,16,20],[23,30,34,60]], target = 13 -> false

struct Solution;

impl Solution {
    /// 1. BRUTE FORCE:
    /// Time: O(m * n) | Space: O(1)
    pub fn search_matrix_brute(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        for row in matrix {
            for val in row {
                if val == target {
                    return true;
                }
            }
        }
        false
    }

    /// 2. STAIRCASE SEARCH:
    /// Start from top-right corner (0, n-1). If target < current, go left; if target > current, go down.
    /// Time: O(m + n) | Space: O(1)
    pub fn search_matrix_staircase(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        if matrix.is_empty() || matrix[0].is_empty() {
            return false;
        }
        let m = matrix.len();
        let n = matrix[0].len();
        let mut r = 0;
        let mut c = n as isize - 1;

        while r < m && c >= 0 {
            let val = matrix[r][c as usize];
            if val == target {
                return true;
            } else if val > target {
                c -= 1;
            } else {
                r += 1;
            }
        }
        false
    }

    /// 3. OPTIMAL (Binary Search over Virtual 1D Array):
    /// Map 1D index mid to (row, col) = (mid / n, mid % n).
    /// Time: O(log(m * n)) | Space: O(1)
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        if matrix.is_empty() || matrix[0].is_empty() {
            return false;
        }
        let (m, n) = (matrix.len(), matrix[0].len());
        let (mut left, mut right) = (0, (m * n) as isize - 1);

        while left <= right {
            let mid = left + (right - left) / 2;
            let (r, c) = (mid as usize / n, mid as usize % n);
            let val = matrix[r][c];

            if val == target {
                return true;
            } else if val < target {
                left = mid + 1;
            } else {
                right = mid - 1;
            }
        }
        false
    }
}

fn main() {
    let mat = vec![
        vec![1, 3, 5, 7],
        vec![10, 11, 16, 20],
        vec![23, 30, 34, 60],
    ];

    assert!(Solution::search_matrix_brute(mat.clone(), 3));
    assert!(Solution::search_matrix_staircase(mat.clone(), 3));
    assert!(Solution::search_matrix(mat.clone(), 3));

    assert!(!Solution::search_matrix_brute(mat.clone(), 13));
    assert!(!Solution::search_matrix_staircase(mat.clone(), 13));
    assert!(!Solution::search_matrix(mat.clone(), 13));

    assert!(Solution::search_matrix(mat.clone(), 60));
    assert!(Solution::search_matrix(mat.clone(), 1));
    assert!(!Solution::search_matrix(vec![vec![1]], 2));
    assert!(Solution::search_matrix(vec![vec![1]], 1));

    println!("All test cases passed for Search in a 2D Matrix (Linear, Staircase O(m+n), Binary Search O(log(m*n)))!");
}
