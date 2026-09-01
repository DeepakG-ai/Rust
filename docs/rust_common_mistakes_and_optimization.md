# Rust — Common Mistakes, Hidden Behaviour, and Writing Optimised Code

A rule sheet for while you are **typing**. Not concepts — habits.
Every ❌/✅ pair below is something that either silently produces a wrong answer, or makes
a reviewer ask "why did you write it that way?"

Verified on **rustc 1.96, edition 2024**. Everything marked ✔ *verified* was actually
compiled and run — not recalled.

**Companion docs:** `rust-gaps-and-developer-essentials.md` covers Python→Rust differences
and build-level performance; this file is code-level. `rust_100_questions.md` is the drill
version of the same material.

---

## 0. The three habits that prevent most of this

| Habit | Command | Why |
|---|---|---|
| Lint everything | `cargo clippy --all-targets -- -D warnings` | Clippy catches ~80% of the mistakes in this document, by name, with the fix |
| Format everything | `cargo fmt` | Ends all formatting arguments permanently |
| Fast feedback | `cargo check` | 3–10× faster than `cargo build`; use it in a watch loop |

Put clippy in CI on day one. It is the single highest-value habit in Rust and it is
free. Everything below is what clippy *cannot* catch.

---

## 1. Hidden behaviour — things the language does that will surprise you

### 1.1 Integer overflow is checked in debug, wrapping in release ✔ *verified*

```rust
fn add(a: u8, b: u8) -> u8 { a + b }
add(250, 10)
```
- `cargo run` → **panics**: `attempt to add with overflow`
- `cargo run --release` → **prints 4**

Your tests pass, production returns garbage. This is the most dangerous default in the
language. Be explicit whenever the values are attacker- or data-controlled:

```rust
a.checked_add(b)      // -> Option<u8>, None on overflow    (parsers, untrusted input)
a.saturating_add(b)   // clamps at u8::MAX                  (counters, scores)
a.wrapping_add(b)     // deliberate wraparound              (hashes, checksums)
a.overflowing_add(b)  // -> (u8, bool) value + did-it-wrap
```

Or turn the checks on in release and pay ~1–3%:
```toml
[profile.release]
overflow-checks = true      # worth it for anything handling money or user data
```

### 1.2 `as` casts truncate silently ✔ *verified*

```rust
300_i32 as u8     // 44          — top bits thrown away, no warning
-1_i32 as u32     // 4294967295  — sign reinterpreted
1e10_f64 as i32   // 2147483647  — float→int saturates instead
```

❌ `let n = big_value as u8;`
✅ `let n = u8::try_from(big_value)?;`

Rule: **`as` is only for casts you have proven cannot lose information** (e.g. `u8 as u32`,
or `usize as u64`). Everywhere else use `TryFrom`/`try_into`. Reach for `as` on a
narrowing cast and you have written a bug you will not find for six months.

### 1.3 `let _ = x` drops immediately; `let _x = x` does not ✔ *verified*

```rust
let _a = Noisy("a");
let _b = Noisy("b");
{ let _c = Noisy("c"); }
let _ = Noisy("d");
println!("end of main");
```
Output:
```
drop c      <- end of inner block
drop d      <- IMMEDIATELY, `_` binds nothing
end of main
drop b      <- locals drop in REVERSE declaration order
drop a
```

This kills real code:
```rust
let _ = file_lock.lock();     // ❌ lock released on this very line
let _guard = file_lock.lock(); // ✅ held to end of scope
```

Also remember: **locals drop in reverse declaration order, but struct fields drop in
declaration order.** If field A must outlive field B, declare B first.

### 1.4 A `match` holds its scrutinee's temporaries for the whole match

```rust
match cache.lock().unwrap().get(key) {      // ❌ lock held across every arm
    Some(v) => expensive_thing_that_also_locks(v),   // deadlock
    None => { }
}
```
✅ End the borrow first:
```rust
let found = cache.lock().unwrap().get(key).cloned();   // guard dropped here
match found { ... }
```
This is the #1 source of self-deadlock in Rust. (`if let` in edition 2024 drops its
temporaries before the `else` block — `match` still does not.)

