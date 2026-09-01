# Rust Tasks — solve one at a time

25 questions. Basics first, LeetCode in the middle, Async & Web with Tokio + Axum at the end.
Q1–Q22 use only standard library `std`. Q23–Q25 introduce Tokio & Axum.

---

## How to run a task

Each task is **one file**. Put it in this folder as `task/q01.rs`.

**Easiest way — no Cargo at all:**

```bash
cd task
rustc q01.rs -o q01.exe
./q01.exe
```

**If you prefer `cargo run`,** add this to `C:\Users\deepa\Projects\Rust\Cargo.toml`
(next to your existing `[[bin]]` blocks) and then `cargo run --bin q01`:

```toml
[[bin]]
name = "q01"
path = "task/q01.rs"
```

Every file needs a `fn main()`. That is the entry point — the thing that runs.

```rust
fn my_function(x: i32) -> i32 {   // defining a function
    x * 2
}

fn main() {                        // the program starts here
    let result = my_function(5);   // calling a function
    println!("{result}");
}
```

---

## Three rules you will need constantly

**1. Borrow (`&`) or take (no `&`)?**

| You want to...                        | Write            |
|---------------------------------------|------------------|
| just read it, caller keeps using it   | `&Vec<T>`, `&str`, `&Struct` |
| change it, caller keeps using it      | `&mut Vec<T>`, `&mut Struct` |
| store it / consume it / return it     | `Vec<T>`, `String`, `Struct` |

**Default to `&`.** Only drop the `&` when you have a reason.

**2. `String` vs `&str`** — `String` is owned and growable, `&str` is a
borrowed view. Function *parameters* take `&str`. Function *returns* are
usually `String`.

**3. When do I write a trait?** When two or more different types need the
**same method name** and you want to call it without caring which type it is.
One type only? Just use `impl MyStruct { }` — no trait needed.

---

# Part 1 — Basics (Q1–Q9)

## Q1 — for loop and if

Write a program that:

- loops over the numbers 1 to 20
- skips any number divisible by 3
- prints every other number
- after the loop, prints the total of the numbers it printed

Expected output ends with:

```
1
2
4
...
20
Total: 147
```

*Hint: `for n in 1..=20 { }`, and `continue` skips to the next iteration.*

---

## Q2 — functions with parameters and return values

Write **three separate functions**, all called from `main`:

- `celsius_to_fahrenheit(c: f64) -> f64`
- `is_freezing(c: f64) -> bool` — true when `c <= 0.0`
- `describe(c: f64) -> String` — returns `"25C = 77F (not freezing)"`

`describe` must **call** the other two functions, not repeat their logic.
In `main`, loop over `[-10.0, 0.0, 25.0, 37.5]` and print each description.

*Hint: the last expression in a function is its return value — no `return`
keyword and no semicolon.*

---

## Q3 — borrow vs move (the important one)

Write two functions that both add up a list of prices:

- `fn total_borrowed(prices: &Vec<f64>) -> f64`
- `fn total_owned(prices: Vec<f64>) -> f64`

In `main`:

1. build `let prices = vec![10.5, 20.0, 3.25];`
2. call `total_borrowed(&prices)` — then print `prices` again. Works.
3. call `total_owned(prices)` — then try to print `prices` again.
4. **The compiler will refuse.** Read the error message fully, then comment
   that line out and write the error as a comment in your file.

This one task is the whole ownership system. Do not skip step 4.

---

## Q4 — `String` vs `&str`

Write:

- `fn shout(text: &str) -> String` — uppercase, with `"!"` on the end
- `fn first_word(text: &str) -> &str` — the text up to the first space
- `fn add_prefix(text: &mut String, prefix: &str)` — inserts at the front,
  changing the caller's variable

In `main`, prove all three work.

Expected:

```
HELLO WORLD!
hello
[LOG] hello world
```

*Hint: `split_whitespace().next()`, and `insert_str(0, ..)`.*

---

## Q5 — struct and methods

```rust
struct Employee {
    name: String,
    monthly_salary: f64,
    years: u32,
}
```

