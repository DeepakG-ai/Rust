# Rust — The Gaps: What Phases 1–3 Missed

> An honest audit from a working-developer perspective.
> The first three documents went deep on *language semantics*. This one covers the **practical** things you need to actually ship code — several of which you'll need earlier than the roadmap implied.

---

## The headline problem with the roadmap

The phases were organized by **conceptual difficulty**, not by **when you need things**. That's a real flaw. Here's the corrected picture:

| Topic | Where I put it | Where you actually need it |
|---|---|---|
| Modules & project structure | never | **day 2** — the moment `main.rs` exceeds 200 lines |
| `cargo clippy` | never | **day 1** — it's the best Rust teacher available |
| Testing | never | **week 1** — Rust culture is test-heavy |
| Structs, enums, `match` | "Phase 1" (unwritten) | **week 1** — you can't write anything without them |
| `Box<T>` | Phase 5 | **Phase 3** — needed for `dyn Trait` and recursive types |
| `serde` | never | **week 3** — the moment you touch JSON |
| Debug vs release performance | never | **the first time you benchmark and panic** |

Nothing in the existing documents is wrong. But you'd have finished all three and still not known how to split a file into modules or write a test. Let's fix that.

---

# PART A — Core language pieces that were skipped

## A1. Functions, control flow, and expression-orientation

I promised the rest of Phase 1 and never wrote it. The essentials:

```rust
fn add(x: i32, y: i32) -> i32 {
    x + y                          // no semicolon = return value
}

fn early(x: i32) -> i32 {
    if x < 0 { return 0; }         // explicit return works too
    x * 2
}
```

**Everything is an expression.** There is no ternary operator because `if` already is one:

```rust
let grade = if score > 90 { "A" } else { "B" };     // if as expression
let x = match n { 0 => "zero", _ => "other" };      // match as expression
let y = loop { break 42; };                          // loop returns a value!
```

Three loop forms, and `loop` is the only one that can return a value via `break`:

```rust
loop { ... }                        // infinite; break with a value
while cond { ... }
for x in collection { ... }         // the one you'll use 95% of the time

'outer: for i in 0..10 {            // labeled loops for nested breaks
    for j in 0..10 {
        if done { break 'outer; }
    }
}
```

Ranges: `0..5` is exclusive, `0..=5` inclusive. There is no C-style `for(;;)` loop — use `for i in 0..n`.

## A2. Structs and `impl`

```rust
struct Model {                          // named-field struct
    name: String,
    layers: u32,
}

struct Point(f64, f64);                 // tuple struct — access with .0, .1
struct Marker;                          // unit struct — zero size, useful as a type tag

impl Model {
    // associated function (no self) — the constructor convention
    fn new(name: &str, layers: u32) -> Self {
        Self { name: name.to_string(), layers }     // field init shorthand
    }

    fn describe(&self) -> String { ... }            // borrows
    fn add_layer(&mut self) { self.layers += 1; }   // mutably borrows
    fn consume(self) -> String { self.name }        // takes ownership
}
```

Key points a Python dev needs:

- **`Self`** (capital) means "the type"; **`self`** (lowercase) is the instance. `Self { .. }` is the idiomatic constructor body.
- **There is no `__init__`.** `new()` is pure convention, not a language feature. You can have `from_config()`, `with_capacity()`, etc.
- **The three receivers matter enormously**: `&self` (read), `&mut self` (write), `self` (consume). This is Phase 2 expressed in method signatures — choosing the wrong one is the #1 cause of borrow errors in your own APIs.
- **Struct update syntax** fills the gap left by keyword arguments:

```rust
let cfg = Config { layers: 24, ..Default::default() };
```

## A3. Enums — far more powerful than Python's

Python's `Enum` is named constants. Rust's enums are **algebraic data types** — each variant can carry different data:

```rust
enum Message {
    Quit,                                  // no data
    Move { x: i32, y: i32 },               // struct-like
    Write(String),                         // tuple-like
    ChangeColor(u8, u8, u8),
}
```

