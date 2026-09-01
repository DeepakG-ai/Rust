# Rust Interview Questions — 50, Screening → MAANG Staff Level

Built from 14 web searches across 2025–2026 interview guides (Index.dev, MentorCruise,
Zero To Mastery, interviewing.io, CoderPad, Turing, Second Talent, Rustify, techinterview.org,
plus the Rustonomicon and Tokio docs for the advanced material). Sources listed at the bottom.

Numbers marked ✔ were verified by compiling and running them on **rustc 1.96 / edition 2024** —
not recalled. If an interviewer disagrees with one, run it.

---

## What the process actually looks like

| Round | Length | What they test |
|---|---|---|
| Phone / video screen | 30–45 min | Ownership, borrowing, `Option`/`Result`. Verbal, no IDE |
| Live coding | 45–60 min | **40–60% is ownership/borrowing/lifetimes.** Practical code, not LeetCode |
| Take-home | 2–4 hrs (some 2–7 days) | API design, error handling, tests, `cargo` hygiene |
| System design (senior+) | 60–90 min | Why Rust: no GC pauses, predictable latency, safe concurrency |
| Behavioral | 30–45 min | A production incident you debugged |

Two things that come up in nearly every write-up:

1. **Most Rust interviews are practical, not algorithmic.** They want to see you use `cargo`, run
   tests, and read compiler errors. Fighting the borrow checker *out loud* is a pass; silence is a fail.
2. **When you hit an error you can't fix, narrate your debugging.** Interviewers say explicitly
   that this scores higher than getting it right silently.

**How to use this file:** answer out loud before reading the answer. The answers here are the
*compressed* version — what a good candidate says in 60–90 seconds. Not essays.

---

# TIER 1 — Foundations (Q1–14)
*Phone screen. If you miss these you don't reach the next round.*

### Q1. What is ownership, and what problem does it solve?
> *Testing: do you understand the model, or just the syntax?*

Every value has exactly one owner. When the owner goes out of scope, the value is dropped and its
memory freed. Ownership can be **moved** to a new owner, and the old one becomes unusable.

It solves memory management **without a garbage collector and without manual `free()`**. No
use-after-free, no double-free, no null dereference, no GC pauses — all enforced at compile time,
so it costs nothing at runtime.

**Follow-up:** *"What's the runtime cost?"* — Zero. It's entirely a compile-time analysis; the
generated code is what you'd have hand-written in C.

### Q2. What are the borrowing rules?
> *Testing: can you state it precisely? Most candidates fumble the wording.*

At any given time you can have **either** one mutable reference (`&mut T`) **or** any number of
immutable references (`&T`) — never both, and never two mutable ones.

This is what makes data races *impossible to compile*: a data race needs two accesses, one of them
a write, unsynchronised. The rule removes that combination by construction.

**Follow-up:** *"Is that only about threads?"* — No, and this is the better answer: it also prevents
iterator invalidation in single-threaded code. `v.push()` while holding `&v[0]` can reallocate the
buffer and leave your reference dangling. Same rule, no threads involved.

### Q3. `String` vs `&str` — what's the difference and when do you use each? ✔
> *Testing: the single most common Rust screening question.*

- `String` — owned, heap-allocated, growable, `Vec<u8>` with a UTF-8 invariant. **24 bytes** on the
  stack (pointer, length, capacity). ✔
- `&str` — a borrowed view into UTF-8 bytes. **16 bytes** (pointer, length). ✔ Owns nothing.

Rule for signatures: **take `&str`, return `String`.** Taking `&String` is a mistake — it rejects
callers holding a literal or a slice, for no benefit. `&String` auto-derefs to `&str` anyway.

**Follow-up:** *"Why can't I do `s[0]` on a string?"* — UTF-8 is variable-width; byte 0 may be half
a character. Indexing would be O(1) but meaningless, so Rust removes the operation. Use `.chars()`,
`.char_indices()`, or `.get(0..2)` which returns `Option`.

### Q4. `Copy` vs `Clone`?
> *Testing: do you know why `String` can't be `Copy`?*

`Copy` is an implicit bitwise duplicate on assignment — cheap, no allocation, opt-in via derive.
`Clone` is an explicit, potentially expensive deep copy you call by name.

A type **cannot be `Copy` if it implements `Drop`**. That's the whole reason `String` and `Vec<T>`
aren't `Copy`: they own a heap allocation, and a bitwise duplicate would give two owners of one
pointer and a double-free. `Vec<T>` is never `Copy` even when `T` is.

**Follow-up:** *"Name four `Copy` types and three that aren't."* — Copy: all integers, `f64`, `bool`,
`char`, `&T`, and arrays/tuples of `Copy` types. Not: `String`, `Vec<T>`, `Box<T>`, `&mut T`.

### Q5. What's the difference between `Option<T>` and `Result<T, E>`?
> *Testing: do you reach for the right one?*

`Option<T>` = the value may be **absent**, and that's not an error (`HashMap::get` on a missing key).
`Result<T, E>` = the operation **failed**, and there's a reason worth propagating (file not found).

Rust has no `null` and no exceptions. Both are ordinary enums, so the compiler forces you to handle
the failure case — you cannot accidentally ignore it the way you can miss a Python exception.

**Follow-up:** *"How do you convert between them?"* — `.ok()` turns `Result` into `Option` (discarding
the error); `.ok_or(err)` / `.ok_or_else(|| err)` goes the other way.

### Q6. What does the `?` operator do?
> *Testing: do you know it calls `From`?*

On `Ok(v)`/`Some(v)` it unwraps and continues. On `Err(e)`/`None` it **returns early** from the
enclosing function.

The part candidates miss: for `Result`, `?` calls **`From::from` on the error** before returning. So
`impl From<io::Error> for MyError` is what lets you `?` an I/O call inside a function returning
`Result<T, MyError>`. That conversion is the entire mechanism — it's why `thiserror`'s `#[from]`
attribute exists.

**Follow-up:** *"Can you use `?` in `main`?"* — Yes, if `main` returns `Result<(), E>` where
`E: Debug`. You get a non-zero exit code for free.

