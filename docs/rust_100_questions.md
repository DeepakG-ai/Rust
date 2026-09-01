# 100 Real-World Rust Questions — Easy → Hard

**Rules:** Type every answer yourself. No AI, no Google on the first attempt.
For "predict the output" questions — write your prediction FIRST, then run it and compare.
For "does it compile?" questions — decide YES or NO **and name the error** before you run `cargo check`.
Mark each question: ✅ got it | ❌ wrong | 🤷 didn't know. Re-do all ❌ and 🤷 after 7 days.

**Setup:** `cargo new scratch && cd scratch`. Put each answer in `src/main.rs` and run `cargo run`.
For borrow-checker questions `cargo check` is faster than `cargo run`.
Toolchain assumed: **rustc 1.96, edition 2024** (matches this repo).

> This is the companion to `python_100_questions.md`. Where a question maps onto a Python one
> you already did, the Python number is marked like *(↔ Py Q1)* — answer the Rust one first,
> then compare the two answers. The comparison is where the learning is.

---

## PART 1 — EASY (Q1–35): Core mechanics that cause real compile errors

### Ownership, moves, and Copy

**Q1.** Does this compile? If not, what is the exact error? *(↔ Py Q1)*
```rust
let a = vec![1, 2, 3];
let b = a;
println!("{:?}", a);
```
In Python, `b = a` makes two names for one list. What does Rust do instead, and what is it called?

**Q2.** Now this: *(↔ Py Q2)*
```rust
let a = vec![1, 2, 3];
let b = a.clone();
println!("{:?} {:?}", a, b);
```
Why does this compile when Q1 doesn't? What did it cost you at runtime that Q1 didn't?

**Q3.** Predict: *(↔ Py Q3)*
```rust
let x = 5;
let y = x;
println!("{} {}", x, y);
```
Why does this compile when Q1 doesn't? Name the trait responsible. List 4 types that have it and 3 that don't.

