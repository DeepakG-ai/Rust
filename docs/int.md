# Rust Interview Questions & Answers

**Prepared by MadMax (Max)**  
35 questions covering fundamentals → advanced → practical coding.  
Answers are below each section. Try answering before reading.

---

## Section 1: Fundamentals (Q1–Q10)

**Q1.** What is ownership? State the three rules.

**Q2.** What is the difference between a move and a copy? Which types are `Copy`?

**Q3.** Explain `&T` vs `&mut T`. What is the borrowing rule?

**Q4.** What is the difference between `String` and `&str`? Describe their memory layout.

**Q5.** What is a lifetime? What are the lifetime elision rules?

**Q6.** What is the difference between `Option<T>` and `Result<T, E>`? When do you use each?

**Q7.** What does the `?` operator do exactly?

**Q8.** What is shadowing? How is it different from mutation?

**Q9.** What is the difference between an array `[T; N]`, a slice `&[T]`, and a `Vec<T>`?

**Q10.** What is a `match` expression, and why must it be exhaustive?

---

## Section 2: Intermediate (Q11–Q22)

**Q11.** `Box<T>`, `Rc<T>`, `Arc<T>`, `RefCell<T>`, `Mutex<T>` — when do you use each?

**Q12.** What are `Send` and `Sync`? Give an example of a type that is `Send` but not `Sync`.

**Q13.** Trait objects (`dyn Trait`) vs generics (`T: Trait`) — trade-offs?

**Q14.** What makes a trait "object-safe" (dyn-compatible)?

**Q15.** What is the `Drop` trait? In what order are values dropped?

**Q16.** What is the orphan rule? How do you work around it?

**Q17.** What is the newtype pattern and why is it useful?

**Q18.** `impl Trait` in argument position vs return position — what's the difference?

**Q19.** How do Rust iterators achieve "zero-cost abstraction"?

**Q20.** Explain `Deref` and deref coercion. Why does `&String` work where `&str` is expected?

**Q21.** What are the differences between `const`, `static`, and `let`?

**Q22.** What is the difference between `panic!` and returning `Err`? When is panicking acceptable?

---

## Section 3: Advanced (Q23–Q30)

**Q23.** What does `unsafe` allow, and what does it NOT turn off?

**Q24.** How does `async`/`.await` work under the hood?

**Q25.** What is `Pin<T>` and why does async need it?

**Q26.** What is variance? Is `&'a mut T` covariant in `T`?

**Q27.** What is the difference between `Rc<RefCell<T>>` and `Arc<Mutex<T>>`? Why not `Arc<RefCell<T>>`?

**Q28.** What is monomorphization? What are its costs?

**Q29.** What is a Higher-Ranked Trait Bound (`for<'a>`)? When do you need it?

**Q30.** What is `PhantomData<T>` used for?

---

## Section 4: Practical / Coding (Q31–Q35)

**Q31.** Implement a generic `Stack<T>` with `push`, `pop`, `peek`.

**Q32.** Reverse a singly linked list: `struct Node { val: i32, next: Option<Box<Node>> }`.

**Q33.** Spawn 4 threads that each increment a shared counter 1000 times. Return the final value.

**Q34.** Fix this code:
```rust
fn first_word() -> &str {
    let s = String::from("hello world");
    &s[..5]
}
```

**Q35.** Remove all even numbers from a `Vec<i32>` in place. Why does the naive index loop fail?

---
---

# ANSWERS

---

## Section 1: Fundamentals

### A1. Ownership
Ownership is Rust's compile-time memory management model. Rules:
1. Each value has exactly one owner.
2. There can only be one owner at a time.
3. When the owner goes out of scope, the value is dropped (destructor runs, memory freed).

This replaces garbage collection and manual `free()`. Memory safety is checked statically — zero runtime cost.

---

### A2. Move vs Copy
- **Move**: ownership transfers; the source becomes invalid. Default for heap-owning types (`String`, `Vec`, `Box`).
- **Copy**: bitwise duplication; both source and destination remain valid. Only for types where a shallow copy is a valid full copy.

`Copy` types: integers, floats, `bool`, `char`, shared references `&T`, tuples/arrays of `Copy` types.  
A type **cannot** be `Copy` if it implements `Drop` — copying would cause a double-free.

```rust
let a = String::from("x");
let b = a;          // move — `a` is now invalid
let x = 5;
let y = x;          // copy — both valid
```

---

### A3. Borrowing
- `&T`: shared (immutable) reference. Many can coexist.
- `&mut T`: exclusive (mutable) reference. Only one, and no `&T` may exist simultaneously.

