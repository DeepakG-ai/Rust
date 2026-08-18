# Master Guide to Rust Iterators: From First Principles to Production Patterns

This document covers everything about Rust Iterators—from foundational concepts to under-the-hood mechanics (`.collect()`, `.filter_map()`) and production patterns extracted from real-world codebases like **OpenAI Codex** and **xAI Grok**.

---

## 1. What is an Iterator?

An iterator is an object that yields **one item at a time** from a sequence or data source.

### First Principles: Before Iterators
Without iterators, loops require manual index management:

```rust
let names = vec!["Deepak", "Ravi", "Gowda"];

// Manual index management
let mut i = 0;
while i < names.len() {
    println!("{}", names[i]);
    i += 1;
}
```

**Problems with manual loops:**
1. **Off-by-one errors**: Easy to accidentally write `i <= names.len()` and panic.
2. **Bounds checking overhead**: Every `names[i]` access performs runtime bounds checking.
3. **State clutter**: You manually track index variable `i`.

### With Iterators
```rust
let names = vec!["Deepak", "Ravi", "Gowda"];

for name in names.iter() {
    println!("{}", name);
}
```
* **Safe**: No index tracking, no bounds-check panics.
* **Declarative**: Focuses on *what* to do per item, not *how* to step through memory.

---

## 2. How Iterators Work Internally (`next()`)

In Rust, every iterator implements the standard `std::iter::Iterator` trait. At its core, the trait requires only one method: **`next()`**.

```rust
pub trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;
    
    // ... plus 70+ default adapter methods (.map, .filter, .collect, etc.)
}
```

Calling `next()` steps forward and returns `Some(item)` if an element exists, or `None` when the sequence ends:

```rust
let names = vec!["Deepak", "Ravi"];
let mut iter = names.iter();

assert_eq!(iter.next(), Some(&"Deepak"));
assert_eq!(iter.next(), Some(&"Ravi"));
assert_eq!(iter.next(), None); // Sequence finished!
```

---

## 3. The Three Types of Iteration

How you iterate depends on whether you want to **borrow**, **mutably borrow**, or **consume** (move) the data:

| Method | Item Type | Original Collection State | Common Use Case |
| :--- | :--- | :--- | :--- |
| **`.iter()`** | `&item` (immutable ref) | ✅ Preserved (borrowed) | Reading / searching without modifying |
| **`.iter_mut()`** | `&mut item` (mutable ref) | ✅ Preserved (modified in-place) | Updating elements in-place |
| **`.into_iter()`** | `item` (owned value) | ❌ Consumed (moved) | Transforming or transferring ownership |

```rust
let mut names = vec!["Deepak".to_string(), "Ravi".to_string()];

// 1. Borrowing (.iter())
for name in names.iter() {
    println!("Length of {}: {}", name, name.len());
}

// 2. Mutable Borrowing (.iter_mut())
for name in names.iter_mut() {
    name.push_str(" (Verified)");
}

// 3. Consuming (.into_iter())
for name in names.into_iter() {
    // `names` is moved here and cannot be used after this loop!
    println!("Moved: {}", name);
}
```

### `for` Loop Desugaring
In Rust, a `for` loop is syntax sugar for `.into_iter()`:

```rust
let names = vec!["Deepak", "Ravi"];

// These are identical:
for name in names { }              // Sugar for names.into_iter() (consumes vec)
for name in names.into_iter() { }

// Borrowing syntax sugar:
for name in &names { }             // Sugar for names.iter()
for name in &mut names { }        // Sugar for names.iter_mut()
```

---

## 4. Key Combinator Methods

### Transformation & Filtering

