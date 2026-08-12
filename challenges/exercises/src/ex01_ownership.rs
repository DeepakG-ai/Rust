//! # Exercise 01 — Ownership, borrowing, slices
//!
//! **Scenario.** You are writing the config loader for a service. Config
//! arrives as raw `KEY=VALUE` text and you must handle it without cloning
//! strings all over the place.
//!
//! **Python contrast.** In Python every string is one shared, refcounted
//! object and you never think about it. In Rust you decide *per function*:
//!
//! | You want to...              | Take          | Python analogue        |
//! |-----------------------------|---------------|------------------------|
//! | consume / store it          | `String`      | (no analogue)          |
//! | read it                     | `&str`        | passing a `str`        |
//! | modify the caller's copy    | `&mut String` | mutating a list in place |
//!
//! Rule of thumb: **take `&str`, return `String`** unless you can borrow.
//!
//! Run: `cargo test -p exercises ex01`

/// Task 1 — take ownership and return the number of **bytes**.
///
/// After the caller passes their `String` here, they can never use it again.
/// Look at the commented-out line in `t1_consume_and_measure` to see why.
pub fn consume_and_measure(s: String) -> usize {
    todo!("return the byte length of `s`")
}

/// Task 2 — borrow, and return the number of **characters**.
///
/// This is the classic Rust gotcha: `"héllo".len()` is 6 (bytes), but it is
/// 5 characters. `len()` on a string is *always* bytes.
pub fn char_count(s: &str) -> usize {
    todo!("count chars, not bytes")
}

/// Task 3 — mutate the caller's `String` in place.
///
/// Append `suffix`, but only if it is not already there. Must be idempotent:
/// calling it twice with the same suffix changes nothing the second time.
///
/// Note: until you implement this, `cargo clippy` warns that `&mut String`
/// should be `&mut str`. It is wrong here — and it stops warning the moment
/// you call `push_str`, which only a growable `String` has. Clippy reasons
/// from the body, and right now the body is `todo!()`.
pub fn ensure_suffix(s: &mut String, suffix: &str) {
    todo!("append `suffix` unless `s` already ends with it")
}

/// Task 4 — zero-copy parse of one `KEY=VALUE` line.
///
/// The returned `&str`s **borrow from `line`** — you must not allocate here.
/// That is what the elided lifetime in the signature is promising.
///
/// Rules:
/// - trim whitespace around both key and value
/// - blank / whitespace-only line  -> `None`
/// - line whose first non-space char is `#` -> `None`  (a comment)
/// - line with no `=` at all -> `None`
/// - split on the **first** `=` only, so `URL=https://x.dev/a=b` works
pub fn parse_env_line(line: &str) -> Option<(&str, &str)> {
    todo!("split_once('='), trim both halves, reject comments and blanks")
}

/// Task 5 — return the longest line in `text`, borrowed from `text`.
///
/// Ties go to the **first** one. Empty input returns `""`.
///
/// Careful: `Iterator::max_by_key` returns the *last* maximum on a tie, so it
/// will not pass this test as-is. Think about `fold`, or an explicit loop.
pub fn longest_line(text: &str) -> &str {
    todo!("longest line, first one wins a tie")
}

/// Task 6 — the "borrow or own" decision point.
///
/// Return the **uppercased keys** of every valid env line, in order. Uppercase
/// produces new data, so this one genuinely has to allocate: `Vec<String>`.
pub fn env_keys(text: &str) -> Vec<String> {
    todo!("reuse parse_env_line, uppercase each key")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t1_consume_and_measure() {
        let s = String::from("hello");
        assert_eq!(consume_and_measure(s), 5);
        // `s` was MOVED into the function. Uncomment the next line and read
        // the compiler error — that error is the whole point of this task.
        // assert_eq!(s.len(), 5);
    }

    #[test]
    fn t2_char_count() {
        assert_eq!("héllo".len(), 6, "sanity: len() is bytes");
        assert_eq!(char_count("héllo"), 5, "but you must return chars");
        assert_eq!(char_count("hello"), 5);
        assert_eq!(char_count(""), 0);
        assert_eq!(char_count("日本語"), 3);
    }

    #[test]
    fn t3_ensure_suffix() {
        let mut s = String::from("service");
        ensure_suffix(&mut s, ".toml");
        assert_eq!(s, "service.toml");
        ensure_suffix(&mut s, ".toml");
        assert_eq!(s, "service.toml", "must be idempotent");

        let mut empty = String::new();
        ensure_suffix(&mut empty, ".log");
        assert_eq!(empty, ".log");
    }

    #[test]
    fn t4_parse_env_line() {
        assert_eq!(parse_env_line("PORT=8080"), Some(("PORT", "8080")));
        assert_eq!(
            parse_env_line("  HOST = localhost  "),
            Some(("HOST", "localhost"))
        );
        assert_eq!(
            parse_env_line("URL=https://x.dev/a=b"),
            Some(("URL", "https://x.dev/a=b")),
            "split on the FIRST '=' only"
        );
        assert_eq!(parse_env_line("EMPTY="), Some(("EMPTY", "")));
        assert_eq!(parse_env_line("# a comment"), None);
        assert_eq!(parse_env_line("   # indented comment"), None);
        assert_eq!(parse_env_line("     "), None);
        assert_eq!(parse_env_line(""), None);
        assert_eq!(parse_env_line("NOEQUALS"), None);
    }

    #[test]
    fn t5_longest_line() {
        let text = "aa\nbbbb\ncccc\nd";
        assert_eq!(longest_line(text), "bbbb", "first of the tied longest");
        assert_eq!(longest_line("only"), "only");
        assert_eq!(longest_line(""), "");
    }

    #[test]
    fn t6_env_keys() {
        let text = "PORT=1\n# skip me\nhost=local\n\nnot_a_pair\nDb_Url=x";
        assert_eq!(
            env_keys(text),
            vec![
                String::from("PORT"),
                String::from("HOST"),
                String::from("DB_URL")
            ]
        );
        assert_eq!(env_keys(""), Vec::<String>::new());
    }
}
