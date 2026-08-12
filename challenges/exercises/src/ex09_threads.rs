//! # Exercise 09 — OS threads and channels
//!
//! **Scenario.** A batch job runner: fan work out to N workers, fan results
//! back in. Do this with real OS threads *before* touching tokio, because the
//! async version is the same shape with different keywords.
//!
//! **Python contrast.** No GIL here — `std::thread` gives you genuine
//! parallelism on all cores. `std::sync::mpsc` is `queue.Queue`, except the
//! *type* tells you it is multi-producer / single-consumer, and the channel
//! closing is signalled by the sender being dropped rather than by a sentinel
//! value.
//!
//! The mental model to carry into exercise 10:
//!
//! | OS threads                  | tokio                       |
//! |-----------------------------|-----------------------------|
//! | `thread::spawn`             | `tokio::spawn`              |
//! | `handle.join()`             | `handle.await`              |
//! | `std::sync::mpsc`           | `tokio::sync::mpsc`         |
//! | `thread::sleep` (blocks!)   | `tokio::time::sleep` (yields) |
//! | one thread per task, ~8 MB stack | thousands of tasks per thread |
//!
//! Run: `cargo test -p exercises ex09`

use std::collections::VecDeque;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

/// A deliberately slow unit of work, so parallelism is observable.
pub fn slow_square(n: u64) -> u64 {
    thread::sleep(Duration::from_millis(25));
    n * n
}

/// Task 1 — fan out, then fan back in **in the original order**.
///
/// Split `jobs` across at most `workers` threads, apply `slow_square` to each,
/// and return the results positionally aligned with the input.
///
/// The ordering requirement is the interesting part: threads finish in
/// whatever order they like, so you either chunk the input and reassemble, or
/// send `(index, value)` pairs down a channel and sort at the end.
///
/// `workers == 0` behaves like `1`. Empty input returns an empty `Vec`.
pub fn map_parallel(jobs: Vec<u64>, workers: usize) -> Vec<u64> {
    todo!("chunk the input, spawn a thread per chunk, join and flatten")
}

/// Task 2 — many producers, one consumer.
///
/// Split `values` across `producers` threads. Each thread sends its items into
/// a shared `mpsc` channel. The main thread receives everything and returns
/// the sum.
///
/// Gotcha: the receive loop only ends when **every** sender has been dropped.
/// If you keep the original `tx` alive in the main thread, `for v in rx` hangs
/// forever. Drop it (or move it into the last producer) before you start
/// receiving.
pub fn channel_sum(values: Vec<i64>, producers: usize) -> i64 {
    todo!("clone tx per producer, drop the original, then drain rx")
}

/// Task 3 — a two-stage pipeline wired with channels.
///
/// - stage 1: parse each string to `i64`, silently dropping anything that
///   fails, and forward it on
/// - stage 2: keep only even numbers
/// - main: collect and return them **sorted ascending**
///
/// Each stage runs in its own thread and is connected only by a channel.
/// This is the shape of every stream-processing system you will ever write.
pub fn pipeline(raw: Vec<String>) -> Vec<i64> {
    todo!("two channels, two spawned stages, collect in main")
}

/// Task 4 — a shared work queue (poor man's work stealing).
///
/// Put all `jobs` into an `Arc<Mutex<VecDeque<u64>>>`. Spawn `workers`
/// threads; each loops: lock, `pop_front`, unlock, then compute `n * n` and
/// push the result to a shared results vector. Stop when the queue is empty.
///
/// Return the results **sorted ascending**. Every job must be processed
/// exactly once — no duplicates, no drops.
///
/// Critical detail: release the queue lock *before* doing the work, or your
/// "parallel" pool runs strictly serially.
pub fn work_stealing(jobs: Vec<u64>, workers: usize) -> Vec<u64> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Instant;

    #[test]
    fn t1_map_parallel_correctness() {
        assert_eq!(map_parallel(vec![1, 2, 3, 4], 2), vec![1, 4, 9, 16]);
        assert_eq!(map_parallel(vec![5], 4), vec![25]);
        assert_eq!(map_parallel(vec![], 4), Vec::<u64>::new());
        assert_eq!(map_parallel(vec![1, 2, 3], 0), vec![1, 4, 9]);

        let jobs: Vec<u64> = (1..=9).collect();
        let want: Vec<u64> = (1..=9).map(|n| n * n).collect();
        assert_eq!(
            map_parallel(jobs, 4),
            want,
            "results must stay in input order"
        );
    }

    #[test]
    fn t1b_map_parallel_is_actually_parallel() {
        // 8 jobs x 25ms = 200ms serially. With 4 workers it should be ~50ms.
        // The bound is loose because OS sleep granularity is coarse on
        // Windows; if this is flaky on a loaded machine, trust t1 instead.
        let start = Instant::now();
        let out = map_parallel((1..=8).collect(), 4);
        let elapsed = start.elapsed();

        assert_eq!(out.len(), 8);
        assert!(
            elapsed < Duration::from_millis(150),
            "took {elapsed:?} — are you actually spawning threads, \
             or looping sequentially?"
        );
    }

    #[test]
    fn t2_channel_sum() {
        assert_eq!(channel_sum((1..=100).collect(), 4), 5050);
        assert_eq!(channel_sum(vec![-5, 5], 2), 0);
        assert_eq!(channel_sum(vec![], 4), 0);
        assert_eq!(channel_sum(vec![7], 8), 7, "more producers than work");
    }

    #[test]
    fn t3_pipeline() {
        let raw: Vec<String> = ["4", "not_a_number", "7", "2", "", "10"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        assert_eq!(pipeline(raw), vec![2, 4, 10]);
        assert_eq!(pipeline(vec![]), Vec::<i64>::new());
        assert_eq!(pipeline(vec![String::from("3")]), Vec::<i64>::new());
    }

    #[test]
    fn t4_work_stealing() {
        let jobs: Vec<u64> = (1..=50).collect();
        let mut want: Vec<u64> = jobs.iter().map(|n| n * n).collect();
        want.sort_unstable();

        let got = work_stealing(jobs, 5);
        assert_eq!(got.len(), 50, "every job exactly once");
        assert_eq!(got, want);

        assert_eq!(work_stealing(vec![], 4), Vec::<u64>::new());
        assert_eq!(work_stealing(vec![3], 4), vec![9]);
    }
}
