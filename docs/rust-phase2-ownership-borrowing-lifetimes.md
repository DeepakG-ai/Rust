# Rust Phase 2 — Ownership, Borrowing, Lifetimes & Error Handling (Deep Dive)

> The entry fee. This is the phase where Python intuition actively works against you, and the phase that makes everything after it easy.
> Written as a continuation of Phase 1 (Variables & Data Types).

---

## 0. The one question Rust is answering

Every language must answer: **when is a value's memory freed, and who decides?**

- **C/C++**: you decide, manually. Fast, and the source of roughly every serious security vulnerability of the last forty years — use-after-free, double-free, dangling pointers, data races.
- **Python/Java/Go**: a runtime decides. Reference counting and/or a garbage collector traces what's still reachable and frees the rest. Safe, but costs you a runtime, unpredictable pauses, and memory overhead.
- **Rust**: **the compiler decides, at compile time**, using rules you encode in the types. Zero runtime cost, no GC, and the unsafe patterns are rejected before the program ever runs.

Ownership is that third answer. It is not a memory-management feature bolted onto the language — it is the language. Once you see that lifetimes, `&`/`&mut`, `String` vs `&str`, iterators, and later `Send`/`Sync` are all the *same idea* viewed from different angles, Rust stops feeling arbitrary.

**The frustration you're about to feel is real and it is temporary.** In Python you never once thought about who owns a list. Here you must, constantly, for about two weeks. Then it becomes invisible again — but now the compiler is catching bugs that Python would have shipped to production.

---

## 1. The three rules of ownership

Memorize these. Everything in this document is a consequence of them.

1. **Each value in Rust has a variable that is its *owner*.**
2. **There can only be one owner at a time.**
3. **When the owner goes out of scope, the value is *dropped*** (its memory is freed).

```rust
{
    let s = String::from("hello");  // s owns the heap buffer
    // ... use s ...
}                                   // s goes out of scope → drop(s) → heap freed
```

That closing brace is where the compiler inserts the deallocation. Not a GC. Not you. The compiler, deterministically, at a location it knows at compile time.

This pattern — resource tied to scope, cleanup automatic — is called **RAII** (Resource Acquisition Is Initialization), borrowed from C++. It generalizes beyond memory: files close, locks release, and sockets shut down at the closing brace too. The closest Python analog is a `with` block, except in Rust *every* value gets it for free, and you can't forget to write it.

### Drop order

Variables are dropped in **reverse declaration order** (like a stack unwinding); struct fields are dropped in **declaration order**. You can drop early with `drop(value)` — which, delightfully, is just a function that takes ownership and does nothing, letting scope rules do the work.

---

## 2. Move semantics — the first thing that breaks your Python brain

```rust
let s1 = String::from("hello");
let s2 = s1;
println!("{}", s1);   // ❌ COMPILE ERROR: borrow of moved value: `s1`
```

In Python, `s2 = s1` gives you two names for one object. In Rust it **moves** ownership from `s1` to `s2`, and `s1` becomes statically invalid — the compiler will refuse to let you use it.

### Why, mechanically

A `String` is three words on the stack: **pointer, length, capacity** — pointing at a heap buffer.

```
     s1 (stack)              heap
   ┌──────────┬────┐       ┌───────────┐
   │ ptr      │ ●──┼──────►│ h e l l o │
   │ len      │  5 │       └───────────┘
   │ capacity │  5 │
   └──────────┴────┘
```

`let s2 = s1;` copies those three words (cheap — always a shallow, bitwise copy). Now **two owners point at one heap buffer.** When both go out of scope, both call `drop` → **double free** → memory corruption.

