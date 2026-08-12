//! # Exercise 07 — Lifetimes and zero-copy parsing
//!
//! **Scenario.** Parsing an HTTP request line and a CSV payload *without
//! allocating*. This is where lifetimes stop being an abstract annoyance and
//! start being the reason Rust parsers are fast.
//!
//! **Python contrast.** `line.split()` in Python allocates a new list of new
//! string objects. In Rust, `&str` is a *view* — pointer + length — into
//! memory somebody else owns. A lifetime `'a` is the compiler's note to itself
//! saying "this view must not outlive what it points into".
//!
//! You have already met elision: `fn f(s: &str) -> &str` is shorthand for
//! `fn f<'a>(s: &'a str) -> &'a str`. Elision only works when there is exactly
//! one input reference (or a `&self`). With two, you must be explicit — which
//! is exactly what tasks 2 and 3 are about.
//!
//! Run: `cargo test -p exercises ex07`

/// A parsed HTTP request line. Every field is a **view into the original
/// buffer** — this struct allocates nothing.
///
/// The `<'a>` on the struct means: an instance of `RequestLine` can never
/// outlive the string it was parsed from. The compiler enforces that for you.
#[derive(Debug, PartialEq, Eq)]
pub struct RequestLine<'a> {
    pub method: &'a str,
    pub path: &'a str,
    pub version: &'a str,
}

/// Task 1 — parse `"GET /api/users HTTP/1.1"`.
///
/// Exactly three whitespace-separated parts, or `None`.
/// The `'_` in the return type is "the lifetime of `line`".
pub fn parse_request_line(line: &str) -> Option<RequestLine<'_>> {
    todo!("split_whitespace into exactly 3 parts")
}

/// Task 2 — two inputs, one output. Elision cannot guess which one the result
/// borrows from, so `'a` is mandatory here. Ties return `a`.
pub fn longest<'a>(a: &'a str, b: &'a str) -> &'a str {
    todo!()
}

/// Task 3 — **different** lifetimes on purpose.
///
/// The result borrows from `text`, never from `needle`. Saying so lets a
/// caller pass a short-lived `needle` and keep the result around.
pub fn find_line<'t>(text: &'t str, needle: &str) -> Option<&'t str> {
    todo!("first line containing `needle`")
}

/// Task 4 — a cursor over a string.
///
/// The interesting part is `next_token`: it takes `&mut self` but returns
/// `&'a str`, **not** `&str`. The token borrows from the original input, not
/// from the scanner, so the caller can hold onto tokens while continuing to
/// scan. Getting that signature right is the whole exercise.
pub struct Scanner<'a> {
    input: &'a str,
    pos: usize,
}

impl<'a> Scanner<'a> {
    pub fn new(input: &'a str) -> Self {
        Self { input, pos: 0 }
    }

    /// Next whitespace-delimited token, or `None` at the end.
    pub fn next_token(&mut self) -> Option<&'a str> {
        todo!("skip whitespace, take until whitespace, advance self.pos")
    }

    /// Everything not yet consumed, leading whitespace included.
    pub fn rest(&self) -> &'a str {
        todo!()
    }
}

/// Task 5 — a zero-copy CSV table.
#[derive(Debug, PartialEq, Eq)]
pub struct Csv<'a> {
    pub header: Vec<&'a str>,
    pub rows: Vec<Vec<&'a str>>,
}

impl<'a> Csv<'a> {
    /// Parse comma-separated text whose first line is the header.
    ///
    /// `None` if the input has no lines, or if any row's field count differs
    /// from the header's. Trailing newlines are fine.
    pub fn parse(input: &'a str) -> Option<Csv<'a>> {
        todo!()
    }

    /// All values in the named column, top to bottom. `None` if no such column.
    pub fn column(&self, name: &str) -> Option<Vec<&'a str>> {
        todo!("find the header index, then pluck it from every row")
    }
}

/// The other side of the boundary: sometimes you *must* own.
///
/// Task 6 — convert a borrowed `RequestLine` into an owned struct that can be
/// stored in a struct, sent to another thread, or returned from a function
/// whose input has gone out of scope.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct OwnedRequest {
    pub method: String,
    pub path: String,
}

impl<'a> From<RequestLine<'a>> for OwnedRequest {
    fn from(value: RequestLine<'a>) -> Self {
        todo!("this is where you pay for the allocation, deliberately")
    }
}

