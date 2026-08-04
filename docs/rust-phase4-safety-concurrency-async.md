# Rust Phase 4 — Memory Safety, Advanced Lifetimes, Concurrency & Async

> Closing the four topics that were missing or partial. This is the phase where the ownership rules stop being a tax and start being a superpower.
> Continuation of Phase 1 (Data Types), Phase 2 (Ownership), Phase 3 (Traits & Iterators).

---

## What was actually covered before this

| Topic | Previously | Here |
|---|---|---|
| Ownership | ✅ complete | applied to threads |
| Lifetimes | basics: `'a`, elision, `'static`, structs | bounds, HRTB, variance, `Pin` |
| Memory safety | assumed, never stated | **Part 1 — explicit** |
| Concurrency | previewed only | **Part 3 — full** |
| Async | one sentence | **Part 4 — full** |

---

# PART 1 — Memory safety, stated explicitly

Every previous document assumed you'd absorb this by osmosis. Let's name it directly, because "Rust is memory safe" is a claim with a precise meaning and precise limits.

## 1.1 What memory safety actually means

**Memory safety = the absence of undefined behavior arising from memory access.** Concretely, seven bug classes:

| Bug | What happens | Rust's defense |
|---|---|---|
| **Use-after-free** | read/write memory that was freed | ownership + borrow checker: references can't outlive their owner |
| **Double-free** | free the same allocation twice | one owner, one `drop` — moves invalidate the source |
| **Dangling pointer** | pointer to memory that's gone | lifetimes prove every reference is valid |
| **Buffer overflow** | index past the end | bounds checking on every index |
| **Null dereference** | deref a null pointer | **no null** — absence is `Option<T>` |
| **Uninitialized read** | read memory never written | compiler requires initialization before use |
| **Data race** | concurrent unsynchronized access, one writing | `Send`/`Sync` + shared-XOR-mutable (Part 3) |

These are not exotic. **Roughly 70% of CVEs at Microsoft and Google are memory-safety bugs.** That statistic is why Rust exists, why the NSA and CISA recommend memory-safe languages, and why Linux and Windows are both adopting Rust for new components.

The remarkable part is that all seven are caught **at compile time, with no runtime cost** — except bounds checking, which is a predictable-branch comparison that LLVM often eliminates entirely when it can prove the index is in range.

## 1.2 What Rust does NOT protect you from

This is the part that gets oversold, so be clear-eyed:

**Memory leaks are safe in Rust.** Leaking is not undefined behavior — it's just wasteful. `Rc` reference cycles leak. `std::mem::forget` leaks deliberately. `Box::leak` returns a `&'static mut` on purpose. Rust guarantees you won't access memory you shouldn't; it does not guarantee you'll release it.

**Deadlocks are safe.** Two threads waiting on each other's locks is perfectly well-defined behavior — and perfectly broken. Rust prevents data races, not deadlocks. Nothing stops you writing a lock-ordering bug.

**Logic bugs are safe.** The compiler verifies memory access, not correctness. Your off-by-one in an algorithm compiles fine.

**Integer overflow wraps in release builds.** Panics in debug, wraps silently in release (Phase 1). Not a memory-safety issue, but a real bug source.

**Panics still crash.** A `panic!` unwinds and aborts the thread. Safe, but your program still stops.

**Race conditions ≠ data races.** Rust eliminates data races (concurrent unsynchronized memory access). It does not eliminate logical races — two threads doing correct-but-badly-ordered operations still produce wrong answers.

> **The accurate claim:** Rust eliminates *undefined behavior* in safe code. It does not eliminate bugs. That's still an enormous win — undefined behavior is what turns a bug into a security vulnerability — but it isn't magic.

## 1.3 `unsafe` — what it actually does

The most misunderstood keyword in the language. `unsafe` unlocks exactly **five** abilities:

1. Dereference a raw pointer (`*const T`, `*mut T`)
2. Call an `unsafe` function (including all FFI)
3. Access or modify a mutable `static`
4. Implement an `unsafe` trait (`Send`, `Sync`)
5. Access fields of a `union`

**That's the complete list.** Critically:

> **`unsafe` does NOT turn off the borrow checker.** Ownership, lifetimes, and the borrowing rules apply identically inside an `unsafe` block. It only permits those five operations. It is not "C mode."

What `unsafe` really means is: *"I, the programmer, am asserting an invariant the compiler cannot verify."* It shifts the burden of proof from compiler to human for a small, auditable region.

```rust
let mut v = vec![1, 2, 3];
let ptr = v.as_mut_ptr();