Rust's fix isn't to deep-copy (expensive, and you didn't ask for it) or to reference-count (runtime cost). It's to declare `s1` **moved-out and unusable**. One owner, one drop. The problem is deleted rather than managed.

This is a *compile-time* invalidation. Nothing happens at runtime — the machine code for a move is identical to a copy. The safety is free.

### Move vs Copy

Some types don't have this problem: types living entirely on the stack with no heap resource to double-free. Those implement the **`Copy`** trait, and assignment duplicates them instead of moving.

```rust
let x = 5;
let y = x;
println!("{x}");   // ✅ totally fine — i32 is Copy
```

**`Copy` types:** all integers, `f32`/`f64`, `bool`, `char`, shared references `&T`, and tuples/arrays composed entirely of `Copy` types (`[i32; 5]` is `Copy`; `(i32, String)` is not).

**Not `Copy`:** `String`, `Vec<T>`, `Box<T>`, `&mut T`, and any type that owns a resource or has a custom `Drop`.

A type can never be both `Copy` and `Drop` — the compiler enforces this, and the reason is exactly the double-free above.

> **Mental shortcut:** if it touches the heap or manages a resource, it moves. If it's a plain stack scalar, it copies.

### `Clone` — the explicit deep copy

```rust
let s1 = String::from("hello");
let s2 = s1.clone();   // allocates a NEW heap buffer, copies bytes
println!("{s1} {s2}"); // ✅ both valid, two independent owners
```

`.clone()` is Rust's version of `copy.deepcopy()`. The design point: **it is always explicit and always visible.** Rust never silently deep-copies. When you see `.clone()` in a hot loop during profiling, you know exactly where your allocations came from — a diagnostic that Python simply doesn't give you.

---

## 3. Ownership and functions

Passing a value to a function moves it, exactly like assignment:

```rust
fn consume(s: String) {        // takes ownership
    println!("{s}");
}                              // s dropped here

let s = String::from("hi");
consume(s);
println!("{s}");               // ❌ ERROR: s was moved into consume
```

Returning a value moves ownership back out:

```rust
fn produce() -> String {
    String::from("hi")         // ownership moves to the caller
}

fn pass_through(s: String) -> String {
    s                          // take it, give it back
}
```

That `pass_through` pattern — take ownership just to hand it back so the caller can keep using it — is tedious and obviously wrong as a general solution. Which is precisely why borrowing exists.

---

## 4. Borrowing — references without ownership

A **reference** lets you access a value without taking ownership of it. Creating one is *borrowing*.

```rust
fn calculate_length(s: &String) -> usize {   // borrows
    s.len()
}                                            // s goes out of scope, but it
                                             // doesn't own anything → no drop

let s1 = String::from("hello");
let len = calculate_length(&s1);
println!("{s1} has length {len}");           // ✅ s1 still valid
```

`&s1` creates a reference *to* `s1`. `&String` is the type "reference to a String." No allocation, no move, no drop.

### Mutable references

References are immutable by default, like everything else:

```rust
fn append(s: &mut String) {
    s.push_str(", world");
}

let mut s = String::from("hello");
append(&mut s);          // explicit &mut at the call site
println!("{s}");         // "hello, world"
```

Note that the mutability is visible **at the call site** (`&mut s`), not just in the signature. Reading Rust, you can see which calls might mutate their arguments — something you cannot do in Python at all.

### The borrowing rules

> **At any given time, you may have *either* any number of immutable references (`&T`) *or* exactly one mutable reference (`&mut T`) — never both.**
>
> **References must always be valid** (never outlive the data they point to).

This is often shortened to **"shared XOR mutable"** or "aliasing XOR mutation." It is *the* central invariant of Rust.

```rust
let mut s = String::from("hello");

let r1 = &s;       // ✅ immutable borrow
let r2 = &s;       // ✅ another immutable borrow — fine, readers don't conflict
let r3 = &mut s;   // ❌ ERROR: cannot borrow `s` as mutable
                   //    because it is also borrowed as immutable
println!("{r1} {r2} {r3}");
```

### Why this rule is worth the pain

It eliminates, at compile time:

- **Data races.** A race needs two accessors where one writes. Shared-XOR-mutable makes that structurally impossible. This is the entire basis of "fearless concurrency" in Phase 4 — you get it for free from a rule you already had to follow.
- **Iterator invalidation.** In Python, mutating a list while iterating it produces silently wrong results. In Rust it doesn't compile, because iterating borrows the collection.
- **Unexpected aliasing.** The classic Python bug where you pass a list to a function, it mutates it, and a caller three frames up sees corrupted state. Rust makes that either impossible or explicitly declared in the signature.

It also enables **optimization**: knowing a `&mut T` is the *only* pointer to that data lets the compiler cache values in registers and reorder operations aggressively — guarantees C compilers would need `restrict` annotations to approach.

### Non-lexical lifetimes (NLL) — the rule is smarter than it looks

A borrow ends at its **last use**, not at the end of the enclosing block. This makes many "obviously fine" programs actually compile:

```rust
let mut s = String::from("hello");

let r1 = &s;
println!("{r1}");     // last use of r1 — its borrow ENDS HERE

let r2 = &mut s;      // ✅ fine! no overlapping borrow
r2.push_str(" world");
```

If you're reading an older tutorial and a snippet is described as an error but compiles for you, NLL is usually why.

### No dangling references, ever

```rust
fn dangle() -> &String {
    let s = String::from("hello");
    &s                  // ❌ ERROR: returns a reference to local data
}                       // s is dropped here — the reference would dangle
```

The C equivalent compiles happily and hands you a pointer to freed stack memory. Rust rejects it. Fix: return the owned value and move it out.

```rust
fn no_dangle() -> String {
    String::from("hello")
}
```

---

## 5. Slices — borrowed views into a sequence

A slice is a reference to a *contiguous run* of elements: a pointer plus a length, owning nothing.

```rust
let v = vec![1, 2, 3, 4, 5];
let all:    &[i32] = &v;         // whole vec as a slice
let middle: &[i32] = &v[1..4];   // [2, 3, 4]

let s = String::from("hello world");
let hello: &str = &s[0..5];      // string slice
let word:  &str = first_word(&s);
```

This is what `&str` actually is: **a slice into UTF-8 bytes.** Phase 1 said "`&str` borrows, `String` owns" — now you know what borrowing means, and the whole distinction should click.

Because a slice borrows, the borrow checker prevents the classic bug where a view outlives what it views:

```rust
let mut s = String::from("hello world");
let word = first_word(&s);   // immutable borrow of s
s.clear();                   // ❌ ERROR: needs &mut s while word still borrows
println!("{word}");          // word would point into freed memory
```

Python has no defense against the equivalent bug. Rust makes it a compile error.

### The API lesson: take slices, not owned types

```rust
fn process(data: &str)    { /* ... */ }   // ✅ accepts &String AND &str AND literals
fn process(data: &String) { /* ... */ }   // ❌ needlessly restrictive
```

Thanks to **deref coercion**, `&String` automatically becomes `&str`, and `&Vec<T>` becomes `&[T]`, at call sites. So a function taking `&str`/`&[T]` accepts strictly more inputs at zero cost.

> **Rule of thumb:** parameters take `&str` and `&[T]`; return types and struct fields use `String` and `Vec<T>`.

For your AI work this is the everyday shape: `fn softmax(logits: &[f32]) -> Vec<f32>` — borrow the input, own the output.

---

## 6. Lifetimes — naming relationships the compiler can't infer

Every reference has a lifetime: the region of code over which it's valid. **You've been using lifetimes this whole document** — the compiler inferred them. Annotations are only needed when inference is ambiguous.

### The critical misconception

**Lifetime annotations do not change how long anything lives.** They don't extend, shorten, or manage anything. They are *descriptions of relationships that already exist*, so the compiler can verify them. Think of them as type-level documentation the compiler checks — closer to a generic parameter than to a runtime mechanism.

### When you need them

```rust
fn longest(x: &str, y: &str) -> &str {     // ❌ ERROR: missing lifetime specifier
    if x.len() > y.len() { x } else { y }
}
```

The compiler's problem: the returned reference borrows from `x` *or* `y`, and it can't tell which. Without knowing, it can't verify the result doesn't outlive its source. You supply the relationship:

```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}
```

Read `'a` as a generic lifetime parameter: *"for some lifetime `'a`, both inputs live at least that long, and the output is valid for exactly that long."* Concretely, `'a` resolves to the **shorter** of the two input lifetimes — the conservative, always-safe choice.

If a return value doesn't actually borrow from the inputs, no annotation is needed — return an owned `String` instead and the whole question disappears. **That's often the right answer while learning.**

### Lifetime elision — why you rarely write them

The compiler applies three rules; if they fully determine every output lifetime, you write nothing:

1. Each elided **input** lifetime gets its own distinct parameter.
2. If there is **exactly one** input lifetime, it's assigned to **all** output lifetimes.
3. If one of the inputs is `&self` or `&mut self`, **`self`'s lifetime** is assigned to all outputs.

Rule 2 is why `fn first_word(s: &str) -> &str` needs no annotation. Rule 3 is why methods almost never do. `longest` fails because it has two inputs (rule 2 doesn't apply) and no `self` (rule 3 doesn't apply).

### Lifetimes in structs

A struct holding a reference must declare that it cannot outlive what it borrows:

```rust
struct Parser<'a> {
    input: &'a str,       // Parser cannot outlive the string it points into
    pos: usize,
}

impl<'a> Parser<'a> {
    fn rest(&self) -> &str {      // elision rule 3 handles this
        &self.input[self.pos..]
    }
}
```

This is a real pattern (zero-copy parsers, tokenizers) — but note that structs holding references are a meaningful step up in complexity. **While learning, prefer owned fields (`String`, `Vec<T>`).** Reach for borrowed fields when profiling proves the copies matter.

### `'static`

`'static` means "valid for the entire program duration." String literals are `&'static str` — they're baked into the binary.

```rust
let s: &'static str = "I live in the executable";
```

Two warnings. First, `'static` is not a way to silence lifetime errors; if the compiler suggests it and the data isn't genuinely program-long, the suggestion is wrong and you have a design problem. Second, the bound `T: 'static` on a generic means something subtler than "lives forever" — it means "contains no non-`'static` references," which `String` and `i32` satisfy. You'll meet this in Phase 4 with `thread::spawn`.

---

## 7. Error handling — the other half of Phase 2

Rust has **no exceptions**. There is no `try`/`except`, no invisible control flow that can unwind out of any line. Errors are **values**, returned from functions, and the type system forces you to acknowledge them.

This belongs in Phase 2 because `Option` and `Result` are enums holding *owned* values, and everything you just learned about moving, borrowing, and matching applies directly.

### The two-way split

| | Recoverable | Unrecoverable |
|---|---|---|
| Mechanism | `Result<T, E>` | `panic!` |
| Meaning | "this can legitimately fail" | "a bug / broken invariant" |
| Python analog | a caught exception | an assertion failure or crash |
| Example | file missing, bad input, network timeout | index out of bounds, failed invariant |

### `Option<T>` — the null that can't bite you

```rust
enum Option<T> { Some(T), None }
```

**Rust has no `null`.** Absence is encoded in the type, so "forgetting to check for `None`" is a compile error rather than an `AttributeError: 'NoneType' object has no attribute ...` at 3am. This single design choice removes Tony Hoare's "billion dollar mistake."

```rust
let maybe: Option<i32> = Some(5);

match maybe {
    Some(n) => println!("got {n}"),
    None    => println!("nothing"),
}

if let Some(n) = maybe {          // when you only care about one arm
    println!("got {n}");
}

let value = maybe.unwrap_or(0);   // supply a default
```

`match` is **exhaustive**: if you forget the `None` arm, it does not compile. That exhaustiveness is one of Rust's quiet superpowers, and it applies to every enum you write.

### `Result<T, E>` — fallible operations

```rust
enum Result<T, E> { Ok(T), Err(E) }
```

```rust
use std::fs::File;

let f = File::open("config.toml");   // Result<File, std::io::Error>

let f = match f {
    Ok(file)  => file,
    Err(e)    => panic!("failed to open: {e}"),
};
```

`Result` is marked `#[must_use]` — ignoring one produces a compiler warning. You cannot silently swallow a failure the way a bare `except: pass` does.

### `unwrap` and `expect`

```rust
let f = File::open("config.toml").unwrap();                    // panic on Err
let f = File::open("config.toml").expect("config.toml missing"); // panic with message
```

Both panic on failure. They're fine in prototypes, examples, and tests. **In real code prefer `expect` with a message stating the invariant you believe holds** — when it panics anyway, the message tells future-you what assumption broke. Treat `unwrap` in production paths as a code smell.

### The `?` operator — the workhorse

```rust
use std::fs;
use std::io;

fn read_config() -> Result<String, io::Error> {
    let content = fs::read_to_string("config.toml")?;   // Err → return early
    Ok(content.trim().to_string())
}
```

`?` means: if `Ok(v)`, unwrap to `v` and continue; if `Err(e)`, **return early** from the function with that error. It's the ergonomic payoff that makes value-based errors pleasant rather than verbose.

Two details worth knowing now:

1. **`?` converts error types automatically** via the `From` trait. If your function returns `MyError` and `?` encounters an `io::Error`, it calls `From::from` to convert — provided you implemented `From<io::Error> for MyError`. This is what makes error propagation across layers clean.
2. **`?` also works on `Option`**, returning `None` early, inside a function that returns `Option`.

`?` only works in functions returning `Result`/`Option` — including `main`:

```rust
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let content = std::fs::read_to_string("config.toml")?;
    println!("{content}");
    Ok(())
}
```

### Combinators — handling errors without `match` pyramids

```rust
maybe.map(|n| n * 2)              // transform the Some/Ok value
maybe.and_then(|n| checked(n))    // chain another Option/Result-returning call
maybe.unwrap_or(0)                // default value
maybe.unwrap_or_else(|| compute())// lazily computed default
maybe.ok_or(MyError::Missing)?    // Option → Result, then propagate
result.map_err(MyError::from)?    // transform the error type
```

`and_then` is monadic bind; `map` is `map`. Coming from Python, these read like a cleaner version of chained `if x is not None`.

### Defining your own errors

For **libraries**, define a concrete error enum so callers can match on variants. `thiserror` removes the boilerplate:

```rust
use thiserror::Error;

#[derive(Error, Debug)]
enum ConfigError {
    #[error("could not read config file")]
    Io(#[from] std::io::Error),      // #[from] generates the From impl for ?
    #[error("invalid value for {key}")]
    BadValue { key: String },
}
```

For **applications**, where you mostly want to propagate and print, `anyhow` is the standard choice:

```rust
use anyhow::{Context, Result};

fn load() -> Result<Config> {
    let raw = std::fs::read_to_string("config.toml")
        .context("reading config.toml")?;     // adds human-readable context
    Ok(parse(&raw)?)
}
```

> **The convention:** `thiserror` for libraries (typed, matchable), `anyhow` for binaries (ergonomic, contextual). `Box<dyn Error>` is the dependency-free middle ground.

### When to panic

Panic when a **bug** has occurred — a broken invariant, an impossible state, a contract violation. Use `Result` when failure is a **legitimate, expected outcome** the caller should decide about. A missing file is a `Result`; an index out of bounds in code you wrote to be in-bounds is a panic.

---

## 8. Fighting the borrow checker — and winning

You will hit these. Here's the pattern-match, so you lose less time.

**"cannot borrow `x` as mutable more than once"**
You're holding two `&mut` into the same data. Restructure so the borrows don't overlap in time (NLL helps), or use `split_at_mut` for disjoint slice halves, or scope one borrow in a `{ }` block.

**"cannot borrow `x` as mutable because it is also borrowed as immutable"**
Classic case: iterating a collection while modifying it. Fixes: collect the changes into a `Vec` and apply them after the loop; iterate over indices instead of elements; or use in-place APIs like `retain`, `iter_mut`, or `drain`.

**"cannot move out of borrowed content"**
You have a `&T` and are trying to take ownership. Options: `.clone()` it; change the signature to take `T` by value; or use `std::mem::take(&mut x)` (leaves `Default::default()` behind) or `std::mem::replace(&mut x, new)` to swap the value out.

**"borrowed value does not live long enough"**
Something outlives its source. Usually the fix is to return an owned value rather than a reference, or to hoist the owner into a longer-lived scope.

**Two methods on `self` both want `&mut self`**
The borrow checker is field-sensitive *within* a function body but treats a method call as borrowing the whole struct. Fix: destructure fields into locals first, or split the struct so the two concerns own separate data.

### Two pieces of pragmatic advice

**First: `.clone()` is allowed while learning.** Cloning to get past the borrow checker is not cheating; it's trading a little performance for forward progress, and it's still faster than most Python. Get it compiling, then profile and remove the clones that matter. Fighting for zero-copy perfection on day three is how people quit.

**Second: when a design fights the borrow checker relentlessly, the design is usually wrong.** Deeply interlinked object graphs — the kind Python encourages, where everything holds a reference to everything — are exactly what ownership rejects. The Rust answer is usually to restructure around ownership (a tree with clear parents, or indices/IDs into a central `Vec` instead of pointers). `Rc<T>`/`RefCell<T>` exist as escape hatches for genuine shared ownership (Phase 5), but reaching for them in week two is usually avoiding the lesson.

---

## 9. Exercises

Do these in order — each targets one specific misconception.

1. **Feel the move.** Create a `String`, assign it to another variable, then print the first. Read the compiler error in full. Now do the same with an `i32` and observe that it works. Explain the difference in a comment.
2. **Fix it three ways.** Take the broken snippet from #1 and make it compile three different ways: with `clone`, with a reference, and by restructuring so ownership is returned. Note the cost of each.
3. **Ownership through functions.** Write `fn takes(s: String)` and `fn borrows(s: &String)`. Call each and observe which lets you use the variable afterwards.
4. **Break the borrow rules on purpose.** Hold a `&` and a `&mut` to the same `String` simultaneously. Read the error. Then move the `println!` of the immutable reference *above* the mutable borrow and watch NLL make it compile.
5. **Iterator invalidation.** Try to `push` to a `Vec` inside a `for` loop over that same `Vec`. Then fix it by collecting the new items into a separate `Vec` and extending after the loop. This is the Python bug that Rust refuses to let you write.
6. **Write `first_word`.** `fn first_word(s: &str) -> &str` returning the first whitespace-delimited word. Then try calling `s.clear()` while holding the result, and understand the error.
7. **Lifetime annotation.** Implement `longest` yourself. Then try to return a reference to a `String` created *inside* the function and understand why no annotation can rescue it.
8. **Elision check.** For each of these, decide whether elision applies before compiling: `fn f(x: &str) -> &str`, `fn g(x: &str, y: &str) -> &str`, `fn h(&self, x: &str) -> &str`.
9. **`Option` without null.** Write `fn find_user(id: u32) -> Option<String>`. Handle the result with `match`, then `if let`, then `unwrap_or_else`.
10. **Error propagation.** Write a function that reads a file, parses its contents as an integer, and returns `Result<i32, Box<dyn Error>>`. Use `?` for both fallible steps — note that two *different* error types propagate through the same `?`.
11. **Custom error type.** Redo #10 with a `thiserror` enum and `#[from]` conversions. Match on the variants at the call site.
12. **Zero-copy struct.** Build the `Parser<'a>` struct from §6. Then try to make it outlive the string it borrows and read the error carefully.

---

## 10. Quick reference card

| Concept | Python | Rust |
|---|---|---|
| Memory management | garbage collector, runtime | ownership, compile time, no GC |
| `b = a` | two names, one object | **move** (or copy if `Copy`) |
| Deep copy | `copy.deepcopy()` | `.clone()` — always explicit |
| Pass to function | passes a reference implicitly | **moves** unless you write `&` |
| Read-only access | convention only | `&T` — enforced |
| Mutable access | any reference can mutate | `&mut T` — exclusive, visible at call site |
| Aliasing rule | anything goes | many `&T` **XOR** one `&mut T` |
| Cleanup | GC, `with` for resources | scope-based `Drop` (RAII), automatic |
| Dangling pointer | impossible (GC) | impossible (borrow checker) |
| Null | `None`, checked at runtime | **no null**; `Option<T>`, checked at compile time |
| Errors | exceptions, invisible control flow | `Result<T, E>` values, `?` to propagate |
| Ignoring an error | `except: pass` | `#[must_use]` warning; must be handled |
| Unrecoverable failure | uncaught exception | `panic!` |
| Function param style | `list`, `str` | `&[T]`, `&str` (accept borrowed views) |
| Function return style | anything | `Vec<T>`, `String` (return owned) |

### The one-paragraph summary

Every value has exactly one owner; when the owner's scope ends, the value is dropped. Assigning or passing a heap-owning value **moves** it, invalidating the source, so nothing is ever freed twice. To use a value without taking it, **borrow** it: any number of shared `&` readers, or exactly one exclusive `&mut` writer, never both, and never outliving the data. **Lifetimes** name those relationships when the compiler can't infer them. Errors are ordinary values — `Option<T>` for absence, `Result<T, E>` for failure — propagated with `?` and impossible to silently ignore.

---

### What's next (Phase 3 preview)

You now own the hard part. Phase 3 — **traits, generics, iterators, and collections** — is where the effort pays off: traits are compile-time-checked interfaces (Python protocols with teeth), generics give you abstraction at zero runtime cost, and iterator chains compile down to loops as fast as anything you'd write by hand. You'll also finally understand why `Vec<T>`, `HashMap<K, V>`, and `String` behave the way they do, because you now know what owning `T` means.

And a note for Phase 4: the shared-XOR-mutable rule you just internalized is *literally* what makes Rust's concurrency safe. Data races require aliasing plus mutation. You already banned that combination. Threads come almost free.