### 1.5 Integer division truncates toward zero, unlike Python ✔ *verified*

```
Rust:   -7 / 2 == -3    -7 % 2 == -1
Python: -7 // 2 == -4   -7 %  2 ==  1
```
Porting an algorithm that relies on floor division? Use `div_euclid` / `rem_euclid`
(verified: `-4` and `1`). Silent off-by-one in any modular-arithmetic or
grid-indexing code otherwise.

### 1.6 Some panics are compile errors ✔ *verified*

```rust
let a = 10; let b = 0;
let _ = a / b;      // error: this operation will panic at runtime
```
`#[deny(unconditional_panic)]` catches it via const-propagation. But `vec![1,2,3][10]`
compiles fine and panics at runtime — the lint only fires when the compiler can *prove*
it. Don't read "it compiled" as "it can't panic."

### 1.7 `HashMap` iteration order is random and changes between runs ✔ *verified*

Two runs, same insertions, different order. Never rely on it. If output order matters —
tests, logs, generated files, anything a human diffs — use `BTreeMap`, or collect and
`sort`. Flaky tests come from here.

### 1.8 `.len()` on a string is **bytes**, not characters

```rust
"héllo".len()            // 6
"héllo".chars().count()  // 5  (and it is O(n), not free)
```
`s[0]` doesn't compile at all. `&s[0..2]` compiles and **panics at runtime** if index 2
splits a UTF-8 character. Use `chars()`, `char_indices()`, or `s.get(0..2)` (returns
`Option`) when the content isn't ASCII.

### 1.9 `enumerate()` after `filter()` counts the *filtered* items ✔ *verified*

```rust
[10, 20, 30, 40].iter().filter(|x| **x > 15).enumerate()
// gives (0,20), (1,30), (2,40)  — NOT (1,20), (2,30), (3,40)
```
If you want original positions, `.enumerate()` **first**, then filter on `|(_, x)|`.

### 1.10 A trailing semicolon changes the return type

```rust
fn f() -> i32 { 5; }   // ❌ expected i32, found ()
fn f() -> i32 { 5 }    // ✅
```
Rust is expression-oriented. The last expression *without* a semicolon is the value.
This is 30% of every beginner's compile errors and it stops entirely once it clicks.

### 1.11 Other small ones worth memorising

| Surprise | Reality |
|---|---|
| `Vec::remove(i)` | **O(n)** — shifts everything. `swap_remove(i)` is O(1) if order doesn't matter ✔ |
| `f64` has no `Ord` | Because of `NaN`. `vec.sort()` won't compile; use `sort_by(f64::total_cmp)` |
| `#[derive(PartialEq)]` on a float field | Compiles, but `0.1+0.2 != 0.3`. Comparing floats for equality is almost always a bug |
| Integer literals default to `i32`, floats to `f64` | An untyped `let count = 0;` is an `i32` with a ~2.1 billion ceiling. With literals the compiler catches the overflow (`error: this arithmetic operation will overflow` ✔); with **runtime** values it does not, and you're back to §1.1. Annotate: `let count: u64 = 0;` |
| `..` vs `..=` | `0..5` excludes 5, `0..=5` includes it. Off-by-one factory |
| `zip` | Stops at the **shorter** iterator, silently. No error if lengths mismatch |
| Struct field privacy | Module-based, not type-based — private fields *are* visible elsewhere in the same module |
| `write!` to a `String` | Needs `use std::fmt::Write`; to a file it's `use std::io::Write`. Same macro, different trait |
| Closures capture *disjoint fields* since edition 2021 | `\|\| self.a += 1` borrows only `self.a`, not all of `self` |

---

## 2. Ownership and borrowing — mistakes while typing

### 2.1 Don't `.clone()` to silence the borrow checker

Cloning to make an error go away is fine *while learning*. In code you keep, it means you
have not worked out who owns the data — and it hides an allocation in a loop.