| Method | Purpose | Example |
| :--- | :--- | :--- |
| **`.map(f)`** | Transform every item | `(1..3).map(|x| x * 2)` $\rightarrow$ `[2, 4]` |
| **`.filter(p)`** | Keep items matching predicate | `(1..5).filter(|x| x % 2 == 0)` $\rightarrow$ `[2, 4]` |
| **`.filter_map(f)`** | Combined filter + map (returns `Option`) | `strings.iter().filter_map(|s| s.parse().ok())` |
| **`.flat_map(f)`** | Map item to iterator and flatten | `words.iter().flat_map(|w| w.split(' '))` |
| **`.enumerate()`** | Pair items with index `(index, item)` | `vec.iter().enumerate()` |
| **`.zip(other)`** | Pair elements from two iterators | `a.iter().zip(b.iter())` $\rightarrow$ `[(a0, b0), (a1, b1)]` |
| **`.chain(other)`** | Append two iterators end-to-end | `a.iter().chain(b.iter())` |
| **`.take(n)`** | Take first `n` items | `(1..100).take(3)` $\rightarrow$ `[1, 2, 3]` |
| **`.skip(n)`** | Skip first `n` items | `(1..5).skip(2)` $\rightarrow$ `[3, 4]` |
| **`.peekable()`** | Enable `peek()` without advancing | `let mut iter = vec.iter().peekable();` |
| **`.by_ref()`** | Borrow iterator to preserve ownership | `iter.by_ref().take(3)` |

### Consumption & Reduction

| Method | Purpose | Return Value |
| :--- | :--- | :--- |
| **`.collect()`** | Gather elements into a collection (`Vec`, `String`, etc.) | `Container<T>` or `Result<Container<T>, E>` |
| **`.fold(init, f)`** | Custom reduction with initial accumulator | `AccumulatorType` |
| **`.sum()` / `.product()`** | Sum or multiply all numbers | Numeric type |
| **`.count()`** | Count total items | `usize` |
| **`.find(p)`** | Return first item matching predicate | `Option<Item>` |
| **`.position(p)`** | Index of first matching item | `Option<usize>` |
| **`.any(p)` / `.all(p)`** | Check if any/all items match predicate | `bool` |

---

## 5. `.collect()` — The Universal Consumer

Iterator adapters like `.map()` and `.filter()` are **lazy** — they build a pipeline but do nothing until a **consumer** drives it. `.collect()` is the most common consumer: it runs the iterator and packs every yielded element into a concrete collection (`Vec`, `String`, `HashSet`, `HashMap`, etc.).

### Two ways to specify the target type

Rust needs to know *what* collection you want. Two ways:

```rust
// Method 1: Type annotation on the variable
let numbers: Vec<i32> = (1..=5).collect();

// Method 2: Turbofish on collect
let numbers = (1..=5).collect::<Vec<i32>>();
```

### Example: filter & transform into a Vec

```rust
fn main() {
    let numbers = vec![1, 2, 3, 4, 5, 6];

    let even_multiplied: Vec<i32> = numbers
        .into_iter()
        .filter(|x| x % 2 == 0) // Keeps: 2, 4, 6
        .map(|x| x * 10)         // Transforms to: 20, 40, 60
        .collect();             // Packs them into a Vec<i32>

    println!("{:?}", even_multiplied); // Output: [20, 40, 60]
}
```

### Example: collecting characters into a String

```rust
fn main() {
    let text = "Hello 2026 World!";

    // Filter out digits and collect remaining chars into a String
    let letters_only: String = text
        .chars()
        .filter(|c| !c.is_numeric())
        .collect();

    println!("{}", letters_only); // Output: "Hello  World!"
}
```

### Example: collecting tuples into a HashMap

If the iterator yields `(key, value)` tuples, `.collect()` can build a `HashMap`:

```rust
use std::collections::HashMap;

fn main() {
    let fruit_list = vec![("Apple", 3), ("Banana", 5), ("Orange", 2)];

    let inventory: HashMap<&str, i32> = fruit_list.into_iter().collect();

    println!("{:?}", inventory);
    // Output: {"Apple": 3, "Banana": 5, "Orange": 2}
}
```