### Q7. When is `.unwrap()` acceptable?
> *Testing: judgment. This is a seniority signal.*

Tests, examples, prototypes — yes. A case you have **proven** impossible — use `.expect("why")`
instead, so the message explains the proof. Anything touching I/O, network, user input, or config —
never. Inside a **library** — never; you're panicking in someone else's process.

`expect` costs exactly the same as `unwrap` and turns a mystery stack trace into a sentence. There
is no reason to ever write bare `unwrap` in code you keep.

### Q8. Stack vs heap — what goes where in Rust?
> *Testing: systems fundamentals.*

Stack: fixed size known at compile time, LIFO, very fast, automatically reclaimed. Locals,
integers, `&T`, arrays `[T; N]`, and the *struct headers* of `String`/`Vec`.

Heap: dynamic size or lifetime that outlives the frame. `Box<T>`, `String`'s bytes, `Vec`'s buffer,
`Rc`/`Arc` contents.

The key insight: a `Vec<u8>` is 24 bytes **on the stack** ✔ — pointer, length, capacity — pointing at
a heap buffer. `Box<T>` is 8 bytes ✔; it's just a pointer that owns what it points to.

### Q9. What is a lifetime? Does it change how long a value lives?
> *Testing: the most common misconception in Rust.*

**No — and that's the answer they want.** A lifetime is a *compile-time label* the borrow checker
uses to verify that a reference never outlives the data it points to. It's purely descriptive
analysis; it does not extend, shorten, or affect any value's actual lifetime. Erased entirely at
codegen.

`fn longest<'a>(a: &'a str, b: &'a str) -> &'a str` is not saying "these live as long as 'a." It's
saying "the returned reference is valid only as long as *both* inputs are."

**Follow-up:** *"Why does most code have no lifetime annotations?"* — Elision. Three rules cover the
common cases: each elided input gets its own lifetime; if there's exactly one input lifetime it's
assigned to all outputs; if one parameter is `&self`, its lifetime is assigned to all outputs.

### Q10. What's the difference between `&T`, `&mut T`, and `T` in a parameter?
> *Testing: signature literacy.*

`T` takes ownership — the caller loses it. `&T` borrows immutably, caller keeps it, many allowed.
`&mut T` borrows exclusively for mutation, caller keeps it, only one at a time.

Take `T` when you must store or consume it (constructors, builders). Take `&T` by default. Take
`&mut T` when you mutate in place.

### Q11. Explain `move` in a closure.
> *Testing: closures + ownership together.*

By default a closure borrows what it captures, with the loosest capture it can get away with.
`move` forces it to **take ownership** of everything it captures.

You need it when the closure outlives the current scope — `thread::spawn` and `tokio::spawn` both
require `'static`, so anything borrowed from the stack has to be moved in instead.

**Follow-up:** *"`Fn` vs `FnMut` vs `FnOnce`?"* — `Fn` callable repeatedly, no mutation. `FnMut`
callable repeatedly, mutates captured state. `FnOnce` consumes its captures, callable once. In a
signature take the **loosest** one you actually need — defaulting to `Fn` rejects valid callers.

### Q12. What is a trait? How is it different from an interface?
> *Testing: type system basics.*

A trait is a set of method signatures a type can implement — shared behaviour without inheritance.

Two things Java/Python interfaces don't do:
1. **You can implement a trait for a type you didn't define** — `impl MyTrait for u32` is legal.
2. **Static dispatch by default.** Generic code is monomorphised — the compiler stamps out a
   specialised copy per type and inlines it. Zero runtime cost, unlike a Java interface call.

**Follow-up:** *"What's the orphan rule?"* — You can implement a trait for a type only if you own
the trait **or** the type. Otherwise two crates could add conflicting impls. Work around it with the
newtype pattern: wrap the foreign type in your own struct.

### Q13. How does Rust handle errors without exceptions?
> *Testing: the mental switch from Python/Java.*

Errors are **values**, returned in `Result<T, E>`, propagated with `?`. There is no invisible control
flow: every function that can fail says so in its type, and every caller must deal with it.

`panic!` exists but is for **unrecoverable** bugs — a violated invariant, not a missing file. It
unwinds (or aborts) rather than being caught; `catch_unwind` exists but is for FFI boundaries and
thread supervisors, not routine control flow.

