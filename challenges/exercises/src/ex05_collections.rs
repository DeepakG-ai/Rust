//! # Exercise 05 — Collections and the `entry` API
//!
//! **Scenario.** Text analytics, config merging, and an in-memory rate limiter
//! — the three collection shapes you will reach for constantly.
//!
//! **Python contrast.**
//!
//! | Python            | Rust                | Notes                              |
//! |-------------------|---------------------|------------------------------------|
//! | `dict`            | `HashMap<K, V>`     | **unordered**, iteration order is random |
//! | `dict` (sorted)   | `BTreeMap<K, V>`    | always sorted by key               |
//! | `set`             | `HashSet<T>`        |                                    |
//! | `list`            | `Vec<T>`            |                                    |
//! | `collections.deque` | `VecDeque<T>`     | O(1) push/pop at both ends         |
//! | `d[k] += 1` (KeyError!) | `*m.entry(k).or_insert(0) += 1` | the `entry` API   |
//!
//! The `entry` API is the single most useful thing here. It is Rust's answer
//! to `defaultdict` and `dict.setdefault`, and it does one hash lookup, not two.
//!
//! Run: `cargo test -p exercises ex05`

use std::collections::BTreeMap;
use std::collections::HashMap;
use std::collections::VecDeque;

/// Task 1 — word frequency, `BTreeMap` so the output is deterministic.
///
/// - lowercase everything
/// - a "word" is a run of alphanumeric characters; split on everything else
/// - skip empty pieces
pub fn word_freq(text: &str) -> BTreeMap<String, usize> {
    todo!("split_whitespace won't cut it — use split(|c: char| !c.is_alphanumeric())")
}

/// Task 2 — deduplicate while preserving first-seen order.
///
/// (Python 3.7+ gets this free from `dict.fromkeys`. In Rust you pair a
/// `HashSet` for membership with a `Vec` for order.)
pub fn dedupe_preserve_order(items: &[&str]) -> Vec<String> {
    todo!()
}

/// Task 3 — layered config merge: `overlay` wins over `base`.
///
/// Neither input may be mutated — build and return a new map.
pub fn merge_config(
    base: &HashMap<String, String>,
    overlay: &HashMap<String, String>,
) -> HashMap<String, String> {
    todo!()
}

/// Task 4 — invert a map, grouping keys that shared a value.
///
/// The `Vec<String>` values must be sorted ascending so the result is
/// deterministic.
///
/// Example: `{a: x, b: y, c: x}` -> `{x: [a, c], y: [b]}`
pub fn invert(map: &HashMap<String, String>) -> HashMap<String, Vec<String>> {
    todo!("entry(..).or_default().push(..), then sort each Vec")
}

/// Task 5 — a sliding-window rate limiter, the real thing.
///
/// Keeps timestamps of recent events in a `VecDeque`. On each call:
/// 1. evict every event `t` where `now_ms - t >= window_ms`
/// 2. if fewer than `max_events` remain, record `now_ms` and allow (`true`)
/// 3. otherwise reject (`false`) **without** recording
///
/// Time is passed in rather than read from the clock — that is what makes it
/// testable. You will see this same trick in project 2.
pub struct RateLimiter {
    window_ms: u64,
    max_events: usize,
    events: VecDeque<u64>,
}

impl RateLimiter {
    pub fn new(window_ms: u64, max_events: usize) -> Self {
        Self {
            window_ms,
            max_events,
            events: VecDeque::new(),
        }
    }

    /// Number of events currently inside the window.
    pub fn len(&self) -> usize {
        self.events.len()
    }

    pub fn is_empty(&self) -> bool {
        self.events.is_empty()
    }

    pub fn allow(&mut self, now_ms: u64) -> bool {
        todo!("evict from the front, then decide")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn map(pairs: &[(&str, &str)]) -> HashMap<String, String> {
        pairs
            .iter()
            .map(|(k, v)| (k.to_string(), v.to_string()))
            .collect()
    }

    #[test]
    fn t1_word_freq() {
        let got = word_freq("The quick brown fox; the QUICK fox!");
        let want: BTreeMap<String, usize> = [("brown", 1), ("fox", 2), ("quick", 2), ("the", 2)]
            .iter()
            .map(|(k, v)| (k.to_string(), *v))
            .collect();
        assert_eq!(got, want);

        assert_eq!(word_freq(""), BTreeMap::new());
        assert_eq!(word_freq("   ...   "), BTreeMap::new());

        // Digits count as alphanumeric.
        let got = word_freq("err404 err404 ok");
        assert_eq!(got.get("err404"), Some(&2));
    }

    #[test]
    fn t2_dedupe_preserve_order() {
        assert_eq!(
            dedupe_preserve_order(&["b", "a", "b", "c", "a"]),
            vec![
                String::from("b"),
                String::from("a"),
                String::from("c")
            ]
        );
        assert_eq!(dedupe_preserve_order(&[]), Vec::<String>::new());
    }

    #[test]
    fn t3_merge_config() {
        let base = map(&[("host", "localhost"), ("port", "80"), ("tls", "false")]);
        let overlay = map(&[("port", "8443"), ("tls", "true")]);

        let got = merge_config(&base, &overlay);
        assert_eq!(got.get("host").map(String::as_str), Some("localhost"));
        assert_eq!(got.get("port").map(String::as_str), Some("8443"));
        assert_eq!(got.get("tls").map(String::as_str), Some("true"));
        assert_eq!(got.len(), 3);

        // Inputs untouched.
        assert_eq!(base.get("port").map(String::as_str), Some("80"));
    }

    #[test]
    fn t4_invert() {
        let m = map(&[("a", "x"), ("b", "y"), ("c", "x")]);
        let got = invert(&m);

        assert_eq!(
            got.get("x"),
            Some(&vec![String::from("a"), String::from("c")])
        );
        assert_eq!(got.get("y"), Some(&vec![String::from("b")]));
        assert_eq!(got.len(), 2);
    }

    #[test]
    fn t5_rate_limiter() {
        // 3 events per 1000ms window.
        let mut rl = RateLimiter::new(1000, 3);

        assert!(rl.allow(0));
        assert!(rl.allow(100));
        assert!(rl.allow(200));
        assert!(!rl.allow(300), "4th event inside the window is rejected");
        assert_eq!(rl.len(), 3, "a rejected event must NOT be recorded");

        // At t=1000 the event at t=0 has aged out (1000 - 0 >= 1000).
        assert!(rl.allow(1000));
        assert_eq!(rl.len(), 3, "t=100, t=200, t=1000");

        assert!(!rl.allow(1001));

        // Long gap: everything ages out.
        assert!(rl.allow(9999));
        assert_eq!(rl.len(), 1);
    }

    #[test]
    fn t5b_rate_limiter_zero_budget() {
        let mut rl = RateLimiter::new(1000, 0);
        assert!(!rl.allow(0));
        assert!(rl.is_empty());
    }
}