### Example: collecting Results (stop on first error)

When every item is a `Result`, collecting into `Result<Vec<T>, E>` short-circuits on the first `Err`:

```rust
fn main() {
    let valid_inputs = vec!["10", "20", "30"];
    let invalid_inputs = vec!["10", "abc", "30"];

    // Case A: All inputs are valid numbers -> Returns Ok(Vec[10, 20, 30])
    let parsed_ok: Result<Vec<i32>, _> = valid_inputs
        .into_iter()
        .map(|s| s.parse::<i32>())
        .collect();
    println!("{:?}", parsed_ok); // Output: Ok([10, 20, 30])

    // Case B: Contains an invalid string -> Short-circuits & returns Err!
    let parsed_err: Result<Vec<i32>, _> = invalid_inputs
        .into_iter()
        .map(|s| s.parse::<i32>())
        .collect();
    println!("{:?}", parsed_err); // Output: Err(ParseIntError { ... })
}
```

### Cheat sheet

| Iterator item | Target type | Syntax |
|---|---|---|
| `T` | `Vec<T>` | `iter.collect::<Vec<_>>()` |
| `char` or `&str` | `String` | `iter.collect::<String>()` |
| `(K, V)` | `HashMap<K, V>` | `iter.collect::<HashMap<_, _>>()` |
| `T` | `HashSet<T>` | `iter.collect::<HashSet<_>>()` |
| `Result<T, E>` | `Result<Vec<T>, E>` | `iter.collect::<Result<Vec<_>, _>>()` |

### Under the hood

`.collect()` is powered by **trait delegation**, **`size_hint()` pre-allocation**, and **compiler specialization**.

**Trait delegation** — `.collect()` delegates to `FromIterator::from_iter`:

```rust
fn collect<B: FromIterator<Self::Item>>(self) -> B where Self: Sized {
    FromIterator::from_iter(self)
}
```

**Memory pre-allocation** — When collecting into `Vec<T>`, Rust queries `size_hint()`. If the iterator knows its length (e.g. from a slice), `Vec::with_capacity(n)` pre-allocates in a single allocation, eliminating repeated resizing.

**Short-circuiting** — When collecting `Result<T, E>` items, the moment `next()` yields `Err(e)`, iteration stops immediately, the partial collection is dropped via RAII, and `Err(e)` is returned.

---

## 6. `.filter_map()` — Filter + Map in One Pass

`.filter_map()` combines `.filter()` and `.map()` in a **single pass**. The closure returns `Option<T>`:

- `Some(value)` → keep the item, output `value`
- `None` → discard the item

### Why not just `.filter().map()`?

When filtering depends on the same computation as mapping, separate steps force you to compute twice (or use an ugly `.unwrap()`):

```rust
// Cluttered: parses twice
let strings = vec!["10", "abc", "20", "xyz"];

let numbers: Vec<i32> = strings
    .into_iter()
    .filter(|s| s.parse::<i32>().is_ok()) // Check 1
    .map(|s| s.parse::<i32>().unwrap())   // Check 2 (duplicate work!)
    .collect();
```

With `.filter_map()` — one parse, no unwrap:

```rust
let strings = vec!["10", "abc", "20", "xyz"];

let numbers: Vec<i32> = strings
    .into_iter()
    .filter_map(|s| s.parse::<i32>().ok()) // Parses ONCE!
    .collect();

println!("{:?}", numbers); // Output: [10, 20]
```

### Example: discarding errors with `Result::ok`

Pass `Result::ok` directly — it converts `Ok(x)` → `Some(x)` and `Err(_)` → `None`:

```rust
fn main() {
    let results: Vec<Result<i32, &str>> = vec![
        Ok(100),
        Err("Disk Full"),
        Ok(200),
        Err("Network Timeout"),
        Ok(300),
    ];

    // Keep only the Ok values!
    let successes: Vec<i32> = results
        .into_iter()
        .filter_map(Result::ok) // Converts Ok(x) -> Some(x), Err(_) -> None
        .collect();

    println!("{:?}", successes); // Output: [100, 200, 300]
}
```

