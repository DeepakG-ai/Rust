//! Rendering a [`Summary`] as text.
//!
//! Kept separate from `stats` on purpose: aggregation is tested against data,
//! formatting is tested against strings. Mixing them gives you tests that
//! break every time you move a column.

use crate::stats::Summary;

/// TODO — render the report. The format is pinned exactly:
///
/// ```text
/// requests: 8
/// errors:   2 (25.0%)
/// latency:  p50=10ms p95=200ms p99=200ms
///
/// TOP PATHS
/// path             count  errors  p95_ms
/// /api/users           3       1      90
/// ```
///
/// Details that the tests check:
///
/// - the percentage is `error_rate * 100.0` with **one** decimal (`{:.1}`)
/// - the table uses exactly `{:<16}{:>6}{:>8}{:>8}` for both the header row
///   and every data row
/// - there is one blank line between the summary block and `TOP PATHS`
/// - the whole string ends with a single `\n`
/// - when there are no paths, the header row is still printed
///
/// Hint: push each line into a `Vec<String>`, then
/// `lines.join("\n") + "\n"`. That is much easier to get right than
/// hand-managing newlines.
pub fn render(summary: &Summary) -> String {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::stats::PathStat;

    fn summary() -> Summary {
        Summary {
            total: 8,
            errors: 2,
            error_rate: 0.25,
            p50_ms: 10,
            p95_ms: 200,
            p99_ms: 200,
            top_paths: vec![
                PathStat {
                    path: String::from("/api/users"),
                    count: 3,
                    errors: 1,
                    p95_ms: 90,
                },
                PathStat {
                    path: String::from("/health"),
                    count: 3,
                    errors: 0,
                    p95_ms: 3,
                },
                PathStat {
                    path: String::from("/api/orders"),
                    count: 2,
                    errors: 1,
                    p95_ms: 200,
                },
            ],
        }
    }

    #[test]
    fn renders_exactly() {
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

        assert_eq!(render(&summary()), want);
    }

    #[test]
    fn renders_empty_summary() {
        let empty = Summary {
            total: 0,
            errors: 0,
            error_rate: 0.0,
            p50_ms: 0,
            p95_ms: 0,
            p99_ms: 0,
            top_paths: vec![],
        };

        let want = [
            "requests: 0",
            "errors:   0 (0.0%)",
            "latency:  p50=0ms p95=0ms p99=0ms",
            "",
            "TOP PATHS",
            "path             count  errors  p95_ms",
        ]
        .join("\n")
            + "\n";

        assert_eq!(render(&empty), want);
    }
}
