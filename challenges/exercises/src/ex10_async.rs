//! # Exercise 10 — async/await and tokio
//!
//! **Scenario.** A service that talks to slow backends: fetch many things at
//! once, cap the concurrency, time things out, race them, and abstract the
//! backend behind a trait.
//!
//! **Python contrast.** `async def` / `await` / `asyncio.gather` map almost
//! one-to-one. Two things are genuinely different:
//!
//! 1. **Futures are lazy.** In Python, calling a coroutine and never awaiting
//!    it at least schedules nothing but warns. In Rust, an un-awaited future
//!    is *literally a struct sitting on the stack doing nothing*. No runtime,
//!    no progress.
//! 2. **`Send` is enforced.** tokio's multi-threaded runtime may resume your
//!    task on a different thread after each `.await`, so everything held
//!    across an await must be `Send`. That is the origin of the famous
//!    "future cannot be sent between threads safely" error.
//!
//! | Python                        | Rust + tokio                       |
//! |-------------------------------|------------------------------------|
//! | `await f()`                   | `f().await`                        |
//! | `asyncio.gather(*fs)`         | `futures::future::join_all(fs)`    |
//! | `asyncio.create_task(f())`    | `tokio::spawn(f())`                |
//! | `asyncio.wait_for(f(), 1.0)`  | `tokio::time::timeout(dur, f())`   |
//! | `asyncio.Semaphore(4)`        | `tokio::sync::Semaphore::new(4)`   |
//! | `asyncio.wait(FIRST_COMPLETED)` | `tokio::select! { .. }`          |
//! | `asyncio.sleep(1)`            | `tokio::time::sleep(dur).await`    |
//!
//! The tests use `#[tokio::test(start_paused = true)]`, which gives the
//! runtime a **virtual clock**: sleeps resolve instantly but elapsed time is
//! reported as if they really happened. That makes timing assertions exact
//! instead of flaky. Note that it only works with `tokio::time::Instant`,
//! not `std::time::Instant`.
//!
//! Run: `cargo test -p exercises ex10`

use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::time::Duration;

/// Provided: a fake network call. Always takes 30ms. Ids divisible by 7 fail.
pub async fn fetch(id: u32) -> Result<String, String> {
    tokio::time::sleep(Duration::from_millis(30)).await;
    if id.is_multiple_of(7) {
        Err(format!("id {id} failed"))
    } else {
        Ok(format!("payload-{id}"))
    }
}

/// Task 1 — fetch every id **concurrently**, results in input order.
///
/// The trap: a plain `for id in ids { out.push(fetch(id).await) }` compiles,
/// runs, and is completely sequential — 10 ids would take 300ms instead of 30.
/// The timing assertion in the test exists specifically to catch that.
pub async fn fetch_all(ids: Vec<u32>) -> Vec<Result<String, String>> {
    todo!("build the futures first, then drive them all at once")
}

/// Task 2 — same, but with **at most `limit` requests in flight**.
///
/// Unbounded concurrency will happily open 10,000 sockets and get you rate
/// limited. Every real client caps it.
///
/// Two idiomatic routes: a `tokio::sync::Semaphore` shared across the tasks,
/// or `futures::stream::iter(..).buffered(limit)`. Order must be preserved.
pub async fn fetch_limited(ids: Vec<u32>, limit: usize) -> Vec<Result<String, String>> {
    todo!()
}

/// Task 3 — give up after `ms` milliseconds.
///
/// - completes in time -> whatever `fetch` returned
/// - too slow          -> `Err("timeout")`
pub async fn fetch_with_timeout(id: u32, ms: u64) -> Result<String, String> {
    todo!("tokio::time::timeout returns Result<T, Elapsed> — flatten it")
}

/// Task 4 — race two timers with `select!` and report the winner.
///
/// Returns `"fast"` if the `fast_ms` timer wins, `"slow"` otherwise.
///
/// Remember what `select!` does to the loser: it **drops** that future
/// mid-flight. Here that is harmless. With a half-consumed socket read it
/// would not be — that is the "cancellation safety" property.
pub async fn first_wins(fast_ms: u64, slow_ms: u64) -> &'static str {
    todo!("tokio::select! over two sleeps")
}

