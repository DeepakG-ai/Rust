// Word Search - LeetCode 79
// Method: DFS with backtracking (temporarily mark cells)
// Time: O(n*m*3^L) | Space: O(L) recursion
//
// Try every cell as a starting point. DFS matches word[index]; mark the cell
// visited, explore 4 directions, then UNMARK (backtrack) so the same cell
// can still be used in OTHER candidate paths.
//
// Example:
//   A B C E          "ABCCED" -> true
//   S F C S          "SEE"    -> true
//   A D E E          "ABCB"   -> false

struct Solution;

impl Solution {
    pub fn exist(board: Vec<Vec<char>>, word: &str) -> bool {
        if board.is_empty() || word.is_empty() {
            return false;
        }
        let n = board.len();
        let m = board[0].len();
        let chars: Vec<char> = word.chars().collect();
        const DIRS: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

        // dfs(r,c,idx) = can we match chars[idx..] starting at board[r][c]?
        fn go(
            board: &mut Vec<Vec<char>>,
            r: usize,
            c: usize,
            idx: usize,
            chars: &[char],
            n: usize,
            m: usize,
        ) -> bool {
            // base case: matched everything!
            if idx == chars.len() {
                return true;
            }
            // bounds / mismatch / already-visited ('#') checks
            if r >= n || c >= m || board[r][c] != chars[idx] {
                return false;
            }

            board[r][c] = '#'; // mark as used on THIS path

            for (dr, dc) in DIRS {
                let nr = r as isize + dr;
                let nc = c as isize + dc;
                if nr >= 0 && nc >= 0 && (nr as usize) < n && (nc as usize) < m {
                    if go(board, nr as usize, nc as usize, idx + 1, chars, n, m) {
                        board[r][c] = chars[idx]; // restore before returning true too
                        return true;
                    }
                }
            }

            board[r][c] = chars[idx]; // BACKTRACK: un-mark
            false
        }

        let mut b = board;
        for r in 0..n {
            for c in 0..m {
                if go(&mut b, r, c, 0, &chars, n, m) {
                    return true; // any starting cell that works is enough
                }
            }
        }
        false
    }
}

fn main() {
    let board = vec![
        vec!['A', 'B', 'C', 'E'],
        vec!['S', 'F', 'C', 'S'],
        vec!['A', 'D', 'E', 'E'],
    ];

    assert!(Solution::exist(board.clone(), "ABCCED")); // snakes around top-left
    assert!(Solution::exist(board.clone(), "SEE")); // down-right-right area
    assert!(!Solution::exist(board, "ABCB")); // B reuse not allowed

    println!("All test cases passed!");
}