**Follow-up:** *"anyhow or thiserror?"* — `thiserror` in libraries (typed enum, so callers can match
on the variant and react differently). `anyhow` in binaries (the error is going to a log and an exit
code; nobody's matching on it). Mixing both is normal and correct.

### Q14. What does `#[derive(Debug)]` do, and why should every public type have it?
> *Testing: API hygiene.*

It generates a `Debug` impl so `{:?}` and `{:#?}` work. Without it your users can't put the type in
`assert_eq!`, `dbg!`, `.unwrap()`'s panic message, or a log line. The Rust API Guidelines treat it as
effectively mandatory on public types.

**Follow-up:** *"`Debug` vs `Display`?"* — `Debug` is for programmers, derivable, `{:?}`. `Display`
is for end users, **cannot** be derived (only you know how your type should read to a human), `{}`.
Implementing `Display` also gives you `.to_string()` free via a blanket impl.

---

# TIER 2 — Working engineer (Q15–29)
*Mid-level startup live-coding round. This is where most interviews are decided.*

### Q15. Predict this. Does it compile? ✔
```rust
let mut v = vec![1, 2, 3];
let first = &v[0];
v.push(4);
println!("{}", first);
```
> *Testing: the canonical borrow-checker question. Asked constantly.*

**No.** `E0502: cannot borrow 'v' as mutable because it is also borrowed as immutable.`

Say *why it's a real bug*, not just that the rule forbids it: `push` may exceed capacity, which
reallocates the buffer and copies the elements to a new address. `first` would then point at freed
memory. In C++ this compiles and is a use-after-free — the exact bug class Rust was built to kill.

**Follow-up:** *"How do you fix it?"* — Copy the value out (`let first = v[0];`), or finish using
`first` before the `push`. Thanks to NLL the borrow ends at its **last use**, not the end of scope.

### Q16. `Rc<T>` vs `Arc<T>` — and why can't `Rc` cross threads?
> *Testing: do you know the actual failure mode, or just "one is atomic"?*

Both are shared-ownership reference-counted pointers that free the value when the count hits zero.
`Rc` uses a plain non-atomic counter; `Arc` uses atomic increments/decrements.

**Why `Rc` isn't `Send`:** the refcount update is a non-atomic read-modify-write. Two threads
dropping clones simultaneously can interleave and lose a decrement — the value is freed while
someone still holds it (use-after-free), or never freed (leak). `Arc` pays for atomics to prevent it.

Use `Rc` when single-threaded — the atomics are a real cost you shouldn't pay for nothing.

### Q17. What is interior mutability? Why isn't `RefCell` cheating?
> *Testing: understanding of where the checks live.*

Mutating data through a shared `&` reference. `Cell<T>` (get/set whole values, `Copy` types),
`RefCell<T>` (hands out `&`/`&mut` with **runtime** borrow counting), `Mutex`/`RwLock` (the
thread-safe versions).

It isn't cheating — it **relocates** the check from compile time to runtime. `RefCell` enforces the
exact same one-mutable-or-many-immutable rule; break it and you get a panic
(`already borrowed: BorrowMutError`) instead of a compile error. You trade a compile-time guarantee
and a small runtime cost for flexibility.

**Follow-up:** *"What actually makes it possible?"* — `UnsafeCell<T>`, the only legal way in the
language to get `&mut T` from `&T`. Every interior-mutability type is built on it.

### Q18. When would you use `Rc<RefCell<T>>`? What's wrong with it?
> *Testing: whether you've been burned yet.*

It's the single-threaded shared-mutable building block: `Rc` gives multiple owners, `RefCell` lets
them mutate. Classic use is a graph or tree with shared nodes.

What's wrong: runtime borrow **panics** instead of compile errors, reference **cycles leak** (two
nodes pointing at each other never reach refcount zero), and pointer-chasing destroys cache locality.

**The senior answer:** for graphs and trees, don't. Use an arena —
`struct Tree { nodes: Vec<Node> }` with `usize` indices instead of pointers. No `Rc`, no `RefCell`,
no cycles, no leaks, contiguous memory, trivially serialisable. This is how rustc itself does it.

**Follow-up:** *"How do you break a cycle if you must keep `Rc`?"* — `Weak<T>` for the back-edge.
`Weak` doesn't contribute to the strong count; `.upgrade()` returns `Option<Rc<T>>` — `None` if the
target is gone, which is exactly the right type.

### Q19. Does Rust prevent memory leaks?
> *Testing: precision. Candidates over-claim here.*

**No** — and being precise scores points. Rust guarantees memory *safety* (no use-after-free, no
double-free, no data races). Leaks are **safe**, just wasteful. `Rc` cycles leak;
`std::mem::forget` and `Box::leak` leak deliberately and are entirely safe functions.

The design decision: leaking can't corrupt memory or violate any invariant, so it doesn't need to
be `unsafe`.

### Q20. Static vs dynamic dispatch. When are you forced into `dyn`? ✔
> *Testing: named as a "common stumbling block" in multiple 2026 guides.*

- `fn f<T: Trait>(x: T)` / `fn f(x: impl Trait)` — **static**. Monomorphised: one specialised copy per
  concrete type, fully inlinable, zero overhead. Costs binary size and compile time.
- `fn f(x: &dyn Trait)` — **dynamic**. One copy of the code; calls go through a vtable pointer.
  One indirection, and it blocks inlining — usually the bigger cost.

`&dyn Trait` is a **fat pointer: 16 bytes** ✔ (data pointer + vtable pointer), versus 8 for `&T`.

**You're forced into `dyn` when the concrete type isn't known at compile time** — heterogeneous
collections (`Vec<Box<dyn Plugin>>`), plugin/callback registries, or when monomorphisation is
genuinely bloating the binary.

**Follow-up:** *"What makes a trait not usable as `dyn`?"* — Dyn-compatibility (called "object
safety" before Rust 1.83). A vtable is a table of function pointers, so it breaks with: generic
methods (infinitely many instantiations, no single pointer), methods taking or returning `Self` by
value, associated constants, and `Sized` supertraits. Methods marked `where Self: Sized` are excluded
from the vtable and so are allowed.

### Q21. Write a custom iterator.
> *Testing: a very common live-coding warmup.*

```rust
struct Fib { a: u64, b: u64 }

impl Iterator for Fib {
    type Item = u64;
    fn next(&mut self) -> Option<Self::Item> {
        let out = self.a;
        self.a = self.b;
        self.b = out + self.b;
        Some(out)
    }
}
```
Implement `next`; you get `map`, `filter`, `take`, `zip`, `fold`, `collect` and ~70 more free from
the trait's default methods. Say the two things that matter: **iterators are lazy** (nothing runs
until a consumer like `collect`/`sum`/`for` drives it), and **they compile to the same machine code
as a hand-written loop** — with bounds checks removed that an indexed loop keeps.

**Follow-up:** *"`iter()` vs `iter_mut()` vs `into_iter()`?"* — Yields `&T`, `&mut T`, `T`. The last
one **consumes** the collection.

### Q22. What's the difference between these two, and which handles a 50 GB file?
```rust
let a: Vec<String> = reader.lines().collect::<Result<_,_>>()?;
let b = reader.lines();
```
> *Testing: laziness, and whether you've processed real data.*

`a` materialises every line into memory — 50 GB of RAM, OOM. `b` is a lazy iterator that reads and
yields one line at a time; memory stays constant regardless of file size.

The general rule: **only `collect()` when you need the collection** — to return it, to index it, or
to iterate it more than once. A `collect()` in the middle of a chain is an allocation you're about
to throw away.

### Q23. `Vec::remove` vs `swap_remove`? And why `with_capacity`? ✔
> *Testing: do you know the cost of what you call?*

`remove(i)` is **O(n)** — it shifts every later element down to preserve order. `swap_remove(i)` is
**O(1)** — it moves the last element into the hole. Use `swap_remove` whenever order doesn't matter.