/// Task 5 — an async trait, the way the codex codebase mandates it.
///
/// Spelling the return type as `impl Future<Output = ..> + Send` (instead of
/// writing `async fn` in the trait) is what guarantees callers can
/// `tokio::spawn` the result. Implementors may still use `async fn`.
pub trait Backend {
    fn load(&self, key: &str) -> impl Future<Output = Result<String, String>> + Send;
}

pub struct MemoryBackend {
    pub data: HashMap<String, String>,
}

impl MemoryBackend {
    pub fn new(pairs: &[(&str, &str)]) -> Self {
        Self {
            data: pairs
                .iter()
                .map(|(k, v)| (k.to_string(), v.to_string()))
                .collect(),
        }
    }
}

impl Backend for MemoryBackend {
    /// Writing this as a plain `async fn` is allowed: the compiler checks that
    /// the future it produces really does satisfy the `+ Send` bound above.
    ///
    /// Missing key -> `Err(format!("no such key: {key}"))`.
    async fn load(&self, key: &str) -> Result<String, String> {
        todo!()
    }
}

/// Task 5b — generic over the trait: **static** dispatch, no allocation.
pub async fn load_or_default<B: Backend>(backend: &B, key: &str, default: &str) -> String {
    todo!("call backend.load(key).await and fall back to `default` on Err")
}

/// A boxed, type-erased future. `futures::future::BoxFuture` is this exact
/// alias; it is written out here so the shape is visible.
pub type BoxFuture<'a, T> = Pin<Box<dyn Future<Output = T> + Send + 'a>>;

/// Task 6 — the **dyn-compatible** twin of `Backend`.
///
/// `impl Trait` in return position cannot be used behind `dyn`, so a trait you
/// intend to store as `Box<dyn ..>` must box its futures instead. That costs
/// one heap allocation per call and buys you a heterogeneous collection.
/// This is precisely the `Tool` vs. `ToolDyn` split in `xai-tool-runtime`.
pub trait DynBackend: Send + Sync {
    fn load(&self, key: &str) -> BoxFuture<'_, Result<String, String>>;
}

impl DynBackend for MemoryBackend {
    fn load(&self, key: &str) -> BoxFuture<'_, Result<String, String>> {
        todo!("wrap an async move block in Box::pin — same behaviour as the Backend impl")
    }
}

/// Task 6b — query several backends at once through trait objects.
pub async fn load_all(
    backends: &[Box<dyn DynBackend>],
    key: &str,
) -> Vec<Result<String, String>> {
    todo!()
}

/// Task 7 — spawn real tasks and collect their results.
///
/// Spawn one task per `i` in `0..n`, each returning `i * i`, then sum them.
///
/// `tokio::spawn` requires `Future + Send + 'static`. If you try to borrow a
/// local inside the spawned block, the `'static` bound will reject it — that
/// is the compiler telling you the task might outlive the borrow.
pub async fn spawn_sum(n: u64) -> u64 {
    todo!("tokio::spawn in a loop, then await every JoinHandle")
}