The rule: **aliasing XOR mutability**. You may have either many readers or one writer, never both. This eliminates data races and iterator invalidation at compile time.

---

### A4. `String` vs `&str`
| | `String` | `&str` |
|---|---|---|
| Owns data? | Yes (heap) | No |
| Layout | `(ptr, len, capacity)` — 24 bytes on 64-bit | `(ptr, len)` — fat pointer, 16 bytes |
| Growable? | Yes | No |
| Where stored | Heap buffer | Anywhere: static memory, heap, stack |

Both are guaranteed valid UTF-8. `&str` is a view; `String` is an owner. Prefer `&str` in function parameters (accepts both via deref coercion).

---

### A5. Lifetimes
A lifetime is the compiler's name for a scope during which a reference is valid. It prevents dangling references.

**Elision rules** (compiler infers when):
1. Each input reference gets its own lifetime parameter.
2. If there is exactly one input lifetime, it is assigned to all output references.
3. If there are multiple inputs but one is `&self` / `&mut self`, `self`'s lifetime goes to outputs.

If none apply, you must annotate:
```rust
fn longest<'a>(a: &'a str, b: &'a str) -> &'a str { if a.len() > b.len() { a } else { b } }
```
`'a` here means: the output lives at least as long as the *shorter* of the two inputs.

---

### A6. `Option` vs `Result`
- `Option<T>`: `Some(T)` | `None` — absence of a value is **expected and not an error**. (e.g. lookup in a map)
- `Result<T, E>`: `Ok(T)` | `Err(E)` — an operation **can fail** and you want to know why. (e.g. file I/O, parsing)

Both are plain enums. `Option<&T>` and `Option<Box<T>>` are niche-optimized to the same size as the pointer (null = `None`).

---

### A7. The `?` operator
Desugars roughly to:
```rust
match expr {
    Ok(v) => v,
    Err(e) => return Err(From::from(e)),
}
```
Key points:
- Early returns on `Err` / `None`.
- Calls `From::from` on the error — this is how `?` converts `io::Error` into your custom error type automatically if you `impl From<io::Error> for MyError`.
- Works in functions returning `Result`, `Option`, or any type implementing `Try` (nightly).

---

### A8. Shadowing
Declaring a new variable with the same name. The old binding is hidden, not mutated. The new variable can have a **different type**.

```rust
let x = "42";        // &str
let x: i32 = x.parse().unwrap();  // i32 — new binding
```
Mutation (`let mut x`) changes the value in place and cannot change the type.

---

### A9. Array vs Slice vs Vec
- `[T; N]`: fixed size known at compile time, stored inline (stack if local). Size is part of the type.
- `&[T]`: a borrowed view `(ptr, len)` into any contiguous sequence. Size known at runtime.
- `Vec<T>`: owned, heap-allocated, growable `(ptr, len, capacity)`.

`Vec<T>` derefs to `&[T]`, and `[T; N]` coerces to `&[T]` — so write functions that take `&[T]`.

---

### A10. `match`
Pattern matching expression. Must be exhaustive because Rust guarantees no undefined behavior from unhandled cases — the compiler proves every variant is covered. `_` is the catch-all.

Since it's an expression, all arms must evaluate to the same type. Supports guards (`x if x > 5`), bindings (`n @ 1..=9`), destructuring, and nested patterns.

---

## Section 2: Intermediate

### A11. Smart pointer selection
| Type | Ownership | Mutability | Thread-safe | Cost |
|---|---|---|---|---|
| `Box<T>` | Single | via `&mut` | if `T` is | Heap alloc |
| `Rc<T>` | Shared | No (immutable) | **No** | Non-atomic refcount |
| `Arc<T>` | Shared | No (immutable) | Yes | Atomic refcount |
| `RefCell<T>` | N/A (wrapper) | Interior, runtime-checked | **No** | Borrow flag + panic on violation |
| `Mutex<T>` | N/A (wrapper) | Interior, locked | Yes | OS lock / blocking |

Common combos: `Rc<RefCell<T>>` (single-thread shared mutable), `Arc<Mutex<T>>` or `Arc<RwLock<T>>` (multi-thread shared mutable).

---

### A12. `Send` and `Sync`
Both are **auto traits** (implemented automatically if all fields are).
- `Send`: `T` can be moved to another thread.
- `Sync`: `&T` can be shared across threads, i.e. `&T: Send`.

