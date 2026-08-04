# Rust Phase 1 — Variables & Data Types (Deep Dive)

> A foundations reference written for a Python developer / AI engineer.
> Every section contrasts Rust with the Python intuition you already have, then goes one layer deeper into *why* Rust does it that way.

---

## 0. The mental shift before any syntax

In Python, a variable is a **name pointing at an object on the heap**. The object knows its own type at runtime, owns its own memory, and the garbage collector frees it when no names point to it. Types are checked while the program runs.

In Rust, a variable is a **binding to a value with a fixed, compile-time-known type and a known size**. There is no garbage collector and no runtime type. The compiler must know, before the program runs, exactly how many bytes each value occupies and exactly when it is freed. Almost every "weird" rule in this document falls out of that one constraint.

Two consequences worth holding in your head from the start:

1. **Types are erased at runtime.** `i32`, `String`, your custom struct — none of them carry a type tag at runtime the way a Python object does. The type only exists to instruct the compiler. This is why Rust is fast and why the compiler is strict.
2. **Size must be known at compile time** (for values living on the stack). This is why `String` and `&str` are different types, why arrays carry their length in the type, and why you'll meet `Box` later.

---

## 1. Variables and bindings

### 1.1 `let` and immutability by default

```rust
let x = 5;        // x is bound to 5, type inferred as i32
x = 6;            // ❌ COMPILE ERROR: cannot assign twice to immutable variable
```

In Python everything is mutable by default and you opt into immutability (tuples, frozensets, conventions like `UPPER_CASE`). Rust inverts this: **bindings are immutable unless you say otherwise.** This is not a minor style choice — immutability is the default the compiler optimizes around and the borrow checker reasons about.

```rust
let mut x = 5;    // explicitly mutable
x = 6;            // ✅ fine
```

`mut` is part of the *binding*, not the type. It says "this name is allowed to be rebound to a new value of the same type."

### 1.2 Shadowing (not the same as reassignment)

```rust
let x = 5;
let x = x + 1;        // new binding, shadows the old x — totally legal
let x = x * 2;        // and again
let x = "now a str";  // even the TYPE can change
```

This looks like Python reassignment but is fundamentally different. Each `let x` creates a **brand-new variable** that happens to reuse the name. The old value still exists (until its scope ends); you've just lost the name for it. Because it's a new binding, the type is allowed to change.

Why it matters: shadowing lets you transform a value through stages while keeping one clean name, *without* making the binding `mut`. A very common idiom:

```rust
let spaces = "   ";              // &str
let spaces = spaces.len();       // usize — same name, different type
```

In Python you'd just reassign and the type would silently change anyway. The difference is that in Rust this is an explicit, scoped, compiler-visible act — not mutation.

### 1.3 Scope and block expressions

Rust scopes are lexical and block-based like Python, but **a block is an expression that returns a value**:

```rust
let y = {
    let a = 3;
    let b = 4;
    a * a + b * b      // no semicolon → this is the block's value
};                     // y == 25
```

The missing semicolon is significant. An expression *without* a trailing semicolon is the value of the block; *with* a semicolon it becomes a statement that evaluates to `()` (the unit type — Rust's "nothing", analogous to Python's `None` but a real zero-size type). You will trip on this early; it's worth internalizing now.

### 1.4 `const` and `static`

```rust
const MAX_TOKENS: usize = 8192;          // compile-time constant
static GREETING: &str = "hello";         // fixed memory location for whole program
```

| | `const` | `static` |
|---|---|---|
| Type annotation | **required** | **required** |
| When evaluated | inlined at compile time | lives at a fixed address for the whole run |
| Mutable? | never | only via `static mut` (unsafe, discouraged) |
| Analogy | a `#define`/literal substituted everywhere | a single global object |

Both must be set to a value computable at compile time. Note the convention is `SCREAMING_SNAKE_CASE`, and unlike `let`, the type annotation is mandatory — the compiler won't infer it for items that live at the module level.

---

## 2. A stack/heap primer (you need this to understand types)

Python hides this from you completely; Rust makes it central.

- **Stack**: fast, LIFO, automatically managed. Stores values whose size is known at compile time. Pushing/popping is just moving a pointer.
- **Heap**: for data whose size is dynamic or large. Slower; requires bookkeeping. In Rust there's no GC — the compiler inserts the cleanup based on ownership rules (Phase 2).

The rule for Phase 1: **scalar types, fixed-size arrays, and tuples of fixed-size things live on the stack.** Growable things (`String`, `Vec`) keep a small fixed-size *handle* on the stack (pointer + length + capacity) and the actual data on the heap. That single fact explains the `String` vs `&str` distinction later.

