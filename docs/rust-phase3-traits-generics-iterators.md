# Rust Phase 3 — Traits, Generics, Iterators & Collections (Deep Dive)

> Where the effort starts paying you back. Phase 2 taught you the rules; Phase 3 teaches you the abstractions that make following those rules pleasant.
> Written as a continuation of Phase 1 (Data Types) and Phase 2 (Ownership).

---

## 0. The organizing idea: zero-cost abstraction

Bjarne Stroustrup's formulation, which Rust adopts wholesale:

> **What you don't use, you don't pay for. And what you do use, you couldn't hand-code any better.**

In Python, abstraction costs you. A `for` loop over a generator, a call through an ABC, a `map()` — each one goes through dynamic dispatch, allocates objects, and touches the interpreter. The abstraction is convenient and it is *slow*, so performance-critical Python means dropping into NumPy or C.

In Rust, the abstractions in this phase compile away entirely. A ten-stage iterator chain becomes the same machine code as a hand-written loop. A generic function becomes a specialized copy per type, with no runtime type checks. A trait method call becomes a direct call. **You write high-level code and get assembly-level performance**, which is exactly why `ruff`, `polars`, `uv`, and `tokenizers` — tools you already use daily — are written in Rust.

Two mechanisms make this work, and understanding the split is most of Phase 3:

- **Generics + traits → monomorphization → static dispatch.** Resolved at compile time. Free.
- **Trait objects (`dyn Trait`) → vtables → dynamic dispatch.** Resolved at runtime. Cheap, but not free.

You choose. Python chose for you, and it always chose the second one.

---

## 1. Generics

### 1.1 Generic functions

```rust
fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}
```

`<T: PartialOrd>` declares a type parameter `T` **bounded** by the `PartialOrd` trait. The bound is not optional decoration — without it, `item > largest` won't compile, because the compiler has no reason to believe an arbitrary `T` is comparable.

This is the fundamental contrast with Python's duck typing. Python's equivalent function works on anything until it doesn't, and you discover the failure at runtime, in production, on the one input type you didn't test. Rust's version is verified at the definition site: if it compiles, it works for **every** type satisfying the bound. Generics in Rust are checked once, not per instantiation like C++ templates.

### 1.2 Generic structs, enums, and impls

```rust
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {                    // methods for ALL T
    fn x(&self) -> &T { &self.x }
}

impl Point<f64> {                     // methods for ONLY Point<f64>
    fn distance_from_origin(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}
```

Note the `impl<T>` — you declare the parameter after `impl` so the compiler knows `T` is generic rather than a concrete type named `T`. That second block is a **specialized impl**: `distance_from_origin` exists only on `Point<f64>`. Calling it on a `Point<i32>` is a compile error, not a runtime one.

You've already been using generic enums extensively: `Option<T>` and `Result<T, E>` are exactly this.

### 1.3 Monomorphization — where the "zero cost" comes from

At compile time, Rust generates a **separate concrete copy** of each generic item for every type it's used with:

```rust
let a = largest(&[1, 2, 3]);           // compiler generates largest_i32
let b = largest(&["a", "b"]);          // and largest_str
```

There's no type parameter left at runtime, no boxing, no lookup — just two ordinary functions that can be inlined and optimized independently. This is why generics cost nothing.

**The tradeoff is binary size and compile time.** Every instantiation is real code. This is a genuine reason Rust compiles slower than Go, and why heavily generic crates (like `serde`) inflate build times. It's usually the right trade, but it's a trade.

### 1.4 `where` clauses

When bounds get long, move them out of the signature:

```rust
fn process<T, U>(t: &T, u: &U) -> String
where
    T: Display + Clone,
    U: Clone + Debug,
{
    format!("{t} {u:?}")
}
```

Same meaning, readable signature. Use `where` as soon as you have more than one or two bounds.

---

## 2. Traits — interfaces the compiler enforces

A trait defines shared behavior. It's the closest thing to a Python ABC or `typing.Protocol`, but checked at compile time and with no runtime cost.