### Example: extracting specific enum variants

```rust
enum UserEvent {
    Login(String),
    Logout,
    Message(String),
}

fn main() {
    let events = vec![
        UserEvent::Login("Alice".to_string()),
        UserEvent::Logout,
        UserEvent::Message("Hello!".to_string()),
        UserEvent::Message("How are you?".to_string()),
    ];

    // Extract ONLY the chat message strings
    let chat_messages: Vec<String> = events
        .into_iter()
        .filter_map(|event| match event {
            UserEvent::Message(msg) => Some(msg), // Keep message payload
            _ => None,                             // Discard Login & Logout
        })
        .collect();

    println!("{:?}", chat_messages);
    // Output: ["Hello!", "How are you?"]
}
```

### Under the hood

```rust
pub struct FilterMap<I, F> {
    iter: I,
    f: F,
}

impl<B, I: Iterator, F> Iterator for FilterMap<I, F>
where
    F: FnMut(I::Item) -> Option<B>,
{
    type Item = B;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(x) = self.iter.next() {
            if let Some(y) = (self.f)(x) {
                return Some(y); // Yields transformed item immediately
            }
        }
        None // Iterator exhausted
    }
}
```

### Comparison

| Method | Closure returns | Behavior |
|---|---|---|
| `.filter()` | `bool` | Keeps item if `true`, discards if `false`. Item type unchanged. |
| `.map()` | `T` | Transforms every item. Count unchanged. |
| `.filter_map()` | `Option<T>` | `None` → discard, `Some(val)` → keep & transform in one step. |

---

## 7. Advanced Combinators: `.peekable()` & `.by_ref()`

### `.peekable()` for Parsers & Lexers
Allows inspecting the next item without consuming it:

```rust
let mut chars = "a=10;".chars().peekable();

while let Some(&c) = chars.peek() {
    if c.is_alphabetic() {
        println!("Found letter: {}", chars.next().unwrap());
    } else {
        chars.next();
    }
}
```

### `.by_ref()` for Partial Stream Iteration
`by_ref()` borrows an iterator temporarily so you can use methods like `.take()` without losing ownership of the main iterator:

```rust
let mut numbers = vec![1, 2, 3, 4, 5, 6].into_iter();

// Take first 2 items
let first_two: Vec<_> = numbers.by_ref().take(2).collect(); // [1, 2]

// Remaining items are STILL accessible in `numbers`!
let rest: Vec<_> = numbers.collect(); // [3, 4, 5, 6]
```

---

## 8. API Design with `impl Iterator<Item = T>`

In production Rust codebases (like OpenAI Codex & xAI Grok), functions return `impl Iterator<Item = T>` rather than allocating vectors:

```rust
// Zero-allocation public API!
pub fn sanitize_tokens(raw: &str) -> impl Iterator<Item = &str> {
    raw.split(',')
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
}
```

### Benefits:
1. **Zero Heap Allocations**: No intermediate `Vec` is created.
2. **Encapsulated Types**: Hides ugly concrete iterator type signatures (`Filter<Map<Split<...>>>`).
3. **Lazy Execution**: Callers process items on-demand.

---

## 9. Production Custom Iterator Patterns (From Codex & Grok)

Only implement a custom iterator `struct` when standard combinators (`.map`, `.filter`) cannot encapsulate complex state.

