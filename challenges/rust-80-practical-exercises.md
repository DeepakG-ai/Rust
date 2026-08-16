# 80 Practical Rust Exercises — Beginner to Solid Intermediate

> Hands-on problems, ordered so each block builds on the last.
> Deliberately **not** specialist territory: no macros, no `unsafe`, no `PhantomData`, no const generics, no `Pin`. Everything here is what working Rust developers actually write.

---

## How to use this

**Difficulty:** ⭐ warm-up · ⭐⭐ makes you think · ⭐⭐⭐ will take a while

**Rules that make this work:**

1. One `cargo new` project per block. Put each exercise in its own module or function so you can keep them.
2. **Run `cargo clippy` after every single exercise.** Not optional. It will teach you idiomatic Rust faster than any book.
3. Fight for 15 minutes before searching. The struggle is the learning.
4. If you're stuck on a borrow error for more than 20 minutes: `.clone()` it, write `// TODO: remove clone`, move on. Come back in two weeks and you'll see the fix immediately.
5. Read compiler errors **in full**. Rust's diagnostics usually contain the answer.
6. Write a `#[test]` for anything with logic. It's how Rust is actually written.

**Rough pace:** 3–5 exercises per session, 4–6 weeks total. Blocks 1–6 are the foundation — don't rush them to get to the shiny concurrency stuff.

---

## Block 1 — Basics, Functions & Control Flow (1–8)

**1.** ⭐ `fn fahrenheit_to_celsius(f: f64) -> f64` and the reverse. Print a conversion table from -40°F to 212°F in steps of 20 using a `for` loop and formatted output (`{:.1}`).

**2.** ⭐ `fn is_prime(n: u32) -> bool`, then print all primes below 100. Use `?` — no wait, use a plain loop. Notice `for i in 2..=n/2`.

**3.** ⭐ FizzBuzz, but return `String` from a function instead of printing, and write three `#[test]` cases for it.

**4.** ⭐ `fn grade(score: u32) -> &'static str` using `match` with range patterns (`90..=100 => "A"`). Add a guard arm for invalid scores.

**5.** ⭐ Use `loop` with `break value` to find the first number above 1000 divisible by both 7 and 13. Then rewrite with a `while` loop and compare readability.

**6.** ⭐⭐ `fn reverse_words(s: &str) -> String` — "hello world rust" → "rust world hello". No `.rev()` on the whole string; split, reverse, rejoin.

**7.** ⭐⭐ `fn count_vowels(s: &str) -> usize` handling uppercase and lowercase. Then make a version that returns a count per vowel.

**8.** ⭐⭐ Integer division gotcha: write a function that computes `n / d` and `n % d` for negative numbers, then compare against Python's results. Fix it using `div_euclid` and `rem_euclid`. *(Real porting bug source.)*

---

## Block 2 — Ownership & Borrowing (9–16)

*This block is the whole ballgame. Do not skip it.*

**9.** ⭐ Create a `String`, assign it to another variable, try to print the first. Read the error. Do the same with an `i32`. Write a comment explaining why one works.

**10.** ⭐ Write `fn takes(s: String)` and `fn borrows(s: &String)`. Call both and observe which lets you use the variable afterwards.

**11.** ⭐⭐ `fn append_exclaim(s: &mut String)` that pushes "!" onto the end. Call it three times on the same string.

**12.** ⭐⭐ Deliberately hold a `&` and a `&mut` to the same `Vec` at once. Read the error. Then move the immutable use *above* the mutable borrow and watch it compile — that's non-lexical lifetimes.

**13.** ⭐⭐ Try to `push` to a `Vec` while iterating over it with a `for` loop. Read the error. Fix it by collecting new items into a second `Vec` and extending afterwards. *(This is the Python bug Rust refuses to let you write.)*

**14.** ⭐⭐ Write `fn first_word(s: &str) -> &str`. Then try calling `.clear()` on the source string while holding the result. Understand exactly why it fails.

**15.** ⭐⭐ Same function three ways: taking `Vec<String>`, `&Vec<String>`, and `&[String]`. Show which call sites each accepts. Conclude why `&[T]` is the right default for parameters.

**16.** ⭐⭐⭐ A `struct Inventory { items: Vec<Item>, total: f64 }` with a method that needs to iterate `items` while updating `total`. Hit the borrow error, then fix it *without* cloning. *(Hint: compute first, assign after — or destructure the fields.)*

---

## Block 3 — Structs & Enums (17–26)

**17.** ⭐ `struct Rectangle { width: f64, height: f64 }` with `area()`, `perimeter()`, `is_square()`, and an associated function `square(size)`.