unsafe {
    *ptr.add(1) = 99;      // I promise index 1 is in bounds
}
```

**The safe-abstraction pattern** is the reason this is workable. `Vec`, `String`, `Rc`, `Mutex` — all built on `unsafe` internally, all exposing a 100% safe API. A few thousand carefully-audited lines in the standard library let millions of lines of application code be safe. That's the whole strategy.

Practical guidance: you will likely never write `unsafe` outside of FFI. When you do — validate invariants at the boundary, keep the block as small as possible, document the safety contract with a `// SAFETY:` comment, and test it under **Miri** (`cargo +nightly miri test`), which detects undefined behavior at runtime.

## 1.4 The ownership → safety chain

Worth seeing all at once:

```
Ownership (one owner, scope-bound drop)
    → no double-free, no use-after-free, deterministic cleanup
Borrowing (shared XOR mutable)
    → no aliasing bugs, no iterator invalidation
Lifetimes (references can't outlive data)
    → no dangling pointers
No null (Option<T>)
    → no null dereference
Bounds checking
    → no buffer overflows
Send + Sync (Part 3)
    → no data races
```

Every safety guarantee traces back to ownership. This is why Phase 2 was the hard part and everything since has been comparatively easy.

---

# PART 2 — Lifetimes: the advanced half

Phase 2 covered annotations, elision, `'static`, and structs holding references. Here's what was left out.

## 2.1 Lifetime bounds

```rust
struct Wrapper<'a, T: 'a> {     // T must outlive 'a
    value: &'a T,
}

fn longest<'a, 'b: 'a>(x: &'a str, y: &'b str) -> &'a str {
    //        ^^^^^^^ 'b outlives 'a
    if x.len() > y.len() { x } else { y }
}
```

`'b: 'a` reads **"`'b` outlives `'a`"** — anything valid for `'b` is valid for at least `'a`. `T: 'a` means "type `T` contains no references shorter than `'a`."

The common case you'll actually hit is **`T: 'static`**, and it means something subtler than "lives forever":

> `T: 'static` means **"`T` contains no non-`'static` references."**

`String`, `i32`, and `Vec<u8>` all satisfy `T: 'static` — they own their data and borrow nothing. `&'a str` does not. This distinction matters the moment you call `thread::spawn`, which requires `'static` because the thread might outlive the caller.

## 2.2 Higher-ranked trait bounds (HRTB)

```rust
fn apply<F>(f: F)
where
    F: for<'a> Fn(&'a str) -> &'a str,     // for ALL lifetimes 'a
{ ... }
```

`for<'a>` means the closure must work for *every possible* lifetime, not one specific one the caller picks. You need this when passing closures that take references, because the lifetime isn't known until the call site.

Mostly this is inferred and invisible. You'll meet it explicitly when writing generic code over closures, and the error message ("implementation is not general enough") is the tell.

## 2.3 Variance — the thing that silently works

Given `'long: 'short`, is `&'long T` usable where `&'short T` is expected? Yes — this is **covariance**, and it's why you can pass a long-lived reference to a function wanting a short-lived one.

- `&'a T` is **covariant** in `'a` and `T` — longer works where shorter is wanted.
- `&'a mut T` is covariant in `'a` but **invariant** in `T` — no substitution allowed.
- `fn(T)` is **contravariant** in `T`.
- `Cell<T>`, `RefCell<T>`, `Mutex<T>` are invariant in `T`.

You will almost never think about this. It's here so that when you hit a baffling lifetime error involving `&mut`, you know the word to search for. Invariance of `&mut T` is the usual culprit.

## 2.4 Lifetimes on trait objects

```rust
Box<dyn Trait>              // implicitly Box<dyn Trait + 'static>
Box<dyn Trait + 'a>         // explicitly allowed to borrow with lifetime 'a
&'a (dyn Trait + 'a)
```

