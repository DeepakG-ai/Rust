//! The storage abstraction.
//!
//! **This file is complete — nothing to implement here.** It is the contract
//! that `memory.rs` must satisfy, and it is what a future `RedisStore` or
//! `SqliteStore` would satisfy too.

/// A key–value store with optional per-key expiry.
///
/// Two things about this trait shape are deliberate:
///
/// 1. **`Send + Sync` supertraits.** Auto traits are *not* inherited by
///    `dyn Trait` — you have to say them. Writing them here means every
///    `Arc<dyn Store>` is automatically shareable across threads, and callers
///    never have to repeat the bound.
///
/// 2. **Every method takes `&self`.** A store is shared; if mutation required
///    `&mut self`, only one caller could ever hold it. The lock goes inside
///    the implementation instead. This is *interior mutability*.
pub trait Store: Send + Sync {
    /// Insert or overwrite `key`. Clears any TTL the key previously had.
    fn set(&self, key: &str, value: &str);

    /// Insert or overwrite `key`, expiring `ttl_ms` from now.
    fn set_with_ttl(&self, key: &str, value: &str, ttl_ms: u64);

    /// Current value, or `None` if the key is absent or expired.
    ///
    /// Returns an owned `String`, not a `&str`: handing out a reference into
    /// data guarded by a lock you are about to drop is exactly the bug the
    /// borrow checker exists to prevent.
    fn get(&self, key: &str) -> Option<String>;

    /// Remove `key`. Returns whether it was present and live.
    fn delete(&self, key: &str) -> bool;

    /// Number of live (non-expired) entries.
    fn len(&self) -> usize;

    /// All live keys, sorted ascending.
    fn keys(&self) -> Vec<String>;

    /// Eagerly drop every expired entry. Returns how many were removed.
    fn purge_expired(&self) -> usize;

    /// Provided method — implementors get this for free.
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
}
