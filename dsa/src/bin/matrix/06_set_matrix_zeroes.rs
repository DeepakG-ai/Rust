// LeetCode Problem 73: Set Matrix Zeroes
// Approaches:
//   1) Better (Row & Col Marker Vectors): -> O(m * n) time | O(m + n) space
//   2) Optimal (First Row & Column In-Place Markers): -> O(m * n) time | O(1) space
// Link: https://leetcode.com/problems/set-matrix-zeroes/
//
// Examples:
//   [[1,1,1],        [[1,0,1],
//    [1,0,1],   ->    [0,0,0],
//    [1,1,1]]         [1,0,1]]

struct Solution;

impl Solution {
    /// 1. ROW & COLUMN MARKERS:
    /// Time: O(m * n) | Space: O(m + n)
    pub fn set_zeroes_extra_space(matrix: &mut Vec<Vec<i32>>) {
        let (m, n) = (matrix.len(), matrix[0].len());
        let mut row_zero = vec![false; m];
        let mut col_zero = vec![false; n];

        for r in 0..m {
            for c in 0..n {
                if matrix[r][c] == 0 {
                    row_zero[r] = true;
                    col_zero[c] = true;
                }
            }
        }

        for r in 0..m {
            for c in 0..n {
                if row_zero[r] || col_zero[c] {
                    matrix[r][c] = 0;
                }
            }
        }
    }

    /// 2. OPTIMAL (In-Place First Row & Column as Flag Markers):
    /// Use matrix[0][..] and matrix[..][0] to track zeroes, with extra flags for first row/col.
    /// Time: O(m * n) | Space: O(1)
    pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
        let (m, n) = (matrix.len(), matrix[0].len());
        let mut first_row_has_zero = false;
        let mut first_col_has_zero = false;

        // Check if first row has any zero
        for c in 0..n {
            if matrix[0][c] == 0 {
                first_row_has_zero = true;
                break;
            }
        }

        // Check if first col has any zero
        for r in 0..m {
            if matrix[r][0] == 0 {
                first_col_has_zero = true;
                break;
            }
        }

        // Use first row and column as markers
        for r in 1..m {
            for c in 1..n {
                if matrix[r][c] == 0 {
                    matrix[r][0] = 0;
                    matrix[0][c] = 0;
                }
            }
        }

        // Fill zeroes based on markers
        for r in 1..m {
            for c in 1..n {
                if matrix[r][0] == 0 || matrix[0][c] == 0 {
                    matrix[r][c] = 0;
                }
            }
        }

        // Handle first row
        if first_row_has_zero {
            for c in 0..n {
                matrix[0][c] = 0;
            }
        }

        // Handle first col
        if first_col_has_zero {
            for r in 0..m {
                matrix[r][0] = 0;
            }
        }
    }
}

fn main() {
    let mut mat1 = vec![
        vec![1, 1, 1],
        vec![1, 0, 1],
        vec![1, 1, 1],
    ];
    let exp1 = vec![
        vec![1, 0, 1],
        vec![0, 0, 0],
        vec![1, 0, 1],
    ];

    let mut mat1_extra = mat1.clone();
    Solution::set_zeroes_extra_space(&mut mat1_extra);
    assert_eq!(mat1_extra, exp1);

    Solution::set_zeroes(&mut mat1);
    assert_eq!(mat1, exp1);

    let mut mat2 = vec![
        vec![0, 1, 2, 0],
        vec![3, 4, 5, 2],
        vec![1, 3, 1, 5],
    ];
    let exp2 = vec![
        vec![0, 0, 0, 0],
        vec![0, 4, 5, 0],
        vec![0, 3, 1, 0],
    ];

    Solution::set_zeroes(&mut mat2);
    assert_eq!(mat2, exp2);

    println!("All test cases passed for Set Matrix Zeroes (Row/Col Vector, In-Place Markers O(1) space)!");
}