Boxed trait objects default to `'static`. If your implementor holds references, you must say so explicitly — otherwise you get the confusing "does not live long enough" error on a type that looks like it has no lifetimes at all.

## 2.5 Self-referential structs and `Pin`

A struct that holds a reference into its own field is **impossible in safe Rust**:

```rust
struct SelfRef {
    data: String,
    slice: &str,      // ❌ pointing into self.data — no lifetime can express this
}
```

Why: moving the struct would move `data`, leaving `slice` dangling. Rust assumes all types are freely movable (a memcpy), so this cannot be expressed.

**`Pin<P>`** is the answer: a wrapper asserting the pointee will never move again. This is not something you use directly in application code — but it's the reason **async blocks work at all**, since an `async` block that holds a reference across an `.await` is exactly a self-referential struct. When you see `Pin<Box<dyn Future>>` in async code, this is why.

## 2.6 Generic associated types (GATs)

Stabilized in Rust 1.65 — associated types that themselves take generic parameters:

```rust
trait Container {
    type Item<'a> where Self: 'a;
    fn get<'a>(&'a self) -> Self::Item<'a>;
}
```

This unlocks lending iterators (yielding items that borrow from the iterator) and is the foundation for future async-trait improvements. Library-author territory; know the name, move on.

---

# PART 3 — Concurrency

## 3.1 "Fearless concurrency" — what it actually means

A **data race** requires three things simultaneously:

1. Two or more pointers to the same data
2. At least one is writing
3. No synchronization

You already banned #1 + #2 in Phase 2 — **shared XOR mutable**. That rule wasn't designed for single-threaded code; it eliminates data races as a side effect. This is the design payoff of the entire language.

In Python, the GIL papers over this by preventing true parallelism for CPU-bound work, and you *still* get race conditions with threads. In Go, you get true parallelism and races are your problem — `go run -race` is a runtime detector that finds them if you're lucky. In Rust, **data races are compile errors.**

## 3.2 Threads

```rust
use std::thread;

let handle = thread::spawn(|| {
    println!("from a thread");
    42
});

let result = handle.join().unwrap();   // wait, get the value back
```

These are real OS threads (1:1), not green threads. `join()` returns `Result` because the thread may have panicked — a panic in one thread doesn't kill the process.

Closures usually need `move` to take ownership, because the thread may outlive the spawner:

```rust
let data = vec![1, 2, 3];
thread::spawn(move || println!("{data:?}"));   // ownership transferred
// data unusable here
```

`thread::spawn` requires `F: Send + 'static` — the closure must be transferable to another thread and must not borrow anything from the current stack frame.

**Scoped threads** (stable since 1.63) relax the `'static` requirement, letting threads borrow locals because the scope guarantees they finish first:

```rust
let mut data = vec![1, 2, 3];

thread::scope(|s| {
    s.spawn(|| println!("{data:?}"));       // borrows! no 'static needed
    s.spawn(|| println!("also {data:?}"));
});   // all threads joined here — guaranteed
```

This is much more ergonomic than the old `Arc` dance and underused by people who learned Rust before 1.63.

## 3.3 `Send` and `Sync` — the two traits that make it work

These are **marker traits** — no methods, pure compile-time metadata.

> **`Send`**: it is safe to **transfer ownership** of this type to another thread.
> **`Sync`**: it is safe to **share a reference** (`&T`) to this type across threads.
> Formally: `T: Sync` if and only if `&T: Send`.

Both are **auto traits** — the compiler implements them structurally. A struct is `Send` if all its fields are `Send`. You almost never write these; you just occasionally get told a type isn't one.

The instructive cases:

| Type | `Send` | `Sync` | Why |
|---|---|---|---|
| `i32`, `String`, `Vec<T>` | ✅ | ✅ | plain owned data |
| `Rc<T>` | ❌ | ❌ | **non-atomic** refcount — concurrent clones would corrupt it |
| `Arc<T>` | ✅ | ✅ | atomic refcount (the `A`) |
| `RefCell<T>` | ✅ | ❌ | non-atomic borrow flag — safe to move, unsafe to share |
| `Mutex<T>` | ✅ | ✅ | synchronization is its entire job |
| `MutexGuard<'_, T>` | ❌ | ✅ | must be released by the locking thread |
| raw pointers | ❌ | ❌ | no guarantees at all |