**18.** ⭐ Add `can_hold(&self, other: &Rectangle) -> bool`. Derive `Debug` and print with `{:?}` and `{:#?}`.

**19.** ⭐⭐ Tuple structs `Celsius(f64)` and `Fahrenheit(f64)`. Implement `From` both ways. Try to add them together and enjoy the type error. *(The newtype pattern — used constantly in real code.)*

**20.** ⭐⭐ `enum Shape { Circle(f64), Rect(f64, f64), Triangle { base: f64, height: f64 } }` with an `area()` method using `match`. Add a variant later and watch the compiler point at every place needing an update.

**21.** ⭐⭐ `enum Command { Quit, Move { x: i32, y: i32 }, Say(String) }` plus `fn parse(input: &str) -> Option<Command>` handling `"move 3 4"`, `"say hello"`, `"quit"`.

**22.** ⭐⭐ `struct User { name: String, email: String, age: u32, active: bool }`. Derive `Default`, then build one with `User { age: 30, ..Default::default() }`.

**23.** ⭐⭐ **Builder pattern**: `RequestBuilder` with chained `.method()`, `.url()`, `.header(k, v)`, and `.build() -> Result<Request, String>` that errors if url is missing. *(This is how Rust replaces keyword arguments — you'll see it in every major crate.)*

**24.** ⭐⭐ `impl Display for Rectangle` printing `10x5 (area: 50)`. Note how `Display` differs from `Debug`.

**25.** ⭐⭐ State machine: `enum Light { Red, Yellow, Green }` with `fn next(self) -> Self` that takes `self` **by value**. Loop through 10 transitions.

**26.** ⭐⭐⭐ `struct BankAccount` with private balance, plus `deposit()`, `withdraw() -> Result<f64, String>`, and `balance()`. Enforce that balance can never go negative. Write tests for the failure cases.

---

## Block 4 — Collections (27–33)

**27.** ⭐ `Vec<i32>` operations: build one, `push`, `pop`, `insert`, `remove`, `contains`, `sort`, `dedup`. Print after each.

**28.** ⭐ Compare `v[10]` (panics) against `v.get(10)` (returns `Option`). Handle the `Option` properly.

**29.** ⭐⭐ **Word frequency counter**: read a paragraph, split on whitespace, count with `HashMap` and the entry API (`*map.entry(word).or_insert(0) += 1`). Lowercase and strip punctuation first.

**30.** ⭐⭐ Extend #29: return the top 5 words by count. *(Hint: collect into a `Vec`, then `sort_by_key` with `Reverse`.)*

**31.** ⭐⭐ `HashMap<String, Vec<String>>` — a grouping problem. Group a list of names by first letter using `.entry(k).or_insert_with(Vec::new).push(v)`.

**32.** ⭐⭐ Use `HashSet` to find the intersection, union, and difference of two lists of numbers. Then dedup a `Vec` using a `HashSet`.

**33.** ⭐⭐ Use `BTreeMap` for the same word count as #29 and observe the ordering difference. Explain when you'd pick each.

---

## Block 5 — Iterators & Closures (34–42)

*The highest-value block for day-to-day productivity.*

**34.** ⭐ Given `Vec<i32>`, use iterator chains for: sum, product, max, count of evens, and all doubled. No manual loops.

**35.** ⭐ Rewrite a Python comprehension you've written before as `.iter().filter().map().collect()`. Compare line by line.

**36.** ⭐⭐ Demonstrate `iter()`, `iter_mut()` (double every element in place), and `into_iter()` on the same `Vec`. After `into_iter()`, try to use the vec and read the error.

**37.** ⭐⭐ Prove laziness: build a chain with a `println!` inside `map` and don't consume it. Observe nothing prints and note the `must_use` warning. Then add `.collect::<Vec<_>>()`.

**38.** ⭐⭐ Use `enumerate`, `zip`, `take`, `skip`, `chain`, and `rev` — one small example each.

**39.** ⭐⭐ `filter_map` to parse `vec!["1", "two", "3"]` into `vec![1, 3]`, discarding failures silently.

**40.** ⭐⭐ Then do it the other way: collect into `Result<Vec<i32>, _>` so the first bad value short-circuits the whole thing. *(Best trick in the standard library.)*

**41.** ⭐⭐ Closures: write functions taking `Fn`, `FnMut`, and `FnOnce`. Pass a closure that mutates a captured counter, and one that consumes a captured `String`. See which compile where.

**42.** ⭐⭐⭐ Given `Vec<Employee { name, dept, salary }>`: average salary per department, highest earner overall, and names sorted by salary descending — all with iterator chains. Use `fold` for at least one.

---

## Block 6 — Error Handling (43–49)

**43.** ⭐ `fn divide(a: f64, b: f64) -> Option<f64>` returning `None` on zero. Handle the result with `match`, `if let`, and `unwrap_or`.

**44.** ⭐⭐ `fn parse_age(s: &str) -> Result<u32, String>` rejecting non-numbers and values over 150. Test both failure paths.

**45.** ⭐⭐ Chain three fallible operations with `?` in a function returning `Result`. Then rewrite the same logic with nested `match` and appreciate the difference.

**46.** ⭐⭐ Read a file and parse its first line as an integer, returning `Result<i32, Box<dyn Error>>`. Note that **two different error types** flow through the same `?`.

**47.** ⭐⭐ Define a custom error enum with variants `NotFound`, `InvalidFormat(String)`, `Io(std::io::Error)`. Implement `Display` and `From<io::Error>` by hand.

**48.** ⭐⭐ Redo #47 using the `thiserror` crate with `#[from]`. Compare the amount of code.

**49.** ⭐⭐ Use `anyhow` in a `main()` that returns `Result<(), anyhow::Error>`, adding `.context("reading config")` to a failing operation. Observe the error output.

---

## Block 7 — Traits (50–58)

**50.** ⭐ `trait Greet { fn name(&self) -> String; fn greet(&self) -> String { format!("Hello, {}!", self.name()) } }`. Implement for two structs; only override `name`.

**51.** ⭐⭐ `trait Shape` with required `area()` and `name()`, plus default `describe()`. Implement for `Circle`, `Square`, `Triangle`.

**52.** ⭐⭐ `Vec<Box<dyn Shape>>` holding all three, then a function computing total area. *(Dynamic dispatch.)*

**53.** ⭐⭐ Rewrite #52 as `fn total_area<T: Shape>(shapes: &[T])`. Explain why you now *can't* mix Circle and Square in one call. *(Static dispatch.)*

**54.** ⭐⭐ Implement `From<&str>` for a `Config` struct, then use `.into()` at the call site. Confirm `Into` came free.

**55.** ⭐⭐ Implement `TryFrom<u32>` for a `Port` newtype that rejects anything below 1024.

**56.** ⭐⭐ Implement `Iterator` for a `Countdown` struct. Then use `.filter().take().sum()` on it and marvel that it all works from one method.

**57.** ⭐⭐⭐ Operator overloading: `impl Add`, `Sub`, and `AddAssign` for a `Vector2D { x: f64, y: f64 }`. Add a `dot()` method.

**58.** ⭐⭐⭐ Orphan rule: try `impl Display for Vec<String>`. Read the error. Fix it with a newtype wrapper `struct Lines(Vec<String>)`.

---

## Block 8 — Generics (59–63)

**59.** ⭐⭐ `fn largest<T: PartialOrd>(list: &[T]) -> &T`. Test with numbers and with `&str`. Then remove the bound to see what breaks.

**60.** ⭐⭐ Generic `Stack<T>` with `push`, `pop`, `peek`, `len`, `is_empty`. Write tests using both `i32` and `String`.

**61.** ⭐⭐ Generic `Pair<T>` where `cmp_display()` exists **only** when `T: Display + PartialOrd`. Confirm it's unavailable on other types.

**62.** ⭐⭐ `fn greet<T: Into<String>>(name: T)` accepting both `&str` and `String`. Then `fn print_all<T: AsRef<str>>(items: &[T])` accepting `Vec<String>` and `Vec<&str>`.

**63.** ⭐⭐⭐ Generic `Cache<K, V>` wrapping a `HashMap` with `K: Eq + Hash + Clone`. Add `get_or_insert_with(key, closure)`. Use a `where` clause.

---

## Block 9 — Lifetimes (64–68)

**64.** ⭐⭐ `fn longest<'a>(x: &'a str, y: &'a str) -> &'a str`. Then try returning a `String` created inside the function and understand why no annotation saves you.

**65.** ⭐⭐ For each, predict whether elision applies *before* compiling: `fn f(x: &str) -> &str`, `fn g(x: &str, y: &str) -> &str`, `fn h(&self, x: &str) -> &str`.

**66.** ⭐⭐⭐ `struct Parser<'a> { input: &'a str, pos: usize }` with `next_word(&mut self) -> Option<&'a str>`. Zero allocation.

**67.** ⭐⭐⭐ Zero-copy CSV splitter: `fn parse_line(line: &str) -> Vec<&str>`. Then try to make the returned slices outlive the input and read the error.

**68.** ⭐⭐⭐ `fn normalize(s: &str) -> Cow<str>` — return borrowed when the input needs no change, owned when it does. Test both paths.

---

## Block 10 — Smart Pointers (69–71)

**69.** ⭐⭐ Recursive `enum List { Cons(i32, Box<List>), Nil }` with `sum()` and `len()`. Remove the `Box` and read the error about infinite size.

**70.** ⭐⭐ `Rc<T>`: create shared ownership of a config struct across three "components". Print `Rc::strong_count` as clones are made and dropped.

**71.** ⭐⭐⭐ `Rc<RefCell<Vec<String>>>` shared log that two structs can both append to. Then deliberately hold two `borrow_mut()`s at once and watch it **panic at runtime** instead of failing to compile.

---

## Block 11 — Modules, Testing & Cargo (72–74)

**72.** ⭐⭐ Take any earlier exercise and split it across `main.rs`, `lib.rs`, and two modules in separate files. Use `pub`, `pub(crate)`, and private items. Confirm private fields are actually inaccessible.

**73.** ⭐⭐ Add unit tests in a `#[cfg(test)] mod tests` block (testing private functions) **and** an integration test in `tests/` (public API only). Note what the integration test can't reach.

**74.** ⭐⭐ Write doc comments with a `# Examples` section containing runnable code. Run `cargo test` and confirm the doc example executes. Then break the example and watch the test fail.

---

## Block 12 — Concurrency (75–78)

**75.** ⭐⭐ Spawn 5 threads that each print their id and sleep briefly. Collect the `JoinHandle`s and join them all. Then try to use a `Vec` inside a thread without `move` and read the error.

**76.** ⭐⭐ `Arc<Mutex<i32>>` counter incremented by 10 threads. Confirm the result is exactly 10. Then try it with `Rc` instead of `Arc` and read the `Send` error.

**77.** ⭐⭐ `mpsc` channel: three producer threads sending numbers, one consumer summing them. Forget to drop the original `tx` and watch it hang — then understand why.

**78.** ⭐⭐ Add `rayon`, take a CPU-heavy iterator chain from Block 5, change `.iter()` to `.par_iter()`, and time both with `--release`. *(Also: notice how slow the debug build was.)*

---

## Block 13 — Async (79–80)

**79.** ⭐⭐ With `tokio`: write three `async fn`s that each sleep 1 second. Time them awaited sequentially, then with `tokio::join!`. Confirm 3s vs 1s.

**80.** ⭐⭐⭐ Fetch 5 URLs concurrently with `reqwest` + `tokio::spawn`, collecting results. Then put a `std::thread::sleep` inside one task and observe how it stalls the others — fix it with `tokio::time::sleep`.

---

## Appendix — Three projects (this is where it clicks)

The 80 above build fluency. **Projects build judgment.** Do at least the first two.

**Project A — CLI tool** *(after Block 7)*
A file-stats tool: `mytool count <path> --lines --words --top 10`. Use `clap` for args, walk a directory, count word frequencies, handle errors with `anyhow`, print a formatted table. Add tests.

**Project B — Web service** *(after Block 12)*
A JSON API with `axum` + `tokio` + `serde`: in-memory store behind `Arc<RwLock<HashMap>>`, CRUD endpoints, proper error responses via a custom error type implementing `IntoResponse`.

**Project C — Python extension** *(your actual payoff)*
Pick one slow function from your real work — tokenization, a preprocessing loop, a distance calculation. Rewrite it in Rust with `PyO3`, build with `maturin`, benchmark against the Python original. This is where Rust starts earning its keep for you specifically.

---

## Coverage check

| Topic | Exercises |
|---|---|
| Syntax, control flow, `match` | 1–8 |
| Ownership & borrowing | 9–16 |
| Structs, enums, methods | 17–26 |
| Collections | 27–33 |
| Iterators & closures | 34–42 |
| Error handling | 43–49 |
| Traits & dispatch | 50–58 |
| Generics | 59–63 |
| Lifetimes | 64–68 |
| Smart pointers | 69–71 |
| Modules, tests, docs | 72–74 |
| Threads & parallelism | 75–78 |
| Async | 79–80 |

**Deliberately excluded** (specialist, not needed yet): macros, `unsafe`/FFI, `PhantomData`, const generics, typestate, `Pin`, variance, atomics with memory ordering, GATs.

Finish these plus two projects and you'll be a genuinely productive Rust developer — the kind who can be handed a real codebase and contribute. That's the goal, not encyclopedic coverage.
