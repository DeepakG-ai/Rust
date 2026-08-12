//! # Exercise 04 — Iterators, closures, generics
//!
//! **Scenario.** Request-log analytics: error rates, latency percentiles,
//! top endpoints. This is real SRE work and it is almost pure iterator code.
//!
//! **Python contrast.** Iterators replace comprehensions *and* `itertools`,
//! and they are lazy in the same way generators are — nothing runs until you
//! call a "consuming" adaptor like `collect`, `sum`, `count`, `fold`, `max`.
//!
//! | Python                          | Rust                                |
//! |---------------------------------|-------------------------------------|
//! | `[f(x) for x in xs]`            | `xs.iter().map(f).collect()`        |
//! | `[x for x in xs if p(x)]`       | `xs.iter().filter(|x| p(x)).collect()` |
//! | `sum(xs)`                       | `xs.iter().sum::<i64>()`            |
//! | `sorted(xs, key=f)`             | `xs.sort_by_key(f)`  (in place!)    |
//! | `sorted(xs, key=f, reverse=True)` | `xs.sort_by(|a,b| f(b).cmp(&f(a)))` |
//! | `max(xs, key=f)`                | `xs.iter().max_by_key(|x| f(x))`    |
//! | `Counter(xs)`                   | `HashMap` + the `entry` API         |
//! | `enumerate(xs)`                 | `xs.iter().enumerate()`             |
//! | `zip(a, b)`                     | `a.iter().zip(b.iter())`            |
//!
//! Run: `cargo test -p exercises ex04`

use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Request {
    pub path: String,
    pub status: u16,
    pub latency_ms: u64,
}

impl Request {
    pub fn new(path: &str, status: u16, latency_ms: u64) -> Self {
        Self {
            path: path.to_string(),
            status,
            latency_ms,
        }
    }

    pub fn is_server_error(&self) -> bool {
        self.status >= 500
    }
}

/// Task 1 — fraction of requests that are 5xx. Empty input -> `0.0`.
///
/// Watch the integer division trap: `2 / 8` is `0` in Rust just like in C.
/// You must cast to `f64` before dividing.
pub fn error_rate(reqs: &[Request]) -> f64 {
    todo!("count 5xx, divide by total as f64")
}

/// Task 2 — nearest-rank percentile. This is the p50/p95/p99 every dashboard
/// shows.
///
/// Algorithm (use exactly this so the tests match):
/// 1. copy and sort ascending
/// 2. `rank = ceil(p / 100 * n)`
/// 3. `index = max(rank, 1) - 1`, clamped to `n - 1`
/// 4. return `sorted[index]`
///
/// Empty input -> `None`.
pub fn percentile(latencies: &[u64], p: f64) -> Option<u64> {
    todo!("sort a copy, then nearest-rank")
}

/// Task 3 — the `Counter.most_common(n)` of Rust.
///
/// Count requests per path, then return the top `n` as `(path, count)`,
/// sorted by **count descending, then path ascending** (so ties are stable
/// and the output is deterministic).
pub fn top_n_paths(reqs: &[Request], n: usize) -> Vec<(String, usize)> {
    todo!("HashMap entry API, then collect into a Vec and sort")
}

/// Task 4 — worst latency seen per path.
///
/// Hint: `*map.entry(k).or_insert(0) = ...` or `entry(..).and_modify(..).or_insert(..)`.
pub fn slowest_by_path(reqs: &[Request]) -> HashMap<String, u64> {
    todo!()
}

/// Task 5 — a generic higher-order function.
///
/// Return the `n` largest items by `key`, largest first. Ties keep their
/// original relative order (use a **stable** sort — `sort_by`, not
/// `sort_unstable_by`).
///
/// Read the `where` clause out loud: "for any T, any orderable K, and any
/// closure F that maps &T to K".
pub fn top_n_by<T, K, F>(items: Vec<T>, n: usize, key: F) -> Vec<T>
where
    F: Fn(&T) -> K,
    K: Ord,
{
    todo!()
}

/// Task 6 — implement `Iterator` by hand.
///
/// Exponential backoff, the way every retry loop in production does it:
/// yields `base`, `base*2`, `base*4`, ... each capped at `max_delay_ms`,
/// and stops after `max_retries` items.
///
/// Once you implement `next`, you get `.map`, `.take`, `.collect`, `for` loops
/// and the other ~70 adaptors for free. That is the payoff of the trait.
pub struct Backoff {
    next_delay_ms: u64,
    max_delay_ms: u64,
    remaining: usize,
}