---

## 3. Scalar types

Rust has four scalar categories: integers, floats, booleans, characters.

### 3.1 Integers — the part Python never made you think about

Python has one unbounded `int`. Rust has **twelve** integer types, and you must pick one (or let inference pick `i32`, the default).

| Length | Signed | Unsigned |
|---|---|---|
| 8-bit | `i8` | `u8` |
| 16-bit | `i16` | `u16` |
| 32-bit | `i32` *(default)* | `u32` |
| 64-bit | `i64` | `u64` |
| 128-bit | `i128` | `u128` |
| pointer-sized | `isize` | `usize` |

- **Signed** (`iN`) range: −2ⁿ⁻¹ to 2ⁿ⁻¹−1. **Unsigned** (`uN`) range: 0 to 2ⁿ−1.
- **`usize` / `isize`** are the width of a pointer on the target machine — 64 bits on a normal 64-bit build. **`usize` is the type of every index and length in Rust.** When you index a `Vec` or call `.len()`, you're in `usize` land. Get comfortable with it.
- `u8` is special: it's also how Rust represents a single raw byte. `Vec<u8>` is the idiomatic "bag of bytes."

**Integer literals** are flexible and readable:

```rust
let a = 98_222;       // _ is a visual separator (like 98,222)
let b = 0xff;         // hex
let c = 0o77;         // octal
let d = 0b1010_0011;  // binary
let e = b'A';         // byte literal → u8 value 65
let f = 42u64;        // type suffix: this 42 is a u64
let g: i64 = 42;      // or annotate the binding
```

**Overflow — read this carefully, it surprises Python developers.** Because integers are fixed-width, they can overflow. Rust's behavior is mode-dependent:

- **Debug builds**: overflow **panics** (crashes loudly). This is a feature — it catches bugs.
- **Release builds** (`--release`): overflow **wraps** using two's complement (255 + 1 == 0 for a `u8`), silently. Optimized for speed.

Relying on silent wrapping is a bug waiting to happen, so Rust gives you explicit methods to say what you mean:

```rust
let x: u8 = 255;
x.wrapping_add(1);     // 0   — wrap around deliberately
x.checked_add(1);      // None — returns Option, Some(v) if it fits
x.saturating_add(1);   // 255  — clamp at the max
x.overflowing_add(1);  // (0, true) — value + did-it-overflow flag
```

This is a recurring Rust theme: where Python picks one behavior for you, Rust makes you name the behavior you want.

### 3.2 Floating point

Two types, both IEEE 754:

```rust
let x = 2.0;        // f64 — the default
let y: f32 = 3.0;   // single precision
```

`f64` is the default because on modern hardware it's roughly as fast as `f32` and more precise. As an AI engineer you'll deliberately reach for `f32` (and later `bf16`/`f16` via crates) when memory bandwidth or model size dominates — but the language default is `f64`.