Write an `impl Employee` block with:

- `fn new(name: &str, monthly_salary: f64, years: u32) -> Employee`
- `fn annual_salary(&self) -> f64`
- `fn is_senior(&self) -> bool` — 5 years or more
- `fn summary(&self) -> String` — `"Asha: 1200000 per year (senior)"`

In `main`, make a `Vec<Employee>` with 3 people and print every summary.

*Hint: `&self` = read the struct. `Self` is shorthand for the struct's own type.*

---

## Q6 — `&mut self`

```rust
struct BankAccount {
    owner: String,
    balance: f64,
}
```

Methods:

- `fn new(owner: &str) -> BankAccount` — starts at 0.0
- `fn deposit(&mut self, amount: f64)`
- `fn withdraw(&mut self, amount: f64) -> bool` — refuse and return `false` if
  the balance is too low, otherwise subtract and return `true`
- `fn balance(&self) -> f64`

In `main`, declare it as `let mut account = ...`, deposit 500, withdraw 200,
try to withdraw 1000, print the result of each attempt and the final balance.

**Question to answer in a comment:** why does `deposit` need `&mut self` but
`balance` only needs `&self`?

---

## Q7 — enum and `match`

```rust
enum Status {
    Active,
    Suspended { reason: String },
    Closed(String),   // closing date
}
```

Write `fn status_message(status: &Status) -> String` using a `match` that
returns:

- `Active`    → `"Account is active"`
- `Suspended` → `"Suspended: payment overdue"`
- `Closed`    → `"Closed on 2026-01-15"`

In `main`, put all three in a `Vec<Status>` and print each message.

*Hint: `match` must cover every variant — the compiler enforces it. That is a
feature, not a nuisance.*

---

## Q8 — `Option`

Using the `Employee` struct from Q5, write:

- `fn find_by_name<'a>(employees: &'a Vec<Employee>, name: &str) -> Option<&'a Employee>`
- `fn highest_paid(employees: &Vec<Employee>) -> Option<&Employee>` —
  `None` when the list is empty

In `main`, handle both results with `match` — print the person when found,
print `"not found"` when not.

*Hint: this is Rust's replacement for `None` in Python. There is no `null`.*

---

## Q9 — `Result` and `?`

Write:

- `fn parse_age(raw: &str) -> Result<u32, String>` —
  - not a number → `Err("not a number: abc")`
  - over 150 → `Err("age out of range: 200")`
  - otherwise → `Ok(n)`
- `fn parse_two(a: &str, b: &str) -> Result<u32, String>` — parses both and
  returns their sum, using the `?` operator so the first failure stops it

In `main`, test with `("30", "12")`, `("30", "abc")`, `("200", "10")`.

*Hint: `?` means "if this is an Err, return it from this function right now".
It is Rust's version of letting an exception bubble up.*

---

# Part 2 — Traits and generics (Q10–Q14)

## Q10 — your first trait

Define:

```rust
trait Describe {
    fn describe(&self) -> String;
}
```

Implement it **manually** for two different structs: `Employee` (from Q5) and

```rust
struct Product {
    name: String,
    price: f64,
}
```

Then write `fn print_description<T: Describe>(item: &T)` and call it with both.

**The point:** one function, two unrelated types. That is what a trait buys you.

---

## Q11 — default method in a trait

Extend the `Describe` trait with a second method that has a **body already
written inside the trait**:

```rust
trait Describe {
    fn describe(&self) -> String;

    fn short_label(&self) -> String {
        format!("<{}>", self.describe())
    }
}
```

- `Employee` uses the default `short_label`
- `Product` **overrides** it to return `"$29.99"` style instead

Print both labels in `main`.

---

## Q12 — trait objects (`dyn`)

Take Q10's types and put them **in the same Vec**:

```rust
let items: Vec<Box<dyn Describe>> = vec![
    Box::new(employee),
    Box::new(product),
];
```

Loop over it and print every description.

**Question to answer in a comment:** why does this need `Box<dyn Describe>`
instead of `Vec<T>` like Q10 used?