```rust
trait Summary {
    fn summarize_author(&self) -> String;          // required

    fn summarize(&self) -> String {                // default implementation
        format!("(Read more from {}...)", self.summarize_author())
    }
}

struct Tweet { username: String, content: String }

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
    // summarize() comes free from the default
}
```

**Default methods** are a big deal — a trait can be mostly-implemented, requiring implementors to supply only the primitive operations. `Iterator` takes this to its logical extreme: you implement one method and receive seventy.

### 2.1 Traits are decoupled from types

Unlike Python or Java, where a class declares its interfaces at definition, **Rust lets you implement a trait for a type in a separate place entirely** — including implementing *your* trait for *standard library* types:

```rust
trait Doubled { fn doubled(&self) -> Self; }

impl Doubled for i32 {
    fn doubled(&self) -> i32 { self * 2 }
}

println!("{}", 21.doubled());   // extending a primitive. legally.
```

This is enormously powerful and it's why Rust libraries compose so well.

### 2.2 The orphan rule (coherence)

The obvious hazard: if two crates both implement `Display` for `Vec<T>`, which wins? Rust's answer is to forbid the situation.

> **You may implement a trait for a type only if the trait or the type (or both) is local to your crate.**

So you can implement your own trait for `Vec<i32>` ✅, or implement `Display` for your own struct ✅, but **not** `Display` for `Vec<i32>` ❌ — both are foreign.

The standard workaround is the **newtype pattern**: wrap the foreign type in a local one-field tuple struct.

```rust
struct Wrapper(Vec<String>);

impl std::fmt::Display for Wrapper {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}
```

Newtypes cost nothing at runtime (the wrapper compiles away) and are also the idiomatic way to add type safety — `struct Meters(f64)` and `struct Feet(f64)` become genuinely different types the compiler won't let you mix up. For AI work, `struct TokenId(u32)` vs `struct Position(u32)` prevents a whole class of indexing bug.

### 2.3 Derivable traits — free implementations

`#[derive(...)]` asks the compiler to generate a mechanical implementation:

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
struct Config { dim: u32, layers: u32 }
```

| Trait | Gives you | Notes |
|---|---|---|
| `Debug` | `{:?}` formatting | derive this on nearly everything |
| `Clone` | explicit deep copy | required for `Copy` |
| `Copy` | implicit bitwise copy | only if all fields are `Copy` |
| `PartialEq` / `Eq` | `==` | `Eq` adds reflexivity; floats can't be `Eq` |
| `PartialOrd` / `Ord` | `<`, sorting | `Ord` needed for `BTreeMap` keys |
| `Hash` | use as a `HashMap` key | needs `Eq` too |
| `Default` | `Type::default()` | zeros/empties per field |

Deriving `Debug` on essentially every type you write is standard practice — `dbg!(&x)` and `{:?}` are your `print()` replacements.

### 2.4 The standard traits worth knowing now

| Trait | Purpose | Python analog |
|---|---|---|
| `Display` | user-facing `{}` output | `__str__` |
| `Debug` | developer `{:?}` output | `__repr__` |
| `From<T>` / `Into<T>` | infallible conversion | constructors / `__init__` |
| `TryFrom<T>` / `TryInto<T>` | fallible conversion → `Result` | — |
| `Iterator` | produce a sequence | `__next__` |
| `IntoIterator` | can be iterated | `__iter__` |
| `Default` | a sensible zero value | default arguments |
| `Deref` | smart-pointer transparency | — |
| `Drop` | cleanup on scope exit | `__del__` / `__exit__` |
| `AsRef<T>` | cheap reference conversion | — |
| `PartialEq` / `Ord` | comparison, sorting | `__eq__` / `__lt__` |

**Implement `From`, get `Into` free.** This is a *blanket impl* in the standard library — `impl<T, U: From<T>> Into<U> for T`. Write one direction, get both:

```rust
impl From<Config> for String {
    fn from(c: Config) -> String { format!("{}x{}", c.dim, c.layers) }
}