impl Backoff {
    pub fn new(base_ms: u64, max_delay_ms: u64, max_retries: usize) -> Self {
        Self {
            next_delay_ms: base_ms,
            max_delay_ms,
            remaining: max_retries,
        }
    }
}

impl Iterator for Backoff {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        todo!("return None when exhausted; otherwise emit and then double (capped)")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> Vec<Request> {
        vec![
            Request::new("/api/users", 200, 10),
            Request::new("/api/users", 200, 30),
            Request::new("/api/users", 500, 90),
            Request::new("/api/orders", 200, 20),
            Request::new("/api/orders", 503, 200),
            Request::new("/health", 200, 1),
            Request::new("/health", 200, 2),
            Request::new("/health", 200, 3),
        ]
    }

    #[test]
    fn t1_error_rate() {
        assert_eq!(error_rate(&sample()), 0.25);
        assert_eq!(error_rate(&[]), 0.0);
        assert_eq!(error_rate(&[Request::new("/x", 500, 1)]), 1.0);
    }

    #[test]
    fn t2_percentile() {
        let xs = [10, 20, 30, 40];
        assert_eq!(percentile(&xs, 0.0), Some(10));
        assert_eq!(percentile(&xs, 50.0), Some(20));
        assert_eq!(percentile(&xs, 95.0), Some(40));
        assert_eq!(percentile(&xs, 100.0), Some(40));

        // Unsorted input must still work — you sort a copy.
        let unsorted = [40, 10, 30, 20];
        assert_eq!(percentile(&unsorted, 50.0), Some(20));

        assert_eq!(percentile(&[], 50.0), None);
        assert_eq!(percentile(&[7], 99.0), Some(7));
    }

    #[test]
    fn t3_top_n_paths() {
        // /api/users and /health are tied at 3, so path-ascending decides:
        // "/api/users" < "/health". /api/orders (2) is cut off by n = 2.
        assert_eq!(
            top_n_paths(&sample(), 2),
            vec![
                (String::from("/api/users"), 3),
                (String::from("/health"), 3),
            ]
        );
        assert_eq!(
            top_n_paths(&sample(), 10),
            vec![
                (String::from("/api/users"), 3),
                (String::from("/health"), 3),
                (String::from("/api/orders"), 2),
            ]
        );
    }

    #[test]
    fn t3b_top_n_paths_ordering_is_exact() {
        let reqs = vec![
            Request::new("/b", 200, 1),
            Request::new("/a", 200, 1),
            Request::new("/a", 200, 1),
        ];
        assert_eq!(
            top_n_paths(&reqs, 5),
            vec![(String::from("/a"), 2), (String::from("/b"), 1)]
        );
    }

    #[test]
    fn t4_slowest_by_path() {
        let got = slowest_by_path(&sample());
        assert_eq!(got.get("/api/users"), Some(&90));
        assert_eq!(got.get("/api/orders"), Some(&200));
        assert_eq!(got.get("/health"), Some(&3));
        assert_eq!(got.len(), 3);
    }

    #[test]
    fn t5_top_n_by() {
        let items = vec![("a", 3), ("b", 7), ("c", 7), ("d", 1)];
        assert_eq!(top_n_by(items, 2, |x| x.1), vec![("b", 7), ("c", 7)]);

        let words = vec!["hi", "hello", "hey"];
        assert_eq!(top_n_by(words, 1, |w: &&str| w.len()), vec!["hello"]);

        let empty: Vec<i32> = vec![];
        assert_eq!(top_n_by(empty, 3, |x: &i32| *x), Vec::<i32>::new());

        // n larger than the input is fine.
        assert_eq!(top_n_by(vec![1, 2], 99, |x: &i32| *x), vec![2, 1]);
    }

    #[test]
    fn t6_backoff() {
        let delays: Vec<u64> = Backoff::new(100, 1000, 6).collect();
        assert_eq!(delays, vec![100, 200, 400, 800, 1000, 1000]);

        let none: Vec<u64> = Backoff::new(100, 1000, 0).collect();
        assert_eq!(none, Vec::<u64>::new());

        // Because it is a real Iterator, every adaptor just works.
        let total: u64 = Backoff::new(50, 400, 4).sum();
        assert_eq!(total, 50 + 100 + 200 + 400);
    }
}