This is the single most useful modeling tool in the language. `Option<T>` and `Result<T, E>` are just enums — nothing special. **Model your domain with enums** and the compiler will force you to handle every case forever after.

The Python equivalent requires a class hierarchy plus `isinstance` checks, with no exhaustiveness guarantee.

## A4. Pattern matching — deeper than I showed

```rust
match msg {
    Message::Quit => ...,
    Message::Move { x, y } => ...,                 // destructure struct variant
    Message::Write(text) if text.is_empty() => ..., // match GUARD
    Message::Write(text) => ...,
    Message::ChangeColor(r, g, b) => ...,
}

match n {
    0 => "zero",
    1..=9 => "digit",                    // range pattern
    x if x < 0 => "negative",            // guard
    _ => "big",                          // catch-all
}

let (a, b, ..) = tuple;                  // .. ignores the rest
let Point { x, .. } = point;             // partial destructure
let msg @ 1..=5 = n;                     // @ binds AND tests
```

**Exhaustiveness is the superpower.** Add a variant to an enum six months later, and the compiler shows you every single place that needs updating. Refactoring in Rust is qualitatively different from refactoring in Python because of this.

Three shorthands you'll use constantly:

```rust
if let Some(x) = opt { ... } else { ... }    // one case matters
while let Some(x) = stack.pop() { ... }      // loop until None
let Some(x) = opt else { return; };          // let-else: bind or diverge
```

`let ... else` (stable since 1.65) is the cleanest way to write guard clauses and dramatically reduces nesting. Underused by beginners.

## A5. Modules and project structure — **the biggest omission**

You need this the moment `main.rs` gets long.

```
my_project/
├── Cargo.toml
└── src/
    ├── main.rs          # binary crate root
    ├── lib.rs           # library crate root
    ├── config.rs        # a module
    └── model/
        ├── mod.rs       # module root (or model.rs alongside model/)
        └── layers.rs    # submodule
```

```rust
// main.rs
mod config;              // declares & loads src/config.rs
mod model;               // loads src/model/mod.rs or src/model.rs

use config::Settings;    // bring into scope
use model::layers::Dense;

// config.rs
pub struct Settings { pub dim: u32, dropout: f32 }   // per-FIELD visibility!
pub fn load() -> Settings { ... }
fn helper() { }                                       // private to this module
```

Visibility levels — finer-grained than Python's underscore convention:

| Modifier | Visible to |
|---|---|
| *(nothing)* | current module and its children |
| `pub` | everyone |
| `pub(crate)` | anywhere in this crate — **the one you'll want most** |
| `pub(super)` | the parent module |

Path prefixes: `crate::` (crate root), `super::` (parent), `self::` (current).

**Everything is private by default**, including struct fields. Unlike Python, `_private` is enforced, not a suggestion. This is genuinely better — encapsulation is real.

Binary vs library: a project can have both. Put logic in `lib.rs` (testable, reusable) and keep `main.rs` a thin CLI wrapper. **Do this from the start** — it makes integration testing possible.

## A6. Smart pointers & interior mutability — needed sooner than Phase 5

I deferred these, but `Box` is required for Phase 3 material.

**`Box<T>`** — heap allocation with single ownership. Needed for:
1. Trait objects: `Box<dyn Trait>` (you met this in Phase 3)
2. **Recursive types** — a type can't contain itself directly because its size would be infinite:

```rust
enum List {
    Cons(i32, Box<List>),        // Box gives it a known size (a pointer)
    Nil,
}
```
3. Moving large values without copying the bytes.

**`Rc<T>`** — reference-counted shared ownership, single-threaded. Multiple owners; freed when the last one drops. This is exactly Python's model, which is why it feels natural — and why over-reaching for it is a trap.

**`RefCell<T>`** — **interior mutability**: mutate through a `&T`. Moves the borrow check from compile time to **runtime**; violating the rules panics instead of failing to compile.