*Hint: a `Vec` needs all its elements to be the same type and the same size.
`Box<dyn Trait>` makes them all the same size — a pointer.*

---

## Q13 — generic function

Write `fn largest<T: PartialOrd>(list: &[T]) -> &T` that returns the biggest
item.

Call it with:

- `&[34, 50, 25, 100, 65]`
- `&[1.2, 9.8, 3.3]`
- `&["apple", "zebra", "mango"]`

One function, three different types. No copy-paste.

*Hint: `T: PartialOrd` means "any type T that can be compared with `>`".*

---

## Q14 — generic struct

```rust
struct Pair<T> {
    first: T,
    second: T,
}
```

Write:

- `fn new(first: T, second: T) -> Pair<T>` (in `impl<T> Pair<T>`)
- `fn swap(&mut self)`
- `fn largest(&self) -> &T` — but this one only exists when `T: PartialOrd`,
  so put it in a **separate** `impl<T: PartialOrd> Pair<T>` block

Test with `Pair<i32>` and `Pair<String>`.

---

# Part 3 — Lifetimes, iterators, modules (Q15–Q19)

## Q15 — lifetimes

Write:

- `fn longest<'a>(a: &'a str, b: &'a str) -> &'a str` — the longer string
- a struct that **holds a borrowed string**:

```rust
struct Highlight<'a> {
    text: &'a str,
}
```

with a method `fn first_sentence(&self) -> &str`.

In `main`, build a `String`, create a `Highlight` borrowing it, and print.

**Then break it on purpose:** make the `String` go out of scope while the
`Highlight` is still alive. Read the error, write it in a comment.

*Hint: `'a` does not change behaviour. It only tells the compiler "the output
borrows from the input, so the input must outlive it".*

---

## Q16 — iterators (rewrite of Q1 and Q3)

Rewrite these using iterator chains, **no `for` loop at all**:

- Q1's sum → `(1..=20).filter(..).sum()`
- Q3's total → `prices.iter().sum()`
- new: from `vec![15, 8, 42, 4, 23, 16]`, produce a `Vec<String>` of only the
  numbers over 10, formatted as `"n=15"`
- new: count how many are even

Keep the old `for` version in a comment above each so you can see them side by
side.

*Hint: `.iter()` → `.filter()` / `.map()` → `.collect()` or `.sum()` or
`.count()`. Nothing runs until the last one.*

---

## Q17 — HashMap

Write `fn word_count(text: &str) -> HashMap<String, usize>` that counts words,
lowercased.

In `main`, run it on `"the quick the lazy THE dog"`, then print the counts
**sorted by word** so the output is stable.

Expected:

```
dog: 1
lazy: 1
quick: 1
the: 3
```

*Hint: `use std::collections::HashMap;` at the top.
`*map.entry(key).or_insert(0) += 1;` is the whole trick.
To sort, collect into a `Vec` and call `.sort()`.*

---

## Q18 — splitting code across files

This one is about **project structure**, not logic. Make three files:

```
task/q18/main.rs        <- fn main()
task/q18/employee.rs    <- the Employee struct and its impl
task/q18/payroll.rs     <- functions that work on Vec<Employee>
```

In `main.rs`:

```rust
mod employee;                      // "load the file employee.rs"
mod payroll;

use employee::Employee;            // "let me write Employee, not employee::Employee"

fn main() {
    let staff = vec![ Employee::new("Asha", 100000.0, 6) ];
    println!("{}", payroll::total_annual(&staff));
}
```

Rules to discover:

- anything you want visible from another file needs `pub`
- `pub struct` is not enough — **each field** you access from outside also
  needs `pub`
- in `payroll.rs`, reach the other module with `use crate::employee::Employee;`

Build it with `rustc q18/main.rs -o q18.exe`.

Write `fn total_annual(staff: &Vec<Employee>) -> f64` and
`fn seniors(staff: &Vec<Employee>) -> Vec<&Employee>` in `payroll.rs`.

---

## Q19 — retry with backoff (your API question, in Rust)