`Vec` grows by **doubling**, and each growth is an allocate + memcpy of everything so far. That's
amortised O(1) per push, but `Vec::with_capacity(n)` when you know `n` turns ~log₂(n) allocations
into one. Same for `String::with_capacity` and `HashMap::with_capacity`.

**Follow-up:** *"How do you delete many elements?"* — `v.retain(|x| keep(x))`, one pass. Removing in
a loop is O(n²) and usually skips elements as indices shift.

### Q24. Show me the `entry` API and say why it beats `contains_key`.
> *Testing: idiom fluency. Interviewers notice immediately.*

```rust
// two hash lookups, a clone, and an unwrap
if !map.contains_key(&k) { map.insert(k.clone(), Vec::new()); }
map.get_mut(&k).unwrap().push(v);

// one lookup, no clone, no unwrap
map.entry(k).or_default().push(v);
```
One hash computation instead of two or three, no `unwrap`, no clone of the key.

**Follow-up:** *"`or_insert` vs `or_insert_with`?"* — `or_insert(expensive())` evaluates the argument
**every call**, even when the key is already present. `or_insert_with(|| expensive())` is lazy. The
same eager/lazy split applies to `unwrap_or` / `unwrap_or_else` and `ok_or` / `ok_or_else`.

### Q25. Share a counter across 10 threads.
> *Testing: the most common concurrency live-coding task.*

```rust
use std::sync::{Arc, Mutex};

let counter = Arc::new(Mutex::new(0));
let handles: Vec<_> = (0..10).map(|_| {
    let c = Arc::clone(&counter);
    std::thread::spawn(move || { *c.lock().unwrap() += 1; })
}).collect();
for h in handles { h.join().unwrap(); }
```
Explain the layers: `Arc` for shared **ownership** across threads, `Mutex` for exclusive **access**.
Neither alone is enough — `Arc<i32>` gives shared read-only access with no way to mutate;
`Mutex<i32>` can't be moved into multiple threads.

**Follow-up:** *"Make it faster."* — For a plain counter, `Arc<AtomicUsize>` with
`fetch_add(1, Ordering::Relaxed)` — no lock, no syscall on contention. `Relaxed` is correct here
because you only need the count to be atomic, not ordered against other memory operations.

### Q26. `.lock().unwrap()` — why does locking return a `Result`?
> *Testing: a detail that separates readers from users.*

**Lock poisoning.** If a thread panics while holding the lock, the data may be in a half-updated
state, so every subsequent `lock()` returns `Err(PoisonError)` to warn you. You can recover the data
with `.into_inner()` if you know it's still consistent.

Most code writes `.unwrap()` — propagating a panic that already happened is usually right.

### Q27. Explain `Send` and `Sync`.
> *Testing: near-universal at senior screens.*

- **`Send`** — the type can be **moved** to another thread.
- **`Sync`** — the type can be **shared by reference** across threads. Formally: `T: Sync` ⟺ `&T: Send`.

Both are auto traits: the compiler derives them structurally, and you almost never implement them
by hand (doing so is `unsafe`, because you're asserting something the compiler couldn't prove).

Worked examples, which is what they're actually after:
- `Rc<T>` is neither — non-atomic refcount (Q16).
- `RefCell<T>` is `Send` but **not `Sync`** — its borrow counter is non-atomic, so two threads
  borrowing at once corrupt it.
- `Mutex<T>` **is `Sync`** as long as `T: Send` — the lock supplies the synchronisation its
  contents lack. This is the interesting one: `Mutex` *upgrades* a non-`Sync` type into something
  shareable.

### Q28. What is `Deref` coercion?
> *Testing: explains a dozen things that otherwise look like magic.*

The compiler automatically converts `&U` to `&T` when `U: Deref<Target = T>`. That's why `&String`
works where `&str` is expected, `&Vec<T>` where `&[T]` is expected, and why you can call `str`
methods directly on a `String`.

It's also why method calls work through `Box`, `Rc`, and `Arc` without explicit dereferencing — the
compiler inserts as many `*` as needed to find the method.

**Follow-up:** *"Should I implement `Deref` on my own types?"* — Only for genuine smart pointers.
Using it to fake inheritance on a normal struct is a known anti-pattern: it makes method resolution
unpredictable for readers.

### Q29. How do you test Rust code? Where do tests live?
> *Testing: take-homes are graded on this.*

- **Unit tests** — `#[cfg(test)] mod tests` in the same file. Can reach private items.
- **Integration tests** — `tests/*.rs`, each its own crate, public API only.
- **Doc tests** — code in `///` comments is compiled and run by `cargo test`. Your examples can't rot.

The thing that impresses on a take-home: **design for testability**. Take `impl Read`/`impl Write`
instead of `File`, so a test can pass `&b"line one\nline two"[..]` and never touch the filesystem.
Inject the clock instead of calling `Instant::now()` internally, so time-dependent logic is testable
without sleeping.

**Follow-up:** *"How do you assert a specific error?"* — `assert!(matches!(e, MyError::NotFound(_)))`,
or `#[should_panic(expected = "...")]` — always with `expected`, since without it the test passes on
*any* panic including a typo.

---

# TIER 3 — Senior (Q30–42)
*Where the questions stop having one-line answers. Async, lifetimes, unsafe, API design.*

### Q30. Why doesn't this compile, and what is `Pin` for?
```rust
async fn handler(state: Arc<Mutex<State>>) {
    let guard = state.lock().unwrap();
    fetch_remote().await;          // error: future is not Send
    println!("{}", guard.count);
}
```
> *Testing: named in multiple 2026 guides as the #1 async interview question.*

`std::sync::MutexGuard` is not `Send`. An `.await` is a suspension point where the future can be
moved to a different worker thread, so holding a non-`Send` guard across it makes the whole future
non-`Send` — and `tokio::spawn` requires `Send`. You get a wall of compiler text about it.

Even where it compiles it's a **throughput bug**: you're holding a lock across an await, so the task
parks while every other task queues behind the lock.