❌
```rust
for item in items.clone() {
    process(&mut items, item);      // cloned the whole Vec to dodge the error
}
```
✅ Restructure — collect the changes, then apply them:
```rust
let updates: Vec<_> = items.iter().map(compute_update).collect();
for u in updates { apply(&mut items, u); }
```

Three legitimate ways out of a borrow conflict, in order of preference:
1. **Shorten the borrow** — end it before the mutation (NLL means the borrow ends at last *use*, not end of scope).
2. **Split the borrow** — destructure so you borrow two fields, not the whole struct.
3. **`std::mem::take` / `replace` / `swap`** — move the value out of `&mut self`, work on it, put it back.

### 2.2 Split borrows: the compiler is smarter about fields than methods

❌ Doesn't compile — `self.helper()` borrows all of `self`:
```rust
fn run(&mut self) {
    for item in &self.items {
        self.log(item);            // error: cannot borrow `*self` as mutable
    }
}
```
✅ Destructure first — now the compiler sees two independent field borrows:
```rust
fn run(&mut self) {
    let Self { items, logger, .. } = self;
    for item in items.iter() {
        logger.log(item);          // fine: `items` and `logger` are disjoint
    }
}
```
This single trick resolves a large share of "cannot borrow as mutable" errors in
struct-heavy code.

### 2.3 Take the widest type you can, return the narrowest you must

| Instead of | Write | Why |
|---|---|---|
| `fn f(s: &String)` | `fn f(s: &str)` | Callers with a literal or a slice now work |
| `fn f(v: &Vec<i32>)` | `fn f(v: &[i32])` | Arrays, slices, and `Vec` all work |
| `fn f(p: &PathBuf)` | `fn f(p: impl AsRef<Path>)` | Accepts `&str`, `String`, `Path`, `PathBuf` |
| `fn new(name: String)` | `fn new(name: impl Into<String>)` | Caller doesn't have to write `.to_string()` |

Rule of thumb: **generic/borrowed in the parameters, concrete/owned in the return type.**

### 2.4 `mem::take` — the escape hatch you'll want weekly

```rust
fn flush(&mut self) -> Vec<Record> {
    std::mem::take(&mut self.buffer)      // leaves an empty Vec behind, no clone
}
```
Works for any `T: Default`. `mem::replace` when the replacement isn't the default.

---

## 3. Strings

### 3.1 `format!` in a loop is an allocation per iteration

❌
```rust
let mut out = String::new();
for row in rows {
    out += &format!("{},{}\n", row.id, row.name);   // allocates a String, then throws it away
}
```
✅
```rust
use std::fmt::Write;
let mut out = String::with_capacity(rows.len() * 32);
for row in rows {
    writeln!(out, "{},{}", row.id, row.name).unwrap();   // writes in place, no temp
}
```
Same shape applies to `push_str` and to `join`:
```rust
let csv = items.iter().map(|i| i.name.as_str()).collect::<Vec<_>>().join(",");
```

### 3.2 `to_string()` vs `to_owned()` vs `String::from()`

For a `&str` these are **equivalent in performance** (`ToString` is specialised for `str`).
Pick on intent: `to_owned()` says "borrowed → owned", `to_string()` says "render this as
text". Using `to_string()` on a number goes through the formatting machinery and *is*
slower than `itoa` in a hot loop — but not in the other 99% of code.

### 3.3 Don't allocate to compare or to check a prefix

❌ `if s.to_lowercase() == "yes"` — allocates a String just to throw away
✅ `if s.eq_ignore_ascii_case("yes")`

❌ `if s.split('/').collect::<Vec<_>>()[0] == "api"` — allocates a Vec
✅ `if s.starts_with("api/")` or `if s.split('/').next() == Some("api")`

---

## 4. Collections

### 4.1 Use the `entry` API — never `contains_key` then `insert`

❌ Two hash lookups, and the pattern gets worse with nesting:
```rust
if !map.contains_key(&k) { map.insert(k.clone(), Vec::new()); }
map.get_mut(&k).unwrap().push(v);
```
✅ One lookup, no `unwrap`, no clone:
```rust
map.entry(k).or_default().push(v);
```
`or_insert_with(|| expensive())` when the default costs something —
`or_insert(expensive())` evaluates **eagerly, every time**, even on a hit.