Write `fn call_api(url: &str) -> Result<String, String>` that:

- tries up to **3 times**
- sleeps **2 seconds** between attempts (not after the last one)
- logs every attempt as `attempt 1/3 failed: connection refused — retrying in 2s`
- returns `Ok(body)` as soon as one succeeds
- returns `Err("all 3 attempts failed")` after the third failure

Since there is no HTTP here, simulate the network with a helper you write:

```rust
fn fake_request(url: &str, attempt: u32) -> Result<String, String> {
    if attempt < 3 {
        Err(String::from("connection refused"))
    } else {
        Ok(format!("200 OK from {url}"))
    }
}
```

In `main`, call it and print the final result.

*Hint: `use std::thread::sleep;` and `use std::time::Duration;`, then
`sleep(Duration::from_secs(2));`*

---

# Part 4 — LeetCode (Q20–Q22)

## Q20 — Two Sum  *(Vec)*

`nums: Vec<i32>`, `target: i32`. Return the **indices** of the two numbers
that add up to `target`. Exactly one answer exists.

```rust
fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32>
```

```
nums = [2, 7, 11, 15], target = 9   ->  [0, 1]
nums = [3, 2, 4],      target = 6   ->  [1, 2]
nums = [3, 3],         target = 6   ->  [0, 1]
```

Do the nested-loop version first. Then redo it with a `HashMap` so it makes
one pass. Keep both.

*Hint for the fast version: as you walk the list, ask the map "have I already
seen `target - current`?" before inserting the current number.*

---

## Q21 — Valid Anagram  *(String)*

```rust
fn is_anagram(s: String, t: String) -> bool
```

```
"anagram", "nagaram"  ->  true
"rat", "car"          ->  false
"ab", "a"             ->  false
```

*Hint: different lengths → immediately false. Then count characters of `s` in
a `HashMap<char, i32>`, subtract the counts of `t`, and check nothing is left
over. `s.chars()` gives you characters.*

---

## Q22 — Number of Islands  *(Graph)*

Given a grid of `'1'` (land) and `'0'` (water), count the islands. Land
connects horizontally and vertically, not diagonally.

```rust
fn num_islands(grid: Vec<Vec<char>>) -> i32
```

```
[['1','1','0','0','0'],
 ['1','1','0','0','0'],
 ['0','0','1','0','0'],
 ['0','0','0','1','1']]        ->  3
```

*Hint: walk every cell. When you hit a `'1'`, add 1 to the count and then
"sink" the whole island — recursively set that cell and its 4 neighbours to
`'0'` so you never count it twice. Your recursive helper needs
`&mut Vec<Vec<char>>` and must stop at the grid edges.*

This is the hardest one here. Get Q20 and Q21 done first.

---

# Part 5 — Async & Web Backend: Tokio & Axum (Q23–Q25)

## Q23 — Tokio basics (`async`, `await`, and `tokio::spawn`)

In Rust, `async fn` does **not** run on its own like Python's `asyncio` or JavaScript. It produces a `Future` that sits idle until driven by an async runtime (Tokio) using `.await`.

Write a program using `#[tokio::main]`:

1. Write `async fn fetch_user_data(user_id: u32, delay_ms: u64) -> String` that:
   - sleeps asynchronously with `tokio::time::sleep(tokio::time::Duration::from_millis(delay_ms)).await`
   - returns `format!("User {user_id} data loaded")`

2. In `main`:
   - spawn 3 concurrent tasks with `tokio::spawn` for IDs `1`, `2`, `3` with delays `300ms`, `100ms`, `200ms`
   - await all three `JoinHandle` results
   - print the elapsed time and results

Notice how User 2 (100ms) finishes first, then User 3 (200ms), then User 1 (300ms) — total runtime is ~300ms, not 600ms!

*Hint: `tokio::spawn` moves the future to Tokio's background thread pool. It returns a `JoinHandle<T>` which yields `Result<T, JoinError>` when `.await`ed.*

---

## Q24 — Axum REST API (Router, Handlers, JSON)

Axum is Rust's most popular web framework (built on Tokio and Tower).

