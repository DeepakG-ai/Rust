// LeetCode Problem 48: Rotate Image (90 degrees clockwise)
// Approaches:
//   1) Better (Auxiliary Buffer Matrix): -> O(n^2) time | O(n^2) space
//   2) Optimal (Transpose + Reverse Each Row In-Place): -> O(n^2) time | O(1) space
// Link: https://leetcode.com/problems/rotate-image/
//
// Examples:
//   [[1,2,3],        [[7,4,1],
//    [4,5,6],   ->    [8,5,2],
//    [7,8,9]]         [9,6,3]]

struct Solution;

impl Solution {
    /// 1. AUXILIARY BUFFER:
    /// rotated[c][n - 1 - r] = matrix[r][c]
    /// Time: O(n^2) | Space: O(n^2)
    pub fn rotate_buffer(matrix: &mut Vec<Vec<i32>>) {
        let n = matrix.len();
        let mut rotated = vec![vec![0; n]; n];

        for r in 0..n {
            for c in 0..n {
                rotated[c][n - 1 - r] = matrix[r][c];
            }
        }
        *matrix = rotated;
    }

    /// 2. OPTIMAL (In-Place Transpose + Row Reversal):
    /// Step 1: Transpose matrix along main diagonal (swap matrix[r][c] with matrix[c][r]).
    /// Step 2: Reverse each row.
    /// Time: O(n^2) | Space: O(1)
    pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
        let n = matrix.len();

        // 1. Transpose
        for r in 0..n {
            for c in (r + 1)..n {
                let temp = matrix[r][c];
                matrix[r][c] = matrix[c][r];
                matrix[c][r] = temp;
            }
        }

        // 2. Reverse each row
        for row in matrix.iter_mut() {
            row.reverse();
        }
    }
}

fn main() {
    let mut mat1 = vec![
        vec![1, 2, 3],
        vec![4, 5, 6],
        vec![7, 8, 9],
    ];
    let exp1 = vec![
        vec![7, 4, 1],
        vec![8, 5, 2],
        vec![9, 6, 3],
    ];

    let mut mat1_buf = mat1.clone();
    Solution::rotate_buffer(&mut mat1_buf);
    assert_eq!(mat1_buf, exp1);

    Solution::rotate(&mut mat1);
    assert_eq!(mat1, exp1);

    let mut mat2 = vec![
        vec![5, 1, 9, 11],
        vec![2, 4, 8, 10],
        vec![13, 3, 6, 7],
        vec![15, 14, 12, 16],
    ];
    let exp2 = vec![
        vec![15, 13, 2, 5],
        vec![14, 3, 4, 1],
        vec![12, 6, 8, 9],
        vec![16, 7, 10, 11],
    ];

    Solution::rotate(&mut mat2);
    assert_eq!(mat2, exp2);

    let mut mat3 = vec![vec![1]];
    Solution::rotate(&mut mat3);
    assert_eq!(mat3, vec![vec![1]]);

    println!("All test cases passed for Rotate Image (Buffer O(n^2), Transpose + Reverse In-Place O(1) space)!");
}