### 4.2 `contains` on a `Vec` is O(n)

Fine for 10 items in a one-off. In a loop over 10,000 items it's 10⁸ comparisons. Build a
`HashSet` once, outside the loop.

### 4.3 Reserve capacity when you know the size

```rust
let mut v = Vec::with_capacity(rows.len());     // one allocation instead of ~log2(n) reallocs
```
`Vec` grows by doubling; every growth is an allocate + memcpy of everything so far. Free
win whenever the size is known or estimable.

### 4.4 Removing while iterating

❌ `for i in 0..v.len() { if pred(&v[i]) { v.remove(i); } }` — wrong *and* O(n²)
✅ `v.retain(|x| !pred(x));` — one pass, correct

### 4.5 Reuse buffers instead of reallocating

```rust
let mut buf = String::new();
for line in reader.lines() {
    buf.clear();                 // keeps the capacity, drops the contents
    build_into(&mut buf, &line?);
}
```

### 4.6 Swap the hasher for internal maps

The default `HashMap` hasher (SipHash) is DoS-resistant — necessary when keys come from
users, wasteful when they don't. For internal maps keyed by your own IDs, `rustc-hash`
(`FxHashMap`) or `ahash` is typically 2–5× faster on lookups. Do this **after** profiling
says the map is hot, not by default.

---

## 5. `Option`, `Result`, and errors

### 5.1 `unwrap()` is a promise you're making to the reader

| Context | Acceptable? |
|---|---|
| Tests, examples, prototypes | ✅ Yes |
| A case you have logically proven impossible | ✅ Use `expect("...")` and say *why* in the message |
| Anything touching I/O, network, user input, config | ❌ Never |
| Inside a library crate | ❌ Never — you're panicking in someone else's process |

`expect` costs the same as `unwrap` and turns a mystery into a sentence:
```rust
❌ let port = env::var("PORT").unwrap();
✅ let port = env::var("PORT").expect("PORT must be set; see README for required env vars");
```

### 5.2 Errors must name the thing that failed

❌ `Error: No such file or directory (os error 2)` — useless at 2am
✅ `Error: failed to read config`  `Caused by: No such file or directory: /etc/app/config.json`

```rust
use anyhow::Context;
let text = fs::read_to_string(&path)
    .with_context(|| format!("failed to read config at {}", path.display()))?;
```
`with_context` (closure) over `context` (value) when building the message costs anything.

### 5.3 `anyhow` in binaries, `thiserror` in libraries

The rule: **can your caller do something different based on which error it was?**
- Library → yes, they might → typed enum with `thiserror`
- Binary → no, it's going to a log and an exit code → `anyhow::Result`

Mixing is fine and normal: typed errors at module boundaries, `anyhow` in `main`.

### 5.4 Lazy vs eager

```rust
opt.unwrap_or(expensive())        // ❌ runs expensive() even when Some
opt.unwrap_or_else(|| expensive()) // ✅ only on None
opt.ok_or(MyError::new(msg))     // ❌ builds the error every time
opt.ok_or_else(|| MyError::new(msg)) // ✅
```
Any `_or` / `_or_else` pair: the `_else` version takes a closure and is lazy. Same for
`map_or` / `map_or_else`, `or` / `or_else`, `entry().or_insert` / `or_insert_with`.

### 5.5 Prefer flat control flow

❌ Rightward drift:
```rust
if let Some(user) = get_user(id) {
    if let Ok(cfg) = load_config() {
        if cfg.enabled { do_work(user, cfg); }
    }
}
```
✅ `let ... else` and `?` flatten it:
```rust
let Some(user) = get_user(id) else { return Ok(()) };
let cfg = load_config()?;
if !cfg.enabled { return Ok(()); }
do_work(user, cfg);
```

### 5.6 `main` can return `Result`

```rust
fn main() -> anyhow::Result<()> {
    real_main()?;
    Ok(())
}
```
Gives you `?` at top level and a proper non-zero exit code, instead of a stack of unwraps.