**`Rc` vs `Arc` is the canonical lesson.** They're identical except `Arc` uses atomic instructions for the refcount. `Arc` is slightly slower, so Rust makes you choose — and the compiler refuses to let you use `Rc` across threads. In Python this class of bug is invisible; here it's a compile error.

## 3.4 Shared state: `Arc<Mutex<T>>`

```rust
use std::sync::{Arc, Mutex};
use std::thread;

let counter = Arc::new(Mutex::new(0));
let mut handles = vec![];

for _ in 0..10 {
    let counter = Arc::clone(&counter);       // cheap: refcount bump
    handles.push(thread::spawn(move || {
        let mut n = counter.lock().unwrap();  // blocks until acquired
        *n += 1;
    }));                                       // guard dropped → unlocked
}

for h in handles { h.join().unwrap(); }
println!("{}", *counter.lock().unwrap());     // exactly 10, guaranteed
```

Read the type as: **`Arc`** = multiple owners across threads; **`Mutex`** = only one may access at a time. Two orthogonal problems, two composable types.

The crucial design difference from C, Java, and Python:

> **Rust's `Mutex` *owns* the data it protects.** You cannot access the data without locking, because the data is *inside* the mutex. The single most common concurrency bug in other languages — forgetting to take the lock — is structurally impossible.

And unlocking is RAII: the `MutexGuard` releases on drop. **You cannot forget to unlock.** Compare Python, where `lock.acquire()` without a matching `release()` in a `finally` is a live hazard.

`lock()` returns `Result` because of **poisoning**: if a thread panics while holding the lock, the data may be in a broken state, so subsequent lockers get an `Err`. `.unwrap()` is normal here — it propagates the failure.

**`RwLock<T>`** is the many-readers-or-one-writer variant. Use it when reads dominate writes; `Mutex` is otherwise simpler and often faster.

## 3.5 Message passing: channels

The alternative philosophy — *"Do not communicate by sharing memory; share memory by communicating."*

```rust
use std::sync::mpsc;

let (tx, rx) = mpsc::channel();       // multi-producer, single-consumer

for i in 0..3 {
    let tx = tx.clone();
    thread::spawn(move || tx.send(i * 10).unwrap());
}
drop(tx);                              // drop the original or rx never ends

for received in rx {                   // iterates until all senders drop
    println!("{received}");
}
```

Ownership makes channels genuinely safe: **`send` moves the value**, so the sender provably cannot touch it afterward. The "I sent it but also kept using it" bug doesn't exist.

For anything serious use **`crossbeam-channel`** — multi-consumer, faster, with `select` support.

**When to use which:** channels for pipelines and worker pools (ownership flows one direction, easy to reason about); `Arc<Mutex<T>>` for genuinely shared mutable state like a cache or counter. Prefer channels when both fit.

## 3.6 Atomics

Lock-free primitives for simple values:

```rust
use std::sync::atomic::{AtomicUsize, Ordering};

static COUNTER: AtomicUsize = AtomicUsize::new(0);
COUNTER.fetch_add(1, Ordering::Relaxed);
```

`Ordering` controls memory-barrier strength (`Relaxed`, `Acquire`, `Release`, `AcqRel`, `SeqCst`). **Use `SeqCst` unless you have measured a need and understand the memory model** — this is genuinely subtle territory where being clever produces bugs that appear once a week on one CPU architecture.

## 3.7 `rayon` — the one you'll actually use most

```rust
use rayon::prelude::*;

let sum: f64 = data.par_iter().map(|x| expensive(x)).sum();
//                  ^^^^ that's the entire diff
```

Work-stealing data parallelism across all cores, with the borrow checker guaranteeing correctness. Every Phase 3 iterator chain has a parallel twin: `par_iter`, `par_iter_mut`, `into_par_iter`, `par_chunks`, `par_sort`.

