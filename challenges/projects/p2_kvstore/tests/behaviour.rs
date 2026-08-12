//! Integration tests — the real grader for project 2.

use std::sync::Arc;
use std::thread;

use kvstore::MemoryStore;
use kvstore::Store;
use kvstore::TestClock;

fn store_at(t: u64) -> (MemoryStore, TestClock) {
    let clock = TestClock::new(t);
    let store = MemoryStore::with_clock(Arc::new(clock.clone()));
    (store, clock)
}

#[test]
fn basic_crud() {
    let (s, _c) = store_at(0);

    s.set("host", "localhost");
    s.set("port", "8080");

    assert_eq!(s.get("host"), Some(String::from("localhost")));
    assert_eq!(s.len(), 2);
    assert_eq!(s.keys(), vec![String::from("host"), String::from("port")]);

    assert!(s.delete("host"));
    assert!(!s.delete("host"), "second delete is a no-op");
    assert_eq!(s.get("host"), None);
    assert_eq!(s.len(), 1);
}

#[test]
fn keys_are_sorted() {
    let (s, _c) = store_at(0);
    for k in ["zebra", "apple", "mango"] {
        s.set(k, "x");
    }
    assert_eq!(
        s.keys(),
        vec![
            String::from("apple"),
            String::from("mango"),
            String::from("zebra")
        ]
    );
}

#[test]
fn ttl_expires_without_sleeping() {
    let (s, clock) = store_at(1_000);

    s.set_with_ttl("session", "abc", 500);
    assert_eq!(s.get("session"), Some(String::from("abc")));

    clock.advance(499);
    assert_eq!(s.get("session"), Some(String::from("abc")), "not yet");

    clock.advance(1);
    assert_eq!(s.get("session"), None, "expiry is inclusive: now == deadline");
    assert_eq!(s.len(), 0, "len must exclude expired entries");
    assert!(s.keys().is_empty(), "keys must exclude expired entries");
    assert!(!s.delete("session"), "deleting an expired key is a no-op");
}

#[test]
fn set_clears_an_existing_ttl() {
    let (s, clock) = store_at(0);

    s.set_with_ttl("k", "v1", 100);
    s.set("k", "v2");

    clock.advance(10_000);
    assert_eq!(
        s.get("k"),
        Some(String::from("v2")),
        "plain set() must remove the old deadline"
    );
}

#[test]
fn set_with_ttl_refreshes_the_deadline() {
    let (s, clock) = store_at(0);

    s.set_with_ttl("k", "v", 100);
    clock.advance(90);
    s.set_with_ttl("k", "v", 100); // deadline is now 190

    clock.advance(90); // t = 180
    assert_eq!(s.get("k"), Some(String::from("v")));

    clock.advance(10); // t = 190
    assert_eq!(s.get("k"), None);
}

#[test]
fn purge_expired_reclaims_memory() {
    let (s, clock) = store_at(0);

    s.set("permanent", "v");
    s.set_with_ttl("a", "v", 100);
    s.set_with_ttl("b", "v", 100);
    s.set_with_ttl("c", "v", 10_000);

    clock.advance(100);

    assert_eq!(s.len(), 2, "lazy: a and b are already invisible");
    assert_eq!(s.purge_expired(), 2, "eager: a and b actually removed");
    assert_eq!(s.purge_expired(), 0, "nothing left to purge");
    assert_eq!(
        s.keys(),
        vec![String::from("c"), String::from("permanent")]
    );
}

#[test]
fn clones_share_state() {
    let (a, _c) = store_at(0);
    let b = a.clone();

    b.set("from_b", "1");
    assert_eq!(
        a.get("from_b"),
        Some(String::from("1")),
        "a clone is a handle, not a copy"
    );
}

/// The payoff of `Store: Send + Sync`: the trait is object-safe *and*
/// thread-shareable, so callers can hold `Arc<dyn Store>` and swap backends
/// at runtime without changing a single call site.
#[test]
fn works_behind_a_trait_object() {
    let s: Arc<dyn Store> = Arc::new(MemoryStore::new());
    s.set("k", "v");
    assert_eq!(s.get("k"), Some(String::from("v")));
    assert_eq!(s.len(), 1);

    fn assert_send_sync<T: Send + Sync + ?Sized>() {}
    assert_send_sync::<dyn Store>();
    assert_send_sync::<MemoryStore>();
    assert_send_sync::<Arc<dyn Store>>();
}

#[test]
fn survives_eight_threads() {
    let store = Arc::new(MemoryStore::new());
    let writers = 8;
    let per_writer = 500;

    let handles: Vec<_> = (0..writers)
        .map(|w| {
            let store = Arc::clone(&store);
            thread::spawn(move || {
                for i in 0..per_writer {
                    store.set(&format!("w{w}-k{i}"), "v");
                    // Interleave reads so the RwLock is genuinely contended.
                    let _ = store.get(&format!("w{w}-k{i}"));
                }
            })
        })
        .collect();

    for h in handles {
        h.join().expect("a writer thread panicked");
    }

    assert_eq!(store.len(), writers * per_writer);
    assert_eq!(store.get("w0-k0"), Some(String::from("v")));
    assert_eq!(store.get("w7-k499"), Some(String::from("v")));
}

#[test]
fn concurrent_readers_do_not_block_each_other() {
    let store = Arc::new(MemoryStore::new());
    store.set("shared", "value");

    let handles: Vec<_> = (0..8)
        .map(|_| {
            let store = Arc::clone(&store);
            thread::spawn(move || {
                let mut hits = 0;
                for _ in 0..1_000 {
                    if store.get("shared").is_some() {
                        hits += 1;
                    }
                }
                hits
            })
        })
        .collect();

    let total: usize = handles
        .into_iter()
        .map(|h| h.join().expect("a reader thread panicked"))
        .sum();
    assert_eq!(total, 8_000);
}

#[test]
fn the_default_constructor_wires_up_a_real_clock() {
    // No TestClock here — this exercises `MemoryStore::new()`. A 0ms TTL is
    // already expired the instant it is set, so no sleeping is needed.
    let s = MemoryStore::new();

    s.set_with_ttl("gone", "v", 0);
    assert_eq!(s.get("gone"), None, "a 0ms TTL is expired on arrival");

    s.set_with_ttl("here", "v", 60_000);
    assert_eq!(s.get("here"), Some(String::from("v")));
    assert_eq!(s.len(), 1);
}
