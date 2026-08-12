//! The in-memory implementation. **This is the file you write.**

use std::collections::HashMap;
use std::sync::Arc;
use std::sync::RwLock;

use crate::clock::Clock;
use crate::clock::SystemClock;
use crate::store::Store;

/// One stored value plus its optional deadline.
#[derive(Debug, Clone, PartialEq, Eq)]
struct Entry {
    value: String,
    /// Absolute deadline in clock-milliseconds. `None` means "never expires".
    expires_at_ms: Option<u64>,
}

impl Entry {
    /// TODO 1 — is this entry dead as of `now_ms`?
    ///
    /// Careful with the boundary: an entry set at t=0 with `ttl_ms = 100`
    /// expires at exactly t=100. Use `>=`, not `>`.
    fn is_expired(&self, now_ms: u64) -> bool {
        todo!()
    }
}

/// A thread-safe hash map with TTL.
///
/// Cloning is cheap and **shares** state — every clone is a handle onto the
/// same `Arc`. That is the same pattern as `HunkTrackerHandle` in the grok
/// codebase and `Registry` in exercise 8.
#[derive(Clone)]
pub struct MemoryStore {
    inner: Arc<RwLock<HashMap<String, Entry>>>,
    clock: Arc<dyn Clock>,
}

impl Default for MemoryStore {
    fn default() -> Self {
        Self::new()
    }
}

impl MemoryStore {
    /// A store backed by the real wall clock.
    pub fn new() -> Self {
        Self::with_clock(Arc::new(SystemClock))
    }

    /// TODO 2 — a store with an injected clock. Tests use this with
    /// `TestClock` so they can control expiry without sleeping.
    pub fn with_clock(clock: Arc<dyn Clock>) -> Self {
        todo!()
    }

    /// Current time according to this store's clock.
    fn now(&self) -> u64 {
        self.clock.now_ms()
    }
}

impl Store for MemoryStore {
    /// TODO 3 — insert with no expiry, replacing whatever was there.
    ///
    /// Note the shape: take the write lock, mutate, let the guard drop at the
    /// end of the statement. Never hold a lock longer than the mutation.
    fn set(&self, key: &str, value: &str) {
        todo!()
    }

    /// TODO 4 — insert with a deadline of `now + ttl_ms`.
    fn set_with_ttl(&self, key: &str, value: &str, ttl_ms: u64) {
        todo!()
    }

    /// TODO 5 — read. Expired entries report as absent (but you do not have
    /// to remove them here — that is `purge_expired`'s job).
    fn get(&self, key: &str) -> Option<String> {
        todo!()
    }

    /// TODO 6 — remove. `true` only if the key was there **and live**.
    /// Deleting an already-expired key returns `false`.
    fn delete(&self, key: &str) -> bool {
        todo!()
    }

    /// TODO 7 — count only the live entries.
    fn len(&self) -> usize {
        todo!()
    }

    /// TODO 8 — live keys, sorted ascending.
    fn keys(&self) -> Vec<String> {
        todo!()
    }

    /// TODO 9 — sweep. Remove every expired entry, return the count.
    ///
    /// Hint: `HashMap::retain` takes a closure and keeps what returns `true`.
    fn purge_expired(&self) -> usize {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::clock::TestClock;

    fn store_at(t: u64) -> (MemoryStore, TestClock) {
        let clock = TestClock::new(t);
        let store = MemoryStore::with_clock(Arc::new(clock.clone()));
        (store, clock)
    }

    #[test]
    fn entry_expiry_boundary() {
        let never = Entry {
            value: String::from("v"),
            expires_at_ms: None,
        };
        assert!(!never.is_expired(u64::MAX));

        let at_100 = Entry {
            value: String::from("v"),
            expires_at_ms: Some(100),
        };
        assert!(!at_100.is_expired(99));
        assert!(at_100.is_expired(100), "expiry is inclusive");
        assert!(at_100.is_expired(101));
    }

    #[test]
    fn set_and_get() {
        let (s, _clock) = store_at(0);
        assert!(s.is_empty());

        s.set("a", "1");
        assert_eq!(s.get("a"), Some(String::from("1")));
        assert_eq!(s.get("missing"), None);

        s.set("a", "2");
        assert_eq!(s.get("a"), Some(String::from("2")), "overwrite");
        assert_eq!(s.len(), 1);
    }
}