Examples:
- `Rc<T>`: neither (non-atomic refcount).
- `Arc<T>`: both, if `T: Send + Sync`.
- `RefCell<T>`: `Send` but **not `Sync`** — the borrow counter is not atomic, so two threads holding `&RefCell` could race on it. Moving it wholesale to one thread is fine.
- `MutexGuard<T>`: `Sync` but not `Send` (lock must be released on the acquiring thread on many platforms).

---

### A13. `dyn Trait` vs generics
**Generics** (`fn f<T: Trait>(x: T)`):
- Static dispatch via monomorphization — a copy per concrete type.
- Inlinable, fastest. Larger binary. Type known at compile time.

**Trait objects** (`fn f(x: &dyn Trait)`):
- Dynamic dispatch via vtable. Fat pointer `(data ptr, vtable ptr)`.
- One copy of code. Enables heterogeneous collections `Vec<Box<dyn Shape>>`.
- Indirect call cost; no inlining across the call; must be object-safe.

Rule of thumb: generics by default; `dyn` when you need runtime polymorphism or want to reduce compile time / binary size.

---

### A14. Object safety
A trait is object-safe (usable as `dyn Trait`) if:
- No method returns `Self` (unless `where Self: Sized`).
- No generic methods (vtable can't hold infinite instantiations).
- No associated constants.
- The trait doesn't require `Self: Sized`.
- All supertraits are object-safe.

Reason: the vtable needs a finite, fixed layout, and the concrete `Self` type is erased.

---

### A15. `Drop` and drop order
`Drop::drop(&mut self)` is the destructor — runs when the value goes out of scope. Cannot be called manually (use `std::mem::drop(x)` to drop early).

Order:
- **Local variables**: reverse declaration order (last declared, first dropped).
- **Struct fields**: declaration order (first field first).
- **Tuple / array elements**: in order.
- **Temporaries**: at end of the enclosing statement.

Unwinding on panic also runs destructors (unless `panic = "abort"`).

---

### A16. Orphan rule
You may `impl Trait for Type` only if **the trait or the type is defined in your crate**. Prevents two crates from providing conflicting impls (coherence).

Workaround: **newtype pattern** — wrap the foreign type in a local struct.
```rust
struct MyVec(Vec<i32>);
impl fmt::Display for MyVec { /* ... */ }
```

---

### A17. Newtype pattern
A tuple struct with one field: `struct Meters(f64);`

Uses:
- Type safety: `Meters` and `Feet` can't be mixed accidentally, zero runtime cost.
- Bypass orphan rule.
- Restrict / hide API of the inner type.
- Implement different trait behavior for the same underlying type.

Zero-cost: identical memory layout to the inner type.

---

### A18. `impl Trait` positions
- **Argument position**: `fn f(x: impl Display)` — sugar for `fn f<T: Display>(x: T)`. Caller picks the type. You lose the ability to name `T` or use turbofish.
- **Return position**: `fn f() -> impl Iterator<Item = u32>` — the **function body** picks exactly one concrete type; caller only sees the trait. Opaque type, still static dispatch. Essential for returning closures and iterator chains without `Box<dyn>`.

A return-position `impl Trait` cannot return different concrete types from different branches.

---

### A19. Zero-cost iterators
- Adapters (`map`, `filter`, `take`) are **lazy structs** that wrap the previous iterator — no work until consumed.
- Each is generic; monomorphization + inlining lets LLVM collapse `iter().map().filter().sum()` into a single loop with no function calls.
- Bounds checks are often elided since the iterator tracks its own bounds.

Result: typically identical or better assembly than a hand-written `for` loop with indexing.

---

### A20. `Deref` and coercion
`Deref` trait defines `*` for smart pointers: `impl Deref for Box<T> { type Target = T; }`.

**Deref coercion**: at coercion sites (function args, `let` with type annotation, method receivers), the compiler inserts as many `.deref()` calls as needed:
- `&String` → `&str` (`String: Deref<Target = str>`)
- `&Vec<T>` → `&[T]`
- `&Box<T>` → `&T`
- `&Rc<T>` → `&T`

Also how method lookup works: `rc.len()` auto-derefs through `Rc` → `String` → `str`.

---

### A21. `const` / `static` / `let`
- `const`: compile-time constant, **inlined** at every use site; no fixed memory address. Must be a constant expression. Type annotation required.
- `static`: single memory location for the whole program, `'static` lifetime, fixed address. `static mut` is unsafe to access. Type annotation required.
- `let`: runtime local binding; can be inferred; scoped.

Use `const` for values, `static` when you need a single address (FFI, large lookup tables, atomics like `static COUNTER: AtomicUsize`).

---

### A22. `panic!` vs `Err`
- `Err`: **recoverable**, expected failures (bad input, network down). Caller decides.
- `panic!`: **unrecoverable** bug — invariant violated, programmer error. Unwinds the stack (or aborts).

Panicking is acceptable for: unreachable states, `unwrap()` on values you've proven valid, tests, prototypes. Library code should almost never panic on user input.

`unwrap()` / `expect()` / indexing out of bounds / integer overflow in debug — all panic.

---

## Section 3: Advanced

### A23. `unsafe`
`unsafe` **enables** five things:
1. Dereference raw pointers `*const T` / `*mut T`.
2. Call `unsafe fn` (including FFI).
3. Access or modify `static mut`.
4. Implement `unsafe trait` (e.g. `Send`, `Sync`).
5. Access `union` fields.

It does **NOT** disable: the borrow checker, type checking, lifetime checking, or bounds checks on slices.

`unsafe` means: "the compiler cannot verify this; I take responsibility for upholding the invariants." A sound `unsafe` block must not allow UB from *any* safe caller.

---

### A24. `async`/`.await` internals
- `async fn` compiles into an anonymous **state machine** implementing `Future`. Each `.await` is a state where the machine can suspend.
- `Future::poll(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Output>` — returns `Ready(v)` or `Pending`.
- On `Pending`, the future registers the `Waker` from `cx`. When the I/O event fires, the waker tells the executor to poll again.
- Rust ships **no executor** — tokio, async-std, smol provide the event loop and task scheduler.

Futures are lazy: nothing runs until polled. Dropping a future cancels it.

---

### A25. `Pin<T>`
Async state machines can be **self-referential** — a local reference held across an `.await` points into the future's own memory. If the future is moved, that pointer dangles.

`Pin<P>` wraps a pointer `P` and guarantees the pointee will **not be moved** until dropped — unless `T: Unpin`.

- `Unpin` is an auto trait: most types are `Unpin` (moving them is fine). `Pin<&mut T>` for `T: Unpin` is just `&mut T`.
- Compiler-generated futures are `!Unpin`.
- `Box::pin(fut)` or `pin!(fut)` to pin.

`Pin` is a type-level contract, not a runtime mechanism.

---

### A26. Variance
Variance describes how subtyping of a type parameter propagates. In Rust, subtyping exists only for lifetimes: `'long: 'short` means `'long` is a subtype of `'short`.

- **Covariant**: `&'long T` can be used where `&'short T` is expected. `&'a T` is covariant in both `'a` and `T`. `Box<T>`, `Vec<T>` covariant in `T`.
- **Invariant**: `&'a mut T` is covariant in `'a` but **invariant in `T`**. Reason: if you could treat `&mut &'long str` as `&mut &'short str`, you could write a short-lived reference into a location the caller expects to hold a long-lived one → dangling. `Cell<T>`, `RefCell<T>`, `UnsafeCell<T>` are invariant.
- **Contravariant**: `fn(T)` is contravariant in `T`.

---

### A27. `Rc<RefCell<T>>` vs `Arc<Mutex<T>>`
Both give shared ownership + interior mutability.
- `Rc<RefCell<T>>`: single-threaded. Non-atomic refcount, non-atomic borrow flag. Panics on double `borrow_mut`.
- `Arc<Mutex<T>>`: multi-threaded. Atomic refcount, OS/futex lock. Blocks on contention.

**Why not `Arc<RefCell<T>>`?** It won't compile: `Arc<T>: Send` requires `T: Send + Sync`, and `RefCell<T>` is `!Sync`. The type system prevents the data race on `RefCell`'s borrow counter.

---

### A28. Monomorphization
The compiler generates a separate concrete copy of each generic function/struct for every type it's instantiated with. `Vec<i32>` and `Vec<String>` are completely different compiled code.

Benefits: static dispatch, inlining, type-specialized optimization — no runtime cost.  
Costs: **longer compile times**, **larger binaries** (code bloat), instruction cache pressure with many instantiations. Mitigate with `dyn Trait` or non-generic inner functions.

---

### A29. Higher-Ranked Trait Bounds
`for<'a> F: Fn(&'a str) -> &'a str` means: `F` must satisfy the bound for **every** possible lifetime `'a`, not one specific one chosen by the caller.

Needed when a closure/function is called with references whose lifetimes are created *inside* the function you're writing:
```rust
fn apply<F>(f: F) where F: for<'a> Fn(&'a str) -> &'a str {
    let local = String::from("hi");
    f(&local);   // 'a is the lifetime of this borrow — not nameable by the caller
}
```
`Fn(&str) -> &str` already elides to an HRTB; you write `for<'a>` explicitly when elision doesn't apply.

---

### A30. `PhantomData<T>`
A zero-sized marker type that tells the compiler "pretend this struct owns/uses a `T`" without storing one.

Uses:
- **Unused type parameter**: `struct Id<T>(u64, PhantomData<T>);` — type-safe IDs.
- **Lifetime tracking with raw pointers**: `struct Iter<'a, T> { ptr: *const T, _marker: PhantomData<&'a T> }` — ties the iterator to the borrowed data.
- **Drop check**: `PhantomData<T>` tells dropck the struct may drop a `T`.
- **Variance control**: `PhantomData<fn(T)>` makes a struct contravariant/invariant in `T`.
- **Opt out of auto traits**: `PhantomData<*const ()>` makes a type `!Send + !Sync`.

---

## Section 4: Practical / Coding

### A31. Generic Stack
```rust
pub struct Stack<T> {
    items: Vec<T>,
}

impl<T> Stack<T> {
    pub fn new() -> Self { Self { items: Vec::new() } }
    pub fn push(&mut self, v: T) { self.items.push(v); }
    pub fn pop(&mut self) -> Option<T> { self.items.pop() }
    pub fn peek(&self) -> Option<&T> { self.items.last() }
    pub fn len(&self) -> usize { self.items.len() }
    pub fn is_empty(&self) -> bool { self.items.is_empty() }
}
```
Key point: `peek` returns `Option<&T>` (borrow), not `Option<T>` — you can't move out of the vec without removing.

---

### A32. Reverse linked list
```rust
struct Node { val: i32, next: Option<Box<Node>> }

fn reverse(mut head: Option<Box<Node>>) -> Option<Box<Node>> {
    let mut prev: Option<Box<Node>> = None;
    while let Some(mut node) = head {
        head = node.next.take();   // detach the rest
        node.next = prev;          // point backwards
        prev = Some(node);
    }
    prev
}
```
`take()` replaces `node.next` with `None` and gives ownership of the old value — avoids the "cannot move out of borrowed content" error.

---

### A33. Shared counter across threads
```rust
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let counter = Arc::new(Mutex::new(0u64));
    let handles: Vec<_> = (0..4).map(|_| {
        let c = Arc::clone(&counter);
        thread::spawn(move || {
            for _ in 0..1000 {
                *c.lock().unwrap() += 1;
            }
        })
    }).collect();

    for h in handles { h.join().unwrap(); }
    println!("{}", *counter.lock().unwrap()); // 4000
}
```
Alternative with atomics (no lock):
```rust
use std::sync::atomic::{AtomicU64, Ordering};
let counter = Arc::new(AtomicU64::new(0));
// in thread: counter.fetch_add(1, Ordering::Relaxed);
```
`Relaxed` is fine here because we only need the final total after `join`, which provides synchronization.

---

### A34. Dangling reference fix
Problem: `s` is dropped at end of function; returning `&s[..5]` would be a dangling pointer. Compiler error: "missing lifetime specifier" / "returns a reference to data owned by the current function".

Fix — return owned data:
```rust
fn first_word() -> String {
    let s = String::from("hello world");
    s[..5].to_string()
}
```
Or, if the data is truly constant, return `&'static str`:
```rust
fn first_word() -> &'static str { "hello" }
```
Or take the input as a parameter and borrow from it:
```rust
fn first_word(s: &str) -> &str { &s[..5] }
```

---

### A35. Remove evens in place
Idiomatic:
```rust
fn remove_evens(v: &mut Vec<i32>) {
    v.retain(|x| x % 2 != 0);
}
```
`retain` is O(n), single pass, shifts elements once.

Why the naive loop fails:
```rust
for i in 0..v.len() {        // len evaluated once
    if v[i] % 2 == 0 { v.remove(i); }  // shifts elements, skips the next one, then indexes out of bounds
}
```
Two bugs: skipping the element that shifts into position `i`, and eventual index-out-of-bounds panic since `len` was captured before removals. Also `remove` is O(n) each → O(n²) total.

Manual correct version (two-pointer):
```rust
let mut w = 0;
for r in 0..v.len() {
    if v[r] % 2 != 0 { v[w] = v[r]; w += 1; }
}
v.truncate(w);
```

---