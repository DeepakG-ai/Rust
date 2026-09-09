#![allow(dead_code)]

fn is_even(n: i32) -> bool {
    if n % 2 == 0 {
        return true;
    } else {
        return false;
    }
}

fn average(nums: &[f64]) -> Option<f64> {
    if nums.is_empty() {
        None
    } else {
        let n = nums.len();
        let mut add = 0.0;
        for i in 0..n {
            add = add + nums[i];
        }
        let avg = add / n as f64;
        return Some(avg);
    }
}

fn parse_age(s: &str) -> Result<u32, String> {
    match s.parse::<u32>() {
        Ok(age) if age <= 150 => Ok(age),
        Ok(_) => Err("Age cannot be greater than 150".to_string()),
        Err(e) => Err(e.to_string()),
    }
}

// Q26 Questions Answered:
// 1. What does `use super::*;` do?
//    `mod tests` is a child module with its own scope. `use super::*;` brings all items
//    (functions, types, etc.) from the parent module (the enclosing file) into the `tests` scope
//    so they can be called directly without prefixing with `super::`.
//
// 2. Why does `#[cfg(test)]` mean the test code is not in your release binary?
//    `#[cfg(test)]` is a conditional compilation attribute. The compiler only compiles this module
//    when building for tests (via `cargo test`). During normal compilation (`cargo build` or `cargo build --release`),
//    the compiler completely ignores and strips this module, so none of the test functions, test data,
//    or test-only dependencies end up in the final binary.

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn even_numbers_are_even() {
        assert!(is_even(4));
        assert!(!is_even(3));
    }

    #[test]
    fn average_of_empty_slice_is_none() {
        assert_eq!(average(&[]), None);
    }

    #[test]
    fn rejects_impossible_age() {
        assert!(parse_age("200").is_err());
    }
}

fn main() {}
