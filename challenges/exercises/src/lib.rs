//! # Rust exercises — 10 graded tasks
//!
//! Every module below is a self-contained exercise. Each function is a stub
//! that calls `todo!()`; each module has a `mod tests` block at the bottom
//! that defines exactly what "correct" means.
//!
//! Workflow:
//!
//! ```text
//! cargo test -p exercises ex01      # run just exercise 1
//! cargo test -p exercises           # run everything
//! cargo test -p exercises ex04 -- --nocapture   # see your println! output
//! ```
//!
//! Replace `todo!()` with real code until the tests go green. Do not edit the
//! tests — if a test looks wrong, that is a signal to re-read the doc comment
//! above the function.
//!
//! Order matters: 01 → 10 is a difficulty ladder, and later exercises assume
//! the earlier ones clicked.

// Stubs mean lots of unused parameters and unread fields until you fill them
// in. Delete these `allow`s once you have finished — a clean build with zero
// warnings is the real finish line.
#![allow(unused_variables, dead_code, unused_mut, unused_imports)]

pub mod ex01_ownership;
pub mod ex02_option_result;
pub mod ex03_traits;
pub mod ex04_iterators;
pub mod ex05_collections;
pub mod ex06_errors;
pub mod ex07_lifetimes;
pub mod ex08_shared_state;
pub mod ex09_threads;
pub mod ex10_async;