**Fix:** end the borrow before the await.
```rust
let count = { state.lock().unwrap().count };   // guard dropped at the brace
fetch_remote().await;
```
Use `tokio::sync::Mutex` **only** when you genuinely must hold across an await — it's slower, and
reaching for it by default is the wrong instinct. A `std::sync::Mutex` for short critical sections
is correct in async code.

**Follow-up:** *"What's `Pin` then?"* — Futures generated by `async` are **self-referential**: a
borrow that lives across an await becomes a pointer into the future's own struct. If that struct
moved, the pointer would dangle. `Pin<P>` is the type-level promise that the value won't move again,
which is what makes it sound for the executor to poll it repeatedly in place.

### Q31. What actually happens when you `.await`?
> *Testing: do you know futures are lazy and poll-based?*

An `async fn` returns a state machine implementing `Future`. Calling it **runs nothing** — futures
are lazy. Nothing happens until an executor polls it.

`.await` compiles to: poll the inner future; if `Ready(v)`, continue with `v`; if `Pending`, **return
`Pending` to the caller**, saving the current state. The executor parks the task until a `Waker`
signals readiness, then polls again, resuming from the saved state.

`async`/`await` is syntax sugar over a hand-written state machine, with the compiler generating an
enum whose variants are the segments between await points.

**Follow-up:** *"Why does Rust have no built-in runtime?"* — `Future` is a trait in `core`; the
executor is a library choice. That's what lets the same async code target a server (`tokio`) and a
microcontroller with no OS or allocator. The cost is ecosystem fragmentation and the
`tokio`-vs-`async-std` compatibility tax.

### Q32. How do you cancel an async task? What is cancellation safety?
> *Testing: staff-level async. Very few candidates get this.*

**Cancellation in Rust is `Drop`.** You cancel a future by dropping it — no cancellation token, no
exception. Whatever cleanup the future's types implement in `Drop` runs automatically.

**Cancellation safety** matters in `tokio::select!`: when one branch completes, the other futures
are **dropped mid-flight**. If a dropped future had already consumed data that it hadn't yet
returned, that data is silently lost.

`mpsc::Receiver::recv()` is cancel-safe — a dropped `recv` leaves the message queued.
`AsyncReadExt::read()` into a buffer is **not** — bytes may have been read from the socket and
dropped with the future. Rule: in a `select!` loop, only use futures documented as cancel-safe, or
hold the future in a variable across iterations so it isn't recreated.

### Q33. Explain backpressure. Why are unbounded channels dangerous?
> *Testing: production judgment.*

Backpressure is the consumer's ability to slow the producer down. With a **bounded** channel, `send`
awaits when the queue is full — the producer is throttled to the consumer's rate automatically. That
blocking *is* the feature.

An **unbounded** channel has no such mechanism: a producer faster than its consumer grows the queue
until the process OOMs. It converts a throughput problem you could have measured into a memory leak
that kills you at 3am — and the latency degrades quietly the whole way there.

**Follow-up:** *"Where else does this show up?"* — Bounding concurrent outbound requests. 10,000
`join_all`'d futures means 10,000 sockets and an instant rate-limit ban. Use
`buffer_unordered(n)` or a `tokio::sync::Semaphore`.

### Q34. What is variance? Why is `&mut T` invariant in `T`?
> *Testing: deep type-system question. Staff-level at systems shops.*

Variance says when a type with one lifetime/type parameter can be substituted for another.

- `&'a T` is **covariant** in `'a` and `T` — a `&'static str` works where `&'a str` is wanted. A
  longer-lived reference is always safe where a shorter one is expected.
- `&'a mut T` is covariant in `'a` but **invariant in `T`** — `T` must match exactly.

Why invariant: if `&mut Vec<&'static str>` could be used as `&mut Vec<&'a str>`, you could write a
short-lived reference into it through that alias, then read it back out as `&'static` after the
short one died. Dangling pointer, no `unsafe` involved. Invariance closes the hole.

- `Cell<T>` and `RefCell<T>` are invariant in `T` for the same reason — you can write through them.
- `fn(T) -> U` is **contravariant** in `T`, covariant in `U`.

**Follow-up:** *"What's `PhantomData` for?"* — Telling the compiler about a type parameter you don't
actually store: it controls variance, drop-check, and auto-trait inference. `PhantomData<&'a T>`
makes your struct covariant in `'a`; `PhantomData<fn(T)>` makes it contravariant.

### Q35. What is a higher-ranked trait bound (`for<'a>`)?
> *Testing: you've written a real generic API.*

`for<'a> F: Fn(&'a str) -> &'a str` means the closure works for **every** possible lifetime, not one
specific lifetime chosen by the caller.

You need it whenever a function takes a closure that will be handed a reference the function
creates *internally* — the lifetime doesn't exist yet at the call site, so it can't be a normal
generic parameter. It's implicit in most `Fn(&T)` bounds; you only write `for<'a>` explicitly when
inference can't work it out or the bound sits in a `where` clause on a struct.

### Q36. What can you do in `unsafe` that you can't outside it? What does it *not* turn off?
> *Testing: precision. The common wrong answer is "it disables the borrow checker."*

`unsafe` enables exactly **five** things: dereference a raw pointer, call an `unsafe` function,
implement an `unsafe` trait, mutate a `static mut`, and access a `union` field.

It does **not** turn off the borrow checker, ownership, or lifetimes — those still apply inside an
`unsafe` block. It's not "trust me, disable safety"; it's "I'm asserting invariants the compiler
can't verify."

The invariants you're now responsible for: pointers valid and properly aligned, no aliasing a `&mut`
with anything else, no reads past an allocation, no data races, and every value valid for its type
(a `bool` that isn't 0 or 1 is instant UB).

**Follow-up:** *"What is UB and why is it worse than a crash?"* — Undefined behaviour isn't
"crashes"; it's the compiler being permitted to assume it never happens. It optimises on that
assumption, so the symptom can appear in unrelated code, only in release, only on one CPU.
Time-travel bugs. That's why the rule is: smallest possible `unsafe` block, a `// SAFETY:` comment
stating the invariant, and a safe wrapper around it.