**Q4.** Which of these four lines compile? Explain each one:
```rust
let a = [1, 2, 3];           let b = a;  println!("{:?}", a);
let c = vec![1, 2, 3];       let d = c;  println!("{:?}", c);
let e = "hi";                let f = e;  println!("{}", e);
let g = String::from("hi");  let h = g;  println!("{}", g);
```
(Two compile, two don't. If you can explain why `"hi"` behaves differently from `String::from("hi")`, you have understood ownership.)

**Q5.** Fix this two different ways — one that copies the data, one that doesn't: *(↔ Py Q7)*
```rust
fn print_all(v: Vec<i32>) {
    println!("{:?}", v);
}

fn main() {
    let nums = vec![1, 2, 3];
    print_all(nums);
    print_all(nums);
}
```
Which fix belongs in real code, and why is the other one usually wrong?

**Q6.** Predict the error message word for word:
```rust
let s = String::from("hello");
let t = s;
println!("{}", s.len());
```
Then answer: what happened to the heap memory? Who frees it, and exactly when?

**Q7.** What does this print, and why is redefining `nums` legal?
```rust
fn add_one(mut v: Vec<i32>) -> Vec<i32> {
    v.push(1);
    v
}

fn main() {
    let nums = vec![0];
    let nums = add_one(nums);
    println!("{:?}", nums);
}
```
What is *shadowing*, and how does it differ from Python simply rebinding a name?

### Borrowing and references

**Q8.** Does this compile? *(↔ Py Q20 — Python runs the equivalent and silently gives a wrong answer)*
```rust
let mut v = vec![1, 2, 3];
let first = &v[0];
v.push(4);
println!("{}", first);
```
Explain what would go wrong at the machine level if the compiler allowed it. (Hint: what does `push` do when capacity runs out?)

**Q9.** Predict:
```rust
let mut s = String::from("hi");
let r1 = &s;
let r2 = &s;
let r3 = &mut s;
println!("{} {} {}", r1, r2, r3);
```
Now change the last line to `println!("{}", r3);` and predict again. Why does deleting a *print* fix a *borrow* error? (Look up "NLL" / non-lexical lifetimes.)

**Q10.** State the borrowing rule in one sentence. Then name the bug class it eliminates that Python cannot — and note that it isn't only about threads.

**Q11.** Write `fn total(v: &Vec<i32>) -> i32`, then rewrite it as `fn total(v: &[i32]) -> i32`.
Both work. Why is the second strictly better? Which call sites now work that didn't before?

**Q12.** Does this compile?
```rust
let mut count = 0;
let mut inc = || count += 1;
inc();
inc();
println!("{}", count);
```
Now move the `println!` between the two `inc()` calls and predict again. Why does the closure "hold" the borrow?

**Q13.** Write `fn double(x: &mut i32)` and call it. Where do you need `*`, and where does Rust auto-dereference for you? Predict:
```rust
let mut n = 5;
double(&mut n);
println!("{}", n);
```

**Q14.** Why does this not compile, and what is the error?
```rust
fn make() -> &String {
    let s = String::from("hi");
    &s
}
```
Python has the same "bug" and gets away with it. Why? Give two Rust fixes.

### Strings: `String` vs `&str`

**Q15.** One line each: what is `String`? What is `&str`? Which of these compile?
```rust
let a: String = "hello";
let b: &str = "hello";
let c: String = "hello".to_string();
let d: &str = &String::from("hello");
```

**Q16.** One expression that turns `"  Hello World  "` into `"hello world"`. *(↔ Py Q8)*
Then say which type your expression produced — `String` or `&str`?

**Q17.** Predict both numbers: *(↔ Py Q9)*
```rust
let s = "héllo";
println!("{}", s.len());
println!("{}", s.chars().count());
```
Why do they differ? Now try `s[0]` — what does the compiler say, and why does Rust refuse something Python allows?

**Q18.** From `"user@gmail.com"` extract `"gmail.com"` three ways: `split`, `splitn`, and `find` + slicing. *(↔ Py Q10)*
Which one allocates? Which returns an `Option`?

**Q19.** Predict: *(↔ Py Q11)*
```rust
let name = "Deepak";
let msg = format!("{name} is {} years old", 25 + 5);
println!("{}", msg);
```
What is the difference between `format!`, `print!`, `println!`, `eprintln!`, and `write!`?

**Q20.** What does this print, and how do you fix it? *(↔ Py Q12)*
```rust
let path = "C:\new_folder\test.txt";
println!("{}", path);
```
Give the fix two ways: escaping, and a raw string literal.

### `Vec`, `HashMap`, and slices

**Q21.** Count how many times each character appears in `"engineering"`, into a `HashMap<char, usize>`. *(↔ Py Q13)*
Use the `entry` API — write it *without* calling `contains_key` first.

**Q22.** Predict all three lines: *(↔ Py Q14)*
```rust
use std::collections::HashMap;
let mut m = HashMap::new();
m.insert("a", 1);
println!("{:?}", m.get("c"));
println!("{}", m.get("c").unwrap_or(&0));
println!("{}", m["c"]);
```
Which line kills the program, and what is Rust's equivalent of `dict.get(k, default)`?

**Q23.** From `let users = vec!["ram", "sam", "ram", "tom", "sam", "ram"];` produce: *(↔ Py Q15)*
- the unique names
- a count of each name
- the most common name

(No `itertools`. `HashSet`, `HashMap::entry`, and `max_by_key` are enough.)

**Q24.** When do you write `Vec<T>`, when `&[T]`, when `&mut [T]`, and when `[T; N]` in a function signature? Give one realistic example of each.

**Q25.** Predict: *(↔ Py Q17)*
```rust
let a = vec![1, 2, 3];
let b = vec![1, 2, 3];
println!("{}", a == b);
println!("{}", std::ptr::eq(&a, &b));
```
Python has `==` vs `is`. What is Rust's version of that distinction, and which trait powers `==`?

**Q26.** One expression: squares of only the even numbers in `1..=6`, collected into a `Vec<i32>`. *(↔ Py Q18)*

**Q27.** Convert this loop into a single iterator chain that `collect()`s into a `HashMap<&str, usize>`: *(↔ Py Q19)*
```rust
let mut result = HashMap::new();
for word in ["apple", "banana", "kiwi"] {
    result.insert(word, word.len());
}
```

**Q28.** Does this compile? *(↔ Py Q20)*
```rust
let mut items = vec![1, 2, 3, 4, 5];
for item in &items {
    if *item == 3 {
        items.remove(0);
    }
}
```
Python runs the equivalent happily and gives you a wrong answer. Which error does Rust give, and what is the correct way to remove items during iteration? (Hint: `retain`.)

### `Option` and `Result` — Rust's answer to `None` and exceptions

**Q29.** Rust has no `null`. What does `Option<T>` do instead? Write `fn first_even(v: &[i32]) -> Option<i32>` twice: once with a `for` loop, once as a single iterator chain.

**Q30.** Predict: *(↔ Py Q16)*
```rust
let v: Vec<i32> = vec![];
println!("{:?}", v.first());
println!("{}", v.first().unwrap());
```
When is `.unwrap()` acceptable, and when is it the thing that pages you at 2am?

**Q31.** Rewrite this four ways — `match`, `if let`, `unwrap_or`, and `let ... else`:
```rust
let cfg: Option<&str> = None;
let model = cfg.unwrap();
```

**Q32.** What does `?` actually do? Predict the return value for `"8080"`, `"abc"`, and `"99999"`:
```rust
fn parse_port(s: &str) -> Result<u16, std::num::ParseIntError> {
    let p: u16 = s.parse()?;
    Ok(p)
}
```

**Q33.** For each line: does it fail at **compile time** or **runtime**, and what exactly happens? *(↔ Py Q27)*
```rust
"abc".parse::<i32>()
let v = vec![1, 2, 3]; v[10]
let a = 10; let b = 0; a / b
fn add(a: i32, b: i32) -> i32 { a + b }   // then call add(i32::MAX, 1)
std::fs::File::open("no_such_file.txt")
```
One of these behaves **differently in `cargo run` vs `cargo run --release`**. Which one, why, and which three methods let you handle it deliberately? (This one bites people in production.)

**Q34.** Rewrite so it cannot panic, whatever the JSON looks like: *(↔ Py Q30, Q52)*
```rust
// data: serde_json::Value
let price = data["product"]["price"].as_f64().unwrap() * 1.18;
```
Use `.get()` / `.and_then()` and return `Option<f64>`. Then write it again using `?` inside a function.

**Q35.** One line each, plus when you would pick it: `unwrap`, `expect`, `unwrap_or`, `unwrap_or_else`, `unwrap_or_default`, `ok_or`, `map_or`.
Which of these evaluate their argument eagerly, and when does that cost you?

---

## PART 2 — MEDIUM (Q36–75): Real project code

### Structs, enums, and pattern matching

**Q36.** Write a `BankAccount` struct: balance starts at 0, `deposit(&mut self, amount: u64)`, `withdraw(&mut self, amount: u64) -> Result<(), String>` that errors on insufficient funds, and `impl Display` so `println!("{}", acct)` prints `Balance: ₹500`. *(↔ Py Q45)*
Why does Rust make you `impl Display` by hand when Python gives you `__str__` on any class?

**Q37.** What does each of these derives generate for you? *(↔ Py Q49 — the dataclass question)*
```rust
#[derive(Debug, Clone, PartialEq, Default)]
struct Product { name: String, price: f64, in_stock: bool }
```
Print one with `{:?}` and `{:#?}`. Which derive would fail if `price` were a type that isn't itself `Clone`? What is a derive macro actually doing at compile time?

**Q38.** Predict, then extend: *(↔ Py Q46)*
```rust
struct Counter { count: u32 }

impl Counter {
    fn new() -> Self { Counter { count: 0 } }
    fn bump(&mut self) { self.count += 1; }
}
```
Rust has no class attributes. If you need a count shared across **all** `Counter` instances, what are your two options? Write one with `static` + `AtomicUsize`. Why does Rust make this deliberately awkward?

**Q39.** Write `fn describe(s: &Status) -> String` matching every variant:
```rust
enum Status {
    Active,
    Suspended { reason: String },
    Deleted(u64),
}
```
Now add a `Banned` variant and *don't* update `describe`. What happens? Compare with a Python `if/elif` chain over string statuses — when would that break, and who finds out?

**Q40.** Does this compile?
```rust
let n = 3;
let name = match n {
    1 => "one",
    2 => "two",
};
```
Define *exhaustiveness*. Then explain why matching on an `enum` is safer than matching on an `i32`, and when `_ =>` is the right call versus a mistake waiting to happen.

**Q41.** Rewrite as one `match` with guards and bindings:
```rust
fn tier(age: u32, member: bool) -> &'static str {
    if age < 18 { return "minor"; }
    if age >= 65 { return "senior"; }
    if member { return "member"; }
    "standard"
}
```
Use `n if n < 18 => ...` style. Which version is easier to prove correct?

**Q42.** Destructure all of these in a single `match` each — no `unwrap` anywhere:
```rust
let a: Option<i32> = Some(42);
let b: Result<i32, String> = Err("bad".into());
let c: (i32, &str) = (1, "x");
let d: &[i32] = &[1, 2, 3];
```
For `d`, match on `[]`, `[x]`, `[first, .., last]`. When have you seen slice patterns be genuinely useful?

**Q43.** Write a `ServerConfig` with 5 fields, `impl Default`, and construct one overriding only `port` using struct-update syntax (`..Default::default()`).
Then say why this pattern is the Rust answer to Python's keyword arguments with defaults — and what Rust makes you give up.

### Traits and generics

**Q44.** Write a trait `Speak` with one required method and one **default** method. Implement it for `Dog` and `Cat`, then iterate a `Vec<Box<dyn Speak>>` calling both methods. *(↔ Py Q47)*
What is this concept called, and how does it differ from Python duck typing? What can Rust catch that Python catches only at runtime?

**Q45.** Compare these three signatures — cost, flexibility, and when you're *forced* into the third:
```rust
fn a<T: Speak>(x: T)
fn b(x: impl Speak)
fn c(x: &dyn Speak)
```
Which ones produce one copy of the function per type (monomorphization)? Which does a vtable lookup? Which one can go in a `Vec`?

**Q46.** Why won't this compile, and what one bound fixes it?
```rust
fn largest<T>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list {
        if item > largest { largest = item; }
    }
    largest
}
```
Python needs no such annotation. What does Python defer to runtime that Rust demands up front?

**Q47.** Implement `From<&str> for LogLevel` (parsing `"info"`, `"warn"`, …). Then call `.into()` on a `&str` and explain why you never had to write `Into`.
Now connect it to error handling: why does implementing `From<io::Error> for MyError` make `?` work automatically?

**Q48.** Implement both `Debug` and `Display` for `struct Money(i64)` printing `₹5.00`.
Which one can you `#[derive]`, which one can't you, and why did the language make that choice?

**Q49.** Why does Rust have *both* `PartialEq`/`Eq` and `PartialOrd`/`Ord`? Answer with one type that implements the `Partial` version but not the full one, and say what property of that type breaks the guarantee.
Then: why can't you `.sort()` a `Vec<f64>`, and what are the two ways around it?

**Q50.** Write a generic `Stack<T>` with `new`, `push`, `pop -> Option<T>`, `peek -> Option<&T>`, `len`, `is_empty`.
Then add `impl<T: Display> Stack<T>` with a `print_all` method. Why can you put methods behind a bound like that, and what does it mean for a `Stack<SomeNonDisplayType>`?

**Q51.** Write `fn evens(v: &[i32]) -> impl Iterator<Item = &i32>`.
Why is `impl Iterator` in return position better than `Box<dyn Iterator>`? Then look up what edition 2021 required here that edition 2024 does not (hint: `+ '_`) — this repo is on 2024.

**Q52.** Rewrite with a `where` clause, and say when `where` earns its keystrokes:
```rust
fn report<T: Display + Clone, U: Debug + Default>(a: T, b: U) -> String
```

### Iterators and closures

**Q53.** Convert to one iterator chain: *(↔ Py Q61)*
```rust
let mut result = Vec::new();
for row in &rows {
    if row.status == "active" {
        result.push(row.email.to_lowercase());
    }
}
```

**Q54.** Predict the exact output, in order: *(↔ Py Q62, Q98)*
```rust
let v = vec![1, 2, 3];
let it = v.iter().map(|x| { println!("mapping {}", x); x * 2 });
println!("built");
let out: Vec<i32> = it.collect();
println!("{:?}", out);
```
Iterators are lazy. Which line actually runs the closure? What is the Python equivalent of this laziness, and which Python question does it match?

**Q55.** For each, say what the loop variable's type is and whether the original is still usable afterwards:
```rust
for x in v.iter()      { }
for x in v.iter_mut()  { }
for x in v.into_iter() { }
for x in &v            { }
for x in v             { }
```
Which two pairs are the same thing? Which one moves `v`?

**Q56.** One line each on `let v = vec![1, 2, 3, 4, 5, 6];`:
`map`, `filter`, `filter_map`, `flat_map`, `fold`, `sum`, `any`, `all`, `find`, `position`, `take_while`, `skip`, `chain`, `rev`, `enumerate`, `zip`, `last`, `count`, `min_by_key`, `windows`, `chunks`. *(↔ Py Q65)*
Which of these are **not** iterator adapters at all? (Two on that list are slice methods.)

**Q57.** Parse `vec!["1", "2", "x", "4"]` into numbers, three ways:
1. Stop at the first bad value → `Result<Vec<i32>, ParseIntError>` (hint: `collect::<Result<Vec<_>, _>>()`)
2. Skip bad values silently → `Vec<i32>`
3. Keep both → `(Vec<i32>, Vec<ParseIntError>)`

*(↔ Py Q55)* Which one do you want when processing 100 API rows, and why is the answer "it depends on who reads the log"?

**Q58.** For each closure, say whether it is `Fn`, `FnMut`, or `FnOnce`, and why:
```rust
let a = |x: i32| x + 1;
let mut count = 0;
let b = || count += 1;
let s = String::from("hi");
let c = move || s;
```
What does `move` actually change? When are you *forced* to use it?

**Q59.** Predict — and this is the one that separates Rust from Python: *(↔ Py Q70, the closure trap)*
```rust
let mut funcs: Vec<Box<dyn Fn() -> i32>> = Vec::new();
for i in 0..3 {
    funcs.push(Box::new(move || i));
}
let out: Vec<i32> = funcs.iter().map(|f| f()).collect();
println!("{:?}", out);
```
Python prints `[2, 2, 2]`. Rust does not. Explain the difference in one sentence about *what gets captured*.

**Q60.** Sort `products` by price, highest first: *(↔ Py Q66, Q67)*
```rust
struct Product { name: String, price: f64 }
```
Do it with `sort_by`. Now try `sort_by_key(|p| p.price)` — why does that fail, and what are `partial_cmp` and `total_cmp`?
Finally: which of `sort` / `sort_unstable` / `sorted` exists in Rust, and what does `sort` return? (Compare with Python's `list.sort()` returning `None`.)

### Error handling in real programs

**Q61.** Write a custom error type by hand — no crates:
```rust
enum AppError { Io(std::io::Error), BadPort(std::num::ParseIntError), Missing(String) }
```
Implement `Display`, `std::error::Error`, and `From` for both wrapped types. Then write a function that uses `?` on both a file read and a `parse` and returns `Result<u16, AppError>`.
Count the lines of boilerplate you just wrote.

**Q62.** Now rewrite Q61 using `thiserror`, with `#[error("...")]` and `#[from]`. How many lines did it delete? What does the derive generate that you wrote by hand?

**Q63.** `anyhow::Result<T>` vs a typed error enum: which belongs in a **library** and which in a **binary**, and why? What can a caller do with `AppError` that they cannot do with `anyhow::Error`?

**Q64.** Take a function that reads a config file and add `.context("failed to read config at {path}")` so the failure message names the path. *(↔ Py Q71 — reading tracebacks)*
Then print the full chain with `{:?}` on an `anyhow::Error`. Why is "No such file or directory" alone a useless production log line?

**Q65.** Predict the output and the return value: *(↔ Py Q29, the try/finally question)*
```rust
fn run() -> Result<(), String> {
    println!("A");
    Err("boom".to_string())?;
    println!("B");
    Ok(())
}

fn main() {
    println!("{:?}", run());
    println!("D");
}
```
Rust has no `finally`. What plays that role, and which trait guarantees your cleanup runs even on early return?

**Q66.** When is `panic!` the *correct* choice and when is it a bug? *(↔ Py Q28)*
Answer for: an invariant your own code violated; malformed user input; a missing env var at startup; a failed network call.
Then: what does an `.unwrap()` inside a library crate do to the person using it?

### Logging with `log` and `tracing`

**Q67.** Replace these with real logging. Write the full setup, including `Cargo.toml` lines: *(↔ Py Q36)*
```rust
println!("Starting job");
println!("WARNING: config missing, using defaults");
println!("ERROR: could not connect to DB");
```
Use the `log` facade plus `env_logger`, and get output shaped like `2026-09-02T10:30:00Z INFO myapp: Starting job`.

**Q68.** Name the 5 `log` levels in order. *(↔ Py Q37)*
With `RUST_LOG=info`, which appear and which are hidden? What does `RUST_LOG=warn,myapp::db=debug` do?
Then the trap: you call `info!(...)` in your library and **nothing prints anywhere**. Why? (This is the `log` facade's whole design — explain it.)

**Q69.** Set up `tracing` + `tracing_subscriber`: console at INFO, a file at DEBUG, format including timestamp, level, target, and line number. *(↔ Py Q38)*
Then add `#[instrument]` to a function and look at the output. What is a **span**, how does it differ from an **event**, and why do spans matter enormously once you have 10 concurrent async tasks writing to one log?

**Q70.** Two parts: *(↔ Py Q39, Q44)*
1. Set up a **rotating** log file with `tracing-appender`, keeping a file per day. Then try to make it rotate at 5 MB instead — what do you find, and which crate do you actually need? (The answer is a real-world lesson, not a trick.)
2. Is `info!("processing {:?}", big_struct)` evaluated when the level is off? Compare with the Python answer (`logging.info("user %s", user_id)`). What do `?value` and `%value` mean in a `tracing` macro?

### serde, files, and CLI with clap

**Q71.** Read `config.json` into a `#[derive(Deserialize, Serialize)] struct Config`, change `model` to `"claude-sonnet-5"`, and write it back pretty-printed. *(↔ Py Q51)*
Why is deserializing into a **struct** better than into `serde_json::Value`? Which Python question does this correspond to, and what does Rust catch that pydantic catches?

**Q72.** Given a struct with `#[serde(rename_all = "camelCase")]`, work out the behaviour of each: *(↔ Py Q83, Q84)*
- a field marked `#[serde(default)]` that's missing from the JSON
- a field of type `Option<String>` that's missing
- an **extra** field in the JSON that your struct doesn't have
- adding `#[serde(deny_unknown_fields)]`

Then answer the design question: the API adds a new field next month. Which of these settings breaks your deploy, and is breaking actually what you want?

**Q73.** Read a possibly-huge file line by line and print only lines containing `"error"`, case-insensitively. *(↔ Py Q31, Q62)*
Use `BufReader::lines()`. Then explain what `File::open` alone gives you and why `BufReader` matters — how many syscalls does each version make?

**Q74.** Write a `clap` derive CLI: `--input` (required `PathBuf`), `--output` (optional, default `result.txt`), `--verbose` (a flag, countable so `-vv` works). Print the parsed values. *(↔ Py Q58)*
Then wire `--verbose` to the log level from Q68.

**Q75.** File and path work with `std::fs` and `Path`: *(↔ Py Q35, Q57, Q60)*
1. Create `outputs/2026-09-02/` (today's date, built at runtime) without failing if it exists.
2. Find every `.csv` under a folder recursively and print each name with its size in KB.
3. Which of `File::create`, `File::open`, and `OpenOptions::new().append(true)` **silently destroys** existing data?

---

## PART 3 — HARD (Q76–100): Production-grade patterns

### Lifetimes

**Q76.** Fix this, then explain what you actually wrote:
```rust
fn longest(a: &str, b: &str) -> &str {
    if a.len() > b.len() { a } else { b }
}
```
`'a` does **not** mean "lives for duration a". Say in one sentence what it really is — a *constraint relating* which things?
Then predict: does this compile?
```rust
let s1 = String::from("long string");
let result;
{
    let s2 = String::from("short");
    result = longest(&s1, &s2);
}
println!("{}", result);
```

**Q77.** Why does this struct need an annotation, and what does the annotation promise?
```rust
struct Parser<'a> { input: &'a str, pos: usize }
```
Write `impl<'a> Parser<'a>` with `new(input: &'a str)` and `next_word(&mut self) -> Option<&'a str>`.
Then answer: what stops you from storing a `Parser` in a struct that outlives its input?

**Q78.** Which of these need explicit lifetimes and which does the compiler infer? State the elision rule that applies to each:
```rust
fn f(x: &str) -> &str
fn g(x: &str, y: &str) -> &str
fn h(&self, x: &str) -> &str
fn i(x: &str, y: &str) -> (&str, &str)
```
For `h`, which input does the output borrow from by default — and when is that default *wrong*?

**Q79.** `&'static str` and `T: 'static` look alike and mean different things. Explain both.
Then: is `String::from("hi")` `'static`? Is `&String::from("hi")`? Why does `thread::spawn` require `'static`, and why does that *not* mean "lives forever"?

**Q80.** This won't compile. Give three different fixes, each with a different trade-off:
```rust
fn normalize(input: &str) -> &str {
    let cleaned = input.trim().to_lowercase();
    &cleaned
}
```
(Return owned; take an output buffer; use `Cow<'_, str>`.) When is `Cow` genuinely the right answer, and what does it buy you when 90% of inputs are already clean?

### Smart pointers and interior mutability

**Q81.** Why won't this compile, and what does the error literally say?
```rust
enum List { Cons(i32, List), Nil }
```
Fix it with `Box`. Explain in terms of *size* — what does the compiler need to know that it can't figure out here?

**Q82.** Predict `Rc::strong_count(&a)` at each marked point:
```rust
use std::rc::Rc;
let a = Rc::new(vec![1, 2, 3]);
println!("{}", Rc::strong_count(&a));      // 1?
let b = Rc::clone(&a);
println!("{}", Rc::strong_count(&a));      // 2?
{
    let c = Rc::clone(&a);
    println!("{}", Rc::strong_count(&a));  // 3?
}
println!("{}", Rc::strong_count(&a));      // 4?
```
What does `Rc::clone` copy, and what does it *not* copy? When is `Rc` the right tool and when are you reaching for it to dodge the borrow checker?

**Q83.** Predict — does this compile, and does it run?
```rust
use std::cell::RefCell;
let c = RefCell::new(vec![1, 2, 3]);
let a = c.borrow_mut();
let b = c.borrow_mut();
println!("{:?} {:?}", a, b);
```
Define *interior mutability*. What did you trade away to get it — and is `RefCell` "cheating" the borrow rules or relocating them?

**Q84.** `Rc<RefCell<T>>` vs `Arc<Mutex<T>>` — write the type you'd use for each situation:
- a tree where children know their parent, single-threaded
- a config cache read by 8 worker threads, occasionally updated
- a counter incremented by 100 threads

Then: why is `Rc<T>` not `Send`? Answer with what would go wrong to the refcount.

**Q85.** Build a reference cycle with `Rc<RefCell<Node>>` where two nodes point at each other, and prove it leaks (print `strong_count`, add a `Drop` impl, watch it never fire). *(↔ Py Q99)*
Then fix it with `Weak`. What does `Weak::upgrade` return, and why is that return type exactly right?
Rust prevents use-after-free and data races at compile time. Does it prevent memory leaks? Defend your answer.

### Concurrency: threads, channels, `Send`/`Sync`

**Q86.** Spawn 10 threads that each increment a shared counter 1000 times. Write **three** versions:
1. the naive `&mut` version that does not compile — what's the error?
2. `Arc<Mutex<i32>>`
3. `Arc<AtomicUsize>` with `fetch_add`

Time all three. Which is fastest and why? What is `Ordering::Relaxed` versus `SeqCst`, and when does the difference actually matter to you?

**Q87.** CPU-bound vs I/O-bound: for each, which do you reach for — `std::thread`, `rayon`, or `tokio`? *(↔ Py Q85, Q86)*
- hashing 10,000 files
- calling an LLM API 50 times
- parsing a 2 GB CSV
- a web server handling 5,000 open connections

Python's answer is shaped entirely by the GIL. Rust has no GIL. Which of your Python answers changes, and which stays the same?

**Q88.** Use `std::sync::mpsc`: 4 producer threads each send 10 results; main collects all 40 and prints them in arrival order.
Then answer: what does the receiver's `for` loop do when the last sender is dropped? What happens if you forget to drop the original sender? What's the difference between `channel()` and `sync_channel(0)`?

**Q89.** One sentence each: what does `Send` mean? What does `Sync` mean?
Then explain, in terms of those two: why is `Rc<T>` not `Send`? Why is `RefCell<T>` not `Sync`? Why *is* `Mutex<T>` `Sync` even though its contents aren't?
Finally: "fearless concurrency" — name the exact bug class that becomes a compile error, and the class that does **not** (hint: you can still deadlock).

### Async with tokio

**Q90.** Rewrite this blocking loop as async, fetching all URLs concurrently: *(↔ Py Q87)*
```rust
fn fetch(url: &str) -> String {
    reqwest::blocking::get(url).unwrap().text().unwrap()
}
let results: Vec<String> = urls.iter().map(|u| fetch(u)).collect();
```
Use `tokio` + `reqwest`, with `futures::future::join_all` and then with `JoinSet`. What do `async fn`, `.await`, `tokio::spawn`, and `#[tokio::main]` each do? When do you need `spawn` rather than just `join_all`?

**Q91.** Predict the output:
```rust
async fn hello() { println!("hi"); }

fn main() {
    hello();
    println!("done");
}
```
Nothing prints "hi". Why? What does calling an `async fn` actually *return*?
Then: Python ships an event loop in the standard library; Rust does not. Why did Rust make that choice, and what does it cost you?

**Q92.** What is wrong with this, and how bad is it?
```rust
async fn handle() {
    std::thread::sleep(std::time::Duration::from_secs(5));
    println!("done");
}
```
Explain what happens to the *other* tasks scheduled on that worker thread. Give both fixes (`tokio::time::sleep` and `spawn_blocking`), and say which one you'd use for a blocking `rusqlite` query.

**Q93.** You have 50 prompts to send to an LLM API, but the provider allows only 10 concurrent requests. *(↔ Py Q85, Q88)*
Write it two ways: `tokio::sync::Semaphore`, and `futures::stream::buffer_unordered(10)`.
Then: one request fails. What happens to the other 49 in each version? Rewrite so failures are collected into a `Vec<(usize, Error)>` and the batch finishes regardless.

### Tricky snippets

**Q94.** Predict the output in `cargo run` and in `cargo run --release`:
```rust
fn add(a: u8, b: u8) -> u8 { a + b }

fn main() {
    println!("{}", add(250, 10));
}
```
This is Rust's version of a silent Python bug — except Python has arbitrary-precision ints and never hits it. Explain the debug/release difference, then rewrite `add` three ways using `checked_add`, `saturating_add`, and `wrapping_add`. Which would you use in a byte parser? In a money calculation?

**Q95.** Predict: *(↔ Py Q95)*
```rust
println!("{}", 0.1 + 0.2 == 0.3);
println!("{:.20}", 0.1_f64 + 0.2);
```
Same answer as Python — why? Then: what is the correct way to compare two `f64`s, and why does `f64` implement `PartialOrd` but not `Ord`?
Where does this bite you when comparing model loss values or embedding distances?

**Q96.** Predict the exact output and order:
```rust
struct Noisy(&'static str);

impl Drop for Noisy {
    fn drop(&mut self) { println!("drop {}", self.0); }
}

fn main() {
    let _a = Noisy("a");
    let _b = Noisy("b");
    { let _c = Noisy("c"); }
    let _ = Noisy("d");
    println!("end of main");
}
```
Two traps here. Which one is `let _ = ...` versus `let _d = ...`? And in what order do `_a` and `_b` drop?
Then: this is RAII. Name three things it replaces from your Python habits (`with` blocks, `finally`, `__del__`) and say which of those it does *better*.

**Q97.** For each line say whether it compiles, and what `v` looks like afterwards:
```rust
let v = vec![String::from("a"), String::from("b")];
let lens: Vec<usize> = v.iter().map(|s| s.len()).collect();
println!("{:?}", v);

let v2 = vec![String::from("a"), String::from("b")];
let owned: Vec<String> = v2.into_iter().map(|s| s.to_uppercase()).collect();
println!("{:?}", v2);
```
Now the harder half: in the first block, why does `.map(|s| s.len())` work when `s` is `&String` and `len` is defined on `str`? Name the mechanism (two layers of it).

### Real-world mini-systems (write from scratch, no AI)

**Q98.** **`logstats`** — a CLI that reads a log file that may be gigabytes and prints the 10 most frequent ERROR messages, without ever holding the file in memory. *(↔ Py Q93)*
Requirements: `clap` for args, `BufReader::lines()`, a `HashMap<String, usize>`, and a bounded top-10 selection. Normalize messages by stripping timestamps and IDs so `"timeout for user 4821"` and `"timeout for user 9013"` count as the same error.
Ship it with 3 unit tests over an in-memory `&[u8]` rather than a real file — design your function signature so that's possible. (That design constraint is the actual lesson.)

**Q99.** **`RateLimiter` + disk cache** — two small components you will reuse forever. *(↔ Py Q90, Q91)*
1. `RateLimiter::new(max_calls: usize, period: Duration)` with `allow(&self) -> bool`, safe to share across threads (`Arc`, interior mutability, a `VecDeque` of timestamps). Inject the clock so you can test it without sleeping.
2. `fn cached<T, F>(key: &str, f: F) -> Result<T> where T: Serialize + DeserializeOwned, F: FnOnce() -> Result<T>` — check a JSON file on disk first, otherwise compute, save, return.

Why is the injected clock in part 1 the difference between a testable component and one you can only verify by waiting 60 seconds?

**Q100.** **The graduation task.** Build the pipeline: 10,000 documents through an LLM API. *(↔ Py Q100)*

Requirements, each one a question you have already answered:
- config from env vars with `config.json` fallback and built-in defaults (Q71, Q75)
- `clap` CLI for input dir and concurrency (Q74)
- batches of 20 (Q56, iterator `chunks`)
- max 10 concurrent requests (Q93)
- rate limited to 50 calls/minute (Q99)
- retry each failure 3 times with backoff (Q66, Q90)
- results cached to disk so a crash resumes where it stopped (Q99)
- structured progress logged to a rotating file with spans per batch (Q69, Q70)
- one failed document never kills the run; failures collected and reported at the end (Q57, Q93)
- graceful Ctrl+C via `tokio::signal` that finishes in-flight work and flushes the cache (Q96 — `Drop`)

Write the real structure: modules, structs, trait boundaries, the error type, and the main loop. Then answer the design question that has no single right answer: **where does your error type change from `anyhow::Error` to a typed enum, and why there?**

---

## How to use this

1. **Do 5 questions per day**, in order. Do not binge 30 — retention needs sleep between sessions.
2. **Type everything** into a real `cargo` project and run it. Predictions written down BEFORE running.
3. For every compile error, read the **full** `rustc` output including the `help:` line, then run `cargo explain E0502` on the code. The compiler is the tutor; this document just picks the exercises.
4. Keep an **error journal**: every ❌, one line on what you misunderstood. Borrow-checker entries will cluster — that cluster is your actual gap.
5. After 7 days, **redo every ❌ cold.**
6. Q98, Q99, and Q100 are the graduation projects. When you can write Q100 unaided, you are no longer "knows the concepts, can't write the code."

### Where this differs from the Python list

The Python questions are mostly *"predict the runtime behaviour"*, because Python's bugs show up when the code runs.
Roughly half of these are *"will this compile, and what is the error"*, because Rust's bugs show up before the code runs. That shift is the whole point: the questions that used to be 2am pager questions become compile-time questions. The cost is that you argue with the compiler up front — and learning to lose those arguments quickly is the skill this document is training.

### Related material in this repo

- `challenges/` — the test-driven ladder: `cargo test -p exercises ex04`. Overlaps Q1–Q35 and Q86–Q93.
- `docs/rust-phase2-ownership-borrowing-lifetimes.md` — read *after* attempting Q1–Q14, not before.
- `docs/lifetime.md` — for Q76–Q80.
- `docs/concurrency_and_tokio.md` — for Q86–Q93.
- `docs/iterators.md`, `docs/iterator.md` — for Q53–Q60.
