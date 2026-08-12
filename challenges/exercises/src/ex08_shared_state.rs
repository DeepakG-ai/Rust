//! # Exercise 08 — Shared ownership and interior mutability
//!
//! **Scenario.** Two objects need to see the same buffer. Eight threads need
//! to bump the same counter. In Python this is free — every name is a
//! reference and the GIL hides the rest. In Rust you pick the exact tool:
//!
//! | Need                                  | Single-threaded | Multi-threaded    |
//! |---------------------------------------|-----------------|-------------------|
//! | many owners of one value              | `Rc<T>`         | `Arc<T>`          |
//! | mutate through a shared reference     | `RefCell<T>`    | `Mutex<T>` / `RwLock<T>` |
//! | both                                  | `Rc<RefCell<T>>`| `Arc<Mutex<T>>`   |
//!
//! - `Rc` is a non-atomic refcount: fast, but **not `Send`**. Try to move one
//!   into a thread and the compiler stops you. `Arc` is the atomic version.
//! - `RefCell` moves the borrow check from compile time to **run time**: two
//!   simultaneous `borrow_mut()` calls *panic* instead of failing to compile.
//! - The magic word for all of these is **interior mutability**: methods that
//!   mutate while taking `&self`, not `&mut self`. Look at `Registry` below —
//!   `insert` takes `&self`. That is the whole reason the pattern exists.
//!
//! Run: `cargo test -p exercises ex08`

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::RwLock;

/// Task 1 — predict-and-verify. Do this one on paper first.
///
/// Build `let a = Rc::new(String::from("cfg"));` then:
///   1. clone it once, record `Rc::strong_count(&a)`
///   2. open an inner scope, clone again, record the count
///   3. let the inner scope end, record the count again
///
/// Return the three counts in order.
pub fn rc_counts() -> (usize, usize, usize) {
    todo!("Rc::strong_count(&a) at three points in time")
}

/// Task 2 — two "subscribers" writing to one shared buffer.
///
/// Create an `Rc<RefCell<Vec<String>>>`, hand a clone to each of two writers,
/// push all of `a_msgs` through the first and all of `b_msgs` through the
/// second, then return the final contents of the shared buffer.
pub fn shared_buffer(a_msgs: &[&str], b_msgs: &[&str]) -> Vec<String> {
    todo!("Rc::clone for each writer, borrow_mut() to push")
}

/// Task 3 — `Arc<Mutex<T>>` across real threads.
///
/// Spawn `threads` threads; each increments a shared counter
/// `increments_per_thread` times. Join them all and return the final value.
///
/// Expected result: `threads * increments_per_thread`, every single run. If
/// you get a smaller number you have a data race — except you cannot, because
/// Rust would not have compiled it.
pub fn parallel_counter(threads: usize, increments_per_thread: usize) -> usize {
    todo!("Arc::clone into each thread, lock().unwrap() to touch the value")
}

/// Task 4 — `std::thread::scope`: borrow non-`'static` data in threads.
///
/// `thread::spawn` demands `'static` because the thread might outlive the
/// caller. **Scoped** threads are guaranteed to finish before the scope ends,
/// so they can borrow `data` directly — no `Arc`, no clone.
///
/// Split `data` into `chunks` roughly-equal pieces, sum each in its own
/// scoped thread, and return the total. `chunks == 0` behaves like `1`.
/// Empty `data` returns `0`.
pub fn parallel_sum(data: &[i64], chunks: usize) -> i64 {
    todo!("use std::thread::scope so the closure can borrow `data` directly")
}

/// Task 5 — the shareable handle pattern.
///
/// This is the shape you saw in the real codebases: a struct that is nothing
/// but an `Arc`, so cloning it is cheap and every clone sees the same state.
/// Note that **every method takes `&self`**, never `&mut self`.
///
/// `RwLock` over `Mutex` because reads vastly outnumber writes: many readers
/// may hold the lock at once, writers get exclusive access.
#[derive(Clone)]
pub struct Registry {
    inner: Arc<RwLock<HashMap<String, u64>>>,
}