Write an HTTP web server in `q24.rs`:

```rust
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
struct User {
    id: u64,
    name: String,
    role: String,
}
```

Routes to implement:
- `GET /` → returns plain text `"Welcome to Rust Axum Server!"`
- `GET /users/:id` → extracts the path parameter `:id` using `Path(id): Path<u64>` and returns `Json(User { id, name: "Alice".into(), role: "Admin".into() })`
- `POST /users` → receives `Json<User>` payload, prints `"Created user: {user:?}"`, and returns `(StatusCode::CREATED, Json(user))`

In `main`:
- build the `Router::new()`
- bind to `127.0.0.1:3000` with `tokio::net::TcpListener::bind("127.0.0.1:3000").await`
- start the server with `axum::serve(listener, app).await`

*Hint: Axum handlers are plain async functions. Parameters like `Path(id)` and `Json(payload)` are called **Extractors** — Axum automatically parses request data based on their types!*

---

## Q25 — Axum In-Memory CRUD with Shared State (`Arc<Mutex<HashMap>>`)

Web servers handle multiple requests at the same time on different threads. To safely share mutable data (like an in-memory database) across handlers, Rust uses `Arc<Mutex<T>>`.

Build a complete In-Memory **Todo CRUD API**:

```rust
#[derive(Serialize, Deserialize, Clone, Debug)]
struct Todo {
    id: u64,
    title: String,
    completed: bool,
}

#[derive(Deserialize)]
struct CreateTodo {
    title: String,
}

// Shared thread-safe in-memory database
type AppState = Arc<Mutex<HashMap<u64, Todo>>>;
```

Endpoints to implement:
- `GET /todos` → returns `Json<Vec<Todo>>` of all current todos
- `POST /todos` → accepts `Json<CreateTodo>`, generates the next `id`, inserts into the `HashMap`, and returns `(StatusCode::CREATED, Json(new_todo))`
- `DELETE /todos/:id` → removes the item with `:id` from the `HashMap`. Returns `StatusCode::NO_CONTENT` (204) if found, or `StatusCode::NOT_FOUND` (404) if missing.

In `main`:
- create state: `let state: AppState = Arc::new(Mutex::new(HashMap::new()));`
- attach state to router: `Router::new().route(...).with_state(state)`
- run server on port `3000`

*Hint: `use std::sync::{Arc, Mutex};` and `axum::extract::State`. Inside handlers, use `let mut db = state.lock().unwrap();` to access the `HashMap`.*

---

## Progress

- [x] Q1 for loop
- [x] Q2 functions
- [x] Q3 borrow vs move
- [x] Q4 String vs &str
- [x] Q5 struct + methods
- [x] Q6 &mut self
- [x] Q7 enum + match
- [x] Q8 Option
- [x] Q9 Result + ?
- [x] Q10 trait
- [x] Q11 default method
- [x] Q12 dyn trait
- [x] Q13 generic function
- [x] Q14 generic struct
- [x] Q15 lifetimes
- [x] Q16 iterators
- [x] Q17 HashMap
- [x] Q18 modules across files
- [x] Q19 retry
- [x] Q20 Two Sum
- [x] Q21 Valid Anagram
- [x] Q22 Number of Islands
- [ ] Q23 Tokio async & spawn
- [ ] Q24 Axum router & JSON API
- [ ] Q25 Axum shared state CRUD

---

**When you get stuck:** paste the compiler error and ask what it means. Ask
for a hint, not the answer. The error messages are long because the fix is
usually inside them.

Here is exactly how **`Arc`** works under the hood in the Rust standard library.

---

### 1. The Internal Structs (How it is defined in Rust's source code)

In the standard library, `Arc` does not put your data alone on the heap. It wraps it inside an internal struct called **`ArcInner<T>`**:

```rust
// 1. What actually lives on the HEAP:
struct ArcInner<T> {
    strong_count: AtomicUsize, // Atomic integer: number of active Arcs (e.g. 1, 2, 3...)
    weak_count:   AtomicUsize, // Number of Weak references
    data:         T,           // The actual value (e.g. String "hello")
}

// 2. What lives on your STACK:
pub struct Arc<T> {
    ptr: *const ArcInner<T>,   // Just an 8-byte memory pointer!
}
```