// ── Reading exercise (no code to write) ───────────────────────────────────
//
// This does not compile. Predict the error before reading the answer.
//
//     async fn broken() {
//         let counter = std::rc::Rc::new(0);
//         tokio::time::sleep(Duration::from_millis(1)).await;
//         println!("{counter}");
//     }
//     tokio::spawn(broken());
//
// Answer: `Rc` is not `Send`, and it is alive *across* the `.await`, so it
// ends up stored inside the generated future — which makes the whole future
// non-`Send`, which `tokio::spawn` rejects. Fixes: use `Arc`, or drop the
// `Rc` before the await, or use `tokio::task::spawn_local`.
//
// The same trap with `std::sync::MutexGuard` is the #1 async deadlock in
// production Rust. Hold `tokio::sync::Mutex` across awaits; hold
// `std::sync::Mutex` only inside a single synchronous stretch.

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::time::Instant;

    #[tokio::test(start_paused = true)]
    async fn t1_fetch_all_is_concurrent() {
        let start = Instant::now();
        let out = fetch_all((1..=10).collect()).await;
        let elapsed = start.elapsed();

        assert_eq!(out.len(), 10);
        assert_eq!(out[0], Ok(String::from("payload-1")));
        assert_eq!(out[6], Err(String::from("id 7 failed")), "order preserved");
        assert!(
            elapsed < Duration::from_millis(60),
            "took {elapsed:?}; 10 sequential awaits would be 300ms — \
             are you awaiting inside the loop?"
        );
    }

    #[tokio::test(start_paused = true)]
    async fn t1b_fetch_all_empty() {
        assert_eq!(fetch_all(vec![]).await, Vec::new());
    }

    #[tokio::test(start_paused = true)]
    async fn t2_fetch_limited() {
        let start = Instant::now();
        let out = fetch_limited((1..=12).collect(), 4).await;
        let elapsed = start.elapsed();

        assert_eq!(out.len(), 12);
        assert_eq!(out[0], Ok(String::from("payload-1")));
        assert_eq!(out[11], Ok(String::from("payload-12")));

        // 12 requests / 4 at a time = 3 waves x 30ms.
        assert!(
            elapsed >= Duration::from_millis(90),
            "took {elapsed:?}; that is faster than 4-at-a-time allows — \
             is the limit actually being enforced?"
        );
        assert!(
            elapsed < Duration::from_millis(120),
            "took {elapsed:?}; too slow — are you running them one by one?"
        );
    }

    #[tokio::test(start_paused = true)]
    async fn t3_fetch_with_timeout() {
        assert_eq!(
            fetch_with_timeout(1, 100).await,
            Ok(String::from("payload-1"))
        );
        assert_eq!(
            fetch_with_timeout(1, 10).await,
            Err(String::from("timeout"))
        );
        assert_eq!(
            fetch_with_timeout(7, 100).await,
            Err(String::from("id 7 failed")),
            "a real failure is not a timeout"
        );
    }

    #[tokio::test(start_paused = true)]
    async fn t4_first_wins() {
        assert_eq!(first_wins(10, 50).await, "fast");
        assert_eq!(first_wins(50, 10).await, "slow");
    }

    #[tokio::test]
    async fn t5_backend_static_dispatch() {
        let b = MemoryBackend::new(&[("host", "localhost"), ("port", "8080")]);
        assert_eq!(load_or_default(&b, "host", "none").await, "localhost");
        assert_eq!(load_or_default(&b, "missing", "none").await, "none");
    }

    #[tokio::test]
    async fn t6_backend_dynamic_dispatch() {
        let backends: Vec<Box<dyn DynBackend>> = vec![
            Box::new(MemoryBackend::new(&[("a", "1")])),
            Box::new(MemoryBackend::new(&[("b", "2")])),
            Box::new(MemoryBackend::new(&[("a", "3")])),
        ];

        assert_eq!(
            load_all(&backends, "a").await,
            vec![
                Ok(String::from("1")),
                Err(String::from("no such key: a")),
                Ok(String::from("3")),
            ]
        );
    }

    #[tokio::test]
    async fn t7_spawn_sum() {
        assert_eq!(spawn_sum(0).await, 0);
        assert_eq!(spawn_sum(1).await, 0);
        assert_eq!(spawn_sum(5).await, 1 + 4 + 9 + 16);
        assert_eq!(spawn_sum(100).await, (0..100u64).map(|i| i * i).sum());
    }

    /// The future returned by an async fn must be `Send` for `tokio::spawn`
    /// to accept it. This asserts it at compile time.
    #[test]
    fn t8_futures_are_send() {
        fn assert_send<T: Send>(_: T) {}
        assert_send(fetch(1));
        assert_send(fetch_all(vec![1]));
        assert_send(spawn_sum(3));
    }
}