impl Default for Registry {
    fn default() -> Self {
        Self::new()
    }
}

impl Registry {
    pub fn new() -> Self {
        Self {
            inner: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Overwrite `key`.
    pub fn set(&self, key: &str, value: u64) {
        todo!("write().unwrap()")
    }

    /// Read `key`. Returns a **copy**, not a reference — you must not hand out
    /// a reference into data protected by a lock you are about to release.
    pub fn get(&self, key: &str) -> Option<u64> {
        todo!("read().unwrap()")
    }

    /// Increment `key` by 1 (starting from 0) and return the new value.
    /// Must be atomic with respect to other threads: take the write lock once.
    pub fn incr(&self, key: &str) -> u64 {
        todo!()
    }

    pub fn len(&self) -> usize {
        todo!()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

/// A compile-time assertion helper, lifted from `xai-tool-runtime`'s test
/// suite. It has no body and is never called at runtime — its only job is to
/// fail the build if `T` stops being thread-shareable.
pub fn assert_send_sync<T: Send + Sync>() {}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;

    #[test]
    fn t1_rc_counts() {
        assert_eq!(rc_counts(), (2, 3, 2));
    }

    #[test]
    fn t2_shared_buffer() {
        assert_eq!(
            shared_buffer(&["a1", "a2"], &["b1"]),
            vec![
                String::from("a1"),
                String::from("a2"),
                String::from("b1")
            ]
        );
        assert_eq!(shared_buffer(&[], &[]), Vec::<String>::new());
    }

    #[test]
    fn t3_parallel_counter() {
        assert_eq!(parallel_counter(8, 1000), 8000);
        assert_eq!(parallel_counter(1, 5), 5);
        assert_eq!(parallel_counter(4, 0), 0);
    }

    #[test]
    fn t4_parallel_sum() {
        let data: Vec<i64> = (1..=100).collect();
        assert_eq!(parallel_sum(&data, 4), 5050);
        assert_eq!(parallel_sum(&data, 7), 5050, "uneven chunks still work");
        assert_eq!(parallel_sum(&data, 1), 5050);
        assert_eq!(parallel_sum(&data, 0), 5050, "0 chunks behaves like 1");
        assert_eq!(parallel_sum(&[], 4), 0);
        assert_eq!(parallel_sum(&[-5, 5], 2), 0);
    }

    #[test]
    fn t5_registry_basics() {
        let r = Registry::new();
        assert!(r.is_empty());

        r.set("port", 8080);
        assert_eq!(r.get("port"), Some(8080));
        assert_eq!(r.get("missing"), None);

        assert_eq!(r.incr("hits"), 1);
        assert_eq!(r.incr("hits"), 2);
        assert_eq!(r.get("hits"), Some(2));
        assert_eq!(r.len(), 2);
    }

    #[test]
    fn t5b_registry_is_shared_not_copied() {
        let a = Registry::new();
        let b = a.clone();

        b.set("from_b", 1);
        assert_eq!(
            a.get("from_b"),
            Some(1),
            "clone must share state, not duplicate it"
        );
    }

    #[test]
    fn t5c_registry_under_contention() {
        let r = Registry::new();
        let handles: Vec<_> = (0..8)
            .map(|_| {
                let r = r.clone();
                thread::spawn(move || {
                    for _ in 0..1000 {
                        r.incr("hits");
                    }
                })
            })
            .collect();

        for h in handles {
            h.join().expect("worker panicked");
        }
        assert_eq!(r.get("hits"), Some(8000));
    }

    #[test]
    fn t6_registry_is_send_and_sync() {
        // If this line stops compiling, someone put an `Rc` or a `RefCell`
        // inside `Registry`.
        assert_send_sync::<Registry>();
        assert_send_sync::<Arc<Mutex<Vec<u8>>>>();

        // Uncomment either line and read the error — this is the exact
        // diagnostic behind "future cannot be sent between threads safely".
        // assert_send_sync::<Rc<u8>>();
        // assert_send_sync::<RefCell<u8>>();
    }
}