// ── Reading exercise (no code to write) ───────────────────────────────────
//
// This function cannot compile. Read it and work out why before you look at
// the answer below.
//
//     pub fn broken() -> &str {
//         let owned = String::from("hello");
//         &owned
//     }
//
// Answer: `owned` is dropped at the closing brace, so the returned reference
// would dangle. Rust rejects it at compile time. In C this is a use-after-free;
// in Python it simply cannot happen because the object is refcounted. Rust
// gets Python's safety with C's performance by making you say who owns what.

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t1_parse_request_line() {
        assert_eq!(
            parse_request_line("GET /api/users HTTP/1.1"),
            Some(RequestLine {
                method: "GET",
                path: "/api/users",
                version: "HTTP/1.1",
            })
        );
        assert_eq!(
            parse_request_line("  POST   /x   HTTP/2  "),
            Some(RequestLine {
                method: "POST",
                path: "/x",
                version: "HTTP/2",
            })
        );
        assert_eq!(parse_request_line("GET /only-two"), None);
        assert_eq!(parse_request_line("A B C D"), None);
        assert_eq!(parse_request_line(""), None);
    }

    #[test]
    fn t2_longest() {
        assert_eq!(longest("abc", "de"), "abc");
        assert_eq!(longest("de", "abcd"), "abcd");
        assert_eq!(longest("xy", "ab"), "xy", "tie goes to the first");
    }

    #[test]
    fn t3_find_line() {
        let log = "INFO start\nWARN disk 91%\nINFO done";

        // `needle` lives only inside this block, but the result outlives it —
        // that is only legal because the lifetimes are separate.
        let found = {
            let needle = String::from("WARN");
            find_line(log, &needle)
        };
        assert_eq!(found, Some("WARN disk 91%"));

        assert_eq!(find_line(log, "ERROR"), None);
    }

    #[test]
    fn t4_scanner() {
        let input = "  GET   /api/users  HTTP/1.1 ";
        let mut sc = Scanner::new(input);

        assert_eq!(sc.next_token(), Some("GET"));
        assert_eq!(sc.rest(), "   /api/users  HTTP/1.1 ");
        assert_eq!(sc.next_token(), Some("/api/users"));
        assert_eq!(sc.next_token(), Some("HTTP/1.1"));
        assert_eq!(sc.next_token(), None);
        assert_eq!(sc.next_token(), None, "stays exhausted");

        let mut empty = Scanner::new("   ");
        assert_eq!(empty.next_token(), None);
    }

    #[test]
    fn t4b_tokens_outlive_the_scanner() {
        let input = String::from("alpha beta gamma");
        let collected: Vec<&str> = {
            let mut sc = Scanner::new(&input);
            let mut out = Vec::new();
            while let Some(tok) = sc.next_token() {
                out.push(tok);
            }
            out // `sc` is dropped here, but the tokens borrow `input`, not `sc`
        };
        assert_eq!(collected, vec!["alpha", "beta", "gamma"]);
    }

    #[test]
    fn t5_csv() {
        let input = "name,age,city\nalice,30,pune\nbob,25,goa\n";
        let csv = Csv::parse(input).expect("valid csv");

        assert_eq!(csv.header, vec!["name", "age", "city"]);
        assert_eq!(csv.rows.len(), 2);
        assert_eq!(csv.rows[0], vec!["alice", "30", "pune"]);

        assert_eq!(csv.column("age"), Some(vec!["30", "25"]));
        assert_eq!(csv.column("name"), Some(vec!["alice", "bob"]));
        assert_eq!(csv.column("missing"), None);

        assert_eq!(Csv::parse("a,b\n1,2,3"), None, "row width mismatch");
        assert_eq!(Csv::parse(""), None);

        let header_only = Csv::parse("a,b").expect("header-only is valid");
        assert_eq!(header_only.rows.len(), 0);
        assert_eq!(header_only.column("a"), Some(vec![]));
    }

    #[test]
    fn t6_owned_request() {
        let owned: OwnedRequest = {
            let line = String::from("DELETE /jobs/42 HTTP/1.1");
            let parsed = parse_request_line(&line).unwrap();
            OwnedRequest::from(parsed)
            // `line` dies here — but `owned` copied the bytes, so it survives.
        };
        assert_eq!(
            owned,
            OwnedRequest {
                method: String::from("DELETE"),
                path: String::from("/jobs/42"),
            }
        );
    }
}