The usual floating-point caveats apply, and Rust does not hide them: `0.1 + 0.2 != 0.3`, `NaN != NaN` (so floats are only *partially* ordered, which matters when you try to sort them or use them as `HashMap` keys — you can't, directly). Special values exist as `f64::NAN`, `f64::INFINITY`, etc.

### 3.3 Booleans

```rust
let t = true;
let f: bool = false;
```

One byte in size. The important rule: **Rust has no truthiness.** `if x` requires `x: bool`. There is no "0 is falsy", no "empty string is falsy", no "None is falsy." `if some_vec` is a compile error; you write `if !some_vec.is_empty()`. This eliminates a whole class of Python footguns.

### 3.4 Characters — `char` is not a byte

```rust
let c = 'z';
let heart = '❤';
let crab = '🦀';
```

A Rust `char` is **4 bytes** and represents a single **Unicode scalar value** (a code point), not a byte. This is a real semantic difference from C and from Python's bytes:

- Python's `str` is a sequence of code points; iterating gives you 1-char strings. Rust's `char` is the closest analog to one of those.
- But a **`String` is not a sequence of `char`** in memory — it's UTF-8 bytes (next section). A `char` can be 1–4 bytes when encoded into a string. This mismatch is why you can't index a `String` by integer position. Hold that thought.

Single quotes = `char`; double quotes = string. They are different types, unlike Python where `'a'` and `"a"` are identical.

---

## 4. Compound types

### 4.1 Tuples

Fixed-length, ordered, **heterogeneous**, known size, stack-allocated.

```rust
let tup: (i32, f64, char) = (500, 6.4, 'x');

let (a, b, c) = tup;        // destructuring, like Python
let first = tup.0;          // access by index with .N (NOT [0])
let second = tup.1;
```

Differences from Python tuples:

- Access is `.0`, `.1`, `.2` — a compile-time field access, not runtime indexing. You can't index a tuple with a variable, because each position has a different type and the type must be known at compile time.
- The **empty tuple `()`** is the *unit type* — Rust's "no meaningful value." Functions that return "nothing" return `()`. It's a real zero-sized type, not `null`.
- The standard library implements the convenient traits on tuples only up to 12 elements; beyond that you'd use a struct anyway.

### 4.2 Arrays

Fixed-length, **homogeneous**, known size, stack-allocated. The length is **part of the type**.

```rust
let arr: [i32; 5] = [1, 2, 3, 4, 5];   // type is literally "[i32; 5]"
let zeros = [0; 5];                     // [0, 0, 0, 0, 0] — value ; count
let x = arr[0];                         // indexing with usize
```

Key points:

- `[i32; 5]` and `[i32; 6]` are **different types**. The size is baked in. This is nothing like a Python list.
- Indexing is **bounds-checked at runtime**. `arr[10]` compiles but panics at runtime ("index out of bounds") rather than reading garbage memory like C. Safety with a tiny cost.
- Arrays don't grow. The growable, heap-backed, "Python list" equivalent is **`Vec<T>`**, which you'll meet properly in Phase 3 — but mentally: *array = fixed stack buffer, `Vec` = dynamic heap list.*
- A **slice** `&[T]` is a *borrowed view* into a contiguous run of elements (of an array or a `Vec`) — a pointer plus a length, no ownership. Slices are the bridge that lets one function accept both arrays and vectors.

---

## 5. Strings — the one that confuses everyone (Phase 1 level)

Python has one `str`. Rust has two string types you meet immediately, and the distinction is the single most common beginner stumbling block. It exists entirely because of the stack/heap + ownership model.

### `&str` (string slice) vs `String` (owned string)

```rust
let a: &str = "hello";              // string literal — a &str
let b: String = String::from("hi"); // owned, heap-allocated, growable
let c: String = "hi".to_string();   // same thing, different constructor
```

| | `&str` | `String` |
|---|---|---|
| Owns its data? | no — it *borrows* a view | yes — owns heap memory |
| Growable? | no (fixed view) | yes (`push`, `push_str`, `+`) |
| Where the bytes live | could be the binary, a `String`, etc. | the heap |
| Stack footprint | pointer + length | pointer + length + capacity |
| Python analogy | a read-only window onto text | a real mutable string object |

Mental model: **`String` is the owner; `&str` is a borrowed window onto some UTF-8 bytes** (which might be owned by a `String`, or baked into your compiled program for a literal). You can always get a `&str` from a `String` cheaply (`&my_string`); going the other way (`&str` → `String`) allocates, so it's explicit (`.to_string()`).

Both are **guaranteed valid UTF-8.** This is why you **cannot index a string by integer**:

```rust
let s = String::from("héllo");
let c = s[0];     // ❌ COMPILE ERROR — strings aren't indexable by usize
```

Because a character can be multiple bytes in UTF-8, "the byte at position 0" and "the first character" aren't the same question, and Rust refuses to let you conflate them. Instead you choose your intent explicitly:

```rust
for ch in s.chars() { /* iterate Unicode scalar values */ }
for b in s.bytes()  { /* iterate raw u8 bytes */ }
let slice = &s[0..2]; // byte-range slice — panics if it splits a char boundary
```

For Phase 1 you don't need to master strings — just lock in: **literal = `&str` (borrowed), owned/growable = `String`, both are UTF-8, no integer indexing.** Everything else clicks once ownership lands in Phase 2.

---

## 6. Type inference and explicit casting

### 6.1 Inference

Rust has powerful **local type inference** (Hindley–Milner style), so you often skip annotations:

```rust
let x = 5;            // inferred i32
let v = vec![1, 2, 3]; // inferred Vec<i32>
```

But inference is *local and bidirectional*, and sometimes there isn't enough information — most famously when a method could produce many types:

```rust
let guess = "42".parse().unwrap();          // ❌ parse into WHAT?
let guess: u32 = "42".parse().unwrap();     // ✅ annotation resolves it
let guess = "42".parse::<u32>().unwrap();   // ✅ or annotate the call (turbofish)
```

That `::<u32>` syntax is affectionately called the **turbofish**. You'll see it whenever the return type can't be inferred from context.

### 6.2 Casting with `as`

Rust does **no implicit numeric coercion** — none. Adding a `u8` to a `u32` won't compile; you must convert deliberately. Unlike Python, where ints and floats mix freely, Rust makes every conversion visible:

```rust
let a: i32 = 1000;
let b = a as i64;     // widening — always safe
let c = a as u8;      // narrowing — truncates! 1000 as u8 == 232
let d = 3.9_f64 as i32; // float→int truncates toward zero → 3
```

`as` is the blunt instrument: fast, but it *truncates* on narrowing without complaint. For safe, fallible conversions you'll later use the `From`/`TryFrom` traits (Phase 3). For now, know that `as` exists, that there is **no automatic widening**, and that the compiler forcing you to write the conversion is the language preventing a silent precision bug.

---

## 7. Putting it together — a small annotated program

```rust
fn main() {
    // immutable binding, inferred i32
    let model_dim = 512;

    // explicit type + mutable
    let mut layers: u32 = 0;
    layers = 12;

    // shadowing to transform type cleanly
    let name = "transformer";          // &str
    let name = name.to_uppercase();    // String

    // tuple, destructured
    let config = (model_dim, layers, 0.1_f64);
    let (dim, n_layers, dropout) = config;

    // fixed-size array on the stack
    let head_counts: [u32; 3] = [8, 8, 16];

    // explicit, visible numeric conversion
    let total_params = (dim as u64) * (n_layers as u64) * 1_000_000;

    // no truthiness — must be a real bool
    if dropout > 0.0 && !name.is_empty() {
        println!("{name}: {dim}-dim, {n_layers} layers, ~{total_params} params");
    }
}
```

Every line here exercises a Phase 1 rule: immutability vs `mut`, shadowing with a type change, tuples and destructuring, stack arrays, explicit casts, and the absence of truthiness.

---

## 8. Exercises (do these — reading isn't learning)

1. **Overflow modes.** Make a `u8` equal to 250, then add 10 to it four different ways (`wrapping_add`, `checked_add`, `saturating_add`, `overflowing_add`). Print each result and explain the difference in a comment.
2. **Shadowing vs mut.** Write a snippet that takes a `&str` of digits, shadows it into its parsed number, then shadows again into the number doubled — without ever using `mut`.
3. **The unit type.** Write a block expression that computes a value, and a second block that ends in a semicolon. Bind both with `let` and use the compiler errors to discover what type the second one is.
4. **String reality.** Take `let s = String::from("héllo");`. Print `s.len()` and `s.chars().count()` and explain why they differ.
5. **No implicit coercion.** Try to add an `i32` and an `i64` directly; read the error; fix it with `as`; then fix it again by changing a type annotation instead. Notice you had two valid fixes.
6. **Turbofish.** Parse `"3.14"` into an `f64` two ways: with a binding annotation and with the turbofish.

Run everything with `cargo run`. Then run the overflow exercise again with `cargo run --release` and watch the behavior change — that contrast is the lesson.

---

## 9. Quick reference card

| Concept | Python | Rust |
|---|---|---|
| Default mutability | mutable | **immutable** (`let`), opt in with `mut` |
| Reuse a name, new type | reassignment | **shadowing** (`let` again) |
| Integer | one unbounded `int` | 12 sized types, default `i32`, index type `usize` |
| Overflow | never (arbitrary precision) | panics (debug) / wraps (release); use explicit methods |
| Float default | `float` (= f64) | `f64` |
| Truthiness | yes | **none** — `if` needs a real `bool` |
| Character | 1-char `str` | `char` = 4-byte Unicode scalar |
| Tuple access | `t[0]` | `t.0` (compile-time) |
| Array | dynamic `list` | fixed `[T; N]`; dynamic is `Vec<T>` |
| String | one `str` | `&str` (borrowed) vs `String` (owned), both UTF-8 |
| Numeric conversion | implicit | explicit (`as`, later `From`/`TryFrom`) |
| Constant | `UPPER = ...` (convention) | `const`/`static`, type annotation required |

---

### What's next (Phase 2 preview)

Notice how often "ownership" and "borrowing" were deferred in this document — `&str` borrows, `String` owns, slices are borrowed views. That's not an accident. Phase 1 gives you the *vocabulary of values*; Phase 2 (ownership, borrowing, lifetimes) gives you the *rules for how values move and who's responsible for freeing them*. Everything you parked here ("why two string types?", "why can't I index a string?", "what does borrowed mean?") gets its real answer there.