---

### 2. Step-by-Step Walkthrough

#### Step 1: When you call `let arc1 = Arc::new(String::from("hello"));`

1. Rust allocates memory for `ArcInner` on the **heap**.
2. It sets `strong_count = 1`.
3. It places your `String` data inside `data`.
4. It puts a pointer (`ptr`) inside `arc1` on the **stack**.

```
STACK                                  HEAP (Memory Address: 0x1000)
┌─────────────┐                        ┌─────────────────────────────────┐
│ arc1        │                        │ ArcInner<String>                │
│ ptr: 0x1000 ├──────────────────────► │   strong_count: 1               │
└─────────────┘                        │   weak_count:   1               │
                                       │   data:         "hello"         │
                                       └─────────────────────────────────┘
```

---

#### Step 2: When you call `let arc2 = Arc::clone(&arc1);`

Here is the **actual simplified implementation** of `Arc::clone`:

```rust
impl<T> Clone for Arc<T> {
    fn clone(&self) -> Arc<T> {
        // 1. Atomically increment the strong counter by 1
        self.inner().strong_count.fetch_add(1, Ordering::Relaxed);

        // 2. Just copy the 8-byte pointer address!
        Arc { ptr: self.ptr }
    }
}
```

What actually happens:
1. It looks at the heap block at `0x1000`.
2. It runs an atomic CPU instruction: `strong_count += 1` (now **`2`**).
3. It creates `arc2` on the stack with the **exact same pointer** (`0x1000`).
4. **No string bytes are copied!** Only 8 bytes (the pointer) are copied.

```
STACK                                  HEAP (Memory Address: 0x1000)
┌─────────────┐                        ┌─────────────────────────────────┐
│ arc1        │                        │ ArcInner<String>                │
│ ptr: 0x1000 ├──────────┐             │   strong_count: 2  <-- (+1)     │
└─────────────┘          │             │   weak_count:   1               │
                         ├───────────► │   data:         "hello" (NO COPY)│
┌─────────────┐          │             └─────────────────────────────────┘
│ arc2        │          │
│ ptr: 0x1000 ├──────────┘
└─────────────┘
```

---

#### Step 3: When `arc1` goes out of scope (Dropped)

Here is the simplified `Drop` implementation:

```rust
impl<T> Drop for Arc<T> {
    fn drop(&mut self) {
        // 1. Atomically decrement the strong counter by 1
        if self.inner().strong_count.fetch_sub(1, Ordering::Release) == 1 {
            // 2. If count REACHED 0 -> No one is using it anymore!
            drop_data_and_free_heap(self.ptr);
        }
    }
}
```

* When `arc1` drops:
  * `strong_count` becomes **`1`** (2 - 1 = 1).
  * Since `count != 0`, the `"hello"` data **stays alive** for `arc2`.
* Later, when `arc2` drops:
  * `strong_count` becomes **`0`** (1 - 1 = 0).
  * Since `count == 0`, Rust runs the destructor for `String` and **frees the heap memory**.

---

### 3. Why is it called **`Arc`** (Atomic Reference Counted)?

The counter uses **`AtomicUsize`**, not a regular integer `usize`.

* **Regular integer increment (`count += 1`)**: In multithreading, if 2 threads increment at the exact same millisecond, they can overwrite each other (race condition).
* **Atomic increment (`fetch_add`)**: Uses hardware-level CPU instructions (like `LOCK XADD` on x86 processors). The CPU locks the memory bus for a fraction of a nanosecond to guarantee the counter is 100% accurate across all CPU cores.

---

### Summary
* `Arc::new`: Allocates `{ counter: 1, data }` on heap.
* `Arc::clone`: **Counter + 1** and copies an 8-byte pointer (takes ~10 nanoseconds).
* `Arc::drop`: **Counter - 1**. If 0, frees memory.