let s: String = config.into();      // works automatically
let s = String::from(config);       // so does this
```

`From` is also what powers the `?` operator's automatic error conversion from Phase 2 — the pieces are connecting.

### 2.5 Supertraits

A trait can require another:

```rust
trait Loggable: std::fmt::Display {          // Loggable requires Display
    fn log(&self) { println!("[LOG] {self}"); }
}
```

Now anything implementing `Loggable` must also implement `Display`, and `Loggable`'s default methods can rely on it.

---

## 3. Static vs dynamic dispatch

This is the most important design decision in the phase.

### 3.1 Static dispatch — `impl Trait` and generics

```rust
fn notify(item: &impl Summary) { ... }             // syntax sugar for:
fn notify<T: Summary>(item: &T) { ... }            // the same thing
```

Monomorphized, inlined, zero cost. **This is the default choice.**

`impl Trait` also works in return position, where it means "some concrete type implementing this trait, and I'm not telling you which":

```rust
fn make_adder(x: i32) -> impl Fn(i32) -> i32 {
    move |y| x + y
}

fn evens(v: &[i32]) -> impl Iterator<Item = &i32> {
    v.iter().filter(|n| *n % 2 == 0)
}
```

That second one matters: iterator adapter types are absurd (`Filter<Iter<'_, i32>, {closure}>`), and `impl Trait` lets you return them without naming them. The limitation is that all return paths must produce the **same** concrete type — you can't return one iterator type from an `if` and a different one from the `else`.

### 3.2 Dynamic dispatch — `dyn Trait`

When you genuinely need a heterogeneous collection — different concrete types behind one interface — you need a **trait object**:

```rust
let shapes: Vec<Box<dyn Shape>> = vec![
    Box::new(Circle { r: 1.0 }),
    Box::new(Square { side: 2.0 }),
];

for s in &shapes {
    println!("{}", s.area());     // resolved at runtime via vtable
}
```

A `Vec<Box<dyn Shape>>` can't be a `Vec<T>` because the elements have different sizes and layouts. `Box<dyn Shape>` is a **fat pointer**: one pointer to the data, one to a **vtable** of function pointers for that concrete type. Method calls go through the vtable.

This is precisely how *every* Python method call works. In Rust it's opt-in.

### 3.3 Object safety (dyn compatibility)

Not every trait can become a trait object. A vtable needs a fixed layout, so the trait must not have:

