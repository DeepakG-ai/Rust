// LeetCode Problem 5: Longest Palindromic Substring
// Approaches:
//   1) Brute Force: Check all substrings -> O(n^3) time | O(1) space
//   2) Better (2D DP Table): dp[i][j] is palindrome -> O(n^2) time | O(n^2) space
//   3) Optimal (Expand Around Center): 2n-1 centers -> O(n^2) time | O(1) space
// Link: https://leetcode.com/problems/longest-palindromic-substring/
//
// Examples:
//   "babad" -> "bab" (or "aba")
//   "cbbd"  -> "bb"

struct Solution;

impl Solution {
    /// 1. BRUTE FORCE: Test all possible substrings c[i..=j] for palindrome property.
    /// Time: O(n^3) | Space: O(1) extra
    pub fn longest_palindrome_brute(s: String) -> String {
        let c: Vec<char> = s.chars().collect();
        let n = c.len();
        if n == 0 {
            return String::new();
        }

        let mut start = 0;
        let mut max_len = 1;

        for i in 0..n {
            for j in i..n {
                if j - i + 1 <= max_len {
                    continue;
                }
                let (mut a, mut b) = (i, j);
                let mut is_pal = true;
                while a < b {
                    if c[a] != c[b] {
                        is_pal = false;
                        break;
                    }
                    a += 1;
                    b -= 1;
                }
                if is_pal {
                    max_len = j - i + 1;
                    start = i;
                }
            }
        }
        c[start..start + max_len].iter().collect()
    }

    /// 2. BETTER (Dynamic Programming 2D Table):
    /// dp[i][j] is true if substring s[i..=j] is a palindrome.
    /// dp[i][j] = (s[i] == s[j]) && (j - i <= 2 || dp[i+1][j-1])
    /// Time: O(n^2) | Space: O(n^2)
    pub fn longest_palindrome_dp(s: String) -> String {
        let c: Vec<char> = s.chars().collect();
        let n = c.len();
        if n == 0 {
            return String::new();
        }

        let mut dp = vec![vec![false; n]; n];
        let mut start = 0;
        let mut max_len = 1;

        // All single-character substrings are palindromes
        for i in 0..n {
            dp[i][i] = true;
        }

        // Check substrings of length len from 2 to n
        for len in 2..=n {
            for i in 0..=n - len {
                let j = i + len - 1;
                if c[i] == c[j] {
                    if len == 2 || dp[i + 1][j - 1] {
                        dp[i][j] = true;
                        if len > max_len {
                            max_len = len;
                            start = i;
                        }
                    }
                }
            }
        }
        c[start..start + max_len].iter().collect()
    }

    /// 3. OPTIMAL (Expand Around Center):
    /// Expand outward around all 2n-1 potential centers (n odd-length, n-1 even-length).
    /// Time: O(n^2) | Space: O(1) auxiliary
    pub fn longest_palindrome(s: String) -> String {
        let c: Vec<char> = s.chars().collect();
        let n = c.len();
        if n == 0 {
            return String::new();
        }

        fn expand(c: &[char], mut l: isize, mut r: isize, n: usize) -> (usize, usize) {
            while l >= 0 && (r as usize) < n && c[l as usize] == c[r as usize] {
                l -= 1;
                r += 1;
            }
            ((l + 1) as usize, (r - 1) as usize)
        }

        let mut best = (0usize, 0usize);
        for i in 0..n {
            for (l, r) in [
                expand(&c, i as isize, i as isize, n),
                expand(&c, i as isize, i as isize + 1, n),
            ] {
                if r >= l && (r - l) > (best.1.saturating_sub(best.0)) {
                    best = (l, r);
                }
            }
        }
        c[best.0..=best.1].iter().collect()
    }
}

fn main() {
    let test_cases = vec![
        "babad",
        "cbbd",
        "a",
        "ac",
        "racecar",
        "noon",
        "",
    ];

    for s in test_cases {
        let r1 = Solution::longest_palindrome_brute(s.to_string());
        let r2 = Solution::longest_palindrome_dp(s.to_string());
        let r3 = Solution::longest_palindrome(s.to_string());

        // Validate that each approach produces a valid palindrome of maximum length
        let max_len = r3.len();
        assert_eq!(r1.len(), max_len);
        assert_eq!(r2.len(), max_len);

        let is_pal = |str_val: &str| -> bool {
            let chs: Vec<char> = str_val.chars().collect();
            let (mut a, mut b) = (0, chs.len().saturating_sub(1));
            while a < b {
                if chs[a] != chs[b] { return false; }
                a += 1;
                b -= 1;
            }
            true
        };
        assert!(is_pal(&r1));
        assert!(is_pal(&r2));
        assert!(is_pal(&r3));
    }

    println!("All test cases passed for Longest Palindromic Substring (Brute Force, DP, Expand Center)!");
}