For your preprocessing pipelines this is the highest value-per-keystroke in the language. There is no Python equivalent — `multiprocessing` means serializing data across process boundaries, and threads are throttled by the GIL.

## 3.8 What's still your problem

Rust guarantees no data races. It does **not** prevent:

- **Deadlocks** — always acquire locks in a consistent global order
- **Livelocks and starvation**
- **Logical race conditions** — correct operations in a wrong order
- **Holding a lock too long** — the classic scalability killer

---

# PART 4 — Async

## 4.1 Concurrency vs parallelism (the decision that matters)

- **Parallelism**: doing many things *simultaneously* on multiple cores. Bound by CPU. → **threads / rayon**
- **Concurrency**: managing many things *in flight*, most of them waiting. Bound by I/O. → **async**

10,000 open sockets mostly idle: 10,000 OS threads costs ~80 GB of stack and murders the scheduler. 10,000 async tasks costs a few MB. That's the entire value proposition.

> **Rule: CPU-bound → threads/rayon. I/O-bound with high concurrency → async. Don't reach for async because it sounds modern; it's a specific tool for a specific bottleneck.**

## 4.2 The big surprise: Rust ships no runtime

```rust
async fn fetch(url: &str) -> Result<String, Error> {
    let resp = reqwest::get(url).await?;
    Ok(resp.text().await?)
}
```

`async fn` doesn't return a `String` — it returns **`impl Future<Output = Result<String, Error>>`**. And here's what trips up every Python developer:

> **The language defines `async`/`await` and the `Future` trait. It provides no executor.** There is no built-in event loop. You must pick a runtime.

Python bundles `asyncio`. Rust deliberately doesn't, because an embedded device, a web server, and a database need very different schedulers. In practice this means **`tokio`** (the de-facto standard — use it unless you know why not), or `smol`/`embassy` for lighter or embedded cases.

```toml
[dependencies]
tokio = { version = "1", features = ["full"] }
```

```rust
#[tokio::main]                              // macro: builds a runtime, blocks on main
async fn main() {
    let body = fetch("https://example.com").await.unwrap();
    println!("{body}");
}
```

## 4.3 Futures are lazy

```rust
let fut = fetch(url);        // NOTHING has happened yet
let body = fut.await;        // now it runs
```

A `Future` is an inert state machine until polled. Calling an async function does zero work. (Python coroutines behave similarly; **JavaScript promises do not** — they start immediately. If your mental model came from JS, adjust.)

Consequence: `async fn` bodies don't run "in the background" just because you called them. To get background execution you must **spawn**:

```rust
let handle = tokio::spawn(async move { fetch(url).await });   // runs now
let result = handle.await.unwrap();
```

## 4.4 How it works underneath

The compiler transforms each `async` block into a **state machine** implementing `Future`:

```rust
trait Future {
    type Output;
    fn poll(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output>;
}
enum Poll<T> { Ready(T), Pending }
```

Each `.await` is a **suspension point** — a state in the machine. When polled, the future runs until it hits an `.await` that isn't ready, returns `Pending`, and registers a `Waker`. When the I/O completes, the waker tells the executor to poll again, resuming exactly where it left off.

Two things follow:

1. **The state machine holds live variables across `.await` points.** If one is a reference into another, the future is self-referential — hence `Pin` (§2.5). That's the whole reason `Pin` exists.
2. **Async tasks are cheap.** No OS thread, no separate stack — just an enum whose size is the largest state. Tasks are hundreds of bytes, not megabytes.

## 4.5 Running things concurrently

```rust
// Sequential — 2 seconds total
let a = fetch(url1).await;
let b = fetch(url2).await;

// Concurrent on ONE task — 1 second
let (a, b) = tokio::join!(fetch(url1), fetch(url2));

// Concurrent + parallel across threads
let h1 = tokio::spawn(fetch(url1));
let h2 = tokio::spawn(fetch(url2));
let (a, b) = (h1.await?, h2.await?);

// Race — first to finish wins, others dropped
tokio::select! {
    r = fetch(url) => println!("{r:?}"),
    _ = tokio::time::sleep(Duration::from_secs(5)) => println!("timeout"),
}

// Many at once
let results = futures::future::join_all(urls.iter().map(|u| fetch(u))).await;
```