### Pattern A: Custom Iterator Adapter (AST Event Merging)
* **Real Project Example**: `DecodedTextMerge` in [OpenAI Codex](file:///C:/Users/aigroup5/PycharmProjects/codex/codex-rs/tui/src/markdown_text_merge.rs#L25)
* **Goal**: Merge adjacent text events from a Markdown parser while updating source byte ranges.

```rust
use std::iter::Peekable;
use std::ops::Range;
use pulldown_cmark::Event;

pub(crate) struct DecodedTextMerge<I: Iterator> {
    iter: Peekable<I>,
}

impl<'a, I> Iterator for DecodedTextMerge<I>
where
    I: Iterator<Item = (Event<'a>, Range<usize>)>,
{
    type Item = (Event<'a>, Range<usize>);

    fn next(&mut self) -> Option<Self::Item> {
        let (event, mut range) = self.iter.next()?;
        let Event::Text(text) = event else {
            return Some((event, range));
        };

        // If next event is also text, merge them!
        let mut merged = text.into_string();
        while matches!(self.iter.peek(), Some((Event::Text(_), _))) {
            let Some((Event::Text(next_text), next_range)) = self.iter.next() else { break; };
            merged.push_str(&next_text);
            range.end = next_range.end;
        }
        Some((Event::Text(merged.into()), range))
    }
}
```

### Pattern B: Graph / Tree Traversal Iterator
* **Real Project Example**: `ScopeStack` in [xAI Grok](file:///C:/Users/aigroup5/PycharmProjects/grok-build/crates/codegen/xai-codebase-graph/src/scope_graph/graph.rs#L456)
* **Goal**: Traverse scope graph nodes up to the root.

```rust
pub struct ScopeStack<'a> {
    graph: &'a ScopeGraph,
    current: Option<NodeIndex>,
}

impl<'a> Iterator for ScopeStack<'a> {
    type Item = NodeIndex;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(curr) = self.current {
            let parent = self.graph.find_parent_scope(curr);
            self.current = parent;
            Some(curr)
        } else {
            None
        }
    }
}
```

### Pattern C: Fallible Streaming Disk Iterator (`type Item = io::Result<T>`)
* **Real Project Example**: `UpdatesIterator` in [xAI Grok](file:///C:/Users/aigroup5/PycharmProjects/grok-build/crates/codegen/xai-grok-shell/src/session/storage/mod.rs#L517)
* **Goal**: Read lines from disk on demand, yielding `io::Result<T>` so errors don't panic the process.

```rust
impl Iterator for UpdatesIterator {
    type Item = std::io::Result<SessionUpdate>;

    fn next(&mut self) -> Option<Self::Item> {
        self.buffer.clear();
        match self.reader.read_line(&mut self.buffer) {
            Ok(0) => None, // EOF
            Ok(_) => match SessionUpdate::parse(&self.buffer) {
                Ok(update) => Some(Ok(update)),
                Err(e) => Some(Err(std::io::Error::new(std::io::ErrorKind::InvalidData, e))),
            },
            Err(e) => Some(Err(e)),
        }
    }
}
```

---

## 10. Async Streams & Parallel Iterators

### Parallel Iteration with Rayon
For CPU-bound parallel workloads (e.g. indexing codebases across CPU cores):

```rust
use rayon::prelude::*;

let files: Vec<PathBuf> = get_all_files();
files.par_iter().for_each(|file| {
    parse_and_index(file);
});
```

### Async Streams (`StreamExt`)
For IO-bound asynchronous workloads (e.g. LLM token streaming, WebSockets, ACP tool calls):

```rust
use futures::StreamExt;

async fn process_llm_stream(mut stream: impl StreamExt<Item = Token>) {
    while let Some(token) = stream.next().await {
        print!("{}", token);
    }
}
```

---

## 11. Performance Guarantee: Zero-Cost Abstraction

Rust compiles iterator chains into **the exact same machine code as hand-written C/C++ loops**.

```rust
// High-level iterator chain
let sum: i32 = numbers.iter().filter(|x| **x > 0).map(|x| x * 2).sum();
```

Compiles to the exact same assembly as:

```rust
let mut sum = 0;
for i in 0..numbers.len() {
    let val = numbers[i];
    if val > 0 {
        sum += val * 2;
    }
}
```
**No overhead, no extra allocations, maximum performance.**
