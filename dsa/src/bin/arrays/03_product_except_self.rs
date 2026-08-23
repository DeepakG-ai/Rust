// LeetCode Problem 238: Product of Array Except Self
// Approaches:
//   1) Brute Force: Compute product of all elements except i -> O(n^2) time | O(1) space
//   2) Better (Prefix + Suffix Arrays): Explicit prefix & suffix arrays -> O(n) time | O(n) space
//   3) Optimal (In-Place Prefix with Suffix Variable): -> O(n) time | O(1) extra space
// Link: https://leetcode.com/problems/product-of-array-except-self/
//
// Examples:
//   [1,2,3,4]      -> [24,12,8,6]
//   [-1,1,0,-3,3]  -> [0,0,9,0,0]

struct Solution;

impl Solution {
    /// 1. BRUTE FORCE: For each index i, iterate through the whole array and multiply all nums[j] (j != i).
    /// Time: O(n^2) | Space: O(1) extra
    pub fn product_except_self_brute_force(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut answer = vec![1; n];
        for i in 0..n {
            let mut prod = 1;
            for j in 0..n {
                if i != j {
                    prod *= nums[j];
                }
            }
            answer[i] = prod;
        }
        answer
    }

    /// 2. BETTER (Prefix and Suffix Arrays):
    /// prefix[i] = product of nums[0..i]
    /// suffix[i] = product of nums[i+1..n]
    /// Time: O(n) | Space: O(n) auxiliary
    pub fn product_except_self_better(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        if n == 0 {
            return Vec::new();
        }
        let mut prefix = vec![1; n];
        let mut suffix = vec![1; n];

        for i in 1..n {
            prefix[i] = prefix[i - 1] * nums[i - 1];
        }

        for i in (0..n - 1).rev() {
            suffix[i] = suffix[i + 1] * nums[i + 1];
        }

        let mut answer = vec![1; n];
        for i in 0..n {
            answer[i] = prefix[i] * suffix[i];
        }
        answer
    }

    /// 3. OPTIMAL (In-place Prefix + Running Suffix Scalar):
    /// Store prefix products directly in output vector, then sweep backwards with a scalar suffix accumulator.
    /// Time: O(n) | Space: O(1) extra space (output vector does not count towards extra space complexity)
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        if n == 0 {
            return Vec::new();
        }
        let mut answer = vec![1i32; n];

        // Pass 1: prefix products
        let mut prefix = 1;
        for i in 0..n {
            answer[i] = prefix;
            prefix *= nums[i];
        }

        // Pass 2: multiply running suffix products
        let mut suffix = 1;
        for i in (0..n).rev() {
            answer[i] *= suffix;
            suffix *= nums[i];
        }

        answer
    }
}

fn main() {
    let test_cases = vec![
        (vec![1, 2, 3, 4], vec![24, 12, 8, 6]),
        (vec![-1, 1, 0, -3, 3], vec![0, 0, 9, 0, 0]),
        (vec![5], vec![1]),
        (vec![0, 0], vec![0, 0]),
        (vec![2, 3], vec![3, 2]),
    ];

    for (nums, expected) in test_cases {
        assert_eq!(Solution::product_except_self_brute_force(nums.clone()), expected);
        assert_eq!(Solution::product_except_self_better(nums.clone()), expected);
        assert_eq!(Solution::product_except_self(nums), expected);
    }

    println!("All test cases passed for Product of Array Except Self!");
}
