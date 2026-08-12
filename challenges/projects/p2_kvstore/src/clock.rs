//! Time as an injectable dependency.
//!
//! **This file is complete — nothing to implement here.** Read it, though:
//! the pattern is worth more than the code.
//!
//! Reading the wall clock directly from inside business logic makes that logic
//! untestable — you end up writing `thread::sleep(6_000)` in tests, which is
//! slow *and* flaky. Taking the clock as a dependency means tests control time
//! exactly.

use std::sync::Arc;
use std::sync::atomic::AtomicU64;
use std::sync::atomic::Ordering;
use std::time::SystemTime;
use std::time::UNIX_EPOCH;

/// A source of the current time, in milliseconds.
///
/// `Send + Sync` is a **supertrait bound**: it means every implementor is
/// thread-shareable, which in turn is what makes `Arc<dyn Clock>` itself
/// `Send + Sync`. Without it, `MemoryStore` could not be shared across
/// threads no matter what else it did.
pub trait Clock: Send + Sync {
    fn now_ms(&self) -> u64;
}

/// The real clock. Milliseconds since the Unix epoch.
#[derive(Debug, Default, Clone, Copy)]
pub struct SystemClock;

impl Clock for SystemClock {
    fn now_ms(&self) -> u64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_millis() as u64)
            .unwrap_or(0)
    }
}

/// A clock that only moves when a test tells it to.
///
/// Cloning shares the same underlying instant (the `Arc`), so a test can hand
/// one clone to the store, keep another, and drive time from the outside.
#[derive(Debug, Clone, Default)]
pub struct TestClock {
    now_ms: Arc<AtomicU64>,
}

impl TestClock {
    pub fn new(start_ms: u64) -> Self {
        Self {
            now_ms: Arc::new(AtomicU64::new(start_ms)),
        }
    }

    /// Move time forward.
    pub fn advance(&self, ms: u64) {
        self.now_ms.fetch_add(ms, Ordering::SeqCst);
    }

    /// Jump to an absolute instant.
    pub fn set(&self, ms: u64) {
        self.now_ms.store(ms, Ordering::SeqCst);
    }
}

impl Clock for TestClock {
    fn now_ms(&self) -> u64 {
        self.now_ms.load(Ordering::SeqCst)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clock_advances_only_on_demand() {
        let c = TestClock::new(1_000);
        assert_eq!(c.now_ms(), 1_000);
        assert_eq!(c.now_ms(), 1_000, "does not tick by itself");

        c.advance(500);
        assert_eq!(c.now_ms(), 1_500);

        c.set(42);
        assert_eq!(c.now_ms(), 42);
    }

    #[test]
    fn clones_share_one_instant() {
        let a = TestClock::new(0);
        let b = a.clone();
        a.advance(10);
        assert_eq!(b.now_ms(), 10);
    }

    #[test]
    fn system_clock_is_plausible() {
        // Some time after 2020-01-01.
        assert!(SystemClock.now_ms() > 1_577_836_800_000);
    }
}
