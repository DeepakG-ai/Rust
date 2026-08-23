// LeetCode Problem 51: N-Queens
// Approaches:
//   1) Backtracking with board scan validation: -> O(N!) time | O(N^2) space
//   2) Optimal (Backtracking with Column & Diagonal Bitsets/Sets): -> O(N!) time | O(N) space
// Link: https://leetcode.com/problems/n-queens/
//
// Examples:
//   n = 4 -> 2 distinct solutions
//   n = 1 -> [["Q"]]

struct Solution;

impl Solution {
    /// 1. BACKTRACKING WITH BITSET LOOKUPS (Optimal):
    /// Track used columns, main diagonals (r - c), and anti-diagonals (r + c).
    /// Time: O(N!) | Space: O(N)
    pub fn solve_n_queens(n: i32) -> Vec<Vec<String>> {
        let n = n as usize;
        let mut result = Vec::new();
        let mut board = vec![vec!['.'; n]; n];

        // Track attacked lines
        let mut cols = vec![false; n];
        let mut diag1 = vec![false; 2 * n]; // r + c
        let mut diag2 = vec![false; 2 * n]; // r - c + (n - 1)

        fn backtrack(
            row: usize,
            n: usize,
            board: &mut Vec<Vec<char>>,
            cols: &mut [bool],
            diag1: &mut [bool],
            diag2: &mut [bool],
            result: &mut Vec<Vec<String>>,
        ) {
            if row == n {
                let solution: Vec<String> = board
                    .iter()
                    .map(|r| r.iter().collect::<String>())
                    .collect();
                result.push(solution);
                return;
            }

            for col in 0..n {
                let d1 = row + col;
                let d2 = row + n - 1 - col;

                if !cols[col] && !diag1[d1] && !diag2[d2] {
                    // Place queen
                    board[row][col] = 'Q';
                    cols[col] = true;
                    diag1[d1] = true;
                    diag2[d2] = true;

                    backtrack(row + 1, n, board, cols, diag1, diag2, result);

                    // Backtrack
                    board[row][col] = '.';
                    cols[col] = false;
                    diag1[d1] = false;
                    diag2[d2] = false;
                }
            }
        }

        backtrack(
            0,
            n,
            &mut board,
            &mut cols,
            &mut diag1,
            &mut diag2,
            &mut result,
        );
        result
    }
}

fn main() {
    let sol_4 = Solution::solve_n_queens(4);
    assert_eq!(sol_4.len(), 2);
    assert_eq!(
        sol_4,
        vec![
            vec![
                ".Q..".to_string(),
                "...Q".to_string(),
                "Q...".to_string(),
                "..Q.".to_string(),
            ],
            vec![
                "..Q.".to_string(),
                "Q...".to_string(),
                "...Q".to_string(),
                ".Q..".to_string(),
            ],
        ]
    );

    let sol_1 = Solution::solve_n_queens(1);
    assert_eq!(sol_1, vec![vec!["Q".to_string()]]);

    let sol_8 = Solution::solve_n_queens(8);
    assert_eq!(sol_8.len(), 92); // Famous 92 solutions for 8-queens

    println!("All test cases passed for N-Queens (Backtracking with Bitset Lookups O(N!))!");
}
