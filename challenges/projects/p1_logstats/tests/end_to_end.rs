//! Integration test: the whole pipeline against a real fixture file.
//!
//! Note this file lives in `tests/`, not `src/`. Cargo compiles it as a
//! **separate crate** that links your library from the outside, so it can
//! only touch your public API. That is exactly what you want from an
//! integration test — if something here does not compile, your API is not
//! actually usable by a consumer.

use logstats::ParseError;
use logstats::parse_log;
use logstats::render;
use logstats::summarize;

/// Baked in at compile time, so the test does not care about the working
/// directory. (`include_str!` is relative to *this* file.)
const FIXTURE: &str = include_str!("fixtures/access.log");

#[test]
fn fixture_parses_with_two_known_failures() {
    let (records, failures) = parse_log(FIXTURE);

    assert_eq!(records.len(), 8);
    assert_eq!(
        failures,
        vec![
            (9, ParseError::WrongFieldCount { got: 4 }),
            (10, ParseError::InvalidLatency(String::from("abc"))),
        ]
    );
}

#[test]
fn fixture_produces_the_documented_report() {
    let (records, _) = parse_log(FIXTURE);
    let summary = summarize(&records, 3);

    let want = [
        "requests: 8",
        "errors:   2 (25.0%)",
        "latency:  p50=10ms p95=200ms p99=200ms",
        "",
        "TOP PATHS",
        "path             count  errors  p95_ms",
        "/api/users           3       1      90",
        "/health              3       0       3",
        "/api/orders          2       1     200",
    ]
    .join("\n")
        + "\n";

    assert_eq!(render(&summary), want);
}

#[test]
fn malformed_failures_render_useful_messages() {
    let (_, failures) = parse_log(FIXTURE);
    let rendered: Vec<String> = failures
        .iter()
        .map(|(line_no, err)| format!("line {line_no}: {err}"))
        .collect();

    assert_eq!(
        rendered,
        vec![
            String::from("line 9: expected 5 fields, got 4"),
            String::from("line 10: invalid latency: abc"),
        ]
    );
}
