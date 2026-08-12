//! # Exercise 02 — `Option`, `Result`, and the `?` operator
//!
//! **Scenario.** Request handling in an HTTP service: parse a port, look up
//! headers, validate query params.
//!
//! **Python contrast.** Python has `None` and exceptions. Rust has one type
//! for "might be absent" (`Option<T>`) and one for "might have failed"
//! (`Result<T, E>`), and the compiler forces you to deal with both.
//!
//! | Python                      | Rust                                  |
//! |-----------------------------|---------------------------------------|
//! | `d.get(k)`                  | `map.get(k)` -> `Option<&V>`          |
//! | `d.get(k, default)`         | `map.get(k).copied().unwrap_or(d)`    |
//! | `try: ... except: return None` | `.ok()`                            |
//! | `raise` / propagate         | `?`                                   |
//! | `x if x else y`             | `.unwrap_or(y)` / `.unwrap_or_else()` |
//!
//! The goal here is to stop writing `match` for everything and start using
//! combinators — that is the idiomatic style you saw in the real codebases.
//!
//! Run: `cargo test -p exercises ex02`

use std::collections::HashMap;

/// Task 1 — parse a TCP port with precise error messages.
///
/// - `""`      -> `Err("port must not be empty")`
/// - `"abc"`   -> `Err("port is not a number: abc")`
/// - `"0"`     -> `Err("port out of range: 0")`
/// - `"70000"` -> `Err("port out of range: 70000")`
/// - `"8080"`  -> `Ok(8080)`
///
/// Hint: parse as `u32` first so 70000 gives you a range error rather than a
/// confusing parse error.
pub fn parse_port(raw: &str) -> Result<u16, String> {
    todo!("validate, then convert to u16")
}

/// Task 2 — case-insensitive header lookup that returns a **borrow**.
///
/// Note the explicit `'a`: the returned `&str` lives as long as `headers`,
/// *not* as long as `name`. Try removing the lifetimes and read the error.
pub fn get_header<'a>(headers: &'a [(String, String)], name: &str) -> Option<&'a str> {
    todo!("iterate, compare keys ignoring case, return the value as &str")
}

/// Task 3 — chain combinators. **Write this without `match` or `if let`.**
///
/// Return the `content-length` header parsed as a `usize`, or `None` if the
/// header is missing *or* does not parse.
///
/// Hint: `get_header(..).and_then(|v| v.parse().ok())`
pub fn content_length(headers: &[(String, String)]) -> Option<usize> {
    todo!("and_then + .ok()")
}

/// Task 4 — all-or-nothing parsing.
///
/// Parse every element. The first failure aborts the whole thing.
///
/// Hint: this is a one-liner. `collect()` can turn an
/// `Iterator<Item = Result<T, E>>` straight into a `Result<Vec<T>, E>`.
/// That trick shows up constantly in real Rust.
pub fn parse_all(raw: &[&str]) -> Result<Vec<i64>, std::num::ParseIntError> {
    todo!("map(parse).collect()")
}

/// Task 5 — lenient parsing: keep what works, count what did not.
///
/// Returns `(parsed_values, failure_count)`.
pub fn parse_lenient(raw: &[&str]) -> (Vec<i64>, usize) {
    todo!("filter_map, and count the Nones")
}

/// Task 6 — `?` across a whole function.
///
/// Read `retries` and `timeout_ms` out of `map`. Both are optional:
/// defaults are `3` and `5000`. But if a key **is** present it must parse,
/// otherwise return `Err("invalid <key>: <value>")`.
///
/// Returns `(retries, timeout_ms)`.
pub fn read_retry_config(map: &HashMap<String, String>) -> Result<(u32, u64), String> {
    todo!("look up, map_err into a String, then `?`")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn headers() -> Vec<(String, String)> {
        vec![
            (String::from("Content-Type"), String::from("application/json")),
            (String::from("content-length"), String::from("348")),
            (String::from("X-Trace-Id"), String::from("abc123")),
        ]
    }

    #[test]
    fn t1_parse_port() {
        assert_eq!(parse_port("8080"), Ok(8080));
        assert_eq!(parse_port("1"), Ok(1));
        assert_eq!(parse_port("65535"), Ok(65535));
        assert_eq!(parse_port(""), Err(String::from("port must not be empty")));
        assert_eq!(
            parse_port("abc"),
            Err(String::from("port is not a number: abc"))
        );
        assert_eq!(parse_port("0"), Err(String::from("port out of range: 0")));
        assert_eq!(
            parse_port("70000"),
            Err(String::from("port out of range: 70000"))
        );
    }

    #[test]
    fn t2_get_header() {
        let h = headers();
        assert_eq!(get_header(&h, "content-type"), Some("application/json"));
        assert_eq!(get_header(&h, "CONTENT-TYPE"), Some("application/json"));
        assert_eq!(get_header(&h, "X-Trace-Id"), Some("abc123"));
        assert_eq!(get_header(&h, "authorization"), None);
        assert_eq!(get_header(&[], "anything"), None);
    }

    #[test]
    fn t3_content_length() {
        assert_eq!(content_length(&headers()), Some(348));

        let missing = vec![(String::from("host"), String::from("x"))];
        assert_eq!(content_length(&missing), None);

        let garbage = vec![(String::from("Content-Length"), String::from("many"))];
        assert_eq!(content_length(&garbage), None);
    }

    #[test]
    fn t4_parse_all() {
        assert_eq!(parse_all(&["1", "2", "-3"]), Ok(vec![1, 2, -3]));
        assert_eq!(parse_all(&[]), Ok(vec![]));
        assert!(parse_all(&["1", "oops", "3"]).is_err());
    }

    #[test]
    fn t5_parse_lenient() {
        assert_eq!(parse_lenient(&["1", "oops", "3", ""]), (vec![1, 3], 2));
        assert_eq!(parse_lenient(&[]), (vec![], 0));
        assert_eq!(parse_lenient(&["7"]), (vec![7], 0));
    }

    #[test]
    fn t6_read_retry_config() {
        let empty = HashMap::new();
        assert_eq!(read_retry_config(&empty), Ok((3, 5000)));

        let mut set = HashMap::new();
        set.insert(String::from("retries"), String::from("10"));
        assert_eq!(read_retry_config(&set), Ok((10, 5000)));

        set.insert(String::from("timeout_ms"), String::from("250"));
        assert_eq!(read_retry_config(&set), Ok((10, 250)));

        set.insert(String::from("timeout_ms"), String::from("soon"));
        assert_eq!(
            read_retry_config(&set),
            Err(String::from("invalid timeout_ms: soon"))
        );
    }
}