---

## 6. Iterators and closures

### 6.1 Don't `collect()` in the middle of a chain

❌ Allocates a whole `Vec` that is immediately consumed and thrown away:
```rust
let names: Vec<_> = users.iter().map(|u| &u.name).collect();
let count = names.iter().filter(|n| n.starts_with('A')).count();
```
✅ One pass, zero allocations:
```rust
let count = users.iter().filter(|u| u.name.starts_with('A')).count();
```
Only `collect()` when you actually need the collection — to return it, to index it, or to
iterate it more than once.

### 6.2 Iterator chains are not slower than loops

They compile to the same machine code, and they *remove bounds checks* that indexed loops
keep. Prefer them:

❌ `for i in 0..v.len() { total += v[i]; }` — bounds check per iteration
✅ `let total: i32 = v.iter().sum();`

The exception is when you need `break` with a value, complex early exit, or two mutable
cursors — then a plain loop is clearer and you should write one. Clarity wins; there is no
performance argument either way.

### 6.3 Don't clone inside `map`

❌ `v.iter().map(|s| s.clone()).filter(|s| s.len() > 3)` — clones everything, including the items you're about to discard
✅ `v.iter().filter(|s| s.len() > 3).cloned()` — filter first, clone only survivors

General rule: **cheapest operations first in the chain.** `filter` before `map`, `map`
before `sort`, and never allocate before a filter.

### 6.4 `sum()` and `product()` can overflow

`v.iter().sum::<u8>()` panics in debug, wraps in release, exactly like §1.1. Collect into a
wider type: `v.iter().map(|&x| x as u64).sum::<u64>()`.

### 6.5 Know the three collect tricks

```rust
// 1. Fail on the first bad element
let nums: Result<Vec<i32>, _> = strs.iter().map(|s| s.parse::<i32>()).collect();

// 2. Skip bad elements
let nums: Vec<i32> = strs.iter().filter_map(|s| s.parse().ok()).collect();

// 3. Build a map directly
let by_id: HashMap<u64, &User> = users.iter().map(|u| (u.id, u)).collect();
```
Trick 1 surprises people: `Vec<Result<T, E>>` collects into `Result<Vec<T>, E>`. Use it
whenever "one bad row should abort the batch."

### 6.6 `sort_unstable` unless you need stability

`sort_unstable` is faster and allocates nothing; `sort` is stable and allocates. If the
elements have no meaningful equal-but-distinguishable cases (integers, IDs), use unstable.
And use `sort_by_key` over `sort_by` when the key is cheap — it's clearer:

```rust
v.sort_unstable_by_key(|p| p.id);                    // clean
v.sort_by(|a, b| b.price.total_cmp(&a.price));       // floats, descending
```
(`sort_by_key` on an `f64` field does not compile — floats aren't `Ord`. `total_cmp` is the
fix.)

### 6.7 `Fn` / `FnMut` / `FnOnce` in signatures

Take the **loosest** one your function actually needs:

| You call the closure | Take |
|---|---|
| exactly once | `FnOnce` |
| repeatedly, it mutates state | `FnMut` |
| repeatedly, no mutation | `Fn` |

Defaulting everything to `Fn` rejects callers whose closure captures a mutable variable —
for no benefit.

---

## 7. Structs, traits, and API shape

### 7.1 Derive generously on public types

```rust
#[derive(Debug, Clone, PartialEq)]
pub struct Config { ... }
```
`Debug` on every public type is effectively an API requirement — without it your users
can't put your type in an `assert_eq!`, a `dbg!`, or a log line. Add `Clone`, `Default`,
`PartialEq`, `Serialize`/`Deserialize` when they make sense. Omitting `Debug` is a papercut
you inflict on everyone downstream.

### 7.2 Newtype your IDs

❌ `fn transfer(from: u64, to: u64, amount: u64)` — three interchangeable `u64`s, and one
day someone swaps two of them
✅
```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct UserId(pub u64);

fn transfer(from: UserId, to: UserId, amount: Cents)
```
The compiler now catches the swap. Zero runtime cost — the wrapper is erased.