**Follow-up 2:** *"How do you test unsafe code?"* — **Miri**, the interpreter that detects UB
(`cargo +nightly miri test`). For concurrency, **loom** exhaustively explores thread interleavings.
Naming these two is a strong senior signal.

### Q37. How do you design an error type for a library?
> *Testing: API design, which is most of a take-home's grade.*

A typed enum with `thiserror`:
```rust
#[derive(Debug, thiserror::Error)]
pub enum StoreError {
    #[error("key not found: {0}")]
    NotFound(String),
    #[error("store is at capacity ({max} items)")]
    Full { max: usize },
    #[error("io failure at {path}")]
    Io { path: PathBuf, #[source] source: std::io::Error },
}
```
The reasoning: a **library** caller may want to react differently per variant — retry on `Io`,
return 404 on `NotFound` — so they need to `match`. `anyhow::Error` erases that. In a **binary**,
where the error goes to a log and an exit code, `anyhow` is right and the enum is ceremony.

Three details that mark experience: `#[source]` preserves the error chain so `{:?}` prints
`Caused by:`; every message names the **specific** thing that failed (path, key, ID) because
`"No such file or directory"` alone is useless at 2am; and `#[non_exhaustive]` on a public enum so
adding a variant later isn't a breaking change.

### Q38. What's a breaking change in a Rust library?
> *Testing: nobody expects this, and it separates library authors from app authors.*

Non-obvious ones:
- **Adding a public struct field** — breaks struct literals and exhaustive destructuring. Prevent
  with `#[non_exhaustive]`.
- **Adding an enum variant** — breaks every exhaustive `match` downstream. Same fix.
- **Adding a method to a trait** without a default body — breaks all implementors.
- **Adding a `Sized` or auto-trait bound** to an existing generic.
- **Removing a trait impl** — including accidentally, if you stop deriving something.
- Making a previously-`Send`/`Sync` type no longer so, by adding an `Rc` field. Auto traits are
  part of your public API even though you never wrote them down.

### Q39. Walk me through diagnosing a Rust service that's slower than expected.
> *Testing: process, not trivia. Very common at senior level.*

1. **Confirm it's a release build.** Debug is 10–100× slower. Most "Rust is slow" reports die here.
2. **Measure before touching anything** — `hyperfine` for whole-program, `criterion` for
   microbenchmarks, `cargo flamegraph` for where the time goes. Say explicitly that you don't guess.
3. **Look for allocation in hot loops** — this is ~90% of real Rust optimisation: `format!`,
   `to_string()`, `clone()`, `collect()`, `vec![]` inside a loop. `dhat` or a heap profiler finds
   them. Hoist the buffer out and `.clear()` it.
4. **Check the data structures** — `Vec::contains` in a loop is O(n²); swap the `HashMap` hasher
   (`rustc-hash`/`ahash`) if it's hot and keys aren't user-controlled; a linear scan of a `Vec` beats
   a `HashMap` up to ~20 elements because of cache locality.
5. **For async specifically** — look for blocking calls on the runtime (`std::fs`, `std::thread::sleep`,
   a synchronous DB driver). One blocking call starves every task on that worker thread and produces
   exactly the "inexplicable latency spikes" symptom.
6. **Only then** consider dispatch costs, `unsafe`, or SIMD.

Say out loud that you'd stop as soon as the numbers are good enough. Knowing when to stop is the
seniority signal.

### Q40. What is a zero-cost abstraction? Give an example where Rust isn't one.
> *Testing: whether you'll parrot marketing or think.*

"What you don't use, you don't pay for; what you do use, you couldn't hand-code better." Iterator
chains compile to the same machine code as manual loops. Generics are monomorphised and inlined.
`Option<&T>` is **8 bytes — the same as `&T`** ✔ because the compiler uses the null pointer as the
`None` niche.

Where it doesn't hold, which is the better half of the answer:
- **`dyn Trait`** — a real vtable indirection and a blocked inlining opportunity.
- **Bounds checking** — real, though iterators usually let LLVM eliminate it.
- **Monomorphisation** — zero *runtime* cost, but paid in binary size and compile time.
- **`Arc`** — atomic refcount traffic, which can dominate under contention.
- **`async`** — state machines can be surprisingly large, and each await point is a real branch.

**Follow-up:** *"What size is `Option<Box<T>>`?"* — 8 bytes ✔, same as `Box<T>`. Niche optimisation:
`Box` is never null, so null encodes `None`. But `Option<u8>` is **2 bytes** ✔ — `u8` uses all 256
bit patterns, so there's no spare niche and the compiler needs a separate tag byte.
`Option<bool>` is **1 byte** ✔ — `bool` only uses 0 and 1, leaving 254 niches free.

### Q41. How do you call C from Rust, and what are the hazards?
> *Testing: appears at systems shops and anywhere Rust is being introduced into a C++/Python codebase.*

```rust
#[repr(C)]
struct Point { x: f64, y: f64 }

unsafe extern "C" {
    fn process(p: *const Point, len: usize) -> i32;
}
```
`#[repr(C)]` because Rust's default layout is **unspecified** — the compiler reorders fields for
packing, so passing a default-repr struct to C is UB. `bindgen` generates these declarations from C
headers; `cbindgen` goes the other way.

Hazards worth naming: who owns and frees the memory (usually: whoever allocated it must free it —
never `free()` a Rust allocation from C); `CString` must be nul-terminated and must not contain
interior nuls; a panic unwinding across an FFI boundary aborts the process; and C strings aren't
UTF-8 so `CStr::to_str()` returns a `Result` you must handle.

**Follow-up:** *"How would you expose Rust to Python?"* — `PyO3` + `maturin`. The interview point is
that this is how Rust actually lands in a MAANG codebase: a hot path rewritten as a native module
behind an unchanged Python API, not a rewrite.

### Q42. Explain `impl Trait` in argument vs return position.
> *Testing: a small thing people get wrong constantly.*

**Argument** position: `fn f(x: impl Display)` is sugar for `fn f<T: Display>(x: T)` — the **caller**
picks the type. Only difference: you can't turbofish it.