```rust
use std::rc::Rc;
use std::cell::RefCell;

let shared = Rc::new(RefCell::new(vec![1, 2, 3]));
let clone = Rc::clone(&shared);           // cheap — bumps refcount
clone.borrow_mut().push(4);               // mutate through a shared ref
```

`Rc<RefCell<T>>` is the "I want Python's object model" escape hatch. It works, it's safe (panics rather than corrupts), and it costs you refcount updates plus runtime borrow tracking.

> **Use it when the data genuinely is shared and mutable — graphs, observers, trees with parent pointers.** But if you're reaching for it in week two to dodge the borrow checker, restructure instead. The usual Rust answer is indices into a central `Vec` rather than pointers between objects.

**`Cow<'a, T>`** (clone-on-write) — borrow if possible, clone only when mutation is needed. Excellent for text processing where most inputs pass through unmodified:

```rust
fn normalize(s: &str) -> Cow<str> {
    if s.contains(' ') { Cow::Owned(s.replace(' ', "_")) }
    else { Cow::Borrowed(s) }              // zero allocation in the common case
}
```

## A7. Macros — at least know what the `!` means

Every `!` you've seen is a macro invocation: `println!`, `vec!`, `format!`, `panic!`, `assert!`, `dbg!`, `write!`, `matches!`.

Macros run at compile time and generate code, which is why `println!` can type-check its format string and accept variable argument counts — things a Rust *function* cannot do. You don't need to write macros for a long time, but you should know:

- `#[derive(Debug)]` is a **procedural macro** generating an impl block.
- `macro_rules!` is declarative macros — pattern matching on syntax.
- `serde`, `thiserror`, `tokio::main`, `clap` are all macro-driven. This is why they feel magical.

Useful ones immediately:

```rust
dbg!(&x);                       // prints file:line, expr, and value; returns it
assert_eq!(a, b);
matches!(x, Some(1..=5));       // pattern test as a bool
todo!() / unimplemented!()      // typed placeholders that compile
```

## A8. Const generics — relevant to your domain

Generics over *values*, not just types:

```rust
struct Matrix<const R: usize, const C: usize> {
    data: [[f32; C]; R],
}

fn dot<const N: usize>(a: [f32; N], b: [f32; N]) -> f32 { ... }
```

Compile-time-checked dimensions. For ML work this means **shape mismatches become compile errors** rather than runtime exceptions three hours into a training run. Not something to learn on day one, but worth knowing it exists — it's a capability Python fundamentally cannot offer.

---

# PART B — The developer workflow (the real gap)

This is the section that matters most and was completely absent.

## B1. Cargo beyond `cargo run`

```bash
cargo new my_app              # binary project
cargo new my_lib --lib        # library project
cargo check                   # type-check WITHOUT codegen — 5-10x faster
cargo build                   # debug build
cargo build --release         # optimized build
cargo run -- arg1 arg2        # args after --
cargo test
cargo add serde --features derive    # add a dependency (built in since 1.62)
cargo tree                    # dependency graph — find duplicate versions
cargo doc --open              # build & view docs for YOUR crate + all deps
cargo clippy
cargo fmt
```

**`cargo check` is the one to internalize.** Your inner loop is `cargo check` (fast), not `cargo build` (slow). rust-analyzer runs it continuously in your editor.

`Cargo.toml` essentials:

```toml
[package]
name = "my_app"
version = "0.1.0"
edition = "2024"

[dependencies]
serde = { version = "1", features = ["derive"] }
tokio = { version = "1", features = ["full"] }

[dev-dependencies]              # test/bench only, not shipped
criterion = "0.5"

[profile.release]
lto = true                      # link-time optimization
codegen-units = 1               # slower build, faster binary
```

**`Cargo.lock`**: commit it for binaries, don't for libraries. Same logic as `poetry.lock`/`requirements.txt`.