### 7.3 Implement `From`, never `Into`

`impl From<A> for B` gives you `impl Into<B> for A` free via a blanket impl. Writing `Into`
directly is strictly worse. And `impl From<io::Error> for MyError` is what makes `?`
convert automatically — that's the whole mechanism.

### 7.4 `#[must_use]` on anything that would be a bug to ignore

```rust
#[must_use = "this returns a new value; it does not modify in place"]
pub fn with_timeout(self, d: Duration) -> Self { ... }
```
`Result` and `Option` are already `#[must_use]`. Your builder methods and pure functions
should be too.

### 7.5 Prefer `impl Trait` in argument position; `dyn` only when you must

```rust
fn process(items: impl Iterator<Item = Row>)     // static dispatch, inlined, zero cost
fn process(items: &mut dyn Iterator<Item = Row>) // vtable, one copy of the code
```
Use `dyn` when: you need to store mixed types in one collection, you need the type to be
nameable at runtime, or monomorphisation is genuinely bloating your binary. Otherwise
generics.

### 7.6 Avoid `Rc<RefCell<T>>` graphs — use indices

Anyone building a tree or graph reaches for `Rc<RefCell<Node>>` and regrets it: runtime
borrow panics, reference cycles that leak, and terrible cache behaviour.

✅ The arena pattern:
```rust
struct Tree { nodes: Vec<Node> }
struct Node { parent: Option<usize>, children: Vec<usize>, data: String }
```
Indices instead of pointers. No `Rc`, no `RefCell`, no cycles, no leaks, contiguous memory,
trivially serialisable. This is how rustc and most real Rust graph code does it.

---

## 8. Concurrency and async

### 8.1 Never hold a `std::sync::MutexGuard` across `.await`

```rust
let data = state.lock().unwrap();      // ❌ future is now !Send; tokio::spawn rejects it
some_async_call().await;
```
✅ Drop the guard first:
```rust
let value = { state.lock().unwrap().clone() };   // guard dropped at the brace
some_async_call(value).await;
```
If you genuinely must hold a lock across an await point, that's the *only* time to use
`tokio::sync::Mutex` — it's slower, so don't reach for it by default. A `std::sync::Mutex`
for short critical sections is the right choice in async code.

### 8.2 Never block inside async

```rust
async fn handle() {
    std::thread::sleep(Duration::from_secs(5));   // ❌ freezes every other task on this worker
}
```
- Sleeping → `tokio::time::sleep(...).await`
- CPU-heavy work or a blocking library (`rusqlite`, `std::fs`, image decoding) →
  `tokio::task::spawn_blocking(|| ...)`

One blocking call starves every task sharing that worker thread. Symptom: latency spikes
that make no sense.

### 8.3 Bound your concurrency

❌ `join_all(urls.iter().map(fetch))` with 10,000 URLs — 10,000 sockets, instant rate-limit ban
✅
```rust
use futures::StreamExt;
let results: Vec<_> = futures::stream::iter(urls)
    .map(|u| fetch(u))
    .buffer_unordered(10)      // at most 10 in flight
    .collect()
    .await;
```
Or a `tokio::sync::Semaphore` when the limit is shared across call sites.

### 8.4 Unbounded channels are a memory leak with extra steps

`mpsc::unbounded_channel()` means a fast producer and a slow consumer will consume all
your RAM. Use a bounded channel — the send blocking *is* the backpressure, and it's the
feature, not the problem.

### 8.5 `join_all` is concurrency, `spawn` is parallelism

