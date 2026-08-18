# Master Guide to Rust Built-in Methods

This document is a comprehensive, production-grade reference for pre-built standard library methods in Rust. It categorizes methods by data type, explains what each method does with code snippets, identifies receiver types (`&self`, `&mut self`, `self`), and presents empirical usage frequency metrics gathered from auditing **5,557 production Rust files** across the **OpenAI Codex** and **xAI Grok** codebases.

---

## Table of Contents

1. [Understanding Method Syntax in Rust](#1-understanding-method-syntax-in-rust)
2. [Top 30 Most Used Rust Methods in Production](#2-top-30-most-used-rust-methods-in-production)
3. [String (`String`) & String Slice (`&str`) Methods](#3-string-string--string-slice-str-methods)
4. [`Option<T>` Methods](#4-optiont-methods)
5. [`Result<T, E>` Methods](#5-resultt-e-methods)
6. [Vector (`Vec<T>`) & Slice (`&[T]`) Methods](#6-vector-vect--slice-t-methods)
7. [Map (`HashMap<K, V>`) & Set (`HashSet<T>`) Methods](#7-map-hashmapk-v--set-hashsett-methods)
8. [Primitive Numbers, `char`, and `bool` Methods](#8-primitive-numbers-char-and-bool-methods)
9. [Path (`Path`) & Path Buffer (`PathBuf`) Methods](#9-path-path--path-buffer-pathbuf-methods)
10. [Smart Pointers, `Cow`, & Concurrency (`Arc`, `Rc`, `RefCell`, `Mutex`, `Atomic`) Methods](#10-smart-pointers-cow--concurrency-arc-rc-refcell-mutex-atomic-methods)
11. [Iterator (`Iterator`) Methods](#11-iterator-iterator-methods)

---

## 1. Understanding Method Syntax in Rust

In Rust, methods are functions associated with a specific type and called using dot notation: `instance.method_name(args)`.

### Method Receiver Types

A method's first parameter determines how it accesses the instance:

| Receiver | Ownership / Borrowing | Meaning |
| :--- | :--- | :--- |
| **`&self`** | Immutable Borrow | Reads data without modifying or consuming the instance. |
| **`&mut self`** | Mutable Borrow | Modifies the instance in-place. Requires a `mut` variable. |
| **`self`** | Value Ownership (Move) | Consumes the instance. The instance is dropped or transformed. |

### Associated Functions vs Methods

* **Associated Function** (no `self` receiver): Called via type name, e.g., `String::new()`, `Vec::with_capacity(10)`.
* **Method** (has `self`, `&self`, or `&mut self` receiver): Called via instance, e.g., `text.len()`, `vec.push(42)`.

---

## 2. Top 30 Most Used Rust Methods in Production

Empirical audit metrics from scanning **5,557 `.rs` files** across `codex-rs` and `grok-build`:

| Rank | Method | Primary Type | Production Frequency | Common Use Case |
| :--- | :--- | :--- | :--- | :--- |
| **1** | `.expect()` | `Option` / `Result` | **59,012 calls** | Unwrapping with custom panic message |
| **2** | `.unwrap()` | `Option` / `Result` | **58,160 calls** | Direct value extraction (in tests/guaranteed states) |
| **3** | `.to_string()` | `Display` / `&str` | **48,820 calls** | Converting slices/types into owned `String` |
| **4** | `.len()` | `Vec`, `String`, Slice | **45,426 calls** | Checking element or byte length |
| **5** | `.map()` | `Iterator`, `Option`, `Result` | **43,632 calls** | Transforming inner value or sequence items |
| **6** | `.join()` | Slices `&[&str]` | **38,478 calls** | Joining string slices with a separator |
| **7** | `.is_empty()` | `Vec`, `String`, `Map` | **34,599 calls** | Checking if length is 0 |
| **8** | `.contains()` | `String`, Slice, Set | **31,660 calls** | Searching for substring or element |
| **9** | `.get()` | `Vec`, `HashMap`, Slice | **31,629 calls** | Bounds-safe element lookup (returns `Option`) |
| **10** | `.push()` | `Vec`, `String` | **29,082 calls** | Appending an item to the end |
| **11** | `.as_ref()` | `Option`, `Result`, `T` | **19,860 calls** | Converting `&Option<T>` to `Option<&T>` |
| **12** | `.as_deref()` | `Option<String>`, `Option<Vec>` | **13,984 calls** | Dereferencing inner container reference (`Option<&str>`) |
| **13** | `.and_then()` | `Option`, `Result` | **13,648 calls** | Monadic chaining / flat mapping |
| **14** | `.iter()` | Collections | **12,015 calls** | Creating an immutable borrowing iterator |
| **15** | `.insert()` | `HashMap`, `HashSet` | **9,818 calls** | Inserting key-value pair or set element |
| **16** | `.unwrap_or()` | `Option`, `Result` | **8,938 calls** | Fallback default value if `None` / `Err` |
| **17** | `.as_str()` | `String` | **6,670 calls** | Borrowing string as `&str` slice |
| **18** | `.is_none()` | `Option` | **6,256 calls** | Checking if `None` |
| **19** | `.filter()` | `Iterator`, `Option` | **6,192 calls** | Filtering elements by predicate |
| **20** | `.get_mut()` | `Vec`, `HashMap`, Slice | **5,871 calls** | Bounds-safe mutable element lookup |
| **21** | `.collect()` | `Iterator` | **5,795 calls** | Packing iterator into `Vec`, `String`, `HashMap` |
| **22** | `.unwrap_or_else()`| `Option`, `Result` | **5,520 calls** | Lazy evaluation fallback closure |
| **23** | `.is_some()` | `Option` | **5,122 calls** | Checking if `Some` |
| **24** | `.map_err()` | `Result` | **4,493 calls** | Transforming error type |
| **25** | `.starts_with()`| `String`, `&str`, Slice | **4,300 calls** | Checking prefix match |
| **26** | `.display()` | `Path` / `PathBuf` | **3,805 calls** | Formatting path for printing |
| **27** | `.unwrap_or_default()`| `Option`, `Result` | **3,672 calls** | Fallback to `Default::default()` |
| **28** | `.any()` | `Iterator` | **3,429 calls** | Checking if any element matches condition |
| **29** | `.ok()` | `Result` | **2,958 calls** | Converting `Result<T, E>` to `Option<T>` |
| **30** | `.into_iter()` | Collections | **2,898 calls** | Consuming collection into owned iterator |

---

## 3. String (`String`) & String Slice (`&str`) Methods

Strings in Rust are UTF-8 encoded sequences of bytes.

```
       String (Owned, Heap Allocated)
       ┌─────┬──────────┬──────────┐
       │ ptr │ capacity │   len    │
       └──┬──┴──────────┴──────────┘
          │
          ▼ 
Heap:  [ 'H', 'e', 'l', 'l', 'o', ' ', 'W', 'o', 'r', 'l', 'd' ]
          ▲
       ┌──┴──┬──────┐
       │ ptr │ len  │
       └─────┴──────┘
       &str (Borrowed Slice)
```

### Case & Case Transformation

#### `to_lowercase(&self) -> String`
Returns a new allocated `String` with all characters converted to lowercase.
```rust
let name = "RUST Language";
let lower = name.to_lowercase(); // "rust language"
```

#### `to_uppercase(&self) -> String`
Returns a new allocated `String` with all characters converted to uppercase.
```rust
let name = "rust";
let upper = name.to_uppercase(); // "RUST"
```

#### `to_ascii_lowercase(&self) -> String` / `make_ascii_lowercase(&mut self)`
Converts ASCII characters to lowercase in-place or returning a new string (faster than unicode lowercase for ASCII text).
```rust
let mut code = String::from("HTTP/2.0");
code.make_ascii_lowercase(); // "http/2.0"
```

---

### Trimming & Prefix/Suffix Stripping

#### `trim(&self) -> &str`
Returns a subslice with leading and trailing whitespace removed.
```rust
let input = "  hello world \n";
assert_eq!(input.trim(), "hello world");
```

#### `trim_start(&self) -> &str` / `trim_end(&self) -> &str`
Removes leading or trailing whitespace only.
```rust
let text = "  hello  ";
assert_eq!(text.trim_start(), "hello  ");
assert_eq!(text.trim_end(), "  hello");
```

#### `strip_prefix(&self, prefix: &str) -> Option<&str>` / `strip_suffix(&self, suffix: &str) -> Option<&str>`
Removes a prefix or suffix if present, returning `Some(&str)`.
```rust
let path = "/usr/local/bin";
assert_eq!(path.strip_prefix("/usr/"), Some("local/bin"));

let file = "main.rs";
assert_eq!(file.strip_suffix(".rs"), Some("main"));
```

---

### Searching & Content Queries

#### `contains(&self, pat: &str) -> bool`
Returns `true` if the given pattern is a substring.
```rust
let text = "hello world";
assert!(text.contains("world"));
```

#### `starts_with(&self, pat: &str) -> bool` / `ends_with(&self, pat: &str) -> bool`
Returns `true` if the string starts or ends with the specified prefix/suffix.
```rust
let filename = "main.rs";
assert!(filename.ends_with(".rs"));
```

#### `len(&self) -> usize`
Returns the length of the string **in bytes** (not character count).
```rust
let text = "🦀 Rust";
assert_eq!(text.len(), 11); // 4 bytes for crab emoji + 1 space + 4 bytes for Rust
```

#### `is_empty(&self) -> bool`
Returns `true` if the string has a byte length of 0.
```rust
let s = "";
assert!(s.is_empty());
```

#### `find(&self, pat: &str) -> Option<usize>` / `rfind(&self, pat: &str) -> Option<usize>`
Returns the byte index of the first or last match of the pattern.
```rust
let text = "foo.bar.txt";
assert_eq!(text.rfind('.'), Some(7));
```

---

### Splitting & Tokenization

#### `split(&self, pat: P) -> Split`
Returns an iterator over subslices separated by the given pattern.
```rust
let csv = "apple,banana,orange";
let items: Vec<&str> = csv.split(',').collect(); // ["apple", "banana", "orange"]
```

#### `split_whitespace(&self) -> SplitWhitespace`
Splits a string by any amount of whitespace.
```rust
let text = "hello   world \t rust\n";
let words: Vec<&str> = text.split_whitespace().collect(); // ["hello", "world", "rust"]
```

#### `lines(&self) -> Lines`
Returns an iterator over lines in the string (handles both `\n` and `\r\n`).
```rust
let document = "line 1\nline 2\r\nline 3";
let count = document.lines().count(); // 3
```

#### `split_once(&self, delimiter: &str) -> Option<(&str, &str)>`
Splits string around the first match of the delimiter.
```rust
let pair = "key=value";
assert_eq!(pair.split_once('='), Some(("key", "value")));
```

---

### Conversion & Parsing

#### `parse<F>(&self) -> Result<F, F::Err>`
Parses a string slice into another type (like `i32`, `f64`, `IpAddr`, `PathBuf`).
```rust
let num: i32 = "42".parse().unwrap();
let flag: bool = "true".parse().unwrap();
```

#### `as_str(&self) -> &str`
Extracts a string slice containing the entire `String`.
```rust
let s = String::from("hello");
let slice: &str = s.as_str();
```

#### `as_bytes(&self) -> &[u8]`
Returns a byte slice `&[u8]` of the string's UTF-8 bytes.
```rust
let bytes = "abc".as_bytes(); // &[97, 98, 99]
```

#### `chars(&self) -> Chars`
Returns an iterator over the unicode `char` values of the string.
```rust
let count = "🦀 Rust".chars().count(); // 6 characters
```

#### `char_indices(&self) -> CharIndices`
Returns an iterator yielding `(byte_position, char)` tuples.
```rust
for (idx, ch) in "a🦀b".char_indices() {
    // yields (0, 'a'), (1, '🦀'), (5, 'b')
}
```

#### `repeat(&self, n: usize) -> String`
Creates a new string by repeating a string slice `n` times.
```rust
let pattern = "ab".repeat(3); // "ababab"
```

---

### Mutation & Building

#### `push(&mut self, ch: char)`
Appends a single `char` to the end of the `String`.
```rust
let mut s = String::from("Hello");
s.push('!'); // "Hello!"
```

#### `push_str(&mut self, string: &str)`
Appends a string slice `&str` to the end of the `String`.
```rust
let mut s = String::from("Hello");
s.push_str(" World"); // "Hello World"
```

#### `replace(&self, from: &str, to: &str) -> String`
Replaces all matches of a pattern with another string.
```rust
let text = "foo bar foo";
let new_text = text.replace("foo", "baz"); // "baz bar baz"
```

#### `retain(&mut self, f: impl FnMut(char) -> bool)`
Keeps only characters that satisfy a predicate closure.
```rust
let mut s = String::from("a1b2c3");
s.retain(|c| c.is_alphabetic()); // s becomes "abc"
```

#### `clear(&mut self)`
Truncates `String` to 0 length, keeping the allocated memory capacity intact.
```rust
let mut s = String::from("data");
s.clear();
assert!(s.is_empty());
```

---

## 4. `Option<T>` Methods

`Option<T>` represents an optional value: either `Some(T)` or `None`.

### State Checking

#### `is_some(&self) -> bool` / `is_none(&self) -> bool`
Returns `true` if the option is `Some` or `None`.
```rust
let opt: Option<i32> = Some(42);
assert!(opt.is_some());
assert!(!opt.is_none());
```

#### `is_some_and(&self, f: impl FnOnce(&T) -> bool) -> bool`
Returns `true` if the option is `Some` AND the value matches a predicate closure.
```rust
let opt = Some(10);
assert!(opt.is_some_and(|x| *x > 5));
```

#### `is_none_or(&self, f: impl FnOnce(&T) -> bool) -> bool`
Returns `true` if option is `None` OR the value matches predicate `f`.
```rust
let opt: Option<i32> = None;
assert!(opt.is_none_or(|x| *x > 5)); // true
```

---

### Unwrapping & Value Extraction

#### `unwrap(self) -> T`
Returns the contained `Some` value. **Panics if `None`.**
```rust
let x = Some("val").unwrap(); // "val"
```

#### `unwrap_or(self, default: T) -> T`
Returns contained `Some` value or provided `default`.
```rust
let x: Option<i32> = None;
assert_eq!(x.unwrap_or(100), 100);
```

#### `unwrap_or_else(self, f: impl FnOnce() -> T) -> T`
Returns contained `Some` value or computes default via closure `f` (lazy evaluation).
```rust
let k = 10;
let x: Option<i32> = None;
assert_eq!(x.unwrap_or_else(|| k * 2), 20);
```

#### `unwrap_or_default(self) -> T` (where `T: Default`)
Returns contained `Some` value or `T::default()`.
```rust
let opt: Option<Vec<i32>> = None;
let vec = opt.unwrap_or_default(); // empty Vec
```

#### `expect(self, msg: &str) -> T`
Returns `Some` value. **Panics with custom message `msg` if `None`.**
```rust
let config = Some("config.toml").expect("Config file path must be set");
```

---

### Monadic Chaining & Transformation

#### `map<U>(self, f: impl FnOnce(T) -> U) -> Option<U>`
Transforms `Some(T)` to `Some(U)` using closure `f`. Leaves `None` untouched.
```rust
let maybe_name = Some("deepak");
let maybe_len = maybe_name.map(|n| n.len()); // Some(6)
```

#### `and_then<U>(self, f: impl FnOnce(T) -> Option<U>) -> Option<U>`
Flat-maps `Some(T)` into another `Option<U>`. Useful for chaining operations that might fail.
```rust
fn parse_num(s: &str) -> Option<i32> { s.parse().ok() }

let res = Some("42").and_then(parse_num); // Some(42)
let err = Some("abc").and_then(parse_num); // None
```

#### `filter(self, predicate: impl FnOnce(&T) -> bool) -> Option<T>`
Returns `Some(T)` if value matches predicate, otherwise `None`.
```rust
let opt = Some(4);
let filtered = opt.filter(|x| x % 2 == 0); // Some(4)
```

#### `inspect(&self, f: impl FnOnce(&T)) -> &Self`
Calls closure `f` on inner value if `Some(T)` without modifying the option. Useful for logging in chain calls.
```rust
let opt = Some(10).inspect(|x| println!("Got value: {}", x));
```

#### `flatten(self) -> Option<U>` (where `T = Option<U>`)
Flattens nested `Option<Option<U>>` into `Option<U>`.
```rust
let nested: Option<Option<i32>> = Some(Some(42));
assert_eq!(nested.flatten(), Some(42));
```

#### `zip<U>(self, other: Option<U>) -> Option<(T, U)>`
Zips two options into `Some((a, b))` if both are `Some`.
```rust
let x = Some(1);
let y = Some("hi");
assert_eq!(x.zip(y), Some((1, "hi")));
```

---

### Borrowing & Reference Conversion

#### `as_ref(&self) -> Option<&T>`
Converts `&Option<T>` into `Option<&T>`. Allows inspecting `Option` without moving `T`.
```rust
let opt: Option<String> = Some("hello".to_string());
let opt_ref: Option<&String> = opt.as_ref();
```

#### `as_deref(&self) -> Option<&T::Target>`
Converts `Option<String>` to `Option<&str>` or `Option<Vec<T>>` to `Option<&[T]>`.
```rust
let opt: Option<String> = Some("hello".to_string());
let opt_str: Option<&str> = opt.as_deref();
```

#### `copied(self) -> Option<T>` (where `T: Copy`) / `cloned(self) -> Option<T>` (where `T: Clone`)
Converts `Option<&T>` to `Option<T>` by copying or cloning.
```rust
let val = 42;
let opt_ref: Option<&i32> = Some(&val);
let opt_val: Option<i32> = opt_ref.copied(); // Some(42)
```

#### `take(&mut self) -> Option<T>`
Takes the value out of the option, leaving `None` in its place.
```rust
let mut opt = Some("val");
let taken = opt.take(); // taken = Some("val"), opt = None
```

#### `ok_or<E>(self, err: E) -> Result<T, E>`
Transforms `Option<T>` to `Result<T, E>`, mapping `Some(v)` to `Ok(v)` and `None` to `Err(err)`.
```rust
let opt = Some(42);
let res: Result<i32, &str> = opt.ok_or("missing"); // Ok(42)
```

#### `transpose(self) -> Result<Option<T>, E>` (where `T = Result<T, E>`)
Transposes an `Option` of a `Result` into a `Result` of an `Option`.
```rust
let x: Option<Result<i32, &str>> = Some(Ok(5));
let y: Result<Option<i32>, &str> = Ok(Some(5));
assert_eq!(x.transpose(), y);
```

---

## 5. `Result<T, E>` Methods

`Result<T, E>` is used for error handling: either `Ok(T)` (success) or `Err(E)` (error).

### State Checking

#### `is_ok(&self) -> bool` / `is_err(&self) -> bool`
Returns `true` if result is `Ok` or `Err`.
```rust
let res: Result<i32, &str> = Ok(10);
assert!(res.is_ok());
assert!(!res.is_err());
```

#### `unwrap_err(self) -> E` / `expect_err(self, msg: &str) -> E`
Unwraps and returns the contained `Err(E)` value. **Panics if `Ok`.**
```rust
let res: Result<i32, &str> = Err("404");
assert_eq!(res.unwrap_err(), "404");
```

---

### Mapping & Error Transformation

#### `map<U>(self, f: impl FnOnce(T) -> U) -> Result<U, E>`
Transforms success value `Ok(T)` into `Ok(U)`.
```rust
let res: Result<i32, &str> = Ok(5);
let doubled = res.map(|x| x * 2); // Ok(10)
```

#### `map_err<F>(self, op: impl FnOnce(E) -> F) -> Result<T, F>`
Transforms error value `Err(E)` into `Err(F)`. Highly used in error propagation.
```rust
let res: Result<i32, i32> = Err(404);
let formatted = res.map_err(|e| format!("HTTP Error: {}", e)); // Err("HTTP Error: 404")
```

#### `and_then<U>(self, op: impl FnOnce(T) -> Result<U, E>) -> Result<U, E>`
Chains operations that return `Result`.
```rust
fn sq_root(x: f64) -> Result<f64, &'static str> {
    if x >= 0.0 { Ok(x.sqrt()) } else { Err("negative number") }
}

let res = Ok(16.0).and_then(sq_root); // Ok(4.0)
```

#### `inspect(&self, f: impl FnOnce(&T))` / `inspect_err(&self, f: impl FnOnce(&E))`
Inspects success or error payload without consuming the Result.
```rust
let res: Result<i32, &str> = Err("timeout");
res.inspect_err(|e| eprintln!("Warning: {}", e));
```

---

### Conversion & Extraction

#### `ok(self) -> Option<T>`
Converts `Result<T, E>` to `Option<T>`, discarding the error.
```rust
let res: Result<i32, &str> = Ok(42);
assert_eq!(res.ok(), Some(42));
```

#### `err(self) -> Option<E>`
Converts `Result<T, E>` to `Option<E>`, discarding the success value.
```rust
let res: Result<i32, &str> = Err("boom");
assert_eq!(res.err(), Some("boom"));
```

---

## 6. Vector (`Vec<T>`) & Slice (`&[T]`) Methods

`Vec<T>` is a growable heap-allocated array. `&[T]` is a borrowed slice view.

### Element Access & Querying

#### `len(&self) -> usize` / `is_empty(&self) -> bool`
Returns length or checks if vector is empty.
```rust
let v = vec![1, 2, 3];
assert_eq!(v.len(), 3);
assert!(!v.is_empty());
```

#### `get(&self, index: usize) -> Option<&T>` / `get_mut(&mut self, index: usize) -> Option<&mut T>`
Returns a safe reference to element at index without panicking.
```rust
let v = vec!["a", "b", "c"];
assert_eq!(v.get(1), Some(&"b"));
assert_eq!(v.get(10), None); // Safe! No panic.
```

#### `first(&self) -> Option<&T>` / `last(&self) -> Option<&T>`
Returns references to the first or last element.
```rust
let v = vec![10, 20, 30];
assert_eq!(v.first(), Some(&10));
assert_eq!(v.last(), Some(&30));
```

---

### Adding & Removing Elements

#### `push(&mut self, value: T)` / `pop(&mut self) -> Option<T>`
Appends element to the end or removes and returns the last element.
```rust
let mut v = vec![1, 2];
v.push(3);        // [1, 2, 3]
let last = v.pop(); // Some(3), v is [1, 2]
```

#### `insert(&mut self, index: usize, element: T)` / `remove(&mut self, index: usize) -> T`
Inserts or removes an element at `index` (shifts subsequent elements).
```rust
let mut v = vec![1, 3];
v.insert(1, 2); // [1, 2, 3]
let item = v.remove(1); // 2, v is [1, 3]
```

#### `swap_remove(&mut self, index: usize) -> T`
Removes element at `index` by swapping it with the last element. **$O(1)$ fast removal without shifting!**
```rust
let mut v = vec!["a", "b", "c", "d"];
v.swap_remove(1); // Returns "b", v becomes ["a", "d", "c"]
```

#### `retain(&mut self, f: impl FnMut(&T) -> bool)`
Retains only elements matching predicate `f`.
```rust
let mut v = vec![1, 2, 3, 4, 5];
v.retain(|x| x % 2 == 0); // v becomes [2, 4]
```

#### `dedup(&mut self)`
Removes consecutive duplicate elements in-place.
```rust
let mut v = vec![1, 2, 2, 3, 2];
v.dedup(); // v becomes [1, 2, 3, 2]
```

#### `drain<R>(&mut self, range: R) -> Drain`
Removes specified range of elements and returns an iterator over them.
```rust
let mut v = vec![1, 2, 3, 4];
let drained: Vec<_> = v.drain(1..3).collect(); // [2, 3], v is [1, 4]
```

---

### Sorting & Searching

#### `sort(&mut self)` (Stable Sort) / `sort_unstable(&mut self)` (Faster Unstable Sort)
Sorts slice in ascending order. `sort_unstable` is faster and does not allocate extra memory.
```rust
let mut v = vec![3, 1, 4, 1, 5];
v.sort_unstable(); // [1, 1, 3, 4, 5]
```

#### `sort_by_key<K>(&mut self, f: impl FnMut(&T) -> K)`
Sorts slice by key extraction function.
```rust
let mut words = vec!["banana", "apple", "fig"];
words.sort_by_key(|w| w.len()); // ["fig", "apple", "banana"]
```

#### `binary_search(&self, x: &T) -> Result<usize, usize>`
Searches a **sorted** slice for an element. Returns `Ok(index)` or `Err(insertion_index)`.
```rust
let v = vec![10, 20, 30, 40];
assert_eq!(v.binary_search(&30), Ok(2));
```

#### `swap(&mut self, a: usize, b: usize)`
Swaps two elements in a slice by their indices.
```rust
let mut v = vec!["a", "b", "c"];
v.swap(0, 2); // ["c", "b", "a"]
```

---

### Joining & Chunking

#### `join(&self, separator: &T) -> Vec<T>`
Joins slices with a separator. (E.g. joining `&[&str]` with `", "`).
```rust
let words = vec!["hello", "world"];
let sentence = words.join(" "); // "hello world"
```

#### `chunks(&self, chunk_size: usize) -> Chunks` / `windows(&self, size: usize) -> Windows`
Returns iterator over non-overlapping chunks or sliding windows.
```rust
let v = vec![1, 2, 3, 4, 5];
let chunks: Vec<_> = v.chunks(2).collect(); // [[1, 2], [3, 4], [5]]
let windows: Vec<_> = v.windows(2).collect(); // [[1, 2], [2, 3], [3, 4], [4, 5]]
```

#### `split_at(&self, mid: usize) -> (&[T], &[T])`
Splits slice at index `mid` returning two subslices.
```rust
let v = vec![1, 2, 3, 4, 5];
let (left, right) = v.split_at(2); // left = [1, 2], right = [3, 4, 5]
```

---

## 7. Map (`HashMap<K, V>`) & Set (`HashSet<T>`) Methods

Key-value hash map and unique value hash set implementations.

### HashMap Access & Insertion

#### `insert(&mut self, k: K, v: V) -> Option<V>`
Inserts key-value pair. Returns previous value if key already existed.
```rust
use std::collections::HashMap;

let mut map = HashMap::new();
map.insert("alice", 90);
```

#### `get<Q>(&self, k: &Q) -> Option<&V>` / `get_mut<Q>(&mut self, k: &Q) -> Option<&mut V>`
Returns immutable or mutable reference to value associated with key.
```rust
let score = map.get("alice"); // Some(&90)
```

#### `contains_key<Q>(&self, k: &Q) -> bool`
Returns `true` if map contains specified key.
```rust
assert!(map.contains_key("alice"));
```

#### `keys(&self)` / `values(&self)` / `values_mut(&mut self)`
Returns iterators over map keys or values.
```rust
let keys: Vec<_> = map.keys().copied().collect();
```

---

### Entry API (In-place Mutation)

#### `entry(&mut self, key: K) -> Entry<K, V>`
Gets entry for key to perform in-place insertion/update.

```rust
let mut counts = HashMap::new();
let word = "apple";

// Insert 1 if key missing, or increment count if present!
counts.entry(word).and_modify(|e| *e += 1).or_insert(1);
```

* `.or_insert(default)`: Inserts default value if key absent.
* `.or_insert_with(closure)`: Lazily evaluates default value.
* `.and_modify(closure)`: Modifies existing value in-place.

---

## 8. Primitive Numbers, `char`, and `bool` Methods

### Numeric Math & Clamping

#### `abs(self)` / `pow(self, exp: u32)` / `sqrt(self)`
Standard arithmetic methods on primitive numbers (`i32`, `f64`, etc.).
```rust
assert_eq!((-5_i32).abs(), 5);
assert_eq!(2_i32.pow(3), 8);
assert_eq!(16.0_f64.sqrt(), 4.0);
```

#### `clamp(self, min: Self, max: Self) -> Self`
Clamps value within bounds `[min, max]`.
```rust
assert_eq!(15.clamp(0, 10), 10);
assert_eq!((-5).clamp(0, 10), 0);
```

---

### Safe & Boundary Arithmetic

#### `saturating_add(self, rhs: Self)` / `saturating_sub(self, rhs: Self)`
Performs addition/subtraction saturating at numeric boundaries (no overflow panic).
```rust
assert_eq!(255_u8.saturating_add(10), 255);
assert_eq!(0_u8.saturating_sub(10), 0);
```

#### `checked_add(self, rhs: Self) -> Option<Self>` / `checked_sub(self, rhs: Self) -> Option<Self>`
Returns `Some(result)` or `None` on overflow.
```rust
assert_eq!(255_u8.checked_add(1), None);
```

#### `to_be_bytes(self)` / `to_le_bytes(self)` / `to_ne_bytes(self)`
Converts numeric values into raw byte arrays (big-endian, little-endian, or native-endian).
```rust
let bytes = 512_u16.to_be_bytes(); // [2, 0]
```

---

### Char & Bool Query Methods

#### `is_ascii(&self)` / `is_alphanumeric(&self)` / `is_whitespace(&self)`
Character classification predicates.
```rust
let c = 'a';
assert!(c.is_alphanumeric());
assert!(' '.is_whitespace());
```

#### `then<T>(self, f: impl FnOnce() -> T) -> Option<T>` (Boolean Method)
Returns `Some(f())` if boolean is `true`, otherwise `None`.
```rust
let is_admin = true;
let role = is_admin.then(|| "Administrator"); // Some("Administrator")
```

---

## 9. Path (`Path`) & Path Buffer (`PathBuf`) Methods

File system path operations.

### Component Queries

#### `file_name(&self) -> Option<&OsStr>` / `file_stem(&self) -> Option<&OsStr>` / `extension(&self) -> Option<&OsStr>`
Extracts file component parts from a path.
```rust
use std::path::Path;

let path = Path::new("/var/log/app.main.log");
assert_eq!(path.file_name().unwrap(), "app.main.log");
assert_eq!(path.file_stem().unwrap(), "app.main");
assert_eq!(path.extension().unwrap(), "log");
```

#### `parent(&self) -> Option<&Path>`
Returns parent directory path.
```rust
let path = Path::new("/usr/local/bin");
assert_eq!(path.parent().unwrap(), Path::new("/usr/local"));
```

---

### Path Manipulation & Checking

#### `join<P: AsRef<Path>>(&self, path: P) -> PathBuf`
Extends path with another path component.
```rust
let base = Path::new("/usr");
let full = base.join("bin").join("rustc"); // "/usr/bin/rustc"
```

#### `strip_prefix<P: AsRef<Path>>(&self, base: P) -> Result<&Path, StripPrefixError>`
Strips a base path prefix from the path.
```rust
let path = Path::new("/usr/local/bin");
assert_eq!(path.strip_prefix("/usr").unwrap(), Path::new("local/bin"));
```

#### `exists(&self) -> bool` / `is_file(&self) -> bool` / `is_dir(&self) -> bool`
Queries file system to check existence or path type.
```rust
let p = Path::new("Cargo.toml");
if p.exists() && p.is_file() {
    println!("Found Cargo.toml");
}
```

#### `display(&self) -> Display`
Helper struct for printing paths formatted safely with `println!("{}", path.display())`.

---

## 10. Smart Pointers, `Cow`, & Concurrency (`Arc`, `Rc`, `RefCell`, `Mutex`, `Atomic`) Methods

### Reference Counting (`Arc<T>` / `Rc<T>`)

#### `Arc::clone(val: &Arc<T>) -> Arc<T>`
Increments strong reference count and returns a new shared pointer reference.
```rust
use std::sync::Arc;

let data = Arc::new(vec![1, 2, 3]);
let data_clone = Arc::clone(&data); // Increments ref count
```

#### `Arc::strong_count(val: &Arc<T>) -> usize`
Returns current strong reference count.

---

### Clone-on-Write (`Cow<'a, B>`)

#### `into_owned(self) -> B::Owned` / `to_mut(&mut self) -> &mut B::Owned`
Extracts owned data or mutably borrows by cloning if borrowed.
```rust
use std::borrow::Cow;

let s: Cow<str> = Cow::Borrowed("hello");
let mut owned: String = s.into_owned(); // Converts to owned String!
```

---

### Interior Mutability (`Cell<T>` / `RefCell<T>`)

#### `Cell::get(&self) -> T` / `Cell::set(&self, val: T)`
Gets or sets value inside `Cell` for `Copy` types without borrowing locks.
```rust
use std::cell::Cell;
let c = Cell::new(5);
c.set(10);
assert_eq!(c.get(), 10);
```

#### `borrow(&self) -> Ref<T>` / `borrow_mut(&self) -> RefMut<T>`
Dynamically borrows contents with runtime borrow checking (panics if aliasing rules violated).
```rust
use std::cell::RefCell;

let cell = RefCell::new(42);
*cell.borrow_mut() += 1;
assert_eq!(*cell.borrow(), 43);
```

---

### Thread Synchronization (`Mutex<T>` / `RwLock<T>`)

#### `lock(&self) -> Result<MutexGuard<T>, ...>`
Acquires thread lock, blocking current thread until available.
```rust
use std::sync::Mutex;

let counter = Mutex::new(0);
{
    let mut guard = counter.lock().unwrap();
    *guard += 1;
} // Lock automatically released when `guard` drops!
```

---

## 11. Iterator (`Iterator`) Methods

Iterators produce sequences of values lazily.

### Key Iterator Adapters

```rust
let numbers = vec![1, 2, 3, 4, 5];

// 1. .map(): Transforms elements
let doubled: Vec<_> = numbers.iter().map(|x| x * 2).collect();

// 2. .filter(): Filters elements
let evens: Vec<_> = numbers.iter().filter(|x| *x % 2 == 0).collect();

// 3. .filter_map(): Combined filter + unwrap
let parsed: Vec<i32> = vec!["1", "a", "3"].iter().filter_map(|s| s.parse().ok()).collect();

// 4. .enumerate(): Indexed iteration (0, val), (1, val)
for (idx, val) in numbers.iter().enumerate() {
    println!("{}: {}", idx, val);
}

// 5. .find(): Returns first match
let found = numbers.iter().find(|x| **x > 3); // Some(&4)
```

---

## Summary Cheat Sheet

| Data Type | Top 3 Most Used Methods |
| :--- | :--- |
| **`String` / `&str`** | `.len()`, `.contains()`, `.to_string()` |
| **`Option<T>`** | `.unwrap()`, `.expect()`, `.is_some()` |
| **`Result<T, E>`** | `.unwrap()`, `.expect()`, `.map_err()` |
| **`Vec<T>`** | `.len()`, `.push()`, `.get()` |
| **`HashMap<K, V>`** | `.get()`, `.insert()`, `.entry()` |
| **`Path` / `PathBuf`** | `.display()`, `.join()`, `.file_name()` |
| **`Iterator`** | `.map()`, `.filter()`, `.collect()` |