**Return** position: `fn f() -> impl Iterator<Item = u32>` means **the callee** picks one specific
concrete type and hides it. The caller can't name it, and every return path must produce the *same*
type — returning either of two iterator types from different branches doesn't compile. That's when
you need `Box<dyn Iterator>`.

**Follow-up:** *"What changed in edition 2024?"* — Return-position `impl Trait` now captures all
in-scope lifetimes by default. In 2021 you had to write `-> impl Iterator<Item = &T> + '_` explicitly;
in 2024 it's implied. This repo is on 2024.

---

# TIER 4 — Staff / MAANG systems (Q43–50)
*Design and judgment. There's no single right answer; they're grading how you reason.*

### Q43. Design a rate limiter for a shared service. Make it testable.
> *Testing: concurrency design + testability in one.*

Sketch the API first: `RateLimiter::new(max: usize, per: Duration)` with
`fn allow(&self) -> bool`, `&self` not `&mut self` so it can live in an `Arc`.

Inside: `Mutex<VecDeque<Instant>>` — pop timestamps older than the window, compare length to the
limit, push if allowed. Then say the two things that get you the offer:

1. **Inject the clock.** `trait Clock { fn now(&self) -> Instant; }` with a real impl and a test
   impl you can advance manually. Otherwise the only way to test a 60-second window is to wait 60
   seconds, and that test will be deleted within a month.
2. **Name the contention risk.** One global `Mutex` on a hot path serialises every request. If
   that's the bottleneck, move to a token bucket over `AtomicU64` (compare-and-swap loop), or shard
   per-key so different clients don't contend.

**Follow-up:** *"Distributed across 20 instances?"* — Local limiters now allow 20× the intended rate.
Options: divide the budget per instance (simple, wastes capacity when traffic is uneven), or a
shared counter in Redis with `INCR` + TTL (accurate, adds a network hop and a dependency to every
request). State the trade-off; don't just pick.

### Q44. You need to process 10,000 documents through an external API. Design it.
> *Testing: the standard senior async design question.*

Structure the answer around the failure modes, not the happy path:

- **Concurrency bound** — `buffer_unordered(n)` or a `Semaphore`. Never unbounded; you'll be
  rate-limited or run out of file descriptors.
- **Rate limit** — a limiter shared via `Arc` (Q43), because concurrency limit ≠ rate limit.
- **Retries** — exponential backoff **with jitter**. Without jitter, everything that failed at t=0
  retries in lockstep and you DDoS the recovering service.
- **Partial failure** — one document failing must not abort 9,999 others. Collect
  `Vec<(DocId, Error)>` and report at the end; the batch's exit code reflects whether any failed.
- **Resumability** — checkpoint completed IDs to disk so a crash resumes rather than restarts. At
  10,000 items with a paid API, this is a money question, not a convenience one.
- **Observability** — a `tracing` span per document, so interleaved concurrent logs stay
  attributable. Plain log lines from 10 concurrent tasks are unreadable.
- **Shutdown** — `tokio::signal::ctrl_c()`, stop accepting new work, let in-flight finish, flush
  the checkpoint.

### Q45. When would you argue *against* using Rust?
> *Testing: engineering maturity. Candidates who can't answer this look like zealots.*

Honest answers: the team doesn't know it and the deadline is six weeks — the borrow checker is a
real productivity tax for the first 1–3 months. The work is exploratory data analysis or glue
scripting, where Python's ecosystem wins outright. The problem is dominated by an ecosystem Rust
doesn't have (mature ML training, most enterprise SDKs). Compile times are unacceptable for the
iteration loop. Or: it's a small CRUD service where the bottleneck is the database and GC pauses
were never the problem.

Then name where Rust genuinely wins: predictable tail latency with no GC pauses, memory safety in a
security-sensitive or memory-unsafe-adjacent domain, high concurrency at low resource cost, and
replacing C/C++ where the alternative is CVEs.

### Q46. Rust vs Go for a network service — argue both sides.
> *Testing: the most common architecture question when Rust is on the table.*

**Go**: faster to hire for, faster to onboard, much faster compiles, a GC that's genuinely good
enough for most services, simpler concurrency story. If p99 latency budget is 100ms and the team is
10 people, Go probably ships sooner and is cheaper to maintain.

**Rust**: no GC means no tail-latency cliff — this is the argument that actually wins, at p99.9 and
beyond. Lower memory per connection, so higher density per host (a real cost line at scale). Safe
concurrency: the data races Go permits and detects only at runtime with `-race` are compile errors.
Stronger type system for domain modelling.

The honest framing: **choose Go unless you have a specific reason.** GC pause sensitivity, memory
density at scale, a need for FFI or embedding, or a security-critical parser. "We'd like to try
Rust" is not a reason, and saying so is the answer they're looking for.

### Q47. What is false sharing and how do you fix it?
> *Testing: hardware awareness. Common at HFT, infra, and systems teams.*

Two threads writing to **different** variables that happen to sit on the same 64-byte cache line.
The cache coherence protocol invalidates the whole line on every write, so the cores ping-pong
ownership even though they never touch the same data. Throughput can drop by an order of magnitude
with no logical contention at all.

Classic case: `Vec<AtomicU64>` of per-thread counters — 8 counters land on one cache line.

Fix: pad to a cache line — `#[repr(align(64))]` on the wrapper struct, or
`crossbeam_utils::CachePadded<T>`. Or restructure so each thread owns a thread-local accumulator
and you sum at the end, which avoids the sharing entirely and is usually better.

### Q48. Explain atomic memory orderings. When is `Relaxed` correct?
> *Testing: real concurrency depth. Very few candidates handle this well.*

Orderings constrain how the compiler and CPU may reorder *other* memory operations around the atomic
one. The atomicity itself is unconditional.

- **`Relaxed`** — atomic, no ordering guarantees. Correct when the value is self-contained and
  guards nothing: a statistics counter, a metrics tally. If you only need the final count, this is
  the right and fastest choice.
- **`Acquire`** (loads) / **`Release`** (stores) — the pair. A `Release` store publishes everything
  written *before* it; an `Acquire` load that reads that value sees all of it. This is how you build
  a lock or hand off ownership of data through a flag.