`join_all` polls every future on the **current task** — one thread. Perfect for I/O.
For CPU work you need `tokio::spawn` (moves to the runtime's worker pool) or `rayon`.

### 8.6 `Arc::clone(&x)` over `x.clone()`

Same operation, but the explicit form tells the reader "this is a cheap refcount bump, not
a deep copy of the data." A small readability convention that the whole ecosystem follows.

### 8.7 Deadlock is still possible

Rust prevents data races at compile time. It does **not** prevent deadlocks, lock-ordering
bugs, or logical races. Keep lock scopes tiny, always acquire multiple locks in the same
global order, and prefer message passing (channels) over shared state when you can.

---

## 9. Performance — in the order that actually matters

Build-level tuning lives in `rust-gaps-and-developer-essentials.md` §E. This is code-level,
and the ordering is deliberate:

1. **Measure with `--release`, or don't measure.** Debug builds are 10–100× slower. Almost
   every "Rust isn't fast" report is this.
2. **Find the hot spot before touching anything.** `cargo flamegraph` for where time goes,
   `criterion` for microbenchmarks, `hyperfine` for whole-program comparisons. Your
   intuition about which line is slow is wrong more often than it's right.
3. **Kill allocations in hot loops.** In practice this is 90% of real Rust optimisation:
   `format!`, `to_string()`, `collect()`, `clone()`, and `vec![]` inside a loop. Hoist the
   buffer out, `clear()` it, reuse it.
4. **Reserve capacity** when the size is known (§4.3).
5. **Iterate, don't index** — removes bounds checks (§6.2).
6. **`Vec` beats linked structures** basically always. Cache locality dominates asymptotics
   at realistic sizes; a linear scan of a `Vec` beats a `HashMap` lookup up to ~20 elements.
7. **Swap the hasher** for hot internal maps (§4.6).
8. **Only then** consider `unsafe`, SIMD, or hand-tuning. You will almost never get here.

Two things people do that *don't* help: sprinkling `#[inline]` (the compiler already
inlines generics and small functions across crates; it matters only for non-generic
functions across a crate boundary without LTO), and rewriting iterator chains as manual
loops (identical codegen, worse code).

---

## 10. Testing and debugging habits

```rust
let x = dbg!(compute());        // prints file:line, the expression, and the value — and returns it
```
`dbg!` over `println!` for debugging: it goes to stderr, shows the source location, and you
can wrap an expression in place without restructuring anything. Delete before committing —
clippy will remind you.

Other habits worth having from day one:

- `#[cfg(test)] mod tests` in the same file as the code it tests. Rust convention, and it
  can reach private items.
- Take `impl Read`/`impl Write` instead of a `File` in any function you want to test — then
  a test can pass `&b"line one\nline two"[..]` and never touch the filesystem. **Design for
  this before you write the function**, not after.
- `matches!(value, Pattern)` for assertions on enums: `assert!(matches!(e, Error::NotFound(_)))`.
  (`assert_matches!` is still unstable on 1.96 — checked.)
- `#[should_panic(expected = "...")]` with the `expected` string. Without it the test passes
  on *any* panic, including a typo.
- `debug_assert!` for invariant checks you want in dev but not in release hot paths.
- `cargo test -- --nocapture` when you need to see printed output.

---

## Pre-commit checklist

Run down this list before any code you intend to keep:

- [ ] `cargo clippy --all-targets -- -D warnings` clean
- [ ] `cargo fmt` run
- [ ] No `unwrap()` outside tests — every one is `expect("reason")` or `?`
- [ ] No `as` on a narrowing cast — `try_into()` instead
- [ ] Every error message names the file / URL / ID that failed
- [ ] No allocation (`format!`, `clone`, `collect`, `to_string`) inside a hot loop
- [ ] No `MutexGuard` held across an `.await`
- [ ] No unbounded concurrency against an external API
- [ ] `Debug` derived on every public type
- [ ] Anything depending on `HashMap` order rewritten to use `BTreeMap` or an explicit sort
- [ ] Public functions take `&str` / `&[T]` / `impl AsRef<Path>`, not `&String` / `&Vec<T>` / `&PathBuf`

---

## How to actually absorb this

Do not read it end to end. It's a lookup table.

1. Turn on clippy today and fix every warning it raises. That's items 1–4 of the checklist handled by a machine.
2. Pick **one section per week** and deliberately apply it to code you're writing. §1 (hidden behaviour) and §5 (errors) pay off fastest.
3. When a bug takes you more than 20 minutes, check whether it's in here. If it is, that entry has earned its place in your memory. If it isn't, add it — this file should grow.