`join!` multiplexes on one task; `spawn` hands work to the runtime's thread pool. `select!` is how you implement timeouts and cancellation.

## 4.6 The three pitfalls that will bite you

**1. Never block inside async.** This is the big one.

```rust
async fn bad() {
    std::thread::sleep(Duration::from_secs(1));   // ❌ blocks the whole worker thread
    let data = std::fs::read_to_string("x")?;     // ❌ blocking I/O
    expensive_cpu_work();                          // ❌ starves every other task
}
```

An async task that blocks stops *every other task* on that worker thread. Fixes:

```rust
tokio::time::sleep(d).await;                       // ✅ async sleep
tokio::fs::read_to_string("x").await?;             // ✅ async I/O
tokio::task::spawn_blocking(|| expensive()).await?;// ✅ offload CPU work
```

For an inference server: your model forward pass is CPU/GPU-bound and **must** go through `spawn_blocking` (or a dedicated thread pool), or your request handling collapses under load. This is the most common real-world async Rust mistake.

**2. Don't hold a `std::sync::MutexGuard` across `.await`.**

```rust
let guard = mutex.lock().unwrap();
something().await;                    // ❌ guard is not Send → future isn't Send
```

The compiler catches this with a confusing "future is not `Send`" error. Fix by scoping the lock to end before the `.await`, or use `tokio::sync::Mutex` when you genuinely must hold it across a suspension point (it's slower — prefer restructuring).

**3. Dropping a future cancels it.** Unlike Python, where you cancel a `Task` explicitly, in Rust dropping a future stops it at its last suspension point — mid-operation, with no cleanup callback. This makes `select!` dangerous with non-**cancellation-safe** operations: a partially-read stream can lose data. Read the docs on cancellation safety before using `select!` in a loop.

## 4.7 Practical async you'll need

**Async traits** — `async fn` in traits was stabilized in Rust 1.75, but with limits (not `dyn`-compatible, awkward `Send` bounds). For trait objects you still want the `async-trait` crate:

```rust
#[async_trait::async_trait]
trait Fetcher {
    async fn fetch(&self, url: &str) -> Result<String>;
}
```

**Streams** — the async equivalent of `Iterator`, not yet in std. Use `futures::Stream` / `tokio_stream`:

```rust
use tokio_stream::StreamExt;
while let Some(item) = stream.next().await { ... }
```

Directly relevant to your work: this is how you stream LLM tokens back to a client.

**Structured concurrency** — `JoinSet` manages a dynamic group of tasks and cancels them on drop:

```rust
let mut set = tokio::task::JoinSet::new();
for url in urls { set.spawn(fetch(url)); }
while let Some(res) = set.join_next().await { ... }
```

**Function coloring** — async functions can only be `.await`ed from async contexts. This "coloring" splits your codebase in two and is a genuine ergonomic cost, identical to Python's. Design your layering deliberately: keep pure logic sync, push async to the I/O edges.

---

# PART 5 — Choosing the right tool

| Situation | Use |
|---|---|
| CPU-bound work over a collection | **`rayon`** (`par_iter`) |
| A few long-running background jobs | `std::thread` |
| Threads that need to borrow locals | `thread::scope` |
| Thousands of network connections | **`tokio`** (async) |
| HTTP/gRPC server | `axum` / `tonic` (async) |
| Shared counter/cache across threads | `Arc<Mutex<T>>` or `Arc<RwLock<T>>` |
| Pipeline of stages | channels (`crossbeam-channel`) |
| Simple shared counter | `AtomicUsize` |
| Model inference inside a web server | async handler + **`spawn_blocking`** for the forward pass |

**The typical AI inference server**: `tokio` + `axum` for request handling, `spawn_blocking` or a dedicated thread pool for model execution, channels for batching requests together, `rayon` for parallel preprocessing. All four in one binary, each doing what it's good at.

---

# Exercises

1. **Memory safety by demonstration.** Write the use-after-free, double-free, and iterator-invalidation bugs in Rust. Collect all three compiler errors. Now write the same three in Python and observe which ones it lets you do.
2. **Leaks are safe.** Build an `Rc` reference cycle (two structs pointing at each other via `Rc<RefCell<...>>`). Confirm it compiles, runs, and leaks. Then fix it with `Weak`.
3. **`unsafe` doesn't disable the borrow checker.** Write an `unsafe` block that still violates the borrowing rules. Confirm it fails to compile anyway.
4. **`Rc` across threads.** Try to send an `Rc<i32>` to a spawned thread. Read the `Send` error in full. Swap to `Arc` and watch it work.
5. **The counter.** Build the `Arc<Mutex<i32>>` ten-thread counter. Then remove the `Mutex` and try to share `Arc<i32>` mutably — see what the compiler says.
6. **Scoped threads.** Rewrite exercise 5 with `thread::scope` and borrow the counter instead of `Arc`-ing it.
7. **Channels.** Build a producer/consumer: three producer threads, one consumer summing results. Then forget to drop the original `tx` and watch it hang — understand why.
8. **rayon.** Take a CPU-heavy iterator chain from Phase 3, add `par_`, and time both with `--release`. Then try to mutate a shared `Vec` inside `par_iter` without a lock and read the error.
9. **Laziness.** Call an `async fn` without `.await` and confirm nothing happens. Add `.await`. Then `tokio::spawn` it.
10. **Sequential vs concurrent.** Write three async functions that each sleep 1s. Time them awaited in sequence, then with `join!`. Confirm 3s vs 1s.
11. **Block the executor.** Put `std::thread::sleep` inside an async task alongside nine others and watch throughput collapse. Fix with `tokio::time::sleep`, then with `spawn_blocking`.
12. **Guard across await.** Hold a `std::sync::MutexGuard` across an `.await` in a spawned task. Read the "future is not `Send`" error. Fix it two ways: scoping the lock, and `tokio::sync::Mutex`.
13. **Timeout with `select!`.** Race a slow operation against `tokio::time::sleep`. Then consider what state the cancelled operation was left in.

---

# Quick reference

| Concept | Python | Rust |
|---|---|---|
| Memory safety | GC-provided, runtime | ownership, compile time |
| Use-after-free | impossible | impossible (borrow checker) |
| Memory leaks | possible (cycles) | possible — and *safe* |
| Escape hatch | C extensions | `unsafe` (5 specific powers) |
| True parallelism | blocked by GIL | native threads, real |
| Data races | possible & silent | **compile error** |
| Deadlocks | possible | possible |
| Shared ownership | automatic | `Arc<T>` (explicit) |
| Lock discipline | `with lock:`, forgettable | `Mutex<T>` **owns** the data |
| Unlocking | manual / context manager | automatic on guard drop |
| Data parallelism | `multiprocessing` (IPC cost) | `rayon` (`par_iter`) |
| Async runtime | `asyncio` built in | **bring your own** (`tokio`) |
| Coroutine laziness | lazy | lazy |
| Task cost | ~KB | ~hundreds of bytes |
| Cancellation | `task.cancel()` | drop the future |
| Blocking in async | slows the loop | **starves the worker thread** |

### One-paragraph summary

Rust's memory safety means the elimination of *undefined behavior* — use-after-free, double-free, dangling pointers, buffer overflows, null derefs, and data races — all at compile time via ownership, and all at zero runtime cost. It does not prevent leaks, deadlocks, or logic bugs, and `unsafe` unlocks exactly five extra powers without ever disabling the borrow checker. The same shared-XOR-mutable rule that makes single-threaded code safe makes concurrency safe for free: `Send` and `Sync` encode thread-transferability in the type system, `Arc<Mutex<T>>` composes shared ownership with exclusion, and `Mutex` owns its data so you cannot forget to lock. For CPU-bound work use threads or `rayon`; for I/O-bound concurrency use `async` with `tokio` — remembering that futures are lazy, that Rust ships no runtime, and that blocking inside an async task is the one mistake that ruins everything.

---

### Remaining gaps after this document

For completeness, what's *still* not covered anywhere: **`unsafe` in depth and FFI**, **macro authoring** (`macro_rules!` and proc macros), **profiling and optimization workflow**, and **PyO3/maturin** — the Phase 6 payoff that's most relevant to your day job.
