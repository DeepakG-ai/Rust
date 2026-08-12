//! Parsing raw log lines into typed records.

use std::error::Error;
use std::fmt;

/// One successfully parsed access-log line.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LogRecord {
    pub timestamp: String,
    pub method: String,
    pub path: String,
    pub status: u16,
    pub latency_ms: u64,
}

impl LogRecord {
    /// 5xx responses are what we call "errors" in the summary.
    pub fn is_error(&self) -> bool {
        self.status >= 500
    }
}

/// Everything that can be wrong with a single line.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParseError {
    /// Not exactly five whitespace-separated fields.
    WrongFieldCount { got: usize },
    /// The status field was not a number.
    InvalidStatus(String),
    /// The latency field was not a number.
    InvalidLatency(String),
}

/// TODO 1 — the human-readable messages. These strings appear in the CLI's
/// stderr output, so the tests pin them exactly:
///
/// - `WrongFieldCount { got: 4 }` -> `expected 5 fields, got 4`
/// - `InvalidStatus("abc")`       -> `invalid status: abc`
/// - `InvalidLatency("abc")`      -> `invalid latency: abc`
impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
    }
}

impl Error for ParseError {}

/// TODO 2 — parse exactly one line.
///
/// Fields are separated by any run of whitespace. Check the field count
/// first, then the status, then the latency (the tests depend on that order).
pub fn parse_line(line: &str) -> Result<LogRecord, ParseError> {
    todo!()
}

/// TODO 3 — parse a whole file's contents.
///
/// Returns `(records, failures)` where each failure carries the **1-based
/// line number** so the CLI can point the user at it.
///
/// Blank or whitespace-only lines are skipped silently and are *not*
/// failures — but they still consume a line number.
pub fn parse_log(text: &str) -> (Vec<LogRecord>, Vec<(usize, ParseError)>) {
    todo!("enumerate().  remember: enumerate is 0-based, line numbers are 1-based")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display_messages() {
        assert_eq!(
            ParseError::WrongFieldCount { got: 4 }.to_string(),
            "expected 5 fields, got 4"
        );
        assert_eq!(
            ParseError::InvalidStatus(String::from("2x0")).to_string(),
            "invalid status: 2x0"
        );
        assert_eq!(
            ParseError::InvalidLatency(String::from("abc")).to_string(),
            "invalid latency: abc"
        );
    }

    #[test]
    fn parses_a_good_line() {
        assert_eq!(
            parse_line("2026-01-15T10:00:00Z GET /api/users 200 45"),
            Ok(LogRecord {
                timestamp: String::from("2026-01-15T10:00:00Z"),
                method: String::from("GET"),
                path: String::from("/api/users"),
                status: 200,
                latency_ms: 45,
            })
        );
    }

    #[test]
    fn tolerates_extra_whitespace() {
        let r = parse_line("  T   POST   /x   201   7  ").expect("should parse");
        assert_eq!(r.method, "POST");
        assert_eq!(r.status, 201);
        assert_eq!(r.latency_ms, 7);
    }

    #[test]
    fn rejects_bad_lines() {
        assert_eq!(
            parse_line("this line is broken"),
            Err(ParseError::WrongFieldCount { got: 4 })
        );
        assert_eq!(
            parse_line(""),
            Err(ParseError::WrongFieldCount { got: 0 })
        );
        assert_eq!(
            parse_line("T GET /x 2x0 45"),
            Err(ParseError::InvalidStatus(String::from("2x0")))
        );
        assert_eq!(
            parse_line("T GET /x 200 abc"),
            Err(ParseError::InvalidLatency(String::from("abc")))
        );
    }

    #[test]
    fn field_count_is_checked_before_field_contents() {
        assert_eq!(
            parse_line("T GET /x 2x0 abc EXTRA"),
            Err(ParseError::WrongFieldCount { got: 6 })
        );
    }

    #[test]
    fn parse_log_reports_line_numbers() {
        let text = "T GET /a 200 1\n\nbroken\nT GET /b 200 xyz\nT GET /c 500 9";
        let (records, failures) = parse_log(text);

        assert_eq!(records.len(), 2, "lines 3 and 4 are malformed");
        assert_eq!(records[0].path, "/a");
        assert_eq!(records[1].path, "/c");
        assert_eq!(records[1].status, 500);

        assert_eq!(
            failures,
            vec![
                (3, ParseError::WrongFieldCount { got: 1 }),
                (4, ParseError::InvalidLatency(String::from("xyz"))),
            ],
            "blank line 2 is skipped but still counts toward the numbering"
        );
    }

    #[test]
    fn parse_log_handles_empty_input() {
        let (records, failures) = parse_log("");
        assert!(records.is_empty());
        assert!(failures.is_empty());
    }
}