- **`SeqCst`** — a single total order across all threads. Easiest to reason about, most expensive,
  and the correct default when you're unsure.

The honest senior answer: use `SeqCst` until profiling proves it matters, then weaken carefully with
`loom` verifying it. Hand-rolled lock-free code with subtly wrong orderings is a bug class that
survives code review and reproduces once a month in production.

### Q49. How do you introduce Rust into a large existing C++ or Python codebase?
> *Testing: MAANG-specific — Rust almost never arrives greenfield there.*

Not with a rewrite. Pick **one leaf component** with a narrow interface, a clear performance or
safety problem, and no deep coupling: a parser, a codec, a hot serialisation path.

- **Python** → PyO3 + maturin, shipped as a wheel. The Python API doesn't change, so the blast
  radius is one import.
- **C++** → `cxx` for a checked bridge, or a `#[repr(C)]` `extern "C"` boundary with `bindgen`/`cbindgen`.

Then the organisational half, which is what they're actually probing: who reviews Rust code, how it
enters the build system (Bazel/CMake integration is usually the real blocker, not the language),
how you handle a 3am page when the only two people who know Rust are asleep, and what the rollback
plan is. Have a measured before/after — an adoption argued on aesthetics gets reverted; one
argued on a p99 graph and a memory-CVE count survives.

### Q50. Tell me about a bug you fought the borrow checker over, and what you learned.
> *Testing: behavioral, but genuinely technical. Asked in almost every Rust interview.*

Have one real story ready with this shape: what you tried, why the compiler was right, and what the
restructure taught you.

The strongest version ends with the compiler being **correct** — a case where you were about to
alias something you shouldn't have. The weakest is "I added `.clone()` and it worked," because that
says you resolved the symptom without understanding the cause.

Good raw material if you need to build one:
- Holding `&v[0]` across a `push` (Q15) — the compiler prevented a use-after-free.
- Calling `self.helper()` inside `for x in &self.items` — the fix is destructuring
  (`let Self { items, logger, .. } = self;`) so the compiler sees disjoint field borrows, and the
  lesson is that borrows are per-field but methods borrow all of `self`.
- An `Rc<RefCell<Node>>` graph that panicked with `BorrowMutError` under load, rewritten as an
  index-based arena — the lesson being that fighting the checker often means the *data structure*
  is wrong, not the code.

---

## How to prepare

**8 weeks out:** Tier 1 until every answer is automatic out loud, not just recognised on the page.
**4 weeks out:** Tier 2 with a real `cargo` project open. Type every snippet; predict the compile
error before you run it.
**2 weeks out:** Tier 3 — build one small async service using `tokio`, `tracing`, `thiserror`, and
`clap`. Q30, Q32 and Q37 stop being abstract the moment you've shipped one.
**Final week:** Tier 4 out loud, and prepare Q50 with a story you actually lived.

Three things to practise that aren't on this list:

1. **Narrating a compiler error.** Say what it means and what you'll try. Interviewers grade the
   narration explicitly.
2. **Writing code in a plain editor** with no rust-analyzer. Most live rounds have no autocomplete.
3. **Saying "I don't know, here's how I'd find out."** In Rust this is credible — the compiler and
   the docs genuinely *are* how you find out — and it beats a confident wrong answer every time.

## Sources

Interview-question sets and topic guides consulted (searched September 2026):

- [Index.dev — Top 50 Rust Interview Questions 2026](https://www.index.dev/interview-questions/rust)
- [Zero To Mastery — 53 Rust Interview Questions](https://zerotomastery.io/blog/rust-interview-questions-and-answers/)
- [MentorCruise — 40 Rust Interview Questions (2026)](https://mentorcruise.com/questions/rust/)
- [interviewing.io — Rust Interview Questions & Tips for Senior Engineers](https://interviewing.io/rust-interview-questions)
- [Second Talent — 20 Advanced Rust Backend Questions, Senior Role](https://www.secondtalent.com/interview-guide/rust/)
- [Rustify — Rust Technical Interview Prep 2026](https://rustify.rs/articles/rust-technical-interview-prep-2026)
- [CoderPad — 25+ Rust Interview Questions](https://coderpad.io/interview-questions/rust-interview-questions/)
- [Turing — 100 Rust Interview Questions](https://www.turing.com/interview-questions/rust)
- [techinterview.org — How Rust interviews really go at defense-autonomy shops](https://www.techinterview.org/post/3233477224/rust-interview-questions-autonomy-systems/)
- [Medium (Fenn Ignatius Saji) — Preparing for a Rust Interview at MAANG](https://medium.com/@fennsaji/preparing-for-a-rust-interview-in-maang-companies-cf760b47f1f5)
- [Medium (Fenn Ignatius Saji) — Advanced Rust Interview Questions You Didn't See Coming](https://medium.com/@fennsaji/advanced-rust-interview-questions-you-didnt-see-coming-01d86e751510)
- [SharpSkill — Rust Traits and Generics 2026, with Interview Questions](https://sharpskill.dev/en/blog/rust/rust-traits-generics-advanced-guide)
- [Rust Book — Trait Objects and Dynamic Dispatch](https://doc.rust-lang.org/book/ch18-02-trait-objects.html)
- [The Rustonomicon — What Unsafe Can Do](https://doc.rust-lang.org/nomicon/what-unsafe-does.html)
- [Rust Reference — Behavior considered undefined](https://doc.rust-lang.org/reference/behavior-considered-undefined.html)
- [Tokio — Async in depth](https://tokio.rs/tokio/tutorial/async)
- [JetBrains — The Evolution of Async Rust (Feb 2026)](https://blog.jetbrains.com/rust/2026/02/17/the-evolution-of-async-rust-from-tokio-to-high-level-applications/)
- [Rustify — String vs &str, Complete Guide 2026](https://rustify.rs/articles/rust-string-vs-str-explained-2026)

**Companion files in this repo:** `rust_100_questions.md` (drill format, same material as
questions), `rust_common_mistakes_and_optimization.md` (the code-level rule sheet — Q23, Q24, Q30
and Q39 are covered there in depth).
