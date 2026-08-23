// LeetCode Problem 8: String to Integer (atoi)
// Approach: Linear Scan with 32-bit Integer Range Clamping
// Time: O(n) | Space: O(1)
// Link: https://leetcode.com/problems/string-to-integer-atoi/
//
// Examples:
//   "42"             -> 42
//   "   -42"         -> -42
//   "4193 with words"-> 4193
//   "-91283472332"   -> -2147483648 (clamped to INT_MIN)

struct Solution;

impl Solution {
    /// Linear scan with 4 stages:
    /// 1. Discard leading whitespace
    /// 2. Parse optional '+' or '-' sign
    /// 3. Read subsequent digits, with overflow prevention check BEFORE multiplication
    /// 4. Clamp out-of-range values to [i32::MIN, i32::MAX]
    /// Time: O(n) | Space: O(1)
    pub fn my_atoi(s: String) -> i32 {
        const INT_MAX: i64 = i32::MAX as i64; //  2147483647
        const INT_MIN: i64 = i32::MIN as i64; // -2147483648

        let bytes = s.as_bytes();
        let (mut i, n) = (0usize, bytes.len());
        let mut result: i64 = 0;
        let mut sign = 1i64;

        // 1. Skip leading whitespaces
        while i < n && bytes[i] == b' ' {
            i += 1;
        }

        // 2. Parse optional sign
        if i < n && (bytes[i] == b'+' || bytes[i] == b'-') {
            if bytes[i] == b'-' {
                sign = -1;
            }
            i += 1;
        }

        // 3. Process digits with overflow clamping
        while i < n && bytes[i].is_ascii_digit() {
            let digit = (bytes[i] - b'0') as i64;

            // Check if multiplying by 10 and adding digit exceeds 32-bit signed integer limits
            if result > (INT_MAX - digit) / 10 {
                return if sign == 1 { INT_MAX as i32 } else { INT_MIN as i32 };
            }
            result = result * 10 + digit;
            i += 1;
        }

        (result * sign) as i32
    }
}

fn main() {
    let test_cases = vec![
        ("42", 42),
        ("   -42", -42),
        ("4193 with words", 4193),
        ("words and 987", 0),
        ("-91283472332", -2147483648),
        ("+1", 1),
        ("+-12", 0),
        ("2147483647", 2147483647),
        ("2147483648", 2147483647),
        ("-2147483648", -2147483648),
        ("-2147483649", -2147483648),
        ("  0000000000012345678", 12345678),
        ("", 0),
        (" ", 0),
    ];

    for (s, expected) in test_cases {
        assert_eq!(Solution::my_atoi(s.to_string()), expected);
    }

    println!("All test cases passed for String to Integer (atoi)!");
}
