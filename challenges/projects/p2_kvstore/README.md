# Project 2 — `kvstore`

A thread-safe in-memory key–value store with TTL expiry, behind a trait so the
backend can be swapped. This is the shape of every cache layer you will ever
write.

**Skills exercised:** `Send`/`Sync`, `Arc<RwLock<T>>`, interior mutability,
trait objects (`Arc<dyn Store>`), dependency injection for testability,
concurrent testing.

## The two ideas this project is really about

### 1. Methods take `&self`, not `&mut self`

```rust
fn set(&self, key: &str, value: &str);   // note: &self
```

A cache is shared by many callers at once. If `set` took `&mut self`, only one
caller could ever hold a reference. Instead the lock lives *inside* the struct
and mutation happens through a shared reference — **interior mutability**.
`Arc<RwLock<HashMap<..>>>` is the standard shape, and `RwLock` (not `Mutex`)
because reads massively outnumber writes.

### 2. Time is a dependency, not a global

`MemoryStore` never calls `SystemTime::now()` directly. It holds an
`Arc<dyn Clock>`. Production passes `SystemClock`; tests pass `TestClock` and
call `advance(5_000)`.

That is why the TTL tests run in microseconds instead of sleeping for five
seconds — and why they are not flaky. Injecting the clock is the single
highest-value testability trick in systems code.

## Your job

`src/clock.rs` and `src/store.rs` are **already written** — read them first,
they are the spec. Then fill in every `todo!()` in `src/memory.rs`.

## Semantics to get right

| Operation          | Behaviour                                                    |
|--------------------|--------------------------------------------------------------|
| `set`              | overwrite the value **and clear any existing TTL**            |
| `set_with_ttl`     | value expires at `clock.now_ms() + ttl_ms`                    |
| expired            | `expires_at` is `Some(t)` and `now >= t` (note: `>=`, not `>`)|
| `get`              | `None` if absent **or** expired                               |
| `len` / `keys`     | must **exclude** expired entries                              |
| `delete`           | `true` only if the key was present and not expired            |
| `purge_expired`    | actually removes them; returns how many                       |
| `keys`             | sorted ascending, so tests are deterministic                  |

Expiry is **lazy**: a read does not have to remove the entry, it just has to
report it as gone. `purge_expired` is the eager sweep you would run on a timer.

## Run it

```bash
cargo test -p p2_kvstore
cargo test -p p2_kvstore -- --nocapture
```

## Definition of done

- [ ] `cargo test -p p2_kvstore` passes, including the 8-thread test
- [ ] `cargo clippy -p p2_kvstore -- -D warnings` is clean
- [ ] `Arc<dyn Store>` compiles — that is the `Send + Sync` supertrait working
- [ ] no `unsafe`, and every lock is released before the function returns

## Stretch goals

1. Add `get_or_insert_with(&self, key, f)` that computes a value only if
   absent. Careful: do not hold the read lock while calling `f`.
2. Add an LRU eviction cap: a `max_entries` limit that drops the
   least-recently-read key. You will need to track access order — a
   `HashMap` + `VecDeque`, or bring in the `indexmap` crate.
3. Make `Store` async (`async fn get(&self, ..)`) and swap `RwLock` for
   `tokio::sync::RwLock`. Notice what this does to the trait: you now need
   either RPITIT or boxed futures. That is exercise 10 showing up in real work.
