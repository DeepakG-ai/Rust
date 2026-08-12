//! Aggregating records into a summary.

use std::collections::HashMap;

use crate::record::LogRecord;

/// Per-endpoint rollup.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PathStat {
    pub path: String,
    pub count: usize,
    pub errors: usize,
    pub p95_ms: u64,
}

/// The whole report, as data. Rendering is somebody else's job — keeping
/// those separate is why this is testable without string matching.
#[derive(Debug, Clone, PartialEq)]
pub struct Summary {
    pub total: usize,
    pub errors: usize,
    /// Fraction in `0.0..=1.0`, not a percentage.
    pub error_rate: f64,
    pub p50_ms: u64,
    pub p95_ms: u64,
    pub p99_ms: u64,
    pub top_paths: Vec<PathStat>,
}

/// TODO 1 — nearest-rank percentile.
///
/// Input may be in any order; sort a copy. Use exactly this algorithm so the
/// numbers match the tests:
///
/// 1. sort ascending, let `n = len`
/// 2. `rank = ceil(p / 100 * n)`
/// 3. `index = clamp(max(rank, 1) - 1, 0, n - 1)`
///
/// Empty input returns `0`.
pub fn percentile(values: &[u64], p: f64) -> u64 {
    todo!()
}

/// TODO 2 — the aggregation.
///
/// - `total`      : number of records
/// - `errors`     : how many are 5xx
/// - `error_rate` : `errors / total`, or `0.0` when there are no records
/// - `p50/p95/p99`: over **all** latencies
/// - `top_paths`  : one `PathStat` per distinct path, sorted by **count
///   descending, then path ascending**, truncated to `top_n`
///
/// `PathStat::p95_ms` is the p95 of that path's own latencies.
pub fn summarize(records: &[LogRecord], top_n: usize) -> Summary {
    todo!("group latencies per path with the entry API, then build the Vec and sort it")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn rec(path: &str, status: u16, latency_ms: u64) -> LogRecord {
        LogRecord {
            timestamp: String::from("T"),
            method: String::from("GET"),
            path: path.to_string(),
            status,
            latency_ms,
        }
    }

    fn sample() -> Vec<LogRecord> {
        vec![
            rec("/api/users", 200, 10),
            rec("/api/users", 200, 30),
            rec("/api/users", 500, 90),
            rec("/api/orders", 201, 20),
            rec("/api/orders", 503, 200),
            rec("/health", 200, 1),
            rec("/health", 200, 2),
            rec("/health", 200, 3),
        ]
    }

    #[test]
    fn percentiles() {
        let xs = [10, 20, 30, 40];
        assert_eq!(percentile(&xs, 0.0), 10);
        assert_eq!(percentile(&xs, 50.0), 20);
        assert_eq!(percentile(&xs, 95.0), 40);
        assert_eq!(percentile(&xs, 100.0), 40);
        assert_eq!(percentile(&[40, 10, 30, 20], 50.0), 20, "unsorted input");
        assert_eq!(percentile(&[], 95.0), 0);
        assert_eq!(percentile(&[7], 50.0), 7);
    }

    #[test]
    fn summarize_sample() {
        let got = summarize(&sample(), 3);

        assert_eq!(got.total, 8);
        assert_eq!(got.errors, 2);
        assert_eq!(got.error_rate, 0.25);
        assert_eq!(got.p50_ms, 10);
        assert_eq!(got.p95_ms, 200);
        assert_eq!(got.p99_ms, 200);

        assert_eq!(
            got.top_paths,
            vec![
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
            "count desc, then path asc"
        );
    }

    #[test]
    fn summarize_respects_top_n() {
        let got = summarize(&sample(), 1);
        assert_eq!(got.top_paths.len(), 1);
        assert_eq!(got.top_paths[0].path, "/api/users");
        assert_eq!(got.total, 8, "top_n must not affect the global numbers");
    }

    #[test]
    fn summarize_empty() {
        let got = summarize(&[], 5);
        assert_eq!(got.total, 0);
        assert_eq!(got.errors, 0);
        assert_eq!(got.error_rate, 0.0, "must not be NaN");
        assert_eq!(got.p50_ms, 0);
        assert!(got.top_paths.is_empty());
    }
}