- methods that are **generic** over type parameters (each would need its own vtable entry — unbounded),
- methods **returning `Self`** (the caller doesn't know the size),
- methods without a `self` receiver (associated functions),
- a `Sized` supertrait.

If the compiler says a trait "cannot be made into an object," this is why. Common fix: split the object-safe methods into their own trait.

### 3.4 Choosing

| | Static (`impl Trait` / generics) | Dynamic (`dyn Trait`) |
|---|---|---|
| Resolved | compile time | runtime |
| Cost | zero; inlinable | pointer indirection, no inlining |
| Binary size | grows per instantiation | one copy |
| Compile time | slower | faster |
| Heterogeneous collections | ❌ | ✅ |
| Plugin / trait-registry patterns | ❌ | ✅ |

> **Default to static dispatch. Reach for `dyn` when you need runtime heterogeneity** — a `Vec` of different implementations, a plugin system, or to cut monomorphization bloat. The dispatch cost is real but modest; the lost *inlining* is usually the bigger effect.

### 3.5 Associated types vs generic parameters

```rust
trait Container {
    type Item;                          // associated type
    fn get(&self, i: usize) -> Option<&Self::Item>;
}

trait Container2<T> {                   // generic parameter
    fn get(&self, i: usize) -> Option<&T>;
}
```

The difference is **how many times a type may implement the trait**:

- **Associated type**: once per implementing type. `Iterator` uses `type Item` because a `Vec<i32>`'s iterator yields exactly one thing — `i32`. Callers never have to specify it.
- **Generic parameter**: many times. `From<T>` is generic because a type can convert *from* many sources: `impl From<i32> for MyType`, `impl From<&str> for MyType`, etc.

Rule: if there's one natural answer per type, use an associated type; if a type should support many, use a generic parameter.

---

## 4. Closures

Closures are anonymous functions that capture their environment — like Python lambdas, but unrestricted (multi-line, statements allowed) and integrated with ownership.

```rust
let factor = 3;
let scale = |x: i32| x * factor;      // captures `factor` by reference
println!("{}", scale(10));            // 30
```

Types are usually inferred; each closure has its own unique anonymous type.

### 4.1 The three closure traits

How a closure captures determines which traits it implements:

| Trait | Captures by | Callable | Python-ish analog |
|---|---|---|---|
| `Fn` | `&T` (shared) | many times | a pure lambda |
| `FnMut` | `&mut T` (exclusive) | many times, needs `mut` | a lambda mutating a captured var |
| `FnOnce` | `T` (by value, consumes) | **once** | a lambda that consumes what it closed over |

These nest: every `Fn` is also `FnMut` and `FnOnce`; every `FnMut` is also `FnOnce`. The compiler infers the **least restrictive** one that works, so you rarely think about it until you're writing a function that *takes* a closure:

```rust
fn apply<F: Fn(i32) -> i32>(f: F, x: i32) -> i32 { f(x) }
fn apply_mut<F: FnMut()>(mut f: F) { f(); f(); }
fn consume<F: FnOnce() -> String>(f: F) -> String { f() }
```

Take `FnOnce` if you'll call it once, `FnMut` if repeatedly with mutation, `Fn` if repeatedly without. Being *permissive* in what you accept means `FnOnce` is the most flexible bound.

### 4.2 `move` closures

`move` forces capture by value:

```rust
let data = vec![1, 2, 3];
let closure = move || println!("{data:?}");   // takes ownership of data
// data is no longer usable here
```

Essential when a closure outlives the scope that created it — returning it, or (Phase 4) sending it to another thread.

---

## 5. Iterators — the crown jewel

### 5.1 The trait

```rust
trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
    // ...and ~75 default methods built on next()
}
```

**You implement one method and get everything else free.** This is the payoff of default methods in §2.

```rust
struct Fibonacci { a: u64, b: u64 }

impl Iterator for Fibonacci {
    type Item = u64;
    fn next(&mut self) -> Option<u64> {
        let out = self.a;
        self.a = self.b;
        self.b = out + self.b;
        Some(out)
    }
}

// instantly usable with the whole ecosystem:
let sum: u64 = Fibonacci { a: 0, b: 1 }.take(10).filter(|n| n % 2 == 0).sum();
```

### 5.2 Laziness

**Iterator adapters do nothing until consumed.** `map`, `filter`, `take` just build a new struct describing the work.

```rust
let v = vec![1, 2, 3];
v.iter().map(|x| { println!("side effect"); x * 2 });   // ⚠️ prints NOTHING
```

`Iterator` is `#[must_use]`, so the compiler warns you here. To actually run it you need a **consumer**: `collect()`, `sum()`, `for_each()`, `count()`, a `for` loop, etc.

The laziness has real consequences: a chain of ten adapters makes **one pass** over the data with no intermediate allocations, and LLVM typically fuses the whole thing into a single loop. Compare Python, where `[f(x) for x in xs if g(x)]` materializes lists and each step is interpreted.

### 5.3 The three ways to iterate — get this right

| Method | Yields | Effect on the collection |
|---|---|---|
| `.iter()` | `&T` | borrows immutably; collection still usable |
| `.iter_mut()` | `&mut T` | borrows mutably; lets you modify in place |
| `.into_iter()` | `T` | **consumes** the collection; you get owned values |

```rust
let mut v = vec![1, 2, 3];

for x in v.iter()     { println!("{x}"); }      // &i32
for x in v.iter_mut() { *x *= 2; }              // &mut i32 — modifies v
for x in v.into_iter(){ println!("{x}"); }      // i32 — v is GONE after this
```

And the `for`-loop sugar, which trips everyone up once:

```rust
for x in &v      { }   // == v.iter()
for x in &mut v  { }   // == v.iter_mut()
for x in v       { }   // == v.into_iter() — MOVES v!
```

If you've ever written `for x in v` and then gotten "borrow of moved value" on the next line, that's why. This is Phase 2 showing up in Phase 3 syntax.

### 5.4 The adapters you'll use constantly

```rust
.map(|x| ...)              // transform each element
.filter(|x| ...)           // keep matching elements
.filter_map(|x| ...)       // transform + filter in one (returns Option)
.enumerate()               // yields (index, item) — like Python's enumerate
.zip(other)                // pair up two iterators
.take(n) / .skip(n)        // slicing
.take_while(|x| ...)       // stop at first failure
.chain(other)              // concatenate
.rev()                     // reverse (needs DoubleEndedIterator)
.flatten()                 // flatten nested iterables
.flat_map(|x| ...)         // map then flatten
.peekable()                // look ahead without consuming
.windows(n) / .chunks(n)   // (on slices) sliding windows / fixed blocks
```

Consumers:

```rust
.collect()                 // build a collection
.sum() / .product()        // reduce numerically
.fold(init, |acc, x| ...)  // general reduce (Python's functools.reduce)
.count() / .last()
.min() / .max() / .max_by_key(|x| ...)
.any(|x| ...) / .all(|x| ...)
.find(|x| ...) / .position(|x| ...)
.for_each(|x| ...)
```

`windows` and `chunks` are worth flagging for your work — n-gram extraction and batching, respectively, in one call.

### 5.5 `collect()` and the turbofish

`collect()` builds any type implementing `FromIterator`, so it needs to be told which:

```rust
let v: Vec<i32>            = (1..=5).collect();
let s: String              = vec!['a', 'b'].into_iter().collect();
let set: HashSet<i32>      = v.iter().copied().collect();
let map: HashMap<&str, i32>= vec![("a", 1), ("b", 2)].into_iter().collect();

let v = (1..=5).collect::<Vec<i32>>();     // turbofish alternative
```

**The best trick in the standard library:** an iterator of `Result`s collects into a `Result` of a collection, short-circuiting on the first error.

```rust
let nums: Result<Vec<i32>, _> = vec!["1", "2", "x"]
    .iter()
    .map(|s| s.parse::<i32>())
    .collect();
// Err(ParseIntError) — stops at "x"
```

The same works for `Option`. Doing this in Python takes an explicit loop with a try/except.

---

## 6. Collections

### 6.1 `Vec<T>` — your `list`

Contiguous, heap-allocated, growable.

```rust
let mut v: Vec<i32> = Vec::new();
let mut v = vec![1, 2, 3];
let v = Vec::with_capacity(1000);      // pre-allocate — do this when you know the size

v.push(4);
v.pop();                                // Option<T>
let x = v[0];                           // panics if out of bounds
let x = v.get(0);                       // Option<&T> — the safe way
v.sort();
v.sort_by_key(|x| x.abs());
v.retain(|x| *x > 0);                   // in-place filter
v.extend(other);
```

Growth is amortized O(1) via capacity doubling — same as Python's list. `with_capacity` avoids the reallocation churn; in a hot loop over known-size data it's free performance.

Remember the borrow rule bites here: you can't hold `&v[0]` and `push` afterwards, because a push may reallocate and invalidate the reference. Python has the same hazard and no protection against it.

### 6.2 `HashMap<K, V>` — your `dict`

```rust
use std::collections::HashMap;

let mut scores = HashMap::new();
scores.insert(String::from("blue"), 10);

let s = scores.get("blue");                 // Option<&i32>
let s = scores.get("blue").copied().unwrap_or(0);

for (k, v) in &scores { println!("{k}: {v}"); }   // iteration order is RANDOM
```

Three things Python developers should know:

1. **Iteration order is unspecified and randomized.** Unlike modern Python dicts, `HashMap` gives no insertion-order guarantee. If you need ordering, use `BTreeMap` (sorted by key) or `indexmap` (insertion order).
2. **Keys must be `Eq + Hash`.** So `f64` cannot be a key (`NaN != NaN` breaks the contract). This is the Phase 1 float-ordering note coming due.
3. **The default hasher is SipHash 1-3** — cryptographically strong and HashDoS-resistant, but not the fastest. For internal, non-adversarial workloads (token counts, vocabulary maps), swapping in `rustc-hash` (`FxHashMap`) or `ahash` is often a 2–3× speedup on map-heavy code. A very cheap win in AI preprocessing pipelines.

**The entry API** is the idiomatic replacement for `dict.setdefault` / `defaultdict`:

```rust
// word frequency count — the canonical example
let mut counts: HashMap<&str, i32> = HashMap::new();
for word in text.split_whitespace() {
    *counts.entry(word).or_insert(0) += 1;
}

map.entry(k).or_insert_with(Vec::new).push(item);   // defaultdict(list)
map.entry(k).and_modify(|v| *v += 1).or_insert(1);
```

`entry` performs a single lookup for the check-and-insert, where the naive `if !map.contains_key(k) { map.insert(...) }` does two.

### 6.3 The rest

| Collection | Use when | Python analog |
|---|---|---|
| `Vec<T>` | default sequence | `list` |
| `VecDeque<T>` | push/pop at both ends | `collections.deque` |
| `HashMap<K,V>` | default key-value | `dict` |
| `HashSet<T>` | membership, dedup | `set` |
| `BTreeMap<K,V>` | need sorted keys / range queries | `sortedcontainers` |
| `BTreeSet<T>` | sorted set | — |
| `BinaryHeap<T>` | priority queue | `heapq` |
| `String` | owned UTF-8 text | `str` |

`BTreeMap` is O(log n) rather than O(1) but keeps keys sorted and supports range queries (`map.range(10..20)`) — frequently worth it.

---

## 7. Putting it together

```rust
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
struct Document { id: u32, text: String }

trait Tokenize {
    fn tokens(&self) -> Vec<&str>;

    fn token_count(&self) -> usize {          // default method
        self.tokens().len()
    }
}

impl Tokenize for Document {
    fn tokens(&self) -> Vec<&str> {
        self.text.split_whitespace().collect()
    }
}

/// Generic over anything tokenizable; static dispatch, zero cost.
fn vocabulary<'a, T: Tokenize>(docs: &'a [T]) -> HashMap<&'a str, usize> {
    let mut vocab = HashMap::new();
    for token in docs.iter().flat_map(|d| d.tokens()) {
        *vocab.entry(token).or_insert(0) += 1;
    }
    vocab
}

fn main() {
    let docs = vec![
        Document { id: 1, text: "the cat sat".into() },
        Document { id: 2, text: "the dog sat".into() },
    ];

    let vocab = vocabulary(&docs);

    // sort by frequency, descending — iterator chain, single pass
    let mut ranked: Vec<_> = vocab.into_iter().collect();
    ranked.sort_by_key(|&(_, count)| std::cmp::Reverse(count));

    let total: usize = docs.iter().map(|d| d.token_count()).sum();
    println!("{total} tokens, top: {:?}", ranked.first());
}
```

Every Phase 3 concept appears: a trait with a default method, a generic function with a bound, lifetime annotation tying output to input, iterator chaining with `flat_map`, the entry API, `collect` with inference, and a derive.

---

## 8. Exercises

1. **Generic + bound.** Write `fn largest<T: PartialOrd>(list: &[T]) -> &T`. Then remove the bound and read the error to see exactly what the bound buys you.
2. **Trait with default.** Define a `Shape` trait with required `area()` and a default `describe()` that uses it. Implement for `Circle` and `Rectangle`.
3. **Static vs dynamic.** Write `fn print_area(s: &impl Shape)` and `fn print_all(shapes: &[Box<dyn Shape>])`. Explain why the second needs `Box<dyn>` and can't be `&[impl Shape]`.
4. **Object safety.** Add a method returning `Self` to `Shape` and watch `Box<dyn Shape>` stop compiling. Read the error, then fix it by splitting the trait.
5. **Orphan rule + newtype.** Try to `impl Display for Vec<String>`. Read the error. Fix it with a newtype wrapper.
6. **`From` and `?`.** Implement `From<io::Error>` for a custom error enum, then write a function using `?` on an io operation. Confirm the conversion happens implicitly — this ties Phase 2 to Phase 3.
7. **Implement `Iterator`.** Write the `Fibonacci` iterator from §5.1. Then get the sum of the first 20 even Fibonacci numbers in one chain.
8. **The three iterations.** Take a `Vec<String>`. Iterate with `iter()`, `iter_mut()` (uppercase each in place), and `into_iter()`. After the third, try to use the vec and read the error.
9. **Laziness.** Write a chain with a `println!` inside `map` and don't consume it. Observe nothing prints and note the `must_use` warning. Add `.collect::<Vec<_>>()` and watch it run.
10. **Collect into `Result`.** Parse `vec!["1","2","3"]` into `Result<Vec<i32>, _>`, then change one to `"x"` and observe the short-circuit.
11. **Word frequency.** Count word frequencies with the entry API, then return the top 5 by count. This is the canonical Rust exercise and it exercises half this document.
12. **Closure traits.** Write three functions taking `Fn`, `FnMut`, and `FnOnce` respectively. Try passing a closure that consumes a captured `String` to each and see which compile.
13. **Refactor from Phase 1.** Take any Python script of yours that does data preprocessing and port it. Notice how much of the logic becomes one iterator chain.

---

## 9. Quick reference card

| Concept | Python | Rust |
|---|---|---|
| Interface | ABC / `Protocol`, runtime | `trait`, compile time, zero cost |
| Adding methods to a builtin | monkey-patching | `impl MyTrait for i32` (safe, scoped) |
| Generic function | duck typing, fails at runtime | `<T: Bound>`, verified at definition |
| Generic cost | dynamic everything | monomorphized, free |
| Heterogeneous list | native (`list` of anything) | `Vec<Box<dyn Trait>>`, opt-in |
| Dispatch | always dynamic | static by default, `dyn` on request |
| Lambda | `lambda` (expression only) | closure (full body), `Fn`/`FnMut`/`FnOnce` |
| Iteration protocol | `__iter__` / `__next__` | `IntoIterator` / `Iterator` |
| Lazy sequences | generators | **all** iterators are lazy |
| Comprehension | `[f(x) for x in xs if g(x)]` | `xs.iter().filter(g).map(f).collect()` |
| Iterator cost | interpreted, allocates | fused into one loop, no allocation |
| `dict` | insertion-ordered | `HashMap` unordered; `BTreeMap` sorted |
| `defaultdict` | `defaultdict(int)` | `.entry(k).or_insert(0)` |
| `reduce` | `functools.reduce` | `.fold(init, f)` |
| Conversion | `int()`, `str()` | `From` / `Into`, `TryFrom` / `TryInto` |
| `__str__` / `__repr__` | dunder methods | `Display` / `Debug` |

### The one-paragraph summary

**Traits** define behavior; **generics** abstract over types; bounds (`T: Trait`) connect them, letting the compiler verify a generic function once for all valid types. **Monomorphization** compiles each instantiation to concrete code, so this abstraction costs nothing — reach for `dyn Trait` and its vtable only when you need runtime heterogeneity. The **orphan rule** keeps impls coherent; the **newtype pattern** works around it. **Iterators** are the flagship application: implement `next` and inherit seventy adapters, all lazy, all fusing into a single allocation-free loop. Choose `iter`/`iter_mut`/`into_iter` according to whether you want to borrow, mutate, or consume — because ownership never stops mattering.

---

### What's next (Phase 4 preview)

You're now productive in Rust. Phase 4 — **concurrency** — is where the ownership rules pay their final dividend: `Send` and `Sync` are just traits (you now know what those are), threads take closures (you now know `move` and `FnOnce`), and `Arc<Mutex<T>>` is shared ownership made explicit. Data races become compile errors for free, because "shared XOR mutable" already banned them.

A concrete preview of how good this gets: **`rayon`** lets you change `.iter()` to `.par_iter()` and get a work-stealing parallel version of the exact iterator chains you just learned — with the compiler guaranteeing correctness. That one-word change is the closest thing to free lunch in systems programming, and it's only possible because of everything in Phases 2 and 3.
