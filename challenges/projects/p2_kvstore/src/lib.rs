//! # kvstore — a thread-safe in-memory cache with TTL
//!
//! See `README.md` for the spec. Read `clock.rs` and `store.rs` first (they
//! are complete), then implement `memory.rs`.

#![allow(unused_variables, dead_code, unused_imports)]

pub mod clock;
pub mod memory;
pub mod store;

pub use clock::Clock;
pub use clock::SystemClock;
pub use clock::TestClock;
pub use memory::MemoryStore;
pub use store::Store;
