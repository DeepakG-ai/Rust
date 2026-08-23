// LeetCode Problem 85: Maximal Rectangle in Binary Matrix
// Approaches:
//   1) Brute Force: Check all submatrices -> O((R * C)^2) time
//   2) Optimal: Dynamic Histogram Heights + Monotonic Stack -> O(R * C) time | O(C) space
// Link: https://leetcode.com/problems/maximal-rectangle/
//
// Examples:
//   matrix = [["1","0","1","0","0"],
//             ["1","0","1","1","1"],
//             ["1","1","1","1","1"],
//             ["1","0","0","1","0"]] -> 6

struct Solution;

impl Solution {
    /// 1. BRUTE FORCE:
    /// Check every possible rectangle (r1, c1) to (r2, c2) if it contains only '1's.
    /// Time: O((R * C)^2) | Space: O(1)
    pub fn maximal_rectangle_brute(matrix: Vec<Vec<char>>) -> i32 {
        if matrix.is_empty() || matrix[0].is_empty() {
            return 0;
        }
        let (rows, cols) = (matrix.len(), matrix[0].len());
        let mut max_area = 0;

        for r1 in 0..rows {
            for c1 in 0..cols {
                if matrix[r1][c1] == '1' {
                    for r2 in r1..rows {
                        for c2 in c1..cols {
                            // Check if all cells in rectangle are '1'
                            let mut all_ones = true;
                            for r in r1..=r2 {
                                for c in c1..=c2 {
                                    if matrix[r][c] != '1' {
                                        all_ones = false;
                                        break;
                                    }
                                }
                                if !all_ones {
                                    break;
                                }
                            }
                            if all_ones {
                                let area = (r2 - r1 + 1) * (c2 - c1 + 1);
                                max_area = max_area.max(area as i32);
                            }
                        }
                    }
                }
            }
        }
        max_area
    }

    /// 2. OPTIMAL (Histogram DP + Monotonic Stack):
    /// Maintain running histogram heights for each row.
    /// For each row, calculate largest rectangle in histogram in O(C) using monotonic stack.
    /// Total Time: O(R * C) | Space: O(C)
    pub fn maximal_rectangle(matrix: Vec<Vec<char>>) -> i32 {
        if matrix.is_empty() || matrix[0].is_empty() {
            return 0;
        }
        let cols = matrix[0].len();
        let mut heights = vec![0; cols];
        let mut max_area = 0;

        for row in matrix {
            // Update heights
            for c in 0..cols {
                if row[c] == '1' {
                    heights[c] += 1;
                } else {
                    heights[c] = 0;
                }
            }
            max_area = max_area.max(Self::largest_rectangle_histogram(&heights));
        }
        max_area
    }

    fn largest_rectangle_histogram(heights: &[i32]) -> i32 {
        let mut stack = Vec::new();
        let mut max_area = 0;
        let n = heights.len();

        for i in 0..=n {
            let h = if i == n { 0 } else { heights[i] };
            while let Some(&top) = stack.last() {
                if h < heights[top] {
                    stack.pop();
                    let height = heights[top];
                    let width = match stack.last() {
                        Some(&prev_top) => (i - prev_top - 1) as i32,
                        None => i as i32,
                    };
                    max_area = max_area.max(height * width);
                } else {
                    break;
                }
            }
            stack.push(i);
        }
        max_area
    }
}

fn main() {
    let test_cases = vec![
        (
            vec![
                vec!['1', '0', '1', '0', '0'],
                vec!['1', '0', '1', '1', '1'],
                vec!['1', '1', '1', '1', '1'],
                vec!['1', '0', '0', '1', '0'],
            ],
            6,
        ),
        (vec![vec!['0']], 0),
        (vec![vec!['1']], 1),
        (
            vec![
                vec!['0', '0'],
                vec!['0', '0'],
            ],
            0,
        ),
    ];

    for (mat, expected) in test_cases {
        assert_eq!(Solution::maximal_rectangle_brute(mat.clone()), expected);
        assert_eq!(Solution::maximal_rectangle(mat), expected);
    }

    println!("All test cases passed for Maximal Rectangle (Brute Force, Histogram Stack DP O(R*C))!");
}
