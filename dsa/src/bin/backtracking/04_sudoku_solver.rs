// LeetCode Problem 37: Sudoku Solver
// Approach: Backtracking with 9x9 Row, Column, and 3x3 Box constraints
// Time: O(9^(empty_cells)) | Space: O(1) in-place board modification
// Link: https://leetcode.com/problems/sudoku-solver/

struct Solution;

impl Solution {
    /// In-place Sudoku Solver using Backtracking.
    pub fn solve_sudoku(board: &mut Vec<Vec<char>>) {
        Self::solve(board);
    }

    fn solve(board: &mut Vec<Vec<char>>) -> bool {
        for r in 0..9 {
            for c in 0..9 {
                if board[r][c] == '.' {
                    for ch in ['1', '2', '3', '4', '5', '6', '7', '8', '9'] {
                        if Self::is_valid(board, r, c, ch) {
                            board[r][c] = ch;
                            if Self::solve(board) {
                                return true;
                            }
                            board[r][c] = '.'; // backtrack
                        }
                    }
                    return false; // No valid digit worked for this empty cell
                }
            }
        }
        true // All cells filled validly
    }

    fn is_valid(board: &[Vec<char>], row: usize, col: usize, ch: char) -> bool {
        let box_r = (row / 3) * 3;
        let box_c = (col / 3) * 3;

        for i in 0..9 {
            // Check row
            if board[row][i] == ch {
                return false;
            }
            // Check col
            if board[i][col] == ch {
                return false;
            }
            // Check 3x3 box
            if board[box_r + i / 3][box_c + i % 3] == ch {
                return false;
            }
        }
        true
    }
}

fn main() {
    let mut board = vec![
        vec!['5', '3', '.', '.', '7', '.', '.', '.', '.'],
        vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
        vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
        vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
        vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
        vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
        vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
        vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
        vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
    ];

    let expected = vec![
        vec!['5', '3', '4', '6', '7', '8', '9', '1', '2'],
        vec!['6', '7', '2', '1', '9', '5', '3', '4', '8'],
        vec!['1', '9', '8', '3', '4', '2', '5', '6', '7'],
        vec!['8', '5', '9', '7', '6', '1', '4', '2', '3'],
        vec!['4', '2', '6', '8', '5', '3', '7', '9', '1'],
        vec!['7', '1', '3', '9', '2', '4', '8', '5', '6'],
        vec!['9', '6', '1', '5', '3', '7', '2', '8', '4'],
        vec!['2', '8', '7', '4', '1', '9', '6', '3', '5'],
        vec!['3', '4', '5', '2', '8', '6', '1', '7', '9'],
    ];

    Solution::solve_sudoku(&mut board);
    assert_eq!(board, expected);

    println!("All test cases passed for Sudoku Solver (Backtracking with 3x3 Box Constraints)!");
}