**Features** are Cargo's conditional compilation — a crate ships optional functionality you opt into. This is why you write `features = ["derive"]` for serde. There's no Python equivalent; the closest is extras (`pip install pkg[extra]`).

**Workspaces** — a monorepo of related crates sharing one `Cargo.lock` and `target/`:

```toml
[workspace]
members = ["core", "cli", "python-bindings"]
```

This is exactly the layout for a PyO3 project later: pure Rust core, thin Python binding crate.

## B2. rust-analyzer — non-negotiable

Install the rust-analyzer extension for VS Code (or your editor's LSP client). It gives you inline type hints for every inferred binding, inline error display, go-to-definition into the standard library source, and auto-import.

**Rust without rust-analyzer is drastically harder than Rust with it.** The inline type annotations alone teach you inference by showing you what the compiler concluded. If you're evaluating whether Rust is tolerable, do it with rust-analyzer running.

## B3. Clippy — your best teacher

```bash
cargo clippy
cargo clippy -- -W clippy::pedantic     # much stricter
cargo clippy --fix                       # auto-apply safe fixes
```

Clippy has 700+ lints across correctness, performance, style, and complexity. Critically, **it doesn't just flag problems — it explains the idiomatic alternative.** It will tell you to use `if let` instead of `match`, that your `.iter().count()` should be `.len()`, that a `&String` parameter should be `&str`, that your manual loop is a `fold`.

> Run clippy after every exercise you write. It converts "code that compiles" into "code a Rust developer would write" faster than any book. This is the single highest-leverage habit for a Rust beginner and I should have put it in Phase 1.

`cargo fmt` is rustfmt — non-negotiable formatting, zero config debates. Like `black`, but universal in the ecosystem.

## B4. Testing — built into the language

Rust's testing story is better than Python's and requires no third-party framework.

**Unit tests** live in the same file as the code:

```rust
pub fn add(a: i32, b: i32) -> i32 { a + b }

#[cfg(test)]                     // only compiled during `cargo test`
mod tests {
    use super::*;                // import the parent module

    #[test]
    fn adds_correctly() {
        assert_eq!(add(2, 2), 4);
    }

    #[test]
    #[should_panic(expected = "divide by zero")]
    fn panics_on_zero() { divide(1, 0); }

    #[test]
    fn works_with_results() -> Result<(), String> {
        let v = parse("42")?;
        assert_eq!(v, 42);
        Ok(())
    }
}
```

Note the tests are **inside the module**, so they can test private functions — something Python makes awkward.

**Integration tests** go in `tests/` at the project root. Each file is compiled as a separate crate and can only use your **public** API — which is exactly the point.

**Doc tests — the unique one.** Code examples in your documentation are compiled and run as tests:

````rust
/// Adds two numbers.
///
/// # Examples
///
/// ```
/// use my_crate::add;
/// assert_eq!(add(2, 2), 4);
/// ```
pub fn add(a: i32, b: i32) -> i32 { a + b }
````

`cargo test` runs that. **Your documentation cannot rot**, because stale examples break the build. No other mainstream language does this as well.

```bash
cargo test                       # everything, in parallel
cargo test add                   # only tests matching "add"
cargo test -- --nocapture        # show println! output
cargo test -- --test-threads=1   # run serially
```

Ecosystem additions: `criterion` (statistical benchmarking), `proptest`/`quickcheck` (property testing), `insta` (snapshot testing), `mockall` (mocks).

## B5. Documentation

```rust
//! Crate-level docs — goes at the top of lib.rs
//! Describes the whole module/crate.

/// Item-level docs — goes above a function, struct, etc.
/// Supports **Markdown**.
///
/// # Errors
/// Returns `Err` if the input is empty.
pub fn parse(s: &str) -> Result<i32, Error> { ... }
```

`cargo doc --open` generates a browsable site for your crate and every dependency. **Learn to read docs.rs** — every published crate has auto-generated documentation there, and it's typically far better than the average Python package's docs because the tooling makes it automatic.

## B6. Debugging and diagnostics

```bash
RUST_BACKTRACE=1 cargo run       # full stack trace on panic
RUST_BACKTRACE=full cargo run
```

```rust
dbg!(&value);                    // prints file:line + value, returns it
eprintln!("{x:?}");              // stderr
```

For real debugging: `rust-gdb`/`rust-lldb`, or the CodeLLDB extension in VS Code (breakpoints work normally). For structured logging in applications, `tracing` is the modern standard; `log` + `env_logger` is the simpler classic.

**Read compiler errors in full.** Rust's diagnostics are the best in the industry — they name the problem, point at the exact span, explain the rule, and usually suggest a fix. `rustc --explain E0502` gives a full essay on any error code. Coming from Python tracebacks, the instinct is to skim; resist it. The error message is the lesson.

## B7. Editions

Rust guarantees backward compatibility, so opt-in changes ship as **editions**: 2015, 2018, 2021, 2024. Set in `Cargo.toml`. Crates of different editions interoperate freely.

Practically: **use `edition = "2024"` for new projects**, and be aware that tutorials written for 2015/2018 may show outdated idioms (`extern crate`, older module paths, `dyn`-less trait objects). If a tutorial's syntax looks archaic, check its date.

## B8. Debug vs release — **the gotcha that ambushes everyone**

**Debug builds are 10–100× slower than release builds.** Not a typo.

Debug builds have `opt-level = 0`, overflow checks on, debug assertions on, and no inlining. If you benchmark a Rust program with `cargo run` and conclude it's slower than your NumPy code, this is why — and it's the single most common "Rust is slow?" false alarm.

```bash
cargo run --release      # ALWAYS for any performance measurement
```

Corollary: development is fast to compile and slow to run; production is the reverse. If your debug build is genuinely too slow to test with (common with image or tensor work), optimize just your dependencies:

```toml
[profile.dev.package."*"]
opt-level = 3            # deps optimized, your code still debuggable
```

---

# PART C — Python-developer gotchas nobody warns you about

**No default arguments. No keyword arguments. No function overloading.** Rust has none of these. The workarounds:

```rust
// 1. Option parameters
fn connect(host: &str, timeout: Option<u64>) { ... }

// 2. A config struct + Default
fn connect(cfg: Config) { ... }
connect(Config { host: "x".into(), ..Default::default() });

// 3. The builder pattern (idiomatic for many options)
Client::builder().host("x").timeout(30).build()?;

// 4. Differently-named constructors instead of overloads
Vec::new();  Vec::with_capacity(10);  Vec::from(slice);
```

This surprises people more than the borrow checker does. Budget for it in API design.

**No inheritance.** No `class Foo(Bar)`. Rust has traits (shared behavior, with defaults) and composition (structs containing structs). Any design leaning on deep inheritance hierarchies must be restructured. In practice this is a feature — "composition over inheritance" enforced by the language.

**No REPL.** There's no `python -i`. Use [play.rust-lang.org](https://play.rust-lang.org) for snippets, `cargo new scratch` for local experiments, or install `evcxr` for an actual REPL. Expect this to feel limiting at first; the compiler's feedback partly replaces it.

**Integer division and modulo differ from Python.** Rust truncates toward zero; Python floors:

```
Rust:  -7 / 2 == -3      -7 % 2 == -1     (remainder, sign follows dividend)
Python: -7 // 2 == -4    -7 % 2 ==  1     (floor, sign follows divisor)
```

This is a genuine silent-bug source when porting algorithms. Use `div_euclid` / `rem_euclid` for Python semantics.

**No `**` operator.** Use `x.pow(2)` for integers, `x.powi(2)` / `x.powf(0.5)` for floats.

**String formatting:**

```rust
println!("{x}");            // inline capture (1.58+) — prefer this
println!("{}", x);          // positional
println!("{x:?}");          // Debug
println!("{x:#?}");         // pretty Debug — great for structs
println!("{x:.2}");         // 2 decimal places
println!("{x:>10}");        // right-align, width 10
let s = format!("{x}");     // build a String instead of printing
```

**Compile times are genuinely slow** and this is Rust's most legitimate criticism. Mitigate with `cargo check`, rust-analyzer, keeping dependency counts sane, and `sccache` for shared caching.

**`clone()` is not a moral failure.** Clone freely while learning; profile later. Idiomatic-but-unwritten code is worth nothing.

---

# PART D — Ecosystem crates you'll actually use

Rust's stdlib is deliberately small; the ecosystem fills the gaps. Knowing the canonical crate for each job saves enormous time.

| Need | Crate | Python analog |
|---|---|---|
| **Serialization (JSON etc.)** | **`serde` + `serde_json`** | `json`, `pydantic` |
| Async runtime | `tokio` | `asyncio` |
| HTTP client | `reqwest` | `requests` |
| Web server | `axum` (or `actix-web`) | `fastapi` |
| CLI parsing | `clap` (derive API) | `argparse`, `typer` |
| Data parallelism | `rayon` | `multiprocessing` |
| Error handling | `thiserror` (lib), `anyhow` (app) | exceptions |
| Logging/tracing | `tracing` | `logging` |
| Regex | `regex` | `re` |
| Dates/times | `chrono` or `jiff` | `datetime` |
| Random | `rand` | `random` |
| Extra iterator adapters | `itertools` | `itertools` |
| Benchmarking | `criterion` | `pytest-benchmark` |
| Property testing | `proptest` | `hypothesis` |
| Faster hashing | `rustc-hash`, `ahash` | — |
| Env/config | `dotenvy`, `config`, `figment` | `python-dotenv` |

**`serde` deserves special mention.** It's derive-driven, near-zero-cost, and the single most important crate in the ecosystem:

```rust
#[derive(Serialize, Deserialize, Debug)]
struct Config { name: String, layers: u32 }

let cfg: Config = serde_json::from_str(&raw)?;      // parse + validate + type-check
let json = serde_json::to_string(&cfg)?;
```

That's `pydantic`-grade validation with zero runtime cost, checked at compile time. It's usually the moment Rust starts feeling *better* than Python rather than merely faster.

Find crates via [lib.rs](https://lib.rs) (better curated) or [crates.io](https://crates.io). Judge by recent releases, download count, and docs.rs quality.

---

# PART E — Performance realities

Things that actually matter once you're optimizing, roughly in order of impact:

1. **Build with `--release`.** Covered above; it dwarfs everything else.
2. **Avoid allocation in hot loops.** `String::new()` or `vec![]` inside a loop is the usual culprit. Hoist buffers out and reuse with `.clear()`; use `with_capacity` when the size is known.
3. **Prefer borrowing to cloning** — but only after profiling says so.
4. **Swap the hasher.** `FxHashMap`/`AHashMap` over the DoS-resistant default for internal maps.
5. **Iterator chains are already optimal.** They compile to the same code as manual loops. Don't hand-unroll; you'll usually make it worse.
6. **`Vec<T>` beats linked structures** almost always — cache locality dominates asymptotics at realistic sizes.
7. **Profile, don't guess.** `cargo flamegraph`, `perf`, `criterion` for microbenchmarks, `hyperfine` for whole-program timing.
8. **Release profile tuning**: `lto = true`, `codegen-units = 1`, `panic = "abort"` — typically 10–20% for a slower build.

**Never micro-optimize before profiling.** Rust makes it tempting because the control is right there. Resist; the compiler is smarter than your intuition about it.

---

# PART F — For your AI/ML work specifically

Beyond the Phase 6 items (PyO3, maturin, candle, burn):

| Need | Crate |
|---|---|
| N-d arrays | `ndarray` (NumPy-shaped API) |
| Linear algebra | `nalgebra` (small fixed-size, graphics-flavored) |
| DataFrames | `polars` (you likely already use it from Python) |
| Tokenization | `tokenizers` (HuggingFace, Rust-native) |
| ONNX inference | `ort` (ONNX Runtime bindings) |
| PyTorch bindings | `tch-rs` (libtorch) |
| Model weights | `safetensors` |
| Data parallelism | `rayon` |
| GPU compute | `wgpu`, `cudarc` |

Three things worth understanding early:

**Memory layout is now yours to control.** Contiguous `Vec<f32>` with manual striding beats `Vec<Vec<f32>>` decisively — the nested version scatters rows across the heap and destroys cache locality. This is why `ndarray` and every real tensor library use flat buffers with shape metadata.

**`rayon` is nearly free parallelism.** Changing `.iter()` to `.par_iter()` parallelizes a chain with compile-time-verified correctness. For CPU-bound preprocessing this is often a 4–8× win for a one-word diff.

**SIMD**: `std::simd` is still nightly-only; use the `wide` crate on stable, or rely on autovectorization (which LLVM does well when you write simple loops over slices).

**The realistic near-term win** isn't rewriting your models — it's rewriting the *bottleneck*. Tokenization, data loading, feature extraction, custom loss computation. Wrap it with PyO3, ship it as a wheel, keep the rest in Python. That's the pattern `tokenizers` and `polars` themselves follow, and it's where your leverage is highest.

---

# PART G — Two things nobody tells you

**1. The plateau is real and it's not a sign you're failing.**

There's a stretch — usually weeks 2–4 — where you understand every rule individually and still can't make the compiler happy. Everyone hits it. The way through is writing code, not reading more explanations. If you're stuck on a borrow error for more than twenty minutes, `.clone()` it, add a `// TODO: remove clone`, and move on. You'll understand the fix in two weeks; today you need momentum.

**2. Read other people's Rust.**

More than in most languages, Rust has a strong idiom culture that books don't fully convey. Reading real code is how you absorb it. Good targets, in rough order of accessibility:

- `ripgrep` — a clean, well-structured, realistic CLI application
- `tokenizers` — directly relevant to your domain
- `polars` — large, sophisticated, production-scale
- The **standard library source** — click "src" on any docs.rs page; it's more readable than you'd expect

Also: `rustlings` for compiler-driven exercises, [Rust by Example](https://doc.rust-lang.org/rust-by-example/) for snippets, and **Jon Gjengset's "Crust of Rust"** YouTube series, which is the best intermediate Rust content in existence.

---

# The revised roadmap

Slotting the gaps into the right places:

| Phase | Content |
|---|---|
| **0 — Setup (day 1)** | `rustup`, cargo basics, **rust-analyzer**, **clippy**, **fmt**, edition 2024, debug-vs-release awareness |
| **1 — Foundations** | Data types (✅ documented), **functions, control flow, structs, enums, pattern matching, modules** |
| **1.5 — Workflow (week 1)** | **Testing (unit/integration/doc), documentation, `dbg!`, reading compiler errors** |
| **2 — Ownership** | Ownership, borrowing, lifetimes, error handling (✅ documented) |
| **3 — Abstraction** | Traits, generics, iterators, closures, collections (✅ documented) + **`Box`, `Cow`** |
| **3.5 — Ecosystem** | **`serde`, `clap`, `anyhow`/`thiserror`, `rayon`** — build a real CLI here |
| **4 — Concurrency** | Threads, `Send`/`Sync`, `Arc<Mutex<T>>`, channels, `async`/`tokio` |
| **5 — Advanced** | `Rc`/`RefCell` in depth, `unsafe`, macros, const generics, profiling |
| **6 — AI payoff** | PyO3 + maturin, `ndarray`, `candle`, `tokenizers`, `polars` internals |

The additions are Phase 0, Phase 1.5, and Phase 3.5 — all workflow and ecosystem, all things that make the difference between knowing Rust and *using* Rust.

**The most important single change:** run `cargo clippy` on every piece of code you write, starting today. It will teach you more idiomatic Rust per hour than anything else on this list.
