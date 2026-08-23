# Top 65 Rust Interview Questions (2026)

> **Source:** [Devinterview.io — Rust Interview Questions](https://devinterview.io/questions/web-and-mobile-development/rust-interview-questions)
>
> **What was actually scrapeable:** the site publishes all 65 **question titles**, but only the first **15 answers** are public — Q16–Q65 sit behind a paywall.
>
> - **Q1–Q15** — answers reproduced from the public source. Where the original text is factually wrong, a **Correction to the source** note follows it rather than silently editing the source.
> - **Q16–Q65** — questions are the site's; the answers were written for this file (Rust 2021/2024 edition, stable toolchain).

## Contents

- [1. What is _cargo_ and how do you create a new Rust project with it?](#1-what-is-_cargo_-and-how-do-you-create-a-new-rust-project-with-it)
- [2. Describe the structure of a basic Rust program.](#2-describe-the-structure-of-a-basic-rust-program)
- [3. Explain the use of `main` function in _Rust_.](#3-explain-the-use-of-main-function-in-_rust_)
- [4. How does _Rust_ handle _null_ or _nil_ values?](#4-how-does-_rust_-handle-_null_-or-_nil_-values)
- [5. What data types does _Rust_ support for _scalar_ values?](#5-what-data-types-does-_rust_-support-for-_scalar_-values)
- [6. How do you declare and use an _array_ in _Rust_?](#6-how-do-you-declare-and-use-an-_array_-in-_rust_)
- [7. Can you explain the differences between `let` and `let mut` in _Rust_?](#7-can-you-explain-the-differences-between-let-and-let-mut-in-_rust_)
- [8. What is _shadowing_ in _Rust_ and give an example of how it's used?](#8-what-is-_shadowing_-in-_rust_-and-give-an-example-of-how-its-used)
- [9. What is the purpose of `match` statements in _Rust_?](#9-what-is-the-purpose-of-match-statements-in-_rust_)
- [10. What is _ownership_ in _Rust_ and why is it important?](#10-what-is-_ownership_-in-_rust_-and-why-is-it-important)
- [11. Explain the _borrowing rules_ in _Rust_.](#11-explain-the-_borrowing-rules_-in-_rust_)
- [12. What is a _lifetime_ and how does it relate to _references_?](#12-what-is-a-_lifetime_-and-how-does-it-relate-to-_references_)
- [13. How do you create a _reference_ in _Rust_?](#13-how-do-you-create-a-_reference_-in-_rust_)
- [14. Describe the difference between a _shared reference_ and a _mutable reference_.](#14-describe-the-difference-between-a-_shared-reference_-and-a-_mutable-reference_)
- [15. How does the _borrow checker_ help prevent _race conditions_?](#15-how-does-the-_borrow-checker_-help-prevent-_race-conditions_)
- [16. Can a variable hold multiple _mutable references_ at the same time?](#16-can-a-variable-hold-multiple-_mutable-references_-at-the-same-time)
- [17. What are _slices_ and how do they work in relation to _ownership_?](#17-what-are-_slices_-and-how-do-they-work-in-relation-to-_ownership_)
- [18. Explain _lifetimes_ in function signatures and why they're necessary.](#18-explain-_lifetimes_-in-function-signatures-and-why-theyre-necessary)
- [19. What is a _dangling reference_ and how does _Rust_ prevent it?](#19-what-is-a-_dangling-reference_-and-how-does-_rust_-prevent-it)
- [20. How does _Rust_ handle error propagation?](#20-how-does-_rust_-handle-error-propagation)
- [21. Explain the use of `Option` and `Result` types in _Rust_.](#21-explain-the-use-of-option-and-result-types-in-_rust_)
- [22. How do you use the `unwrap` and `expect` methods with `Result` types?](#22-how-do-you-use-the-unwrap-and-expect-methods-with-result-types)
- [23. What are _panics_ in _Rust_ and when should they be used?](#23-what-are-_panics_-in-_rust_-and-when-should-they-be-used)
- [24. How can you handle recoverable errors without `panic!`ing?](#24-how-can-you-handle-recoverable-errors-without-panicing)
- [25. Explain how _Rust_ ensures _memory safety_ in concurrent programs.](#25-explain-how-_rust_-ensures-_memory-safety_-in-concurrent-programs)
- [26. Describe the difference between `std::thread::spawn` and `tokio::spawn`.](#26-describe-the-difference-between-stdthreadspawn-and-tokiospawn)
- [27. How do _channels_ work in _Rust_ and what types of channels are available?](#27-how-do-_channels_-work-in-_rust_-and-what-types-of-channels-are-available)
- [28. What is `async/await` and how does it work in _Rust_?](#28-what-is-asyncawait-and-how-does-it-work-in-_rust_)
- [29. What is the purpose of the `Mutex` type in _Rust_?](#29-what-is-the-purpose-of-the-mutex-type-in-_rust_)
- [30. What are _traits_ in _Rust_?](#30-what-are-_traits_-in-_rust_)
- [31. How do you define and implement a _generic function_ or _struct_ in _Rust_?](#31-how-do-you-define-and-implement-a-_generic-function_-or-_struct_-in-_rust_)
- [32. What are _associated types_ in _Rust_ and how are they different from _generics_?](#32-what-are-_associated-types_-in-_rust_-and-how-are-they-different-from-_generics_)
- [33. Explain _Rust's orphan rule_ in the context of trait implementations.](#33-explain-_rusts-orphan-rule_-in-the-context-of-trait-implementations)
- [34. Describe how to use _trait bounds_ in _Rust_.](#34-describe-how-to-use-_trait-bounds_-in-_rust_)
- [35. What are _enums_ and how are they used in _Rust_?](#35-what-are-_enums_-and-how-are-they-used-in-_rust_)
- [36. How does _pattern matching_ work with enums in _Rust_?](#36-how-does-_pattern-matching_-work-with-enums-in-_rust_)
- [37. Give an example of a function that uses pattern matching to handle different errors.](#37-give-an-example-of-a-function-that-uses-pattern-matching-to-handle-different-errors)
- [38. Can you explain _destructuring_ in the context of pattern matching in _Rust_?](#38-can-you-explain-_destructuring_-in-the-context-of-pattern-matching-in-_rust_)
- [39. What are _macros_ in _Rust_ and how do you define them?](#39-what-are-_macros_-in-_rust_-and-how-do-you-define-them)
- [40. Give an example of when you would use a _macro_ in _Rust_.](#40-give-an-example-of-when-you-would-use-a-_macro_-in-_rust_)
- [41. What is the difference between _declarative macros_ and _procedural macros_ in _Rust_?](#41-what-is-the-difference-between-_declarative-macros_-and-_procedural-macros_-in-_rust_)
- [42. How does _Rust_ achieve _memory safety_ without a garbage collector?](#42-how-does-_rust_-achieve-_memory-safety_-without-a-garbage-collector)
- [43. Describe the concept of _reference counting_ in _Rust_.](#43-describe-the-concept-of-_reference-counting_-in-_rust_)
- [44. What is the significance of the `Drop` trait in _Rust_?](#44-what-is-the-significance-of-the-drop-trait-in-_rust_)
- [45. How do you manage _Rust project dependencies_?](#45-how-do-you-manage-_rust-project-dependencies_)
- [46. Name some widely-used _crates_ in the Rust ecosystem.](#46-name-some-widely-used-_crates_-in-the-rust-ecosystem)
- [47. What features does _Rust_ offer for package documentation?](#47-what-features-does-_rust_-offer-for-package-documentation)
- [48. How do you format _Rust_ code for readability?](#48-how-do-you-format-_rust_-code-for-readability)
- [49. Explain what `unsafe` code is in _Rust_ and when to use it.](#49-explain-what-unsafe-code-is-in-_rust_-and-when-to-use-it)
- [50. How does _Rust_ interface with other languages (FFI)?](#50-how-does-_rust_-interface-with-other-languages-ffi)
- [51. What are some of the considerations for using _Rust_ in _embedded systems_?](#51-what-are-some-of-the-considerations-for-using-_rust_-in-_embedded-systems_)
- [52. Discuss _Rust's support for compile-time function execution (const fn)_.](#52-discuss-_rusts-support-for-compile-time-function-execution-const-fn_)
- [53. How can you compile _Rust_ code for a different target platform?](#53-how-can-you-compile-_rust_-code-for-a-different-target-platform)
- [54. How is _procedural macro expansion_ handled in _Rust_?](#54-how-is-_procedural-macro-expansion_-handled-in-_rust_)
- [55. What are some common idiomatic practices in _Rust_ for error handling?](#55-what-are-some-common-idiomatic-practices-in-_rust_-for-error-handling)
- [56. Describe effective use of the _Rust module system_ in large projects.](#56-describe-effective-use-of-the-_rust-module-system_-in-large-projects)
- [57. Explain how you would optimize _Rust_ code for performance.](#57-explain-how-you-would-optimize-_rust_-code-for-performance)
- [58. What's the recommended way to write _unit tests_ in _Rust_?](#58-whats-the-recommended-way-to-write-_unit-tests_-in-_rust_)
- [59. How would you approach writing a web server in _Rust_?](#59-how-would-you-approach-writing-a-web-server-in-_rust_)
- [60. Discuss the use of _Rust_ for network programming and available libraries.](#60-discuss-the-use-of-_rust_-for-network-programming-and-available-libraries)
- [61. What factors might lead you to choose _Rust_ for a new command-line tool development?](#61-what-factors-might-lead-you-to-choose-_rust_-for-a-new-command-line-tool-development)
- [62. Describe how you would implement _file I/O operations_ in _Rust_.](#62-describe-how-you-would-implement-_file-io-operations_-in-_rust_)
- [63. What are some challenges you might face when integrating _Rust_ in a larger, language-diverse codebase?](#63-what-are-some-challenges-you-might-face-when-integrating-_rust_-in-a-larger-language-diverse-codebase)
- [64. How does _Rust_ handle default parameter values in functions?](#64-how-does-_rust_-handle-default-parameter-values-in-functions)
- [65. Discuss _Rust's release channels_ and the _stability guarantee_.](#65-discuss-_rusts-release-channels_-and-the-_stability-guarantee_)

---

## 1. What is _cargo_ and how do you create a new Rust project with it?

In Rust, **Cargo** serves as both a package manager and a build system, streamlining the development process by managing dependencies, compiling code, running related tasks, and providing tools for efficient project management.

### Key Features

- **Version Control**: Manages packages and their versions using `crates.io`.
- **Dependency Management**: Seamlessly integrates third-party crates.
- **Building & Compiling**: Arranges and optimizes the build process.
- **Tasks & Scripts**: Executes pre-defined or custom commands.
- **Project Generation Tool**: Automates project scaffolding.

### Basic Commands

- `cargo new MyProject` — initializes a fresh Rust project directory.
- `cargo build` — compiles the project, generating an executable or library.
- `cargo run` — builds and runs the project.

### Code Example

```rust
// src/main.rs
fn main() {
    println!("Hello, world!");
}
```

To automatically set up the standard Rust project structure and `MyProject` directory:

```bash
cargo new MyProject --bin
```

> **Note:** `--bin` is the default for `cargo new`; use `--lib` for a library. `cargo init` does the same thing inside an existing directory. Other commands worth knowing: `cargo check` (type-check without codegen — much faster), `cargo test`, `cargo add <crate>`, `cargo clippy`, `cargo fmt`.

<br>

## 2. Describe the structure of a basic Rust program.

### Components of a Rust Program

1. **Basic Structure**
   - Common files: `src/main.rs` (for executables) or `src/lib.rs` (for libraries).
   - `Cargo.toml`: configuration file for managing dependencies and project settings.

2. **Key Definitions**
   - **Main function**: entry point where program execution begins.
   - **`use` declarations**: bring paths into scope.
   - **`extern "C"` functions**: declare functions from external libraries.

3. **Language Syntax**
   - Statements end with `;`; the last expression in a block is its value.
   - Naming convention: `snake_case` for functions/variables, `SCREAMING_SNAKE_CASE` for constants, `PascalCase` for types.

4. **Mechanisms for Sharing Code**
   - Modules and `pub` visibility: used to organize and manage code.
   - `mod`: keyword to define a module.
   - `pub`: keyword to specify visibility.

5. **Error Handling**
   - Employs `Result` and `Option` types, along with methods like `unwrap()` and `expect()`.

6. **Tooling and Management**
   - Uses `cargo` commands responsible for building, running, testing, and packaging Rust applications.

> **Correction to the source:** the original answer claims Rust "utilizes camelCase as the preferred style". That is wrong — Rust's official style (enforced by `rustfmt`, warned about by the `non_snake_case` lint) is `snake_case` for functions, variables, and modules; `PascalCase` only for types, traits, and enum variants. The source also lists `extern crate` as a key definition; that is a Rust 2015 idiom made unnecessary by the 2018 edition.

### Code Example

```rust
// src/main.rs
use std::collections::HashMap;

mod greeting;              // pulls in src/greeting.rs

const MAX_USERS: usize = 100;

struct User {
    name: String,
    age: u32,
}

impl User {
    fn new(name: &str, age: u32) -> Self {
        User { name: name.to_string(), age }
    }
}

fn main() {
    let user = User::new("Alice", 30);
    let mut ages: HashMap<String, u32> = HashMap::new();
    ages.insert(user.name.clone(), user.age);

    println!("{}", greeting::hello(&user.name));
    println!("capacity: {MAX_USERS}, known ages: {}", ages.len());
}
```

<br>

## 3. Explain the use of `main` function in _Rust_.

In **Rust**, the `main` function serves as the **entry point** for the execution of standalone applications. It coordinates key setup and teardown tasks and makes use of capabilities defined in the Rust standard library.

### Role of `main`

The `main` function initiates the execution of Rust applications. Based on its declared return type and the use of `Result`, it facilitates proper error handling and, if needed, early termination of the program.

### Return Type of `main`

The `main` function can have two primary return types:

- **`()`** (unit type): the default when no error handling is required, signifying the program ran successfully.
- **`Result<T, E>`**: allows explicit error signalling. `Ok` denotes a successful run; `Err` communicates a failure with an error value of type `E`.

> More precisely, `main` may return any type implementing `std::process::Termination`. When `main` returns `Err(e)`, Rust prints the error using its `Debug` representation and exits with a non-zero status code.

### Aborting the Program

- **Direct call to `panic!`**: in scenarios where an unrecoverable error occurs, the `panic!` macro halts the application.
- **Using `Result`**: returning an `Err` variant from `main` communicates the cause of failure and ends the program accordingly.
- **`std::process::exit(code)`**: exits immediately with an explicit status code — note that destructors do **not** run.

### Code Example: `main` with `Result`

```rust
use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    let config = fs::read_to_string("config.toml")?;  // `?` propagates the io::Error
    println!("Loaded {} bytes of config", config.len());
    Ok(())
}
```

<br>

## 4. How does _Rust_ handle _null_ or _nil_ values?

In **Rust**, the concept of **null** traditionally found in languages like Java or Swift is replaced by `Option<T>`. The absence of a value is represented by **`None`**, while the presence of a value of type `T` is represented by **`Some(T)`**.

This approach is safer and eliminates the need for many null checks — the compiler forces you to handle the `None` case before you can touch the value.

### Option Enum

```rust
enum Option<T> {
    None,
    Some(T),
}
```

The generic type `T` represents the data type of the potential value.

### Use Cases

- **Functions**: indicate a possible absence of a return value.
- **Variables**: signal that a value may not be present — "nullable" in other languages.
- **Struct fields**: model genuinely optional data.

### Code Example: `Option<T>`

```rust
fn find_index(arr: &[i32], target: i32) -> Option<usize> {
    for (index, &num) in arr.iter().enumerate() {
        if num == target {
            return Some(index);
        }
    }
    None
}

fn main() {
    let my_list = vec![1, 2, 3, 4, 5];
    let target_val = 6;

    match find_index(&my_list, target_val) {
        Some(index) => println!("Target value found at index: {index}"),
        None => println!("Target value not found in the list."),
    }
}
```

> **Zero-cost:** thanks to *niche optimization*, `Option<&T>`, `Option<Box<T>>`, and `Option<NonZeroU32>` are the same size as the inner type — `None` uses the otherwise-impossible bit pattern. `Option` costs nothing over a nullable pointer, but is checked at compile time.

<br>

## 5. What data types does _Rust_ support for _scalar_ values?

Rust offers four **built-in scalar types**:

- **Integers**: `i8/i16/i32/i64/i128/isize` (signed, two's complement) and `u8/u16/u32/u64/u128/usize` (unsigned). Default: `i32`.
- **Floating-point numbers**: `f32` (single precision), `f64` (double precision). Default: `f64`.
- **Booleans**: `bool`, representing `true` or `false`.
- **Characters**: `char` — a **Unicode scalar value**, 4 bytes wide, written in single quotes.

### Example

```rust
fn main() {
    let a: i32 = 42;          // 32-bit signed integer
    let b: f64 = 3.14;        // 64-bit float
    let big = 1_000_000u64;   // underscores as digit separators
    let hex = 0xff_u8;        // 0xff, 0o77, 0b1111_0000 literal forms

    let is_rust_cool = true;  // inferred type: bool
    let emoji = 'R';          // char is 4 bytes, not 1

    println!("{a} {b} {big} {hex} {is_rust_cool} {emoji}");
}
```

> **Overflow behaviour:** integer overflow panics in debug builds and wraps in release builds. When wrapping or saturating is intended, say so explicitly with `wrapping_add`, `saturating_add`, `checked_add`, or `overflowing_add`.

<br>

## 6. How do you declare and use an _array_ in _Rust_?

In Rust, you **declare an array** with explicit type annotations. The size is part of the type, making arrays **fixed-size** and stack-allocated.

### Syntax

```rust
let array_name: [data_type; size] = [value1, value2, /* ... */ last_value];
```

### Example: Declaring and Using an Array

```rust
fn main() {
    let mut lucky_numbers: [i32; 3] = [7, 11, 42];
    let first_number = lucky_numbers[0];
    println!("My lucky number is {first_number}");

    lucky_numbers[2] = 5;  // requires `mut`
    println!("{lucky_numbers:?}");
}
```

> **Correction to the source:** the original example declares `let lucky_numbers` (immutable) and then assigns `lucky_numbers[2] = 5;`, which does **not** compile — `cannot assign to lucky_numbers[_], as lucky_numbers is not declared as mutable`. The array must be declared `let mut`, as above.

### Array Initialization Methods

- **`[value; size]`** — replicates `value` to create an array of the specified size.
- **`[values...]`** — infers the array size from the number of values.

```rust
let same_number = [3; 5];               // [3, 3, 3, 3, 3]
let my_favs = ["red", "green", "blue"]; // [&str; 3]
```

### Arrays vs. Vectors

| | `[T; N]` (array) | `Vec<T>` |
|---|---|---|
| Size | fixed, known at compile time | grows at runtime |
| Storage | stack (part of the value) | heap |
| Length in the type | yes | no |

Indexing out of bounds **panics** — it does not read garbage memory. Use `arr.get(i) -> Option<&T>` for a non-panicking lookup.

<br>

## 7. Can you explain the differences between `let` and `let mut` in _Rust_?

In **Rust**, both `let` and `let mut` are used for variable **declaration**, but they differ in **mutability**.

### `let`: Immutability by Default

When you define a variable with `let`, Rust treats it as **immutable** by default — its value cannot be changed once set.

```rust
let name = "Alice";
name = "Bob";  // error[E0384]: cannot assign twice to immutable variable
```

### `let mut`: Enabling Mutability

Using `let mut` makes the binding **mutable**.

```rust
let mut age = 25;
age = 26;  // allowed
```

### Benefits and Safe Defaults

Rust's design, with immutability as the default, is consistent with **safety** and **predictability**. It helps avoid bugs and produces clearer, more maintainable code. Where mutability is needed, `let mut` is an explicit signal that makes the code easier to comprehend.

> **Important distinction:** `mut` applies to the *binding*, not the data. `let mut x = 5;` lets you reassign `x`; `&mut x` lets you mutate through a reference. `const` and `static` are separate concepts: `const` values are inlined at compile time and require a type annotation, while `let` bindings are runtime values.

<br>

## 8. What is _shadowing_ in _Rust_ and give an example of how it's used?

**Shadowing** allows you to **redeclare a variable with the same name**. This is useful for changing the mutability or the *type* of a value while keeping one meaningful name.

### Key Features

- **Reassignment without `mut`**: a shadowed variable creates a brand-new binding.
- **Flexibility with types**: you can change a variable's type through shadowing.

### Code Example

```rust
fn main() {
    let age = "20";                       // &str
    let age = age.parse::<u8>().unwrap(); // u8 — same name, new type

    println!("Double your age plus 7: {}", (age * 2 + 7));
}
```

### Shadowing vs. Mutability

| | `mut` | shadowing |
|---|---|---|
| Type may change | no | yes |
| New binding created | no | yes |
| Original still reachable | n/a | only outside the shadowing scope |

### Under the Hood

When you shadow a variable, you create a new one in the same scope with the same name, effectively hiding the original. This is an implicit unbinding of the first variable and the binding of a new one in its place.

```rust
let spaces = "   ";
let spaces = spaces.len();   // now a usize

{
    let spaces = "inner";    // shadows only inside this block
    println!("{spaces}");    // "inner"
}
println!("{spaces}");        // 3
```

### When to Use Shadowing

- **Parsing / conversion pipelines**: `let input = input.trim(); let input: i32 = input.parse()?;`
- **Code clarity**: when `mut` would suggest ongoing mutation that isn't happening.
- **Refactoring**: switching between types without inventing `input_str`, `input_num`, `input_final`.

Use it judiciously — the name must stay descriptive across every shadowing step.

<br>

## 9. What is the purpose of `match` statements in _Rust_?

In Rust, **`match`** is a robust way of handling multiple pattern scenarios. It is particularly useful for **enumerations**, though it works with any data type.

### Benefits of `match`

- **Pattern matching**: compare a value against a series of patterns and act on the one that matches. It is foundational to Rust's error handling.
- **Exhaustiveness**: the compiler forces you to handle *every* possible outcome, leaving no room for a forgotten case.
- **Conciseness and safety**: matching is checked statically at compile time.
- **Power across data types**: works with structs, tuples, enums, ranges, slices, and literals.
- **Error handling**: `Option` and `Result` use `match` for value and error handling.
- **It is an expression**: `match` returns a value, so it can appear on the right-hand side of a `let`.

### Code Example

```rust
enum Shape {
    Circle { radius: f64 },
    Rect { w: f64, h: f64 },
    Unit,
}

fn area(s: &Shape) -> f64 {
    match s {
        Shape::Circle { radius } => std::f64::consts::PI * radius * radius,
        Shape::Rect { w, h } => w * h,
        Shape::Unit => 1.0,
    }   // remove any arm and the compiler errors: non-exhaustive patterns
}

fn describe(n: i32) -> &'static str {
    match n {
        0 => "zero",
        1..=9 => "single digit",
        n if n < 0 => "negative",   // match guard
        _ => "large",               // catch-all
    }
}
```

<br>

## 10. What is _ownership_ in _Rust_ and why is it important?

**Ownership** refers to the rules governing memory management and resource handling. It is the foundation of Rust's memory safety, ensuring both thread and memory safety without a garbage collector.

### Key Ownership Principles

- **Each value has one owner**: a single variable owns the data it points to, giving clear accountability for memory management.
- **Ownership is transferred (moved)**: when an owned value is assigned to another variable or passed into a function, ownership moves; the original binding can no longer be used.
- **Only one owner at a time**: this protects against double-frees and unsafe memory access.
- **Owned data is dropped**: when the owner goes out of scope, the value is dropped and its memory is cleaned up.

### Borrowing

If a function or expression temporarily needs access to a value without taking ownership, it can **borrow** it using references. There are **two kinds of borrows**:

- **Immutable borrow (`&T`)**: the borrower can read but not modify. Many immutable borrows may exist at once.
- **Mutable borrow (`&mut T`)**: the borrower gets exclusive write access. No other borrow, mutable or immutable, may coexist with it.

### Ownership Benefits

- **Memory safety**: strong guarantees against dangling pointers, double-frees, and use-after-free.
- **Concurrency safety**: data races are rejected at compile time.
- **Performance**: no garbage collector, no runtime bookkeeping — as efficient as C and C++.
- **Predictable resource management**: files, sockets, and locks are released deterministically at scope exit.

### Code Example: Ownership and Borrowing

```rust
fn main() {
    let mut string = String::from("Hello, ");
    string_push(&mut string);      // passing a mutable reference — no move
    println!("{string}");          // "Hello, World!"

    let owned = string;            // MOVE: `string` is no longer usable
    // println!("{string}");       // error[E0382]: borrow of moved value

    let copied = 5;                // i32 is Copy — this is a copy, not a move
    let also = copied;
    println!("{owned} {copied} {also}");
}

fn string_push(s: &mut String) {
    s.push_str("World!");
}
```

> **`Copy` types** — integers, `bool`, `char`, `f64`, and tuples of them — are duplicated instead of moved, because copying them is trivial and they own no heap resources.

<br>

## 11. Explain the _borrowing rules_ in _Rust_.

Rust's approach to memory safety, **ownership**, includes borrowing. The borrowing rules let the compiler manage memory accurately.

### The Two Core Rules

1. At any given time you may have **either** one mutable reference (`&mut T`) **or** any number of immutable references (`&T`) — never both.
2. References must always be **valid**: a reference may never outlive the data it points to.

Together these eliminate data races and dangling pointers at compile time.

### Illustrations

**Mutable and immutable cannot overlap:**

```rust
let mut data = vec![1, 2, 3];
let s1 = &mut data;
let s2 = &data;   // error[E0502]: cannot borrow `data` as immutable
                  // because it is also borrowed as mutable
s1.push(4);
```

**Non-Lexical Lifetimes (NLL)** — since Rust 2018 a borrow ends at its **last use**, not at the end of the enclosing block, which makes the checker far more permissive:

```rust
let mut data = vec![1, 2, 3];
let first = &data[0];
println!("{first}");     // last use of `first` — the borrow ends here
data.push(4);            // OK under NLL
```

**Dangling references are rejected:**

```rust
fn dangle() -> &String {   // error[E0106]: missing lifetime specifier
    let s = String::from("hi");
    &s                     // `s` is dropped at the end of the function
}
```

**Reborrowing** — passing a `&mut T` to a function reborrows rather than moves it, so the original stays usable:

```rust
fn bump(n: &mut i32) { *n += 1; }

let mut x = 0;
let r = &mut x;
bump(r);        // reborrow: `&mut *r`
bump(r);        // `r` still usable
```

> **Correction to the source:** the original answer claims that
> ```rust
> let x = 5;
> let r1 = &x;
> let r2 = &x;
> println!("{}, {}", r1, r2);
> ```
> "would throw an error". It does not — this compiles fine. **Multiple immutable references are explicitly allowed**; only mixing a mutable borrow with any other borrow is an error. The source's talk of "stamps" and `.borrow_mut()` also conflates the compile-time borrow checker with `RefCell`'s runtime borrow tracking; they are different mechanisms.

<br>

## 12. What is a _lifetime_ and how does it relate to _references_?

**Lifetimes** describe the scope for which a **reference** is valid. The compiler uses this information to ensure data outlives every reference to it, preventing **dangling pointers**.

Lifetimes are a *compile-time-only* concept: they are erased before code generation and cost nothing at runtime.

### Three Ways Lifetimes Appear

1. **`'static`** — a reference valid for the entire duration of the program. String literals have this lifetime.
2. **`&'a T`** — an explicit **lifetime annotation** `'a`, saying the reference is valid for some caller-chosen region. Annotations *describe* relationships; they never change how long data actually lives.
3. **Lifetime elision** — Rust infers lifetimes in common cases, so most code needs no annotations at all.

### Examples

```rust
let s: &'static str = "I'm a static string!";
```

```rust
fn example<'a>(item: &'a i32) {
    let r: &'a i32 = item;
    println!("{r}");
}
```

**Multiple references with a shared lifetime** — `'a` here resolves to the *shorter* of the two input regions:

```rust
fn longest<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() > b.len() { a } else { b }
}

fn main() {
    let x = String::from("long string is long");
    let result;
    {
        let y = String::from("short");
        result = longest(&x, &y);
        println!("Longest: {result}");   // fine — used inside y's scope
    }
    // println!("{result}");  // error: `y` does not live long enough
}
```

### Lifetimes in Structs

A struct holding a reference must declare the lifetime, which forces the struct not to outlive the borrowed data:

```rust
struct Excerpt<'a> {
    part: &'a str,
}
```

<br>

## 13. How do you create a _reference_ in _Rust_?

In Rust, a reference is an indirect **borrowed view** of data. It carries no ownership, unlike a smart pointer. A reference is either **immutable** (`&T`) or **mutable** (`&mut T`).

### Key Concepts

- **Creating**: `&value` for a shared reference, `&mut value` for an exclusive one (the value must be declared `mut`).
- **Using**: `*r` dereferences explicitly; method calls and field access auto-dereference.
- **Ownership relation**: many `&T` may coexist, but only one `&mut T`, and never both at once.
- **Lifetime**: the region for which the reference remains valid.

### Code Example

```rust
fn main() {
    let mut data: i32 = 42;

    // Immutable reference — read only
    let val_reference: &i32 = &data;
    println!("Value through immutable reference: {val_reference}");
    // the immutable borrow ends here (NLL), so the next line is legal

    // Mutable reference — exclusive write access
    let val_mut_reference: &mut i32 = &mut data;
    *val_mut_reference += 10;      // dereference to write

    println!("Data after mutation: {data}");
}
```

> **Correction to the source:** the original example creates `&data` and `&mut data` on adjacent lines and then prints the immutable one *after* the mutation. That version does **not** compile (E0502). It only works if the shared borrow's last use comes before the mutable borrow is created, as rewritten above.

### Borrow Checker

Rust's **borrow checker** verifies that references are only used within their valid lifetime scopes, eliminating an entire class of memory bugs before the program ever runs.

<br>

## 14. Describe the difference between a _shared reference_ and a _mutable reference_.

References let multiple parts of a program interact with the same data under strict safety rules.

### Shared Reference — `&T`

Grants **read-only** access. You cannot modify data through it (barring interior mutability). **Many** shared references may exist simultaneously. Also called an *immutable* or *aliased* reference.

### Mutable Reference — `&mut T`

Grants **read-write** access, and is the **only** live reference to that data while it exists. Also called an *exclusive* reference — the more accurate name, since exclusivity is the property the compiler actually enforces.

### Comparison

| | `&T` (shared) | `&mut T` (mutable) |
|---|---|---|
| Read | yes | yes |
| Write | no | yes |
| How many at once | unlimited | exactly one |
| Coexists with the other kind | no | no |
| Implements `Copy` | yes | no (it moves/reborrows) |

### Code Example

```rust
fn main() {
    let mut value = 5;

    // Many shared references are fine
    let s1 = &value;
    let s2 = &value;
    println!("Shared: {s1} {s2}");   // borrows end here

    // Exactly one mutable reference
    let mut_ref = &mut value;
    *mut_ref += 10;

    println!("Value: {value}");      // 15
}
```

If you `println!` with `s1` *after* creating `mut_ref`, the compiler rejects it: the program would read and write the same location through two live handles, which is exactly what the rule prevents.

### Interior Mutability — the escape hatch

`Cell<T>`, `RefCell<T>`, `Mutex<T>`, and `RwLock<T>` allow mutation through a `&T`, moving the exclusivity check from compile time to runtime (or to the OS lock).

<br>

## 15. How does the _borrow checker_ help prevent _race conditions_?

Rust's type system — specifically the **borrow checker** — makes **data races** a compile-time error rather than a runtime hazard.

A data race requires three things simultaneously: two or more threads accessing the same location, at least one access being a write, and no synchronization. Rust's aliasing rule (`&mut` is exclusive) removes the second and third conditions by construction.

### A Data Race the Compiler Rejects

```rust
use std::thread;

fn main() {
    let mut counter = 0;

    let handle1 = thread::spawn(|| {
        counter += 1;   // error[E0373]: closure may outlive the current function
    });

    let handle2 = thread::spawn(|| {
        counter += 1;   // and: cannot borrow `counter` as mutable more than once
    });

    handle1.join().unwrap();
    handle2.join().unwrap();
    println!("Counter: {counter}");
}
```

Both closures try to hold `&mut counter`, and both would outlive `main`'s stack frame. The compiler refuses on both counts — the racing program never gets built.

### The Three Mechanisms

- **Exclusive mutability**: `&mut T` guarantees no other thread holds a reference while a write is in flight.
- **Lifetimes**: references cannot outlive the data they point to, so a spawned thread cannot hold a reference into a dead stack frame.
- **`Send` and `Sync` marker traits**: `Send` means a type is safe to move to another thread; `Sync` means `&T` is safe to share between threads. `thread::spawn` requires `Send + 'static`, so non-thread-safe types (`Rc<T>`, `RefCell<T>`) are rejected at the boundary.

### The Correct Version: `Arc<Mutex<T>>`

```rust
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        handles.push(thread::spawn(move || {
            let mut n = counter.lock().unwrap();
            *n += 1;
        }));   // guard dropped here — lock released
    }

    for h in handles {
        h.join().unwrap();
    }

    println!("Counter: {}", *counter.lock().unwrap());  // always 10
}
```

`Arc` provides thread-safe shared ownership; `Mutex` provides exclusive access. Crucially, the data lives *inside* the `Mutex` — you cannot reach the value without acquiring the lock, so "forgot to lock" is not an expressible bug.

> **What the borrow checker does *not* prevent:** deadlocks, livelocks, lost wakeups, and race conditions in program logic (for example check-then-act across two separate lock acquisitions). Rust guarantees the absence of *data races*, which is a narrower and more precise claim.

<br>
## 16. Can a variable hold multiple _mutable references_ at the same time?

**No.** Rust allows **exactly one live `&mut T`** to a given value at a time, and no shared `&T` may coexist with it. This is the aliasing rule — often summarized as *"mutable XOR shared"*.

### Why the Rule Exists

If two mutable references pointed at the same `Vec`, one could `push` (reallocating the buffer) while the other held a pointer into the old buffer — a use-after-free. Exclusivity is what makes that impossible.

```rust
let mut v = vec![1, 2, 3];
let a = &mut v;
let b = &mut v;   // error[E0499]: cannot borrow `v` as mutable more than once
a.push(4);
```

### What *Is* Allowed

**Sequential borrows** — NLL ends the first borrow at its last use:

```rust
let mut v = vec![1, 2, 3];
let a = &mut v;
a.push(4);        // `a` dies here
let b = &mut v;   // fine — no overlap
b.push(5);
```

**Reborrowing** — a nested `&mut` derived from the first, valid while the outer one is paused:

```rust
let mut x = 10;
let outer = &mut x;
{
    let inner: &mut i32 = &mut *outer;  // reborrow
    *inner += 1;
}
*outer += 1;   // outer usable again
```

**Disjoint fields** — the compiler tracks borrows per field:

```rust
struct P { x: i32, y: i32 }
let mut p = P { x: 0, y: 0 };
let rx = &mut p.x;
let ry = &mut p.y;   // OK — different fields
*rx += 1; *ry += 1;
```

**Disjoint slice halves** — via `split_at_mut`, which uses `unsafe` internally to hand out two provably non-overlapping `&mut`s:

```rust
let mut arr = [1, 2, 3, 4];
let (left, right) = arr.split_at_mut(2);
left[0] = 10;
right[0] = 30;
```

### The Escape Hatch: Interior Mutability

`RefCell<T>` enforces the same rule at **runtime** instead — `borrow_mut()` panics if another borrow is live. `Mutex<T>` and `RwLock<T>` do the same across threads, blocking instead of panicking.

<br>

## 17. What are _slices_ and how do they work in relation to _ownership_?

A **slice** is a **borrowed view into a contiguous sequence** — it does not own its data. The two you meet constantly are `&[T]` (slice of any element type) and `&str` (slice of UTF-8 text).

### Representation

A slice is a **fat pointer**: a pointer to the first element plus a length. That's 16 bytes on a 64-bit machine, versus 8 for a thin pointer.

```rust
let v = vec![1, 2, 3, 4, 5];
let all:    &[i32] = &v;        // whole vector
let middle: &[i32] = &v[1..4];  // [2, 3, 4]
println!("{} elements, first = {}", middle.len(), middle[0]);
```

### Ownership Relationship

| Owned | Borrowed slice |
|---|---|
| `String` | `&str` |
| `Vec<T>` | `&[T]` |
| `[T; N]` | `&[T]` |
| `Box<[T]>` | `&[T]` |

A slice **borrows from its owner**, so the borrow checker keeps the owner alive and immutable-or-exclusive for the slice's lifetime:

```rust
let mut v = vec![1, 2, 3];
let s = &v[..];
v.push(4);          // error[E0502]: cannot borrow `v` as mutable
println!("{s:?}");  //   because it is also borrowed as immutable
```

That rejection is not pedantry: `push` may reallocate, which would leave `s` pointing at freed memory.

### Why `&str` Parameters Are Idiomatic

Taking `&str` instead of `String` means the caller can pass a literal, a `String`, or a substring — no allocation, no ownership transfer. Same for `&[T]` over `Vec<T>`.

```rust
fn longest_word(text: &str) -> &str {
    text.split_whitespace().max_by_key(|w| w.len()).unwrap_or("")
}

fn sum(nums: &[i32]) -> i32 { nums.iter().sum() }

let owned = String::from("the quick brown fox");
println!("{}", longest_word(&owned));   // deref coercion: &String -> &str
println!("{}", longest_word("literal"));
println!("{}", sum(&vec![1, 2, 3]));
println!("{}", sum(&[1, 2, 3]));
```

> **`&str` gotcha:** string slices index by **byte** offsets and must land on UTF-8 character boundaries. `&"héllo"[0..2]` panics because byte 1 is mid-character. Use `.chars()`, `.char_indices()`, or `.get(0..2)` when the input may be non-ASCII.

<br>

## 18. Explain _lifetimes_ in function signatures and why they're necessary.

A lifetime in a signature tells the compiler **how the output's validity relates to the inputs'**. Without that link, the compiler cannot verify the returned reference at the call site.

### The Problem

```rust
fn longest(a: &str, b: &str) -> &str {   // error[E0106]: missing lifetime specifier
    if a.len() > b.len() { a } else { b }
}
```

The compiler cannot tell whether the result borrows from `a` or from `b`, so it cannot decide how long the result is valid. Note this is *not* a request for information about the function body — the body is right there. It is a request to make the **contract** explicit, so callers can be checked against the signature alone.

### The Fix

```rust
fn longest<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() > b.len() { a } else { b }
}
```

`'a` unifies to the **shorter** of the two input regions, and the result is valid only for that region.

### Independent Lifetimes

When only one input can be returned, say so — this signature is strictly more permissive:

```rust
fn first<'a, 'b>(a: &'a str, _b: &'b str) -> &'a str { a }
```

Now `_b` may be dropped immediately without invalidating the result.

### Lifetime Elision Rules

Most signatures need no annotations, because the compiler applies three rules:

1. Each elided **input** lifetime gets its own distinct parameter.
2. If there is **exactly one** input lifetime, it is assigned to all elided output lifetimes.
3. If one of the inputs is `&self` or `&mut self`, **`self`'s lifetime** is assigned to all elided output lifetimes.

```rust
fn first_word(s: &str) -> &str { ... }
// desugars by rule 2 to:
fn first_word<'a>(s: &'a str) -> &'a str { ... }

impl Parser {
    fn name(&self, buf: &str) -> &str { ... }
    // rule 3 -> returns a reference tied to &self, not to buf
}
```

Rules 2 and 3 cover the overwhelming majority of real code; you write `'a` explicitly mainly when there are multiple input references and the output could come from more than one.

### Why It Matters

Lifetimes let each function be checked **in isolation** — callers are verified against the signature, never by re-analysing the body. That is what makes borrow checking scale to large codebases and stay stable across refactors.

<br>

## 19. What is a _dangling reference_ and how does _Rust_ prevent it?

A **dangling reference** points to memory that has been freed or gone out of scope. Dereferencing one is undefined behaviour — in C it is the classic use-after-free.

### In C

```c
int* dangle() {
    int x = 5;
    return &x;      // x dies when the function returns
}                   // compiles with a warning at best; UB at runtime
```

### In Rust — Rejected at Compile Time

```rust
fn dangle() -> &String {
    let s = String::from("hello");
    &s
}   // error[E0106]: missing lifetime specifier
    // help: this function's return type contains a borrowed value, but there is
    //       no value for it to be borrowed from
```

There is no lifetime the signature could name, because nothing outside the function owns the data.

### The Fixes

**Return the owned value** — move it out instead of borrowing:

```rust
fn no_dangle() -> String {
    String::from("hello")
}
```

**Borrow from an input** — the data outlives the call:

```rust
fn prefix(s: &str) -> &str { &s[..3] }
```

### Other Dangling Cases the Checker Catches

```rust
fn main() {
    let r;
    {
        let x = 5;
        r = &x;             // error[E0597]: `x` does not live long enough
    }                       // `x` dropped here, `r` would dangle
    println!("{r}");
}
```

```rust
let mut v = vec![1, 2, 3];
let first = &v[0];
v.push(4);            // may reallocate the buffer
println!("{first}");  // error[E0502] — `first` would point at freed memory
```

### The Guarantee

Three mechanisms combine: **ownership** (exactly one owner, freed once at scope exit), **lifetimes** (every reference carries the region it is valid for), and the **borrow checker** (rejects any use outside that region). In safe Rust, dangling references are not merely unlikely — they are unrepresentable. Only `unsafe` raw pointers can dangle, which is precisely why dereferencing one requires `unsafe`.

<br>

## 20. How does _Rust_ handle error propagation?

Rust propagates errors through **return values**, not exceptions. The `?` operator makes this concise.

### The `?` Operator

`expr?` unwraps `Ok`/`Some` and **early-returns** the `Err`/`None` from the enclosing function.

```rust
use std::fs::File;
use std::io::{self, Read};

fn read_username() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open("user.txt")?.read_to_string(&mut s)?;
    Ok(s)
}
```

Written out, `?` desugars to roughly:

```rust
match File::open("user.txt") {
    Ok(f) => f,
    Err(e) => return Err(From::from(e)),
}
```

### Automatic Conversion via `From`

That `From::from` is the important part: `?` converts the error into the function's error type, so different error kinds compose in one function.

```rust
use std::num::ParseIntError;

#[derive(Debug)]
enum AppError {
    Io(std::io::Error),
    Parse(ParseIntError),
}

impl From<std::io::Error> for AppError {
    fn from(e: std::io::Error) -> Self { AppError::Io(e) }
}
impl From<ParseIntError> for AppError {
    fn from(e: ParseIntError) -> Self { AppError::Parse(e) }
}

fn read_port() -> Result<u16, AppError> {
    let text = std::fs::read_to_string("port.txt")?;  // io::Error  -> AppError
    let port: u16 = text.trim().parse()?;             // ParseIntError -> AppError
    Ok(port)
}
```

### Trait Objects for Quick Propagation

`Box<dyn Error>` accepts any error type, at the cost of losing the concrete type:

```rust
fn run() -> Result<(), Box<dyn std::error::Error>> {
    let cfg = std::fs::read_to_string("app.toml")?;
    let n: i32 = cfg.trim().parse()?;
    println!("{n}");
    Ok(())
}
```

### `?` with `Option`

The same operator works on `Option`, returning `None` early:

```rust
fn first_char_upper(s: &str) -> Option<char> {
    let c = s.chars().next()?;
    Some(c.to_ascii_uppercase())
}
```

You cannot mix them implicitly — convert with `.ok_or(...)` / `.ok()` at the boundary.

### The Ecosystem Pattern

- **Libraries** — `thiserror` to derive a concrete error enum, so callers can match on variants.
- **Applications** — `anyhow` for `anyhow::Result<T>` plus `.context("reading config")` to build a readable error chain.

<br>

## 21. Explain the use of `Option` and `Result` types in _Rust_.

Both are enums that make "might not produce a value" part of the **type**, so the compiler forces you to deal with it.

```rust
enum Option<T> { Some(T), None }
enum Result<T, E> { Ok(T), Err(E) }
```

### When to Use Which

- **`Option<T>`** — absence is normal and needs no explanation. `HashMap::get`, `Iterator::next`, `Vec::first`.
- **`Result<T, E>`** — the operation *failed*, and the caller deserves to know why. `File::open`, `str::parse`.

A useful test: if you would want to log a reason, use `Result`.

### Core Handling

```rust
let cfg: Option<&str> = Some("8080");

match cfg {
    Some(v) => println!("port {v}"),
    None => println!("no port"),
}

if let Some(v) = cfg { println!("port {v}"); }

let Some(v) = cfg else { return };   // let-else: bind or diverge
```

### Combinators Worth Knowing

```rust
let s: Option<String> = Some("42".into());

s.as_deref()                        // Option<&str>
 .map(str::trim)                    // transform the inner value
 .filter(|t| !t.is_empty())         // keep only if predicate holds
 .and_then(|t| t.parse::<u32>().ok())  // chain another optional step
 .unwrap_or(0);                     // default on None

let r: Result<u32, String> = "abc".parse::<u32>().map_err(|e| e.to_string());

r.unwrap_or_default();              // 0 on Err
r.unwrap_or_else(|e| e.len() as u32);
```

### Converting Between Them

```rust
let opt: Option<i32> = Some(3);
let res: Result<i32, &str> = opt.ok_or("missing");   // Option -> Result
let back: Option<i32> = res.ok();                    // Result -> Option
```

### Collecting

`collect` can turn an iterator of `Result`s into a `Result` of a collection — short-circuiting on the first error:

```rust
let nums: Result<Vec<i32>, _> = ["1", "2", "3"].iter().map(|s| s.parse::<i32>()).collect();
assert_eq!(nums.unwrap(), vec![1, 2, 3]);

let bad: Result<Vec<i32>, _> = ["1", "x"].iter().map(|s| s.parse::<i32>()).collect();
assert!(bad.is_err());
```

### `#[must_use]`

`Result` is marked `#[must_use]`, so ignoring one produces a warning. That is why silently dropped errors are rare in Rust.

<br>

## 22. How do you use the `unwrap` and `expect` methods with `Result` types?

Both extract the success value and **panic** on failure. They differ only in the panic message.

```rust
let n: i32 = "42".parse().unwrap();               // 42
let n: i32 = "abc".parse().unwrap();
// panic: called `Result::unwrap()` on an `Err` value: ParseIntError { kind: InvalidDigit }

let cfg = std::fs::read_to_string("app.toml")
    .expect("app.toml must exist next to the binary");
// panic: app.toml must exist next to the binary: Os { code: 2, kind: NotFound, ... }
```

`expect` is almost always better: the message tells you *which* call site blew up and what the invariant was. `unwrap` in a large codebase gives you a type name and nothing else.

> **Convention:** write the `expect` message as *why you believe this cannot fail*, not as a description of the error. "hardcoded regex is valid" beats "failed to compile regex".

### When They Are Acceptable

- **Prototypes, examples, and scripts** where a panic is a fine outcome.
- **Tests** — a panic *is* the failure report.
- **Provable invariants**: `Regex::new("^[a-z]+$").expect("hardcoded pattern is valid")`.
- **Poisoned mutexes**: `lock().unwrap()` is idiomatic, since poisoning means another thread already panicked.

### When to Avoid Them

In library code and any long-running service, a panic takes down the caller's thread with no chance to recover. Propagate with `?` instead.

### The Safer Alternatives

```rust
let port: u16 = std::env::var("PORT")
    .ok()
    .and_then(|s| s.parse().ok())
    .unwrap_or(8080);                  // default

let v = maybe_value.unwrap_or_else(|| expensive_default());  // lazy default
let v = maybe_value.unwrap_or_default();                     // T: Default

if let Err(e) = risky() { eprintln!("warning: {e}"); }        // log and continue
```

### Related Family Members

| Method | Behaviour |
|---|---|
| `unwrap()` | panic with a generic message |
| `expect(msg)` | panic with your message |
| `unwrap_or(v)` | fall back to `v` |
| `unwrap_or_else(f)` | fall back to `f()`, computed lazily |
| `unwrap_or_default()` | fall back to `T::default()` |
| `unwrap_err()` | panic unless it is `Err`; used in tests |
| `unwrap_unchecked()` | `unsafe`, UB if wrong — no check at all |

To find them all in a codebase: `cargo clippy -- -W clippy::unwrap_used -W clippy::expect_used`.

<br>

## 23. What are _panics_ in _Rust_ and when should they be used?

A **panic** is Rust's response to an **unrecoverable bug**. It unwinds the current thread, running destructors as it goes, then aborts that thread (and the process, if it is the main thread).

### What Triggers a Panic

- Explicit: `panic!("...")`, `unreachable!()`, `todo!()`, `unimplemented!()`, `assert!`/`assert_eq!`.
- Implicit: out-of-bounds indexing, integer division by zero, integer overflow in debug builds, `unwrap`/`expect` on `Err`/`None`, `RefCell` double-borrow.

```rust
fn divide(a: i32, b: i32) -> i32 {
    if b == 0 {
        panic!("division by zero: {a} / {b}");
    }
    a / b
}
```

### Panic vs. `Result` — the Deciding Question

> **Is this a bug in the program, or an expected condition in the world?**

| Situation | Use |
|---|---|
| Missing file, bad user input, network timeout | `Result` |
| Broken invariant, impossible state, contract violation | `panic!` |
| Index out of bounds in your own logic | `panic!` (already does) |
| Parsing untrusted data | `Result` |

A panic says "the programmer made a mistake". A `Result` says "the world did not cooperate". Library code should almost never panic on data it received from a caller.

### Unwinding vs. Aborting

By default a panic **unwinds**, running `Drop` for every value on the stack. You can switch to immediate abort:

```toml
[profile.release]
panic = "abort"     # smaller binary, faster, but no cleanup and no catch_unwind
```

### Catching a Panic

`std::panic::catch_unwind` exists but is **not** a general-purpose `try`/`catch`. Its legitimate uses are narrow: FFI boundaries (unwinding into C is UB), thread supervisors, and test harnesses.

```rust
let result = std::panic::catch_unwind(|| {
    panic!("boom");
});
assert!(result.is_err());
```

### Debugging Panics

```bash
RUST_BACKTRACE=1 cargo run     # backtrace
RUST_BACKTRACE=full cargo run  # including std frames
```

A panic in a spawned thread kills only that thread; `handle.join()` returns `Err`.

<br>

## 24. How can you handle recoverable errors without `panic!`ing?

Use `Result<T, E>` and handle or propagate it. Every technique below avoids terminating the program.

### 1. `match` — full control

```rust
use std::fs::File;

match File::open("config.toml") {
    Ok(f) => println!("opened {f:?}"),
    Err(e) => eprintln!("could not open config: {e}"),
}
```

### 2. Match on the error *kind*, and recover

```rust
use std::fs::File;
use std::io::ErrorKind;

let file = match File::open("config.toml") {
    Ok(f) => f,
    Err(e) if e.kind() == ErrorKind::NotFound => {
        File::create("config.toml").expect("could not create config")
    }
    Err(e) => return Err(e),
};
```

### 3. Propagate with `?`

```rust
fn load() -> Result<Config, ConfigError> {
    let text = std::fs::read_to_string("config.toml")?;
    let cfg = toml::from_str(&text)?;
    Ok(cfg)
}
```

### 4. Supply a fallback

```rust
let port: u16 = std::env::var("PORT")
    .ok()
    .and_then(|s| s.parse().ok())
    .unwrap_or(8080);

let cfg = load().unwrap_or_else(|e| {
    eprintln!("using defaults: {e}");
    Config::default()
});
```

### 5. Retry

```rust
fn with_retry<T, E>(mut op: impl FnMut() -> Result<T, E>, tries: u32) -> Result<T, E> {
    let mut last = op();
    for _ in 1..tries {
        if last.is_ok() { return last; }
        std::thread::sleep(std::time::Duration::from_millis(200));
        last = op();
    }
    last
}
```

### 6. Define a domain error type

```rust
use std::fmt;

#[derive(Debug)]
pub enum ConfigError {
    Missing(String),
    Invalid { field: String, reason: String },
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConfigError::Missing(k) => write!(f, "missing key `{k}`"),
            ConfigError::Invalid { field, reason } => write!(f, "bad `{field}`: {reason}"),
        }
    }
}

impl std::error::Error for ConfigError {}
```

Implementing `Display` + `Error` is what lets `?`, `Box<dyn Error>`, and `anyhow` all work with your type. In practice, derive it with `thiserror` instead of writing the boilerplate by hand.

### 7. Aggregate instead of stopping at the first failure

```rust
let (ok, failed): (Vec<_>, Vec<_>) = inputs
    .iter()
    .map(|s| s.parse::<i32>())
    .partition(Result::is_ok);
```

<br>

## 25. Explain how _Rust_ ensures _memory safety_ in concurrent programs.

Rust's concurrency safety is not a runtime feature — it falls out of ownership plus two marker traits.

### `Send` and `Sync`

```rust
pub unsafe auto trait Send {}   // safe to MOVE this type to another thread
pub unsafe auto trait Sync {}   // safe to SHARE &T between threads
```

They are **auto traits**: a type is `Send`/`Sync` if all its fields are. `T: Sync` is equivalent to `&T: Send`.

| Type | `Send` | `Sync` | Why |
|---|---|---|---|
| `i32`, `String`, `Vec<T>` | yes | yes | plain data |
| `Rc<T>` | **no** | **no** | non-atomic refcount would race |
| `Arc<T>` | yes* | yes* | atomic refcount |
| `RefCell<T>` | yes | **no** | non-atomic borrow flag |
| `Mutex<T>` | yes* | yes* | enforces exclusion |
| `MutexGuard<T>` | **no** | yes* | some platforms require unlock on the locking thread |
| `*const T` | **no** | **no** | no safety information |

\* when `T` allows it.

### How the Guarantee Is Enforced

`thread::spawn` has this bound:

```rust
pub fn spawn<F, T>(f: F) -> JoinHandle<T>
where
    F: FnOnce() -> T + Send + 'static,
    T: Send + 'static,
```

`Send` stops non-thread-safe types from crossing; `'static` stops the closure from capturing references into a stack frame that may die first. So the classic bugs are simply not expressible:

```rust
use std::rc::Rc;
use std::thread;

let data = Rc::new(vec![1, 2, 3]);
let d = Rc::clone(&data);
thread::spawn(move || println!("{d:?}"));
// error[E0277]: `Rc<Vec<i32>>` cannot be sent between threads safely
```

Swap `Rc` for `Arc` and it compiles — the compiler taught you the right type.

### Sharing Mutable State

The data lives **inside** the lock, so unsynchronized access is unrepresentable:

```rust
use std::sync::{Arc, RwLock};
use std::thread;

let table = Arc::new(RwLock::new(std::collections::HashMap::new()));

let writer = { let t = Arc::clone(&table); thread::spawn(move || {
    t.write().unwrap().insert("k", 1);
})};

let reader = { let t = Arc::clone(&table); thread::spawn(move || {
    if let Some(v) = t.read().unwrap().get("k") { println!("{v}"); }
})};

writer.join().unwrap();
reader.join().unwrap();
```

### Scoped Threads — Borrowing Without `'static`

Since Rust 1.63, `thread::scope` guarantees threads finish before the scope exits, so borrowing local data is safe:

```rust
let mut data = vec![1, 2, 3];

std::thread::scope(|s| {
    s.spawn(|| println!("read: {data:?}"));   // borrow, no Arc, no clone
});

data.push(4);   // fine — all scoped threads have joined
```

### The Boundary of the Guarantee

Rust prevents **data races**. It does **not** prevent deadlocks, lock-ordering bugs, livelock, or logical races — those remain your design problem.

<br>

## 26. Describe the difference between `std::thread::spawn` and `tokio::spawn`.

They create fundamentally different things: an **OS thread** versus an **async task multiplexed onto a runtime**.

| | `std::thread::spawn` | `tokio::spawn` |
|---|---|---|
| Unit created | OS thread | async task (green thread) |
| Cost | ~8 KiB stack + kernel object, µs to create | a few hundred bytes, ns to create |
| Practical count | thousands | millions |
| Takes | `FnOnce() -> T + Send + 'static` | `Future<Output = T> + Send + 'static` |
| Needs a runtime | no | yes — panics outside a Tokio context |
| Scheduling | preemptive, by the OS | cooperative, at `.await` points |
| Handle | `JoinHandle<T>`, `.join()` blocks | `JoinHandle<Result<T, JoinError>>`, `.await`s |
| On drop of handle | thread keeps running | task keeps running (detached) |
| Cancellation | none — cannot be killed | `handle.abort()` |

### Code

```rust
// OS thread
let h = std::thread::spawn(|| {
    std::thread::sleep(std::time::Duration::from_secs(1));
    41 + 1
});
let v: i32 = h.join().unwrap();
```

```rust
// Tokio task
#[tokio::main]
async fn main() {
    let h = tokio::spawn(async {
        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
        41 + 1
    });
    let v: i32 = h.await.unwrap();   // Result: Err if the task panicked or was aborted
    println!("{v}");
}
```

Note the extra `Result`: a panicking Tokio task does not kill the process, it surfaces as `JoinError`.

### The Cardinal Sin: Blocking in Async

A Tokio worker thread runs many tasks. Blocking it starves all of them:

```rust
tokio::spawn(async {
    std::thread::sleep(Duration::from_secs(5));   // BAD — blocks a worker
    std::fs::read_to_string("big.txt").unwrap();  // BAD — blocking syscall
});
```

Fix it with the async equivalents, or move the blocking work off the async pool:

```rust
tokio::time::sleep(Duration::from_secs(5)).await;      // async sleep
tokio::fs::read_to_string("big.txt").await?;           // async I/O

let hash = tokio::task::spawn_blocking(|| {
    expensive_cpu_bound_hash()      // runs on the blocking pool
}).await?;
```

### Choosing

- **CPU-bound parallelism** — OS threads, or `rayon`.
- **Many concurrent I/O operations** (sockets, HTTP requests, DB queries) — async tasks.
- **`tokio::task::spawn_local`** for `!Send` futures on a `LocalSet`.

<br>

## 27. How do _channels_ work in _Rust_ and what types of channels are available?

A channel is a **typed queue** connecting a sender and a receiver, moving ownership of values between threads or tasks. This is the "do not communicate by sharing memory; share memory by communicating" style.

### `std::sync::mpsc` — Multi-Producer, Single-Consumer

```rust
use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();

    for i in 0..3 {
        let tx = tx.clone();       // multi-producer: clone the sender
        thread::spawn(move || {
            tx.send(format!("from worker {i}")).unwrap();
        });
    }
    drop(tx);                      // drop the original, or the loop never ends

    for msg in rx {                // iterates until every sender is dropped
        println!("{msg}");
    }
}
```

Key mechanics:

- `send` moves the value; the sender no longer owns it.
- `recv()` **blocks** until a message arrives or all senders drop (then `Err`).
- `try_recv()` returns immediately; `recv_timeout(d)` waits with a deadline.
- Dropping all senders closes the channel — that is how the receiving loop terminates.

### Bounded / Synchronous

```rust
let (tx, rx) = mpsc::sync_channel(4);   // capacity 4; send() blocks when full
```

Capacity `0` makes it a **rendezvous** channel: each `send` blocks until a matching `recv`. Bounded channels give you backpressure, which is usually what you want in a pipeline.

### The Available Kinds

| Channel | Crate | Shape | Notes |
|---|---|---|---|
| `mpsc::channel` | std | multi-producer, single-consumer, unbounded | blocking |
| `mpsc::sync_channel(n)` | std | same, bounded | backpressure |
| `crossbeam_channel` | crossbeam | **mpmc** | faster, has `select!`, `after`, `tick` |
| `tokio::sync::mpsc` | tokio | mpsc, bounded or unbounded | `send().await`, `recv().await` |
| `tokio::sync::oneshot` | tokio | exactly one value | request/response, cancellation |
| `tokio::sync::broadcast` | tokio | every receiver sees every message | fan-out; lags if slow |
| `tokio::sync::watch` | tokio | latest value only | config reload, shutdown flags |
| `flume` | flume | mpmc, sync **and** async | bridges both worlds |

### Async Example

```rust
use tokio::sync::mpsc;

#[tokio::main]
async fn main() {
    let (tx, mut rx) = mpsc::channel::<u32>(32);

    tokio::spawn(async move {
        for i in 0..5 {
            if tx.send(i).await.is_err() { break; }   // Err: receiver dropped
        }
    });

    while let Some(v) = rx.recv().await {
        println!("got {v}");
    }
}
```

### `oneshot` for a Reply

```rust
use tokio::sync::oneshot;

let (reply_tx, reply_rx) = oneshot::channel();
tokio::spawn(async move { let _ = reply_tx.send(compute()); });
let answer = reply_rx.await.unwrap();
```

<br>

## 28. What is `async/await` and how does it work in _Rust_?

`async`/`await` is Rust's syntax for **cooperative, non-blocking concurrency**. It lets one thread make progress on thousands of I/O operations by suspending a task whenever it would otherwise wait.

### The Two Keywords

- **`async fn`** / **`async {}`** — does not run the body; it returns a value implementing `Future`.
- **`.await`** — polls that future, yielding control back to the executor if it is not ready.

```rust
async fn fetch(url: &str) -> Result<String, reqwest::Error> {
    let body = reqwest::get(url).await?.text().await?;
    Ok(body)
}
```

> **Futures are lazy.** Calling `fetch(url)` performs *no work at all* until it is `.await`ed or spawned. This is the single most common beginner surprise — unlike a JavaScript promise, which starts immediately.

### The `Future` Trait

```rust
pub trait Future {
    type Output;
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output>;
}

pub enum Poll<T> { Ready(T), Pending }
```

The compiler transforms an `async fn` into a **state machine**: each `.await` becomes a state, and the local variables live across suspension inside the generated struct. When `poll` returns `Pending`, the future has registered the `Waker` from `Context` with something (a socket, a timer); when that resource is ready, the waker tells the executor to poll again. Nothing spins.

`Pin` appears because that state machine may be **self-referential** (a local borrow held across an `.await`), so it must not move once polled.

### No Runtime in `std`

Rust ships the `Future` trait and the syntax, but **no executor**. You pick one:

```toml
[dependencies]
tokio = { version = "1", features = ["full"] }
```

```rust
#[tokio::main]
async fn main() {
    println!("{}", fetch("https://example.com").await.unwrap());
}
```

`#[tokio::main]` expands to a normal `fn main` that builds a runtime and calls `block_on`.

### Concurrency vs. Parallelism

Sequential — 3 seconds:

```rust
let a = fetch("a").await?;
let b = fetch("b").await?;
```

Concurrent on one task — ~1 second:

```rust
let (a, b) = tokio::join!(fetch("a"), fetch("b"));
```

Parallel across worker threads, and cancellable:

```rust
let h1 = tokio::spawn(fetch("a".into()));
let h2 = tokio::spawn(fetch("b".into()));
let (a, b) = (h1.await??, h2.await??);
```

Race them, dropping the loser:

```rust
tokio::select! {
    r = fetch("primary")  => println!("{r:?}"),
    r = fetch("fallback") => println!("{r:?}"),
    _ = tokio::time::sleep(Duration::from_secs(2)) => println!("timed out"),
}
```

### Practical Rules

- Never block inside `async` — use `spawn_blocking` for CPU-heavy or blocking work.
- Do not hold a `std::sync::MutexGuard` across an `.await` (the guard is `!Send`); use `tokio::sync::Mutex` when a lock must span a suspension point.
- Async traits are supported natively since Rust 1.75 for most cases; `async-trait` is still used where `dyn` compatibility is needed.
- Dropping a future **cancels** it at its last suspension point — write code that tolerates that.

<br>

## 29. What is the purpose of the `Mutex` type in _Rust_?

A `Mutex<T>` provides **mutual exclusion**: only one thread at a time can access the data. Rust's version is distinctive in that the mutex **owns** the data it protects.

### Ownership Is the Design

In C or Java, the lock and the data are separate, and nothing stops you from touching the data without holding the lock. In Rust the value lives inside the `Mutex`, so the only way to reach it is `lock()`:

```rust
use std::sync::Mutex;

let m = Mutex::new(5);
{
    let mut guard = m.lock().unwrap();   // returns MutexGuard<i32>
    *guard += 1;
}   // guard dropped here — lock released automatically (RAII)

println!("{:?}", m.lock().unwrap());     // 6
```

`MutexGuard` implements `Deref`/`DerefMut` and unlocks in its `Drop` — so "forgot to unlock" is not a bug you can write, even on an early return or a panic.

### Sharing Across Threads: `Arc<Mutex<T>>`

```rust
use std::sync::{Arc, Mutex};
use std::thread;

let counter = Arc::new(Mutex::new(0));
let mut handles = vec![];

for _ in 0..10 {
    let c = Arc::clone(&counter);
    handles.push(thread::spawn(move || {
        *c.lock().unwrap() += 1;
    }));
}
for h in handles { h.join().unwrap(); }
assert_eq!(*counter.lock().unwrap(), 10);
```

`Arc` gives shared **ownership**; `Mutex` gives exclusive **access**. Neither replaces the other.

### Poisoning

`lock()` returns `LockResult` because a mutex becomes **poisoned** if a thread panics while holding it — the data may be in a half-updated state. `unwrap()` propagates that panic; `.into_inner()` on the error recovers the value anyway if you know it is fine.

### Relatives

| Type | Use |
|---|---|
| `RwLock<T>` | many concurrent readers **or** one writer — good for read-heavy data |
| `AtomicUsize`, `AtomicBool` | lock-free counters and flags; far cheaper than a mutex |
| `tokio::sync::Mutex` | when the lock must be held **across an `.await`** |
| `parking_lot::Mutex` | smaller, faster, no poisoning |
| `OnceLock<T>` | one-time initialization without a lock on the read path |

### Pitfalls

- **Do not hold a `std::sync::Mutex` guard across `.await`** — its guard is `!Send`, and the compiler will tell you so.
- **Lock ordering**: two locks acquired in opposite orders by two threads is a classic deadlock. Rust does not prevent it.
- **Scope the guard**: hold the lock for as short a span as possible; use an inner block or drop it explicitly before doing slow work.

<br>
## 30. What are _traits_ in _Rust_?

A **trait** defines shared behaviour: a set of method signatures a type can implement. It is Rust's answer to interfaces and typeclasses, and it is the backbone of generics, operator overloading, and dynamic dispatch.

### Defining and Implementing

```rust
trait Summary {
    fn summarize_author(&self) -> String;      // required

    fn summarize(&self) -> String {            // default method
        format!("(Read more from {}...)", self.summarize_author())
    }
}

struct Tweet { username: String, content: String }

impl Summary for Tweet {
    fn summarize_author(&self) -> String { format!("@{}", self.username) }
}

let t = Tweet { username: "rustlang".into(), content: "1.0 released".into() };
println!("{}", t.summarize());   // uses the default implementation
```

### Static vs. Dynamic Dispatch

```rust
// Static: monomorphized, one specialized copy per concrete type, inlinable
fn notify<T: Summary>(item: &T) { println!("Breaking! {}", item.summarize()); }
fn notify_sugar(item: &impl Summary) { println!("{}", item.summarize()); }

// Dynamic: one copy, vtable lookup at runtime, heterogeneous collections possible
fn notify_dyn(item: &dyn Summary) { println!("{}", item.summarize()); }
let feed: Vec<Box<dyn Summary>> = vec![Box::new(t)];
```

Prefer generics by default; reach for `dyn Trait` when you need a collection of mixed types, want to cut compile time and binary size, or must cross an API boundary without generics.

### Std Traits Worth Knowing

| Trait | Purpose |
|---|---|
| `Debug`, `Display` | `{:?}` and `{}` formatting |
| `Clone`, `Copy` | duplication semantics |
| `PartialEq`, `Eq`, `PartialOrd`, `Ord` | comparison and sorting |
| `Default` | `T::default()` |
| `From` / `Into` | conversions (implement `From`, get `Into` free) |
| `Iterator`, `IntoIterator` | `for` loops and adapters |
| `Deref` | smart-pointer transparency |
| `Drop` | destructors |
| `Send`, `Sync` | thread-safety markers |
| `Add`, `Mul`, `Index`, … | operator overloading |

### Deriving

```rust
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
struct Point { x: i32, y: i32 }
```

### Object Safety

Only **dyn-compatible** (object-safe) traits can become `dyn Trait`: no generic methods, no `Self` return types, no associated constants. That is why `Clone` cannot be used as `dyn Clone`.

<br>

## 31. How do you define and implement a _generic function_ or _struct_ in _Rust_?

Generics let one definition work over many types, with **no runtime cost** — the compiler monomorphizes each concrete instantiation into its own specialized machine code.

### Generic Functions

```rust
fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list {
        if item > largest { largest = item; }
    }
    largest
}

println!("{}", largest(&[34, 50, 25]));            // T = i32
println!("{}", largest(&['y', 'm', 'a']));         // T = char
```

The `PartialOrd` bound is required — without it the compiler has no reason to believe `>` exists for `T`.

### Generic Structs

```rust
struct Point<T> { x: T, y: T }

impl<T> Point<T> {
    fn new(x: T, y: T) -> Self { Point { x, y } }
    fn x(&self) -> &T { &self.x }
}

// An impl block restricted to a subset of types
impl Point<f64> {
    fn distance_from_origin(&self) -> f64 { (self.x.powi(2) + self.y.powi(2)).sqrt() }
}

// Conditional methods: available only when T satisfies the bound
impl<T: std::fmt::Display + PartialOrd> Point<T> {
    fn cmp_display(&self) { if self.x >= self.y { println!("x = {}", self.x); } }
}
```

Multiple parameters, and a method that introduces its own:

```rust
struct Pair<T, U> { first: T, second: U }

impl<T, U> Pair<T, U> {
    fn mixup<V, W>(self, other: Pair<V, W>) -> Pair<T, W> {
        Pair { first: self.first, second: other.second }
    }
}
```

### Generic Enums and Traits

```rust
enum MyOption<T> { Some(T), None }
enum MyResult<T, E> { Ok(T), Err(E) }

trait Container<T> {
    fn get(&self, i: usize) -> Option<&T>;
    fn len(&self) -> usize;
}
```

### `where` Clauses

When bounds get long, move them out of the signature:

```rust
fn process<T, U>(t: &T, u: &U) -> String
where
    T: std::fmt::Display + Clone,
    U: Clone + std::fmt::Debug,
{
    format!("{t} {u:?}")
}
```

### Const Generics

Generic over a **value**, not just a type:

```rust
fn sum_array<const N: usize>(arr: [i32; N]) -> i32 { arr.iter().sum() }

println!("{}", sum_array([1, 2, 3]));       // N = 3
println!("{}", sum_array([1, 2, 3, 4]));    // N = 4
```

### Monomorphization

`largest(&[1,2])` and `largest(&['a','b'])` compile to two separate functions with no dispatch overhead. The trade-off is compile time and binary size — the reason a very generic crate can be slow to build.

<br>

## 32. What are _associated types_ in _Rust_ and how are they different from _generics_?

An **associated type** is a type placeholder chosen by the *implementer* of a trait, not by the caller.

```rust
pub trait Iterator {
    type Item;                                  // associated type
    fn next(&mut self) -> Option<Self::Item>;
}

impl Iterator for Counter {
    type Item = u32;                            // fixed once, by the impl
    fn next(&mut self) -> Option<u32> { /* ... */ }
}
```

### The Core Difference

| | Associated type | Generic parameter |
|---|---|---|
| Chosen by | the **implementer** | the **caller** |
| Impls per type | **one** | many |
| Written as | `type Item;` | `trait Foo<T>` |
| Referred to as | `Self::Item`, `T::Item` | `T` |
| Inference | inferred from `Self` | must often be annotated |

### Same Trait, Both Ways

**Associated type** — a type can be an `Iterator` of exactly one item type:

```rust
trait Container { type Item; fn get(&self, i: usize) -> Option<&Self::Item>; }

impl Container for Stack {
    type Item = i32;
    fn get(&self, i: usize) -> Option<&i32> { self.items.get(i) }
}
```

**Generic parameter** — a type can implement it many times:

```rust
trait Container<T> { fn get(&self, i: usize) -> Option<&T>; }

impl Container<i32> for Stack { /* ... */ }
impl Container<String> for Stack { /* also allowed */ }
```

That is exactly why `From` is generic (`String: From<&str>`, `String: From<char>`, …) while `Iterator::Item` is associated (a `Vec<i32>` iterator yields `i32` and nothing else).

### The Ergonomic Payoff

With an associated type, downstream signatures stay clean:

```rust
fn sum_all<C: Container<Item = i32>>(c: &C) -> i32 { /* ... */ }   // associated
fn sum_all<C: Container<i32>>(c: &C) -> i32 { /* ... */ }          // generic
```

The difference grows with the number of parameters: had `Iterator` been `Iterator<Item>`, every adapter signature in the standard library would need to thread that parameter through.

### Choosing

- Use an **associated type** when there is exactly one sensible choice per implementing type — `Iterator::Item`, `Deref::Target`, `Add::Output`.
- Use a **generic parameter** when a type should implement the trait many times over — `From<T>`, `TryFrom<T>`, `AsRef<T>`.
- They combine: `Add<Rhs = Self> { type Output; }` — generic in the right-hand operand, associated in the result.

<br>

## 33. Explain _Rust's orphan rule_ in the context of trait implementations.

The **orphan rule** says: you may write `impl Trait for Type` only if **the trait or the type (or both) is local to your crate**. You cannot implement someone else's trait for someone else's type.

```rust
// In your crate:
impl MyTrait for String  { }   // OK — trait is local
impl Display for MyType  { }   // OK — type is local
impl Display for Vec<u8> { }   // ERROR[E0117] — both are foreign
```

### Why It Exists — Coherence

Rust guarantees that for any (trait, type) pair there is **at most one** implementation in the whole program. Without the orphan rule, crate A and crate B could each `impl Display for Vec<u8>`, and a binary depending on both would have two answers to the same question. Which one runs? Linking would break, or worse, silently pick one. The rule also protects semver: adding an impl in an upstream crate could otherwise break unrelated downstream crates.

### The Newtype Workaround

Wrap the foreign type in a local tuple struct — it costs nothing at runtime:

```rust
use std::fmt;

struct Wrapper(Vec<String>);        // local type

impl fmt::Display for Wrapper {     // now legal
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}

let w = Wrapper(vec!["a".into(), "b".into()]);
println!("{w}");    // [a, b]
```

Add `Deref` to get the inner type's methods for free:

```rust
impl std::ops::Deref for Wrapper {
    type Target = Vec<String>;
    fn deref(&self) -> &Self::Target { &self.0 }
}
println!("{}", w.len());    // works via Deref
```

### The Fundamental-Type Relaxation

`&T`, `&mut T`, and `Box<T>` are `#[fundamental]`, so they are "transparent" for the rule: if you may implement a trait for `T`, you may implement it for `&T` too. And a local type appearing as a **type parameter** is often enough:

```rust
impl From<MyType> for Vec<u8> { }   // OK — MyType is local and appears in the impl
```

### In Practice

- Writing a serializer for a foreign type with serde? Use `#[serde(with = "...")]` or the newtype.
- Need an extension method on a foreign type? Define a **local extension trait** and implement it for the foreign type — legal, because the trait is yours.

```rust
trait VecExt { fn second(&self) -> Option<&i32>; }
impl VecExt for Vec<i32> { fn second(&self) -> Option<&i32> { self.get(1) } }
```

<br>

## 34. Describe how to use _trait bounds_ in _Rust_.

A **trait bound** constrains a generic parameter, telling the compiler which capabilities a type must have. Rust checks bounds at the *definition* site, so a generic function that compiles is guaranteed to work for every type satisfying its bounds.

### The Syntactic Forms

```rust
// 1. Inline
fn print_it<T: std::fmt::Display>(x: T) { println!("{x}"); }

// 2. Multiple bounds with +
fn show<T: std::fmt::Display + Clone + PartialOrd>(x: T) { /* ... */ }

// 3. where clause — clearest when bounds get long
fn process<T, U>(t: T, u: U) -> i32
where
    T: std::fmt::Display + Clone,
    U: Clone + std::fmt::Debug,
{ 0 }

// 4. impl Trait in argument position (anonymous generic)
fn notify(item: &impl std::fmt::Display) { println!("{item}"); }

// 5. impl Trait in return position — "some concrete type I won't name"
fn make_adder(n: i32) -> impl Fn(i32) -> i32 { move |x| x + n }
```

### Bounds on Types, Impls, and Associated Types

```rust
struct Wrapper<T: Clone> { value: T }           // on the struct

impl<T: std::fmt::Display> Wrapper<T> {         // on the impl block
    fn show(&self) { println!("{}", self.value); }
}

fn sum_ints<I>(iter: I) -> i32
where
    I: IntoIterator<Item = i32>,                // associated-type bound
{ iter.into_iter().sum() }
```

### Conditional (Blanket) Implementations

Implement a trait only for types meeting a bound — this is how `ToString` exists for everything `Display`:

```rust
impl<T: std::fmt::Display> ToString for T {
    fn to_string(&self) -> String { /* ... */ }
}
```

### Supertraits

Requiring one trait to imply another:

```rust
trait Loggable: std::fmt::Display {          // Display is a supertrait
    fn log(&self) { println!("[LOG] {self}"); }
}
```

### Lifetime Bounds

```rust
fn longest<'a, T>(x: &'a T, y: &'a T) -> &'a T where T: PartialOrd { /* ... */ }

fn spawn_it<T: Send + 'static>(v: T) { std::thread::spawn(move || drop(v)); }
```

`T: 'static` means "T contains no references shorter than the program" — it does **not** mean the value lives forever.

### Practical Advice

Bound as loosely as the body actually requires. Demanding `T: Clone` when you never clone shrinks the set of usable types for no benefit. When an error says `the trait bound X: Y is not satisfied`, the fix is either adding the bound to your signature or implementing the trait for `X`.

<br>

## 35. What are _enums_ and how are they used in _Rust_?

A Rust `enum` is an **algebraic data type**: a value that is exactly one of several variants, and each variant may carry its own data. This is far more powerful than a C enum, which is only a named integer.

### Variant Shapes

```rust
enum Message {
    Quit,                          // unit
    Move { x: i32, y: i32 },       // struct-like
    Write(String),                 // tuple-like
    ChangeColor(u8, u8, u8),       // multiple fields
}
```

Each variant is a distinct shape, but all are the same type `Message` — so they can live in one `Vec<Message>`.

### Methods on Enums

```rust
impl Message {
    fn call(&self) -> String {
        match self {
            Message::Quit => "quitting".to_string(),
            Message::Move { x, y } => format!("moving to ({x}, {y})"),
            Message::Write(text) => format!("writing: {text}"),
            Message::ChangeColor(r, g, b) => format!("color #{r:02x}{g:02x}{b:02x}"),
        }
    }
}
```

### The Two You Use Every Day

```rust
enum Option<T> { Some(T), None }
enum Result<T, E> { Ok(T), Err(E) }
```

### Modelling State Machines

Enums shine at making illegal states unrepresentable:

```rust
enum Connection {
    Disconnected,
    Connecting { attempt: u32 },
    Connected { session: SessionId, since: Instant },
    Failed(io::Error),
}
```

A struct with `is_connected: bool`, `session: Option<SessionId>`, and `error: Option<Error>` permits nonsense combinations; the enum does not.

### C-Like Enums with Discriminants

```rust
#[derive(Debug, Clone, Copy)]
enum Status { Active = 1, Inactive = 0, Banned = 99 }

let code = Status::Banned as i32;   // 99
```

### Memory Layout

An enum's size is (largest variant + discriminant), rounded for alignment — one allocation, no boxing. **Niche optimization** removes even the discriminant when a variant has an impossible bit pattern, which is why `Option<Box<T>>` is the same size as `Box<T>`.

### Useful Attributes

```rust
#[derive(Debug, Clone, PartialEq)]
#[non_exhaustive]                  // downstream matches must keep a `_` arm
enum ApiError { NotFound, RateLimited { retry_after: u64 } }
```

Recursive enums need indirection, since the size must be finite:

```rust
enum Tree { Leaf(i32), Node(Box<Tree>, Box<Tree>) }
```

<br>

## 36. How does _pattern matching_ work with enums in _Rust_?

`match` on an enum simultaneously **tests which variant** it is and **binds the data inside it** — with exhaustiveness checked by the compiler.

### Basic Form

```rust
enum Shape {
    Circle(f64),
    Rect { w: f64, h: f64 },
    Triangle(f64, f64, f64),
}

fn area(s: &Shape) -> f64 {
    match s {
        Shape::Circle(r) => std::f64::consts::PI * r * r,
        Shape::Rect { w, h } => w * h,
        Shape::Triangle(a, b, c) => {
            let p = (a + b + c) / 2.0;
            (p * (p - a) * (p - b) * (p - c)).sqrt()
        }
    }
}
```

### Exhaustiveness — the Real Value

Add a variant to `Shape` and **every** `match` over it fails to compile until you handle the new case. That turns "find all the places that need updating" from an archaeology exercise into a compiler task.

### Guards, Alternatives, Ranges

```rust
match msg {
    Message::Move { x, y } if x == y   => println!("diagonal"),
    Message::Move { x, .. }            => println!("x = {x}"),     // .. ignores the rest
    Message::Write(s) | Message::Log(s) => println!("text: {s}"),  // or-pattern
    Message::Code(n @ 400..=499)       => println!("client error {n}"), // @ binding
    _ => {}
}
```

### Binding Modes

Matching on `&Shape` gives you references to the fields automatically (match ergonomics). When you need the opposite, `ref` and destructuring by value are available:

```rust
let opt = Some(String::from("hi"));

match &opt {
    Some(s) => println!("{s}"),   // s: &String — opt not moved
    None => {}
}

if let Some(s) = opt {            // moves the String out
    println!("{s}");
}
```

### Lighter-Weight Forms

```rust
if let Some(x) = maybe { println!("{x}"); }
else { println!("nothing"); }

while let Some(top) = stack.pop() { println!("{top}"); }

let Some(cfg) = load_config() else {     // let-else: bind or diverge
    return Err("no config".into());
};

// matches! for a boolean test
if matches!(status, Status::Active | Status::Pending) { /* ... */ }
```

### Nested and Multi-Value Matching

```rust
match (a, b) {
    (Some(x), Some(y)) => x + y,
    (Some(x), None) | (None, Some(x)) => x,
    (None, None) => 0,
}

match msg {
    Message::Response { body: Some(Data { id, .. }) } => println!("id {id}"),
    _ => {}
}
```

<br>

## 37. Give an example of a function that uses pattern matching to handle different errors.

Here is a realistic config loader that matches on error *kinds* and recovers where it sensibly can.

```rust
use std::fs::{self, File};
use std::io::{self, ErrorKind, Write};

/// Load the config file, creating a default one if it does not exist.
fn load_or_create_config(path: &str) -> Result<String, io::Error> {
    match fs::read_to_string(path) {
        Ok(contents) => Ok(contents),

        Err(e) => match e.kind() {
            // Recoverable: write a default and return it
            ErrorKind::NotFound => {
                let default = "port = 8080\nhost = \"localhost\"\n";
                let mut f = File::create(path)?;
                f.write_all(default.as_bytes())?;
                Ok(default.to_string())
            }
            // Actionable: give the user a better message
            ErrorKind::PermissionDenied => Err(io::Error::new(
                ErrorKind::PermissionDenied,
                format!("cannot read {path}: check file permissions"),
            )),
            // Transient: let the caller decide whether to retry
            ErrorKind::Interrupted => Err(e),
            // Everything else: propagate untouched
            _ => Err(e),
        },
    }
}
```

### Matching Across Several Error Types

A domain error enum lets one `match` cover failures from different layers:

```rust
use std::num::ParseIntError;

#[derive(Debug)]
enum ConfigError {
    Io(io::Error),
    BadPort(ParseIntError),
    OutOfRange { field: &'static str, value: i64 },
}

impl From<io::Error> for ConfigError {
    fn from(e: io::Error) -> Self { ConfigError::Io(e) }
}
impl From<ParseIntError> for ConfigError {
    fn from(e: ParseIntError) -> Self { ConfigError::BadPort(e) }
}

fn read_port(path: &str) -> Result<u16, ConfigError> {
    let text = fs::read_to_string(path)?;              // io::Error   -> ConfigError
    let raw: i64 = text.trim().parse()?;               // ParseIntError -> ConfigError
    if !(1..=65535).contains(&raw) {
        return Err(ConfigError::OutOfRange { field: "port", value: raw });
    }
    Ok(raw as u16)
}

fn main() {
    match read_port("config.toml") {
        Ok(port) => println!("listening on {port}"),

        Err(ConfigError::Io(e)) if e.kind() == ErrorKind::NotFound =>
            println!("no config, defaulting to 8080"),

        Err(ConfigError::Io(e)) =>
            eprintln!("I/O failure: {e}"),

        Err(ConfigError::BadPort(e)) =>
            eprintln!("port is not a number: {e}"),

        Err(ConfigError::OutOfRange { field, value }) =>
            eprintln!("{field} = {value} is outside 1..=65535"),
    }
}
```

Note the three techniques working together: **variant matching** to separate categories, a **match guard** (`if e.kind() == ...`) to split one variant by a runtime property, and **destructuring** to pull `field` and `value` straight out of the error.

### Downcasting a Boxed Error

When the error is erased behind a trait object, recover the concrete type:

```rust
fn handle(e: Box<dyn std::error::Error>) {
    if let Some(io_err) = e.downcast_ref::<io::Error>() {
        eprintln!("io: {:?}", io_err.kind());
    } else {
        eprintln!("other: {e}");
    }
}
```

<br>

## 38. Can you explain _destructuring_ in the context of pattern matching in _Rust_?

**Destructuring** breaks a composite value apart and binds its pieces to names. It works anywhere a *pattern* is allowed — not only in `match`.

### Where Patterns Appear

```rust
let (a, b, c) = (1, 2, 3);                       // let
let Point { x, y } = p;                          // let, struct
for (i, item) in v.iter().enumerate() { }        // for
fn dist(Point { x, y }: &Point) -> f64 { }       // function parameter
if let Some(v) = opt { }                         // if let
while let Some(v) = stack.pop() { }              // while let
let Some(v) = opt else { return };               // let-else
|&(a, b)| a + b                                  // closure parameter
```

### Structs

```rust
struct Point { x: i32, y: i32, z: i32 }
let p = Point { x: 1, y: 2, z: 3 };

let Point { x, y, z } = p;                  // shorthand — names match fields
let Point { x: a, y: b, z: c } = p;         // rename while binding
let Point { x, .. } = p;                    // ignore the rest

match p {
    Point { x: 0, y: 0, .. } => println!("on the z axis"),
    Point { x, y: 0, .. }    => println!("on the x axis at {x}"),
    Point { x, y, z }        => println!("({x}, {y}, {z})"),
}
```

### Tuples, Slices, and Nesting

```rust
let ((feet, inches), Point { x, y, .. }) = ((3, 10), p);

match v.as_slice() {
    []              => println!("empty"),
    [only]          => println!("one: {only}"),
    [first, .., last] => println!("{first}..{last}"),
    [head, tail @ ..] => println!("{head} then {} more", tail.len()),
}

match msg {
    Message::Response { body: Some(Data { id, name }), status: 200 } =>
        println!("ok: {id} {name}"),
    _ => {}
}
```

### Ignoring and Binding

```rust
let (first, _, third) = triple;        // _ discards, does not bind (no move)
let (a, ..) = quad;                    // .. discards a range of elements

match age {
    n @ 0..=12  => println!("child, {n}"),     // @ binds AND tests
    n @ 13..=19 => println!("teen, {n}"),
    n           => println!("adult, {n}"),
}
```

> `_` alone does **not** move the value, but `let _x = value;` does. That distinction matters when the value holds a lock or a file handle: `let _ = guard;` drops it immediately, while `let _guard = guard;` keeps it alive to the end of scope.

### Refutability

- **Irrefutable** patterns always match — required by `let`, `for`, and function parameters.
- **Refutable** patterns may fail — allowed in `match` arms, `if let`, `while let`, `let-else`.

```rust
let Some(x) = opt;         // error: refutable pattern in local binding
let Some(x) = opt else { return };   // fine — the else arm handles failure
```

### Nested Mutation

Destructuring gives disjoint `&mut` borrows of separate fields, which the borrow checker accepts:

```rust
let Point { x, y, .. } = &mut p;
*x += 1;
*y += 1;
```

<br>

## 39. What are _macros_ in _Rust_ and how do you define them?

A macro is **code that writes code**: it runs at compile time, taking token trees in and emitting Rust source. Unlike a function, it can accept a variable number of arguments, take arbitrary syntax, and generate items (structs, impls, functions).

### Why Macros Exist

Functions cannot do these things:

- **Variadic arguments** — `println!("{} {} {}", a, b, c)`.
- **Compile-time checking of a format string** against its arguments.
- **Generating items** — `#[derive(Debug)]` writes a whole `impl` block.
- **Custom syntax** — `vec![1; 10]`, `matches!(x, Some(_))`.

### Declarative Macros: `macro_rules!`

They are pattern-matching over syntax, so the shape reads like a `match`:

```rust
macro_rules! my_vec {
    () => { Vec::new() };

    ($elem:expr; $n:expr) => { std::vec::from_elem($elem, $n) };

    ($($x:expr),+ $(,)?) => {{          // one or more, optional trailing comma
        let mut v = Vec::new();
        $( v.push($x); )+               // repeat the body once per match
        v
    }};
}

let a: Vec<i32> = my_vec![];
let b = my_vec![0; 5];
let c = my_vec![1, 2, 3,];
```

**Repetition syntax:** `$( ... )sep rep` where `sep` is an optional separator and `rep` is `*` (zero or more), `+` (one or more), or `?` (zero or one).

### Fragment Specifiers

| Specifier | Matches |
|---|---|
| `expr` | an expression |
| `ident` | an identifier or keyword |
| `ty` | a type |
| `pat` | a pattern |
| `stmt` | a statement |
| `block` | a `{ ... }` block |
| `item` | a function, struct, impl, … |
| `path` | a path like `std::io::Read` |
| `literal` | a literal |
| `tt` | a single token tree (the most flexible) |
| `vis` | a visibility qualifier |
| `lifetime` | `'a` |

### A Practical One

```rust
macro_rules! hashmap {
    ($($k:expr => $v:expr),* $(,)?) => {{
        let mut m = std::collections::HashMap::new();
        $( m.insert($k, $v); )*
        m
    }};
}

let ages = hashmap!{ "alice" => 30, "bob" => 25 };
```

### Hygiene

Identifiers introduced *inside* a macro cannot collide with the caller's:

```rust
macro_rules! shadow {
    () => { let x = 99; };
}
let x = 1;
shadow!();
println!("{x}");   // 1 — the macro's `x` is a different variable
```

Rust macros are hygienic for local variables and labels (not for types or item paths, which is why generated code should use fully-qualified paths like `::std::vec::Vec`).

### Scope and Export

`macro_rules!` macros are textually scoped — usable only *after* their definition. Add `#[macro_export]` to make one available crate-wide and to dependents.

### Debugging

```bash
cargo install cargo-expand
cargo expand           # shows the code after macro expansion
```

<br>

## 40. Give an example of when you would use a _macro_ in _Rust_.

Reach for a macro when a **function cannot express the pattern** — variadic arguments, compile-time validation, or generating items. Here are the cases that come up in real code.

### 1. Table-Driven Tests

Generating one `#[test]` function per case is impossible with a function, because tests are *items*:

```rust
macro_rules! parse_tests {
    ($($name:ident: $input:expr => $expected:expr),* $(,)?) => {
        $(
            #[test]
            fn $name() {
                assert_eq!(parse($input), $expected);
            }
        )*
    };
}

parse_tests! {
    empty:      ""      => Err(ParseError::Empty),
    single:     "1"     => Ok(vec![1]),
    multiple:   "1,2,3" => Ok(vec![1, 2, 3]),
}
```

Each case becomes a separately named, separately reported test.

### 2. Reducing Repetitive Trait Impls

```rust
macro_rules! impl_from_error {
    ($($src:ty => $variant:ident),* $(,)?) => {
        $(
            impl From<$src> for AppError {
                fn from(e: $src) -> Self { AppError::$variant(e) }
            }
        )*
    };
}

impl_from_error! {
    std::io::Error        => Io,
    std::num::ParseIntError => Parse,
    serde_json::Error     => Json,
}
```

### 3. Capturing Source Location

`file!()`, `line!()`, and `column!()` evaluate at the *call site* — a function would only ever see its own location:

```rust
macro_rules! trace {
    ($($arg:tt)*) => {
        eprintln!("[{}:{}] {}", file!(), line!(), format_args!($($arg)*))
    };
}

trace!("value = {}", x);   // [src/main.rs:42] value = 7
```

This is exactly how `dbg!`, `assert!`, and the `log` crate's macros work.

### 4. Lazy Evaluation of Arguments

A function evaluates its arguments before the call; a macro need not:

```rust
macro_rules! log_expensive {
    ($lvl:expr, $($arg:tt)*) => {
        if log_enabled($lvl) { println!($($arg)*); }   // args skipped when disabled
    };
}
```

### 5. Domain-Specific Syntax

```rust
let routes = router! {
    GET  "/users"     => list_users,
    POST "/users"     => create_user,
    GET  "/users/:id" => get_user,
};
```

### 6. Compile-Time Validation (Procedural Macros)

`sqlx::query!` checks your SQL against a live database schema *at compile time*, so a typo in a column name is a build error rather than a 3 a.m. page. `serde`'s `#[derive(Serialize)]` generates hundreds of lines of correct, specialized code you never see.

### When *Not* to Use a Macro

If a generic function with trait bounds does the job, use the function. Macros cost you: worse error messages, no type checking until expansion, IDE support that degrades, and readers who must expand the macro mentally. The bar is "a function genuinely cannot do this".

<br>

## 41. What is the difference between _declarative macros_ and _procedural macros_ in _Rust_?

| | Declarative (`macro_rules!`) | Procedural |
|---|---|---|
| Defined by | pattern → template rules | a Rust **function** over `TokenStream` |
| Lives in | any module | a dedicated crate with `proc-macro = true` |
| Input | matched fragments | raw tokens |
| Power | syntactic substitution | arbitrary computation, file I/O, DB queries |
| Compile cost | cheap | needs `syn`/`quote`, slower builds |
| Hygiene | automatic for locals | manual, via spans |
| Can derive traits | no | yes |

### Declarative

```rust
macro_rules! square {
    ($x:expr) => { $x * $x };
}
let n = square!(4);   // 16
```

Simple, local, no dependencies. Limited to rearranging the tokens it was given.

### Procedural — Three Kinds

**1. Derive macros** — `#[derive(MyTrait)]`:

```rust
// my_derive/src/lib.rs   (Cargo.toml: [lib] proc-macro = true)
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Hello)]
pub fn derive_hello(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let name = &ast.ident;
    quote! {
        impl Hello for #name {
            fn hello(&self) { println!("Hello from {}", stringify!(#name)); }
        }
    }.into()
}
```

```rust
#[derive(Hello)]
struct Pancakes;

Pancakes.hello();
```

**2. Attribute macros** — replace the item they annotate:

```rust
#[route(GET, "/health")]
fn health() -> &'static str { "ok" }
```

`#[tokio::main]` is one: it rewrites your `async fn main` into a synchronous `main` that builds a runtime.

**3. Function-like macros** — look like `macro_rules!` at the call site, but run arbitrary code:

```rust
let q = sql!(SELECT * FROM users WHERE id = 1);
```

### The Real Distinction

A declarative macro can only *rearrange* tokens. A procedural macro is a compiler plugin: it can parse the input into a syntax tree, inspect field types, read files, and query a database. That is how `serde` inspects every field's type to generate a specialized serializer, and how `sqlx::query!` validates SQL against a real schema at compile time.

### Choosing

- Start with `macro_rules!` — no extra crate, fast to compile, easy to read.
- Move to a proc macro when you need to **derive a trait**, **inspect types**, or **validate at compile time**.
- Debug both with `cargo expand`.

<br>

## 42. How does _Rust_ achieve _memory safety_ without a garbage collector?

Rust replaces runtime collection with **compile-time reasoning**. Five mechanisms do the work together.

### 1. Ownership and RAII

Every value has one owner; when the owner goes out of scope, the value is dropped and its memory freed. Allocation and deallocation points are known statically — this is C++'s RAII, made mandatory and checked.

```rust
{
    let s = String::from("hello");   // heap allocation
    // ...
}                                    // drop(s) inserted here by the compiler
```

### 2. Move Semantics

Assignment transfers ownership rather than duplicating a pointer, so **double-free is impossible**:

```rust
let a = String::from("hi");
let b = a;              // ownership moves
// println!("{a}");     // error — `a` is no longer valid
```

Only one variable can free the buffer, because only one owner exists.

### 3. Borrowing and Lifetimes

References are non-owning and are statically proven not to outlive their referent, which eliminates **use-after-free** and **dangling pointers**. The aliasing rule (`&mut` is exclusive) eliminates **iterator invalidation** and **data races**.

```rust
let mut v = vec![1, 2, 3];
let first = &v[0];
v.push(4);            // rejected: push may reallocate
println!("{first}");
```

### 4. No Null

`Option<T>` replaces null pointers, so **null dereference** does not exist in safe Rust. The compiler forces the `None` branch to be handled.

### 5. Checked Access at the Edges

Where static proof is impossible, Rust inserts a **bounds check** and panics on violation — a defined crash instead of undefined behaviour. Overflow panics in debug builds. These are the only meaningful runtime costs, and LLVM elides most of them in loops it can prove safe.

### The Cost Comparison

| | Rust | GC languages | C/C++ |
|---|---|---|---|
| Pauses | none | collection pauses | none |
| Memory overhead | ~0 | 2–5× heap headroom | ~0 |
| Freed when | scope exit (deterministic) | eventually | manually |
| Safety | compiler-enforced | runtime-enforced | programmer-enforced |

### Where Runtime Bookkeeping Is Opt-In

`Rc<T>`/`Arc<T>` use reference counting when a single owner is genuinely wrong (graphs, shared caches), and `RefCell<T>` moves borrow checking to runtime. You pay only where you ask for it.

### And the Escape Hatch

`unsafe` lets you dereference raw pointers and call FFI, moving the proof burden from the compiler to you. The discipline is to wrap a small `unsafe` core in a safe API — `Vec`, `RefCell`, and `Mutex` are all built exactly this way.

> **Honest scope:** Rust guarantees no use-after-free, no double-free, no data races, no null dereference, no buffer overflow. It does **not** prevent memory *leaks* — `Box::leak` and `Rc` cycles are safe, just wasteful. Leaking memory is not memory-unsafe.

<br>

## 43. Describe the concept of _reference counting_ in _Rust_.

**Reference counting** provides *shared ownership*: a value stays alive until the last owner drops, at which point the count reaches zero and the value is freed.

### `Rc<T>` — Single-Threaded

```rust
use std::rc::Rc;

let a = Rc::new(vec![1, 2, 3]);
println!("{}", Rc::strong_count(&a));   // 1

let b = Rc::clone(&a);                  // cheap: bumps the count, no deep copy
println!("{}", Rc::strong_count(&a));   // 2

{
    let _c = Rc::clone(&a);
    println!("{}", Rc::strong_count(&a));  // 3
}                                          // _c dropped
println!("{}", Rc::strong_count(&a));      // 2
```

`Rc::clone(&a)` is the idiomatic spelling (rather than `a.clone()`) precisely because it signals "counter increment", not "deep copy".

### `Arc<T>` — Thread-Safe

Identical API, but the counter uses atomic operations, so `Arc` is `Send + Sync`:

```rust
use std::sync::Arc;
use std::thread;

let data = Arc::new(vec![1, 2, 3]);
for _ in 0..3 {
    let d = Arc::clone(&data);
    thread::spawn(move || println!("{:?}", d));
}
```

Use `Rc` when single-threaded — atomics cost real cycles and `Rc` avoids them.

### Immutability, and Getting Around It

`Rc<T>` hands out only `&T`. To mutate, pair it with interior mutability:

```rust
use std::cell::RefCell;
use std::rc::Rc;

let shared = Rc::new(RefCell::new(vec![1, 2]));
let clone = Rc::clone(&shared);

clone.borrow_mut().push(3);
println!("{:?}", shared.borrow());   // [1, 2, 3]
```

The thread-safe equivalent is `Arc<Mutex<T>>`.

### The Cycle Problem

Two nodes pointing at each other never reach count zero — a genuine leak:

```rust
struct Node { next: RefCell<Option<Rc<Node>>> }
// a.next = Some(b); b.next = Some(a);  →  neither is ever freed
```

### `Weak<T>` — the Fix

A `Weak` reference does **not** keep the value alive, and upgrading it returns `Option`:

```rust
use std::rc::{Rc, Weak};
use std::cell::RefCell;

struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,        // weak: child -> parent
    children: RefCell<Vec<Rc<Node>>>,   // strong: parent -> child
}

let leaf = Rc::new(Node { value: 3, parent: RefCell::new(Weak::new()),
                          children: RefCell::new(vec![]) });
let branch = Rc::new(Node { value: 5, parent: RefCell::new(Weak::new()),
                            children: RefCell::new(vec![Rc::clone(&leaf)]) });

*leaf.parent.borrow_mut() = Rc::downgrade(&branch);

if let Some(p) = leaf.parent.borrow().upgrade() { println!("parent {}", p.value); }
```

**The rule of thumb:** strong references point *down* the ownership tree, weak references point *back up*.

### Costs

`Rc` is one non-atomic increment per clone; `Arc` is an atomic RMW, which can contend across cores. Both add a heap allocation holding the counts alongside the value. Prefer plain ownership and borrows; reach for `Rc`/`Arc` only when the ownership graph genuinely is not a tree.

<br>

## 44. What is the significance of the `Drop` trait in _Rust_?

`Drop` defines the **destructor** — code that runs automatically when a value goes out of scope. It is what makes RAII work for arbitrary resources, not just memory.

```rust
struct Connection { id: u32 }

impl Drop for Connection {
    fn drop(&mut self) {
        println!("closing connection {}", self.id);
    }
}

fn main() {
    let _a = Connection { id: 1 };
    let _b = Connection { id: 2 };
    println!("working");
}
// working
// closing connection 2      <- reverse declaration order
// closing connection 1
```

### The Rules

- Values drop in **reverse order of declaration** (LIFO); struct fields drop in declaration order.
- Drop runs on **every** exit path: normal return, early `return`, `?` propagation, and panic unwinding.
- You **cannot call `.drop()` manually** — that would allow a double-free. Use `std::mem::drop(value)`, which simply takes ownership and lets the value fall out of scope.

```rust
let c = Connection { id: 3 };
drop(c);                    // runs the destructor now
// c.drop();                // error[E0040]: explicit destructor calls not allowed
```

### Why It Matters

Every RAII guard in the standard library is a `Drop` impl:

| Type | What its `Drop` does |
|---|---|
| `Box<T>`, `Vec<T>`, `String` | frees the heap allocation |
| `File` | closes the file descriptor |
| `MutexGuard` | releases the lock |
| `Rc`/`Arc` | decrements the count; frees at zero |
| `JoinHandle` (scoped) | joins the thread |

This is why Rust needs no `finally`, no `defer`, and no `with` statement — cleanup is attached to the type, so it cannot be forgotten at a call site.

### Interactions to Know

**`Drop` and `Copy` are mutually exclusive.** A `Copy` type is duplicated bit-for-bit, which would run the destructor more than once for the same resource.

**Drop order can be controlled** by scoping, or by an explicit `drop`:

```rust
let mut guard = data.lock().unwrap();
*guard += 1;
drop(guard);              // release before the slow work
expensive_computation();
```

**`ManuallyDrop<T>`** suppresses the destructor entirely (needed in `unsafe` code that moves out of a value), and `std::mem::forget` leaks a value deliberately.

**Drop is not guaranteed to run** in every situation: `std::process::exit`, `panic = "abort"`, an `Rc` cycle, or `mem::forget` all skip it. Leaking is safe, so `Drop` must never be your only line of defence for correctness (this is the "leakpocalypse" lesson behind `Pin` and scoped threads).

**Panicking inside `drop` during unwinding aborts the process** — keep destructors infallible; log errors rather than propagating them.

<br>
## 45. How do you manage _Rust project dependencies_?

Dependencies are declared in **`Cargo.toml`** and resolved by Cargo against `crates.io` (or a git repo, or a local path).

### Declaring

```toml
[package]
name = "my-app"
version = "0.1.0"
edition = "2021"
rust-version = "1.75"          # MSRV

[dependencies]
serde = { version = "1.0", features = ["derive"] }
tokio = { version = "1", features = ["rt-multi-thread", "macros", "net"] }
regex = "1.10"
local-crate = { path = "../local-crate" }
some-fork = { git = "https://github.com/user/repo", branch = "main" }

[dev-dependencies]              # tests, examples, benches only
criterion = "0.5"
proptest = "1"

[build-dependencies]            # for build.rs
cc = "1"

[target.'cfg(windows)'.dependencies]
winapi = "0.3"
```

### Version Requirements

`"1.2.3"` is shorthand for `^1.2.3` — "compatible with", meaning any `1.x.y >= 1.2.3` but below `2.0.0`. Cargo follows semver, with the pre-1.0 wrinkle that `0.x` bumps of `x` are breaking.

| Requirement | Allows |
|---|---|
| `"1.2.3"` / `"^1.2.3"` | `>=1.2.3, <2.0.0` |
| `"~1.2.3"` | `>=1.2.3, <1.3.0` |
| `"=1.2.3"` | exactly that version |
| `">=1.2, <1.5"` | explicit range |
| `"*"` | anything (rejected by crates.io for publishing) |

### `Cargo.lock`

Records the exact resolved versions. **Commit it for binaries** (reproducible builds); for libraries it is ignored by consumers, though committing it still stabilizes your own CI.

### Everyday Commands

```bash
cargo add serde --features derive     # edit Cargo.toml for you
cargo add tokio@1.35 --optional
cargo remove regex
cargo update                          # bump within semver ranges, rewrite the lock
cargo update -p serde --precise 1.0.195
cargo tree                            # dependency graph
cargo tree -d                         # duplicate versions pulled in twice
cargo tree -i openssl                 # who depends on this?
cargo outdated                        # (cargo-edit) newer majors available
cargo audit                           # known security advisories
cargo deny check                      # licences, bans, advisories
cargo vendor                          # copy all sources locally
```

### Features

Features keep builds lean by making functionality opt-in:

```toml
[features]
default = ["json"]
json = ["dep:serde_json"]
full = ["json", "compression"]

[dependencies]
serde_json = { version = "1", optional = true }
```

```rust
#[cfg(feature = "json")]
pub mod json_support;
```

Features are **additive** — enabling one must never remove functionality, because Cargo unions the features requested by every dependent.

### Workspaces

For multi-crate projects, one lockfile and one `target/` directory shared across members:

```toml
[workspace]
members = ["crates/core", "crates/cli", "crates/server"]
resolver = "2"

[workspace.dependencies]
serde = { version = "1", features = ["derive"] }
```

```toml
# crates/cli/Cargo.toml
[dependencies]
serde.workspace = true      # inherit the pinned version
```

<br>

## 46. Name some widely-used _crates_ in the Rust ecosystem.

### Serialization

- **serde** — the serialization framework; `serde_json`, `toml`, `serde_yaml`, `bincode`, `csv` plug into it.
- **prost** / **protobuf** — Protocol Buffers.

### Async and Concurrency

- **tokio** — the dominant async runtime (scheduler, timers, async I/O, sync primitives).
- **async-std**, **smol** — alternative runtimes.
- **futures** — combinators and traits shared across runtimes.
- **rayon** — data parallelism; `.iter()` becomes `.par_iter()` and you get a thread pool.
- **crossbeam** — channels, scoped threads, lock-free structures.
- **parking_lot** — faster mutexes without poisoning.

### Web and Networking

- **axum** — ergonomic web framework on tokio + tower + hyper.
- **actix-web** — high-performance actor-based framework.
- **hyper** — low-level HTTP; the foundation under most of the above.
- **reqwest** — the standard HTTP client.
- **tonic** — gRPC.
- **tower** — composable middleware (timeouts, retries, rate limits).

### CLI

- **clap** — argument parsing, with a derive API.
- **indicatif** — progress bars and spinners.
- **console** / **crossterm** / **ratatui** — terminal styling and TUIs.
- **dialoguer** — interactive prompts.

### Error Handling and Logging

- **thiserror** — derive concrete error enums (libraries).
- **anyhow** — one boxed error type with `.context()` (applications).
- **tracing** — structured, span-aware diagnostics; **tracing-subscriber** for output.
- **log** + **env_logger** — the simpler classic facade.

### Data and Databases

- **sqlx** — async SQL with compile-time-checked queries.
- **diesel** — synchronous ORM and query builder.
- **sea-orm** — async ORM.
- **redis**, **mongodb** — official-ish drivers.
- **polars** — DataFrames, an Arrow-backed pandas analogue.

### Utilities

- **itertools** — iterator adapters std does not ship.
- **regex** — linear-time regular expressions.
- **chrono** / **time** — dates and times.
- **uuid** — UUID generation.
- **rand** — random number generation.
- **once_cell** — lazy statics (much of it now in std as `OnceLock`/`LazyLock`).
- **bytes** — efficient buffer management.
- **dashmap** — concurrent hash map.
- **rustls** — TLS in pure Rust.

### Testing and Benchmarking

- **criterion** — statistical benchmarking.
- **proptest** / **quickcheck** — property-based testing.
- **mockall** — mock objects.
- **insta** — snapshot testing.
- **cargo-nextest** — a faster test runner.

### FFI and Interop

- **pyo3** — Python extensions in Rust.
- **napi-rs** / **neon** — Node.js addons.
- **wasm-bindgen** — Rust ↔ JavaScript in WebAssembly.
- **bindgen** / **cbindgen** — generate bindings from C headers, and C headers from Rust.

### Finding More

[blessed.rs](https://blessed.rs) and [lib.rs](https://lib.rs) are the two curated directories worth bookmarking; both rank by real usage rather than raw download counts.

<br>

## 47. What features does _Rust_ offer for package documentation?

Rust ships **rustdoc**, and documentation is a first-class part of the toolchain rather than a bolt-on.

### Doc Comments

```rust
//! Crate-level documentation — goes at the top of lib.rs.
//! Describes what the whole crate is for.

/// Adds two numbers together.
///
/// # Examples
///
/// ```
/// use my_crate::add;
/// assert_eq!(add(2, 2), 4);
/// ```
///
/// # Panics
///
/// Panics if the result overflows `i32`.
///
/// # Errors
///
/// Returns [`ParseError`] if the input is malformed.
///
/// # Safety
///
/// (For `unsafe fn`) the caller must guarantee the pointer is valid.
pub fn add(a: i32, b: i32) -> i32 { a + b }
```

`///` documents the item that follows; `//!` documents the enclosing item (used for crates and modules). Markdown is supported throughout.

### Doctests — the Killer Feature

Code blocks in doc comments are **compiled and run by `cargo test`**. Documentation that drifts out of date fails the build.

````rust
/// ```
/// let v = my_crate::make_vec();
/// assert_eq!(v.len(), 3);
/// ```
///
/// ```should_panic
/// my_crate::explode();
/// ```
///
/// ```no_run
/// // compiled but not executed — good for network examples
/// my_crate::connect("example.com");
/// ```
///
/// ```ignore
/// // neither compiled nor run — use sparingly
/// ```
````

Hide setup lines from the rendered output with a leading `#`:

````rust
/// ```
/// # use my_crate::Thing;
/// let t = Thing::new();
/// ```
````

### Intra-Doc Links

Link to other items by path and rustdoc resolves them, checking they exist:

```rust
/// See [`HashMap`] and [the module docs](crate::collections).
/// Related: [`Self::insert`], [`std::vec::Vec`].
```

### Building and Publishing

```bash
cargo doc --open                    # build and view locally
cargo doc --no-deps                 # your crate only
cargo test --doc                    # run only doctests
```

Anything published to crates.io is built automatically on **docs.rs**, with every version kept and links to the source.

### Attributes Worth Knowing

```rust
#![deny(missing_docs)]              // fail the build on undocumented public items
#![doc = include_str!("../README.md")]   // use the README as crate docs

#[doc(hidden)]                      // keep out of the public docs
#[doc(alias = "sort_by_key")]       // extra search terms
#[doc(cfg(feature = "json"))]       // show which feature gates this item
```

```toml
[package.metadata.docs.rs]
all-features = true                 # docs.rs builds with every feature enabled
rustdoc-args = ["--cfg", "docsrs"]
```

### Conventional Headings

`# Examples`, `# Panics`, `# Errors`, `# Safety`, `# Arguments` — the API Guidelines expect these, and `clippy::missing_panics_doc` / `missing_errors_doc` will nag when they are absent.

<br>

## 48. How do you format _Rust_ code for readability?

Rust has one canonical formatter, so formatting is not a matter of team debate.

### rustfmt

```bash
cargo fmt                # format the whole workspace in place
cargo fmt -- --check     # exit non-zero if anything is unformatted (CI)
cargo fmt -p my-crate    # one package
```

It ships with the toolchain (`rustup component add rustfmt` if missing). The default style is the official Rust style guide: 4-space indent, 100-column max width, trailing commas in multi-line lists, `snake_case`/`PascalCase` naming.

### Configuring — `rustfmt.toml`

Keep it small; the value of rustfmt is the *shared* default.

```toml
edition = "2021"
max_width = 100
use_small_heuristics = "Default"
imports_granularity = "Crate"      # nightly-only options exist too
group_imports = "StdExternalCrate" # std, then external, then crate
```

### Clippy — the Other Half of Readability

`rustfmt` handles layout; **clippy** handles idiom.

```bash
cargo clippy --all-targets --all-features -- -D warnings
```

It catches things like `if x == true`, a manual loop that should be `.iter().sum()`, `&String` parameters that should be `&str`, and needless `clone()`s. Roughly 700 lints, grouped into `correctness`, `suspicious`, `style`, `complexity`, `perf`, `pedantic`, and `nursery`.

```rust
#![warn(clippy::pedantic)]
#[allow(clippy::too_many_arguments)]   // opt out where justified
```

### In CI

```yaml
- run: cargo fmt --all -- --check
- run: cargo clippy --all-targets -- -D warnings
```

Locally, a pre-commit hook or `cargo watch -x fmt -x clippy` keeps it continuous. Most editors run `rustfmt` on save via rust-analyzer.

### Beyond the Tools

The tools cannot judge naming or structure. The Rust API Guidelines cover the rest: getters without a `get_` prefix, `as_`/`to_`/`into_` conveying conversion cost, iterator methods named `iter`/`iter_mut`/`into_iter`, and constructors named `new` (or `with_capacity`-style variants).

<br>

## 49. Explain what `unsafe` code is in _Rust_ and when to use it.

`unsafe` does **not** turn off the borrow checker or the type system. It unlocks exactly **five** operations the compiler cannot verify, and shifts responsibility for their correctness onto you.

### The Five Superpowers

1. Dereference a raw pointer (`*const T`, `*mut T`).
2. Call an `unsafe` function or method (including all FFI).
3. Access or modify a mutable `static`.
4. Implement an `unsafe` trait (`Send`, `Sync`).
5. Access fields of a `union`.

Everything else — ownership, lifetimes, exclusivity — is still enforced inside an `unsafe` block.

```rust
let mut num = 5;
let r1 = &num as *const i32;
let r2 = &mut num as *mut i32;

unsafe {
    println!("r1 is: {}", *r1);
    *r2 = 10;
}
```

### Legitimate Uses

- **FFI** — calling C, or exposing Rust to C. The compiler cannot see across the boundary.
- **Data structures the borrow checker cannot express** — doubly linked lists, arena allocators, self-referential types.
- **Performance**, when a bounds check is provably redundant and profiling shows it matters (`get_unchecked`).
- **Hardware and OS work** — memory-mapped registers, syscalls, custom allocators.
- **Building safe abstractions** — this is the important one.

### The Central Discipline: Safe Abstractions

Wrap a small `unsafe` core behind an API that **cannot be misused**. `Vec`, `RefCell`, `Mutex`, and `split_at_mut` are all built this way — `unsafe` inside, no way for a caller to cause UB from outside.

```rust
pub fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = slice.len();
    let ptr = slice.as_mut_ptr();
    assert!(mid <= len);                     // the check that makes it sound

    unsafe {
        (
            std::slice::from_raw_parts_mut(ptr, mid),
            std::slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}
```

The `assert!` is what upgrades the `unsafe` block into a safe function.

### Rules of Engagement

- **Keep it minimal** — the smallest possible block, never a whole function body.
- **Document the invariant** with a `// SAFETY:` comment explaining *why* it is sound. Clippy's `undocumented_unsafe_blocks` lint enforces this.
- **`unsafe fn` means the caller must uphold something** — say what, under a `# Safety` heading.
- **Test under Miri**: `cargo +nightly miri test` detects UB that ordinary tests miss.
- **Ban it where you can**: `#![forbid(unsafe_code)]` at the crate root.

```rust
// SAFETY: `ptr` is non-null and aligned because it came from a live &mut [T],
// and `mid <= len` was asserted above, so the two ranges do not overlap.
unsafe { /* ... */ }
```

### When *Not* To

Most `unsafe` in application code is unnecessary. If you are reaching for it to silence a borrow-check error, the answer is usually a redesign, `Rc<RefCell<T>>`, an index-based structure, or an existing crate. Undefined behaviour in Rust is just as bad as in C — the difference is that Rust confines it to blocks you can grep for.

<br>

## 50. How does _Rust_ interface with other languages (FFI)?

Rust speaks the **C ABI**, which is the lingua franca of native interop. Anything that can call C can call Rust, and vice versa.

### Calling C from Rust

```rust
use std::os::raw::{c_char, c_int};
use std::ffi::CString;

#[link(name = "m")]                 // link against libm
extern "C" {
    fn abs(input: c_int) -> c_int;
    fn puts(s: *const c_char) -> c_int;
}

fn main() {
    unsafe {
        println!("{}", abs(-3));

        let msg = CString::new("hello from C").unwrap();
        puts(msg.as_ptr());
    }
}
```

Every call is `unsafe` — the compiler cannot verify what happens on the other side.

### Exposing Rust to C

```rust
use std::ffi::{c_char, CStr, CString};

#[no_mangle]                        // keep the symbol name intact
pub extern "C" fn add(a: i32, b: i32) -> i32 { a + b }

#[no_mangle]
pub extern "C" fn greet(name: *const c_char) -> *mut c_char {
    let name = unsafe { CStr::from_ptr(name) }.to_string_lossy();
    CString::new(format!("Hello, {name}!")).unwrap().into_raw()
}

/// Callers MUST return the pointer here — C cannot free Rust allocations.
#[no_mangle]
pub extern "C" fn free_string(s: *mut c_char) {
    if !s.is_null() { unsafe { drop(CString::from_raw(s)) } }
}
```

```toml
[lib]
crate-type = ["cdylib"]     # .so / .dll / .dylib
# or ["staticlib"] for a .a
```

### The Three Hard Parts

**1. Memory layout** — Rust's default layout is unspecified. Any struct crossing the boundary needs `#[repr(C)]`:

```rust
#[repr(C)]
pub struct Point { x: f64, y: f64 }

#[repr(C)]
pub enum Status { Ok = 0, Error = 1 }
```

**2. Strings** — Rust `String` is UTF-8 and not null-terminated; C strings are null-terminated bytes. `CString` (owned, Rust → C) and `CStr` (borrowed, C → Rust) bridge them.

**3. Ownership** — whoever allocates must free. Always export a matching free function, and document which side owns what.

### Panics Must Not Cross

Unwinding into C is undefined behaviour. Catch at the boundary:

```rust
#[no_mangle]
pub extern "C" fn safe_entry() -> i32 {
    std::panic::catch_unwind(|| { risky(); 0 }).unwrap_or(-1)
}
```

(`extern "C"` functions abort rather than unwind by default in recent Rust, but catching gives you a usable error code.)

### Tooling

- **bindgen** — generates Rust bindings from C headers (run in `build.rs`).
- **cbindgen** — generates a C header from your Rust.
- **cc** crate — compiles bundled C sources at build time.

### Higher-Level Language Bindings

| Target | Crate |
|---|---|
| Python | **pyo3** + maturin |
| Node.js | **napi-rs**, **neon** |
| JS / browser | **wasm-bindgen** |
| Java | **jni** |
| C++ | **cxx** (safe bidirectional bridge) |
| Go | via C ABI + cgo |
| Ruby | **magnus** |

```rust
use pyo3::prelude::*;

#[pyfunction]
fn sum_list(v: Vec<i64>) -> i64 { v.iter().sum() }

#[pymodule]
fn fastmath(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_list, m)?)
}
```

This is the most common real-world use of Rust FFI today: a hot loop rewritten in Rust and imported into a Python or Node codebase.

<br>

## 51. What are some of the considerations for using _Rust_ in _embedded systems_?

Rust is a strong fit for embedded work — no runtime, no GC, C-level control, plus memory safety in a domain where a bug means a field recall.

### `#![no_std]`

Microcontrollers have no OS, so the standard library is unavailable. You get **`core`** (types, traits, iterators, `Option`/`Result` — no allocation) and optionally **`alloc`** (`Vec`, `Box`, `String`) if you supply a global allocator.

```rust
#![no_std]
#![no_main]

use panic_halt as _;              // a panic handler is mandatory
use cortex_m_rt::entry;

#[entry]
fn main() -> ! {                  // never returns
    loop { }
}
```

You must provide:

- a **panic handler** (`panic-halt`, `panic-abort`, `panic-probe` for RTT logging),
- an **entry point** (`#[entry]` from `cortex-m-rt`),
- a **global allocator**, if you want `alloc`.

### Memory Discipline

- **No heap by default.** Use fixed-size buffers, `heapless::Vec<T, N>`, and static allocation.
- **Stack size is tiny** — a few KB. Avoid deep recursion and large stack locals.
- **`static mut` is unsafe.** Use `cortex_m::interrupt::Mutex`, `critical-section`, or the safe abstractions in RTIC/Embassy.
- **Binary size matters.** Optimize for it:

```toml
[profile.release]
opt-level = "z"       # or "s"
lto = true
codegen-units = 1
panic = "abort"
strip = true
```

### The Ecosystem

| Layer | Crates |
|---|---|
| Traits | **embedded-hal** — the portable GPIO/SPI/I²C/UART abstraction |
| Peripheral access | **svd2rust**-generated PACs, typestate-checked |
| Runtime | **cortex-m-rt**, **riscv-rt** |
| Concurrency | **RTIC** (interrupt-driven, static priorities), **Embassy** (async/await on bare metal) |
| Collections | **heapless** |
| Debugging | **probe-rs**, **defmt** (deferred, ultra-compact logging) |

### Toolchain

```bash
rustup target add thumbv7em-none-eabihf
cargo build --release --target thumbv7em-none-eabihf
cargo embed        # or: probe-rs run
```

```toml
# .cargo/config.toml
[build]
target = "thumbv7em-none-eabihf"
rustflags = ["-C", "link-arg=-Tlink.x"]
```

### What Rust Buys You Here

**Typestate peripherals** make hardware misuse a compile error — a pin configured as input has a different *type* from one configured as output, so reading from an output pin does not compile. Ownership models exclusive peripheral access naturally: `Peripherals::take()` returns `Option`, and it returns `None` the second time.

### The Trade-offs

- Vendor support is thinner than C — mature HALs exist for STM32, nRF, RP2040, ESP32, but not everything.
- Certification (ISO 26262, DO-178C) is still maturing; **Ferrocene** is the qualified toolchain.
- Some patterns (shared mutable global state, interrupt handlers touching everything) require rethinking, not just translating.
- Debugging infrastructure is good but different: `defmt` + `probe-rs` rather than printf over UART.

<br>

## 52. Discuss _Rust's support for compile-time function execution (const fn)_.

A **`const fn`** can be evaluated by the compiler at compile time. Calling one in a `const` context bakes the result into the binary; calling it at runtime works normally.

```rust
const fn square(n: u32) -> u32 { n * n }

const AREA: u32 = square(8);            // computed during compilation
static TABLE: [u32; square(3) as usize] = [0; 9];

fn main() {
    let runtime = square(read_input());  // same function, ordinary call
    println!("{AREA} {runtime}");
}
```

### Where Const Evaluation Is *Required*

Some positions accept nothing else, which is the main reason `const fn` exists:

- array lengths — `[u8; BUF_SIZE]`
- `const` and `static` initializers
- const generic arguments
- enum discriminants

### What `const fn` Can Do Today

The set has grown a lot. As of recent stable Rust: arithmetic, `if`/`match`, `while`/`loop`, `let` bindings and mutation of locals, references, struct and enum construction, calling other `const fn`s, indexing, casts, and `panic!`/`assert!`.

```rust
const fn fib(n: u32) -> u64 {
    let (mut a, mut b, mut i) = (0u64, 1u64, 0);
    while i < n {
        let t = a + b;
        a = b;
        b = t;
        i += 1;
    }
    a
}

const F30: u64 = fib(30);   // 832040, computed at compile time
```

### What It Still Cannot Do

- Heap allocation (no `Vec`, no `String`, no `Box`).
- Most trait method calls (`const` traits are unstable).
- Floating-point arithmetic in const contexts is limited.
- Anything touching I/O, randomness, or the clock.
- `for` loops (they desugar through `Iterator`) — use `while`.

### Const Generics

Generic over a *value*, enabling compile-time-sized APIs:

```rust
struct Buffer<const N: usize> { data: [u8; N] }

impl<const N: usize> Buffer<N> {
    const fn new() -> Self { Buffer { data: [0; N] } }
    const fn capacity(&self) -> usize { N }
}

let b: Buffer<1024> = Buffer::new();
```

### Compile-Time Assertions

A failing `const` evaluation is a **build error**, which gives you static checks for free:

```rust
const _: () = assert!(std::mem::size_of::<usize>() >= 4, "needs a 32-bit+ target");
```

### `const` Blocks

Force evaluation inline (stable since 1.79):

```rust
let arr = [const { None::<String> }; 100];   // works for non-Copy types
```

### Why It Matters

Lookup tables, CRC and hash constants, protocol size checks, and buffer sizing all move from runtime to compile time — zero startup cost, and mistakes become compile errors. It is especially valuable in embedded code, where `static` initialization must be free and heap allocation is unavailable.

<br>

## 53. How can you compile _Rust_ code for a different target platform?

Cross-compilation is a first-class feature: `rustup` ships pre-built `std` for most targets, and `--target` selects one.

### The Basic Flow

```bash
rustup target list                        # everything available
rustup target add x86_64-unknown-linux-musl
cargo build --release --target x86_64-unknown-linux-musl
# output lands in target/x86_64-unknown-linux-musl/release/
```

### Target Triples

The format is `<arch>-<vendor>-<os>-<abi>`:

| Triple | Platform |
|---|---|
| `x86_64-unknown-linux-gnu` | Linux, glibc (the usual default) |
| `x86_64-unknown-linux-musl` | Linux, **statically linked** — runs anywhere |
| `aarch64-apple-darwin` | macOS on Apple Silicon |
| `x86_64-pc-windows-msvc` | Windows, MSVC toolchain |
| `x86_64-pc-windows-gnu` | Windows, MinGW |
| `aarch64-linux-android` | Android |
| `wasm32-unknown-unknown` | WebAssembly, no host |
| `wasm32-wasip1` | WebAssembly with WASI |
| `thumbv7em-none-eabihf` | ARM Cortex-M4F, bare metal |

Tiers matter: **Tier 1** targets are tested in CI and guaranteed to work; **Tier 2** builds are guaranteed; **Tier 3** targets exist but carry no guarantees.

### The Actual Difficulty: Linking

Rust compiles for the target easily; **linking** needs a cross-linker, and any C dependency needs a cross C toolchain.

```toml
# .cargo/config.toml
[target.aarch64-unknown-linux-gnu]
linker = "aarch64-linux-gnu-gcc"

[target.armv7-unknown-linux-gnueabihf]
linker = "arm-linux-gnueabihf-gcc"

[target.thumbv7em-none-eabihf]
rustflags = ["-C", "link-arg=-Tlink.x"]
```

### `cross` — the Pragmatic Answer

The [`cross`](https://github.com/cross-rs/cross) tool runs the build inside a Docker image that already contains the right toolchain:

```bash
cargo install cross
cross build --release --target aarch64-unknown-linux-gnu
```

Same interface as Cargo, none of the toolchain archaeology. This is what most projects use in CI.

### Fully Static Linux Binaries

`musl` produces a binary with no dynamic dependencies at all — ideal for scratch containers and unknown distros:

```bash
rustup target add x86_64-unknown-linux-musl
cargo build --release --target x86_64-unknown-linux-musl
ldd target/x86_64-unknown-linux-musl/release/app    # "not a dynamic executable"
```

### WebAssembly

```bash
rustup target add wasm32-unknown-unknown
cargo install wasm-pack
wasm-pack build --target web
```

### Conditional Compilation

```rust
#[cfg(target_os = "windows")]
fn config_dir() -> PathBuf { /* ... */ }

#[cfg(unix)]
fn config_dir() -> PathBuf { /* ... */ }

#[cfg(target_arch = "wasm32")]
fn now() -> f64 { js_sys::Date::now() }

if cfg!(target_pointer_width = "64") { /* ... */ }
```

### Caveats

- Crates with C dependencies (`openssl`, `libsqlite3`) are the usual blockers — prefer pure-Rust alternatives (`rustls`, `rusqlite` with the bundled feature).
- You cannot **run** the cross-compiled tests natively; use QEMU, or `cross test`, which wires it up for you.
- macOS targets can only be built legally on Apple hardware in most CI setups.

<br>

## 54. How is _procedural macro expansion_ handled in _Rust_?

A procedural macro is **a compiler plugin**: a Rust function, compiled for the *host*, that the compiler calls during compilation with your source tokens and that returns replacement tokens.

### The Pipeline

1. **Parse** — rustc lexes the source into token trees; it does not need to understand the macro's contents.
2. **Load** — the proc-macro crate is compiled first, for the host architecture, as a dynamic library.
3. **Invoke** — at each call site, rustc hands the relevant `TokenStream` to the macro function and runs it in-process.
4. **Expand** — the returned `TokenStream` is parsed as real Rust and spliced in. Expansion is recursive: generated code may itself contain macros, up to `#![recursion_limit]`.
5. **Resolve and check** — name resolution, type checking, and borrow checking all happen *after* expansion, on the final code.

Because step 4 precedes step 5, a proc macro never sees type information — only syntax. That is why `#[derive(Serialize)]` matches on field types textually rather than knowing what they resolve to.

### The Crate Setup

Proc macros must live in their own crate:

```toml
[lib]
proc-macro = true

[dependencies]
syn = { version = "2", features = ["full"] }
quote = "1"
proc-macro2 = "1"
```

### A Complete Derive Macro

```rust
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields};

#[proc_macro_derive(Describe, attributes(describe))]
pub fn derive_describe(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let name = &ast.ident;
    let (impl_g, ty_g, where_c) = ast.generics.split_for_impl();

    let field_names: Vec<_> = match &ast.data {
        Data::Struct(s) => match &s.fields {
            Fields::Named(f) => f.named.iter().map(|f| &f.ident).collect(),
            _ => vec![],
        },
        _ => return syn::Error::new_spanned(&ast, "Describe only supports structs")
                    .to_compile_error()
                    .into(),
    };

    quote! {
        impl #impl_g Describe for #name #ty_g #where_c {
            fn describe(&self) -> String {
                let mut out = String::from(stringify!(#name));
                #( out.push_str(&format!(" {}={:?}", stringify!(#field_names), self.#field_names)); )*
                out
            }
        }
    }.into()
}
```

`syn` parses tokens into a typed AST; `quote!` builds tokens from a template, with `#var` interpolation and `#(...)*` repetition.

### Spans, Hygiene, and Errors

Every token carries a **`Span`** — a source location plus hygiene information. Spans are what make error messages point at the user's code rather than at the macro:

```rust
syn::Error::new_spanned(&field.ty, "unsupported field type").to_compile_error()
```

Emitting a `compile_error!` this way is the correct pattern; **panicking** inside a proc macro produces a much worse message. Proc macros are *not* automatically hygienic — use `Span::call_site()` deliberately, and refer to items by fully-qualified paths (`::std::string::String`) so callers' local names cannot shadow them.

### Ordering and Constraints

- Derives on the same item expand in declaration order; attribute macros expand outside-in.
- A proc macro runs on the **host**, so it cannot be cross-compiled into the target binary — it exists only during the build.
- It has full ambient authority: it can read files, hit the network, or query a database. `sqlx::query!` uses this to validate SQL against a live schema at compile time. It is also why proc macros are a supply-chain consideration.
- They noticeably slow builds, since `syn` and `quote` must compile first.

### Inspecting the Output

```bash
cargo install cargo-expand
cargo expand                       # the whole crate, post-expansion
cargo expand --lib my_module       # narrower
```

<br>

## 55. What are some common idiomatic practices in _Rust_ for error handling?

### 1. `Result` for Expected Failure, Panic for Bugs

Anything a caller could reasonably encounter — missing file, malformed input, timeout — is a `Result`. Panics are reserved for violated invariants. Library code should essentially never panic on caller data.

### 2. Propagate with `?`, Not `match`

```rust
fn load() -> Result<Config, ConfigError> {
    let text = fs::read_to_string(path())?;
    let cfg = toml::from_str(&text)?;
    Ok(cfg)
}
```

### 3. Libraries: Concrete Error Enums with `thiserror`

Give callers something they can `match` on, not an opaque string:

```rust
use thiserror::Error;

#[derive(Debug, Error)]
pub enum StoreError {
    #[error("record {0} not found")]
    NotFound(u64),

    #[error("database unavailable")]
    Db(#[from] sqlx::Error),          // generates From, and the source chain

    #[error("invalid field `{field}`: {reason}")]
    Invalid { field: String, reason: String },
}
```

`#[from]` gives you the `From` impl that makes `?` work; `#[error(...)]` gives you `Display`; the derive gives you `std::error::Error` including `source()`.

### 4. Applications: `anyhow` with Context

At the top level, the concrete type rarely matters — a good message does:

```rust
use anyhow::{Context, Result};

fn main() -> Result<()> {
    let cfg = fs::read_to_string("app.toml")
        .context("reading app.toml")?;
    let port: u16 = cfg.trim().parse()
        .with_context(|| format!("parsing port from {cfg:?}"))?;
    serve(port).context("starting server")?;
    Ok(())
}
```

The output becomes a readable chain: `starting server: parsing port from "abc": invalid digit found in string`.

### 5. Never Swallow Errors

`let _ = risky();` discards information silently. Log it, or propagate it.

### 6. `main` Returning `Result`

```rust
fn main() -> anyhow::Result<()> { run() }
```

Rust prints the error's `Debug` form and exits non-zero — no manual `process::exit` needed.

### 7. Keep `unwrap` Out of Production Paths

Acceptable in tests, examples, and provable invariants (with `expect` stating *why* it cannot fail). Enforce it:

```rust
#![warn(clippy::unwrap_used, clippy::expect_used)]
```

### 8. Implement `From` for Boundary Conversions

Let `?` do the translation from a lower layer's error type into yours, rather than mapping by hand at every call site.

### 9. Preserve the Source Chain

Wrap, do not stringify. `#[source]`/`#[from]` keeps `source()` intact so the whole chain can be printed or inspected.

### 10. Make Illegal States Unrepresentable

The best error handling is the error you cannot construct. Parse into a validated type once, at the edge, instead of re-checking a `String` everywhere:

```rust
pub struct Email(String);

impl Email {
    pub fn parse(s: String) -> Result<Self, ValidationError> {
        if s.contains('@') { Ok(Email(s)) } else { Err(ValidationError::NoAt) }
    }
}
```

### 11. Use `#[non_exhaustive]` on Public Error Enums

It lets you add variants later without a breaking change, since downstream matches must keep a `_` arm.

<br>
## 56. Describe effective use of the _Rust module system_ in large projects.

Rust's module tree controls **visibility** and **namespacing**. Used well, it lets you refactor internals freely without breaking anyone.

### Files Map to Modules

```
src/
├── main.rs              // crate root (binary)
├── lib.rs               // crate root (library)
├── config.rs            // mod config;
├── db/
│   ├── mod.rs           // mod db;   (or src/db.rs alongside src/db/)
│   ├── pool.rs          // db::pool
│   └── migrations.rs    // db::migrations
└── api/
    ├── mod.rs
    ├── routes.rs
    └── handlers.rs
```

Since the 2018 edition, `src/db.rs` **plus** `src/db/` is preferred over `src/db/mod.rs` — it keeps the directory listing meaningful instead of a wall of files all named `mod.rs`.

### Visibility Is a Gradient

```rust
struct Internal;          // private — this module and descendants
pub struct Public;        // visible wherever the module is
pub(crate) struct Crate;  // this crate only  ← the workhorse
pub(super) struct Parent; // parent module only
pub(in crate::api) struct Scoped;  // one specific subtree
```

**Default to private; promote deliberately.** `pub(crate)` is the right choice for most internal helpers: shared across your codebase, invisible to dependents, and free to change.

### The Facade Pattern

Organize internals however you like, then re-export a flat, stable public API from the root:

```rust
// lib.rs
mod config;
mod db;
mod api;
mod error;

pub use config::Config;
pub use error::{Error, Result};
pub use api::{Server, ServerBuilder};
```

Users write `my_crate::Config`, not `my_crate::config::types::Config`. Your directory structure becomes an implementation detail.

### Prelude Modules

For crates with many commonly-imported traits:

```rust
pub mod prelude {
    pub use crate::{Config, Error, Result};
    pub use crate::traits::{Handler, Middleware};
}
```

```rust
use my_crate::prelude::*;
```

### Import Hygiene

```rust
use std::collections::{HashMap, HashSet};
use std::io::{self, Read, Write};

use serde::{Deserialize, Serialize};

use crate::db::Pool;
use super::handlers::health;
```

Conventions worth keeping: group `std`, then external crates, then `crate`/`super` (rustfmt's `group_imports = "StdExternalCrate"` does this); import **types** but call **functions** through their module (`fs::read_to_string`, not a bare `read_to_string`); and use `as` to disambiguate rather than writing full paths everywhere.

### Tests Live Next to the Code

```rust
#[cfg(test)]
mod tests {
    use super::*;         // child modules can see private parents

    #[test]
    fn it_works() { assert!(internal_helper()); }
}
```

That `use super::*` is why unit tests can exercise private functions — a child module sees its ancestors' privates, but not vice versa.

### When to Split into Crates

Modules stop being enough when you want:

- **Parallel compilation** — crates are the unit of incremental build.
- **A hard API boundary** — `pub(crate)` cannot leak across a crate line.
- **Independent versioning or publishing.**
- **Separate dependency sets** — the CLI needs `clap`, the core library should not.

```toml
[workspace]
members = ["crates/core", "crates/cli", "crates/server"]
resolver = "2"
```

A common shape: a dependency-light `core` crate holding domain types, thin `cli`/`server` crates on top, and integration tests at the workspace level.

### Anti-Patterns

- A single 5,000-line `lib.rs`.
- `pub` on everything "just in case" — every one becomes a semver commitment.
- Deep nesting (`crate::a::b::c::d::Thing`) — flatten with re-exports.
- Circular module dependencies — Rust permits them, but they usually signal that a shared type belongs in a third module.

<br>

## 57. Explain how you would optimize _Rust_ code for performance.

### 0. Measure First

Optimizing without data is guesswork. The tooling:

```bash
cargo build --release              # never benchmark a debug build (10-100x slower)
cargo bench                        # criterion: statistical, detects regressions
cargo flamegraph                   # where the time actually goes
perf stat ./target/release/app     # cache misses, branch mispredictions
```

```rust
// benches/my_bench.rs with criterion
fn bench(c: &mut Criterion) {
    c.bench_function("parse", |b| b.iter(|| parse(black_box(INPUT))));
}
```

`black_box` prevents the optimizer from deleting the work you are trying to time.

### 1. Build Configuration — Free Wins

```toml
[profile.release]
opt-level = 3
lto = "fat"            # link-time optimization across crates
codegen-units = 1      # better optimization, slower build
panic = "abort"        # if you never catch_unwind
```

```bash
RUSTFLAGS="-C target-cpu=native" cargo build --release   # use this machine's SIMD
```

LTO plus `codegen-units = 1` alone is often a 10–20% win.

### 2. Cut Allocations

This is usually the biggest algorithmic-level win in Rust code:

```rust
let mut v = Vec::with_capacity(n);          // avoid repeated reallocation
let mut s = String::with_capacity(1024);

fn takes(s: &str) {}                        // not String — no allocation at the call
fn reads(v: &[i32]) {}                      // not Vec<i32>

// Reuse a buffer across iterations
let mut buf = String::new();
for line in reader.lines() {
    buf.clear();                            // keeps the capacity
    // ...
}

// Cow: allocate only when you actually modify
use std::borrow::Cow;
fn normalize(s: &str) -> Cow<'_, str> {
    if s.contains(' ') { Cow::Owned(s.replace(' ', "_")) } else { Cow::Borrowed(s) }
}
```

Hunt needless clones with `cargo clippy -- -W clippy::redundant_clone`.

### 3. Iterators over Manual Loops

Iterator chains compile to the same code as hand-written loops — often better, because bounds checks are elided:

```rust
let total: u64 = data.iter().filter(|x| x.active).map(|x| x.size).sum();
```

Prefer iteration to indexing (`for x in &v` rather than `for i in 0..v.len()`), which removes bounds checks entirely.

### 4. Pick Better Data Structures

- `HashMap`'s default SipHash is DoS-resistant but slow; swap in `rustc-hash`/`ahash` for internal maps.
- `Vec` beats `LinkedList` essentially always — cache locality wins.
- `SmallVec`/`ArrayVec` keep small collections on the stack.
- `Box<str>` over `String`, `Box<[T]>` over `Vec<T>` when the size is fixed — saves 8 bytes and a level of indirection.
- Consider struct-of-arrays over array-of-structs for hot scans.

### 5. Dispatch and Inlining

```rust
fn process<T: Handler>(h: &T) {}     // static dispatch, inlinable
fn process(h: &dyn Handler) {}       // vtable call, not inlinable

#[inline]                            // hint across crate boundaries
#[inline(always)]                    // use sparingly; can hurt i-cache
```

### 6. Parallelism, Nearly Free

```rust
use rayon::prelude::*;
let total: u64 = data.par_iter().map(expensive).sum();
```

Changing `iter()` to `par_iter()` is frequently the single largest speedup available for CPU-bound work.

### 7. Advanced

- **PGO** — profile-guided optimization via `-Cprofile-generate` / `-Cprofile-use`.
- **SIMD** — `std::simd` (nightly) or the `wide`/`packed_simd` crates.
- **`unsafe` `get_unchecked`** — only with a benchmark proving the bounds check matters.
- **Arena allocation** — `bumpalo` for many short-lived objects.
- **`#[repr(C)]` / field ordering** — control padding in hot structs.

### The Discipline

Profile, change one thing, measure again, keep it only if it won. Most "optimizations" applied blind make code slower and uglier. And check the algorithm before the micro-details: an O(n²) loop in tuned Rust still loses to O(n log n) in plain Rust.

<br>

## 58. What's the recommended way to write _unit tests_ in _Rust_?

Testing is built into the language and Cargo — no framework to choose.

### Unit Tests Live Beside the Code

```rust
pub fn add(a: i32, b: i32) -> i32 { a + b }

fn internal_helper(x: i32) -> i32 { x * 2 }

#[cfg(test)]                       // compiled only under `cargo test`
mod tests {
    use super::*;                  // brings in the parent module, privates included

    #[test]
    fn adds_two_numbers() {
        assert_eq!(add(2, 2), 4);
    }

    #[test]
    fn can_test_private_functions() {
        assert_eq!(internal_helper(3), 6);
    }
}
```

Keeping tests in the same file is idiomatic Rust — it gives access to private items and keeps the test next to what it covers.

### Assertions

```rust
assert!(cond, "context: {value}");
assert_eq!(actual, expected);
assert_ne!(a, b);
debug_assert!(invariant);          // stripped in release builds
```

`assert_eq!` prints both sides on failure, so prefer it to `assert!(a == b)`.

### Failure and Error Cases

```rust
#[test]
#[should_panic(expected = "divide by zero")]
fn panics_on_zero() { divide(1, 0); }

#[test]
fn parses() -> Result<(), std::num::ParseIntError> {
    let n: i32 = "42".parse()?;    // `?` works in tests that return Result
    assert_eq!(n, 42);
    Ok(())
}

#[test]
#[ignore = "slow; run with --ignored"]
fn expensive() { /* ... */ }
```

### Integration Tests

Files in `tests/` are separate crates that see only your **public** API — exactly what a user sees:

```
tests/
├── api_test.rs
└── common/
    └── mod.rs        // shared helpers (a subdirectory, so it is not run as a test)
```

```rust
// tests/api_test.rs
use my_crate::Config;

mod common;

#[test]
fn loads_default_config() {
    common::setup();
    assert!(Config::default().is_valid());
}
```

### Doctests

Examples in `///` comments are compiled and run, keeping docs honest:

````rust
/// ```
/// assert_eq!(my_crate::add(2, 2), 4);
/// ```
````

### Running

```bash
cargo test                       # unit + integration + doctests
cargo test add                   # only tests whose name contains "add"
cargo test -- --ignored          # the slow ones
cargo test -- --nocapture        # show println! output
cargo test -- --test-threads=1   # serial, for tests sharing global state
cargo test --lib                 # unit tests only
cargo test --doc                 # doctests only
```

Tests run **in parallel by default**, so avoid shared mutable global state or use `serial_test`.

### Worth Adding

| Crate | Purpose |
|---|---|
| **cargo-nextest** | much faster runner, per-test process isolation |
| **proptest** / **quickcheck** | property-based testing over generated inputs |
| **insta** | snapshot testing with reviewable diffs |
| **mockall** | mock trait implementations |
| **rstest** | parameterized tests and fixtures |
| **tarpaulin** / **llvm-cov** | coverage |
| **tokio::test** | `#[tokio::test]` for async tests |

```rust
#[tokio::test]
async fn fetches() {
    let body = fetch("http://localhost").await.unwrap();
    assert!(!body.is_empty());
}
```

### Good Practice

Test behaviour rather than implementation; name tests as sentences (`returns_none_when_empty`); use a table or `rstest` for many similar cases; and make each test independent — a suite that only passes in order is a suite you cannot parallelize or trust.

<br>

## 59. How would you approach writing a web server in _Rust_?

### Pick the Stack

**axum** is the default recommendation today: built on tokio + hyper + tower, ergonomic, no macro magic, and the middleware ecosystem is shared with the rest of the tower world. Alternatives: **actix-web** (very fast, actor-based), **rocket** (batteries-included, more magic), **poem**, **warp**.

```toml
[dependencies]
axum = "0.7"
tokio = { version = "1", features = ["full"] }
tower-http = { version = "0.5", features = ["trace", "cors", "compression-gzip"] }
serde = { version = "1", features = ["derive"] }
sqlx = { version = "0.7", features = ["runtime-tokio", "postgres", "macros"] }
thiserror = "1"
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter"] }
```

### A Realistic Skeleton

```rust
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Clone)]
struct AppState {
    db: sqlx::PgPool,
}

#[derive(Serialize, sqlx::FromRow)]
struct User { id: i64, name: String }

#[derive(Deserialize)]
struct CreateUser { name: String }

async fn list_users(State(st): State<Arc<AppState>>) -> Result<Json<Vec<User>>, AppError> {
    let users = sqlx::query_as::<_, User>("SELECT id, name FROM users")
        .fetch_all(&st.db)
        .await?;
    Ok(Json(users))
}

async fn get_user(
    Path(id): Path<i64>,
    State(st): State<Arc<AppState>>,
) -> Result<Json<User>, AppError> {
    sqlx::query_as::<_, User>("SELECT id, name FROM users WHERE id = $1")
        .bind(id)
        .fetch_optional(&st.db)
        .await?
        .map(Json)
        .ok_or(AppError::NotFound(id))
}

async fn create_user(
    State(st): State<Arc<AppState>>,
    Json(body): Json<CreateUser>,
) -> Result<(StatusCode, Json<User>), AppError> {
    let user = sqlx::query_as::<_, User>(
        "INSERT INTO users (name) VALUES ($1) RETURNING id, name")
        .bind(&body.name)
        .fetch_one(&st.db)
        .await?;
    Ok((StatusCode::CREATED, Json(user)))
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let db = sqlx::PgPool::connect(&std::env::var("DATABASE_URL")?).await?;
    let state = Arc::new(AppState { db });

    let app = Router::new()
        .route("/health", get(|| async { "ok" }))
        .route("/users", get(list_users).post(create_user))
        .route("/users/:id", get(get_user))
        .layer(tower_http::trace::TraceLayer::new_for_http())
        .layer(tower_http::compression::CompressionLayer::new())
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    tracing::info!("listening on {}", listener.local_addr()?);
    axum::serve(listener, app).await?;
    Ok(())
}
```

### Error Handling — the Piece People Miss

Give your error type an `IntoResponse` impl once, and every handler can use `?`:

```rust
#[derive(thiserror::Error, Debug)]
enum AppError {
    #[error("user {0} not found")]
    NotFound(i64),
    #[error(transparent)]
    Db(#[from] sqlx::Error),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, msg) = match &self {
            AppError::NotFound(_) => (StatusCode::NOT_FOUND, self.to_string()),
            AppError::Db(e) => {
                tracing::error!("db error: {e:?}");           // log the detail
                (StatusCode::INTERNAL_SERVER_ERROR, "internal error".into())  // leak nothing
            }
        };
        (status, Json(serde_json::json!({ "error": msg }))).into_response()
    }
}
```

### The Rest of a Production Service

- **Extractors** do validation at the boundary: `Json<T>`, `Query<T>`, `Path<T>`, and custom ones for auth tokens. A malformed body becomes a 400 before your handler runs.
- **Middleware** via tower layers: tracing, CORS, compression, timeouts, rate limiting, request-body limits.
- **State**: share a connection pool through `State`, cloned cheaply (`PgPool` is an `Arc` internally).
- **Config**: environment variables via `envy` or `figment`; never hardcode secrets.
- **Graceful shutdown**: `axum::serve(...).with_graceful_shutdown(signal)` so in-flight requests finish.
- **Testing**: call the `Router` directly with `tower::ServiceExt::oneshot` — no network, no port binding, fast.

```rust
let response = app.oneshot(Request::builder().uri("/health").body(Body::empty())?).await?;
assert_eq!(response.status(), StatusCode::OK);
```

- **Deployment**: multi-stage Docker build on `distroless` or `scratch` with a musl target — final images are commonly under 20 MB.

<br>

## 60. Discuss the use of _Rust_ for network programming and available libraries.

Rust is well-suited to network code: no GC pauses, predictable latency, and memory safety in exactly the place where bugs become CVEs (parsing untrusted bytes).

### The Standard Library — Blocking

```rust
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

fn handle(mut stream: TcpStream) -> std::io::Result<()> {
    let mut buf = [0u8; 1024];
    let n = stream.read(&mut buf)?;
    stream.write_all(&buf[..n])?;      // echo
    Ok(())
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080")?;
    for stream in listener.incoming() {
        std::thread::spawn(move || handle(stream?));
    }
    Ok(())
}
```

Fine up to a few thousand connections. Beyond that, one OS thread per connection stops scaling.

### Async — Tokio

```rust
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

async fn handle(mut socket: TcpStream) -> std::io::Result<()> {
    let mut buf = vec![0u8; 1024];
    loop {
        let n = socket.read(&mut buf).await?;
        if n == 0 { return Ok(()); }        // peer closed
        socket.write_all(&buf[..n]).await?;
    }
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("0.0.0.0:8080").await?;
    loop {
        let (socket, addr) = listener.accept().await?;
        tokio::spawn(async move {
            if let Err(e) = handle(socket).await {
                eprintln!("{addr}: {e}");
            }
        });
    }
}
```

Same shape, but each connection is a task costing a few hundred bytes instead of an 8 KB thread stack — hundreds of thousands of concurrent connections on one machine.

### The Library Landscape

| Layer | Crates |
|---|---|
| Runtime / async I/O | **tokio**, async-std, smol, **mio** (raw epoll/kqueue) |
| HTTP server | **hyper** (low-level), **axum**, actix-web, warp, poem |
| HTTP client | **reqwest**, **ureq** (blocking, tiny), **hyper** |
| gRPC | **tonic** |
| WebSocket | **tokio-tungstenite**, fastwebsockets |
| QUIC / HTTP3 | **quinn**, **s2n-quic** |
| TLS | **rustls** (pure Rust, no OpenSSL), native-tls |
| DNS | **hickory-dns** (formerly trust-dns) |
| Serialization | **serde** + `serde_json` / `bincode` / **prost** (protobuf) |
| Parsing | **nom**, **winnow** — zero-copy binary/text protocol parsers |
| Socket options | **socket2** |
| Packets / raw | **pnet**, **etherparse** |
| P2P | **libp2p** |

### Patterns That Come Up

**Framed protocols** — `tokio_util::codec` turns a byte stream into a stream of messages:

```rust
use tokio_util::codec::{Framed, LinesCodec};
use futures::{SinkExt, StreamExt};

let mut framed = Framed::new(socket, LinesCodec::new());
while let Some(Ok(line)) = framed.next().await {
    framed.send(format!("echo: {line}")).await?;
}
```

**Timeouts and cancellation** — always bound network waits:

```rust
match tokio::time::timeout(Duration::from_secs(5), stream.read(&mut buf)).await {
    Ok(Ok(n)) => { /* ... */ }
    Ok(Err(e)) => return Err(e),
    Err(_elapsed) => return Err(io::Error::new(io::ErrorKind::TimedOut, "read timeout")),
}
```

**Graceful shutdown** — a `watch` channel or `CancellationToken` fanned out to every task.

**Backpressure** — bounded channels between accept loop and workers, so a traffic spike blocks rather than exhausting memory.

### Why It Matters Here Specifically

A large share of historical CVEs in network daemons are buffer overflows and use-after-free in packet parsers. In safe Rust those are not reachable, which is why Cloudflare (Pingora), AWS (s2n, Firecracker), Discord, and the Linux kernel network drivers have all moved code in this direction.

<br>

## 61. What factors might lead you to choose _Rust_ for a new command-line tool development?

### 1. A Single Static Binary

No interpreter, no runtime, no `node_modules`, no virtualenv. Users download one file and run it. With a musl target the binary has zero dynamic dependencies and runs on any Linux from the last decade.

```bash
cargo build --release --target x86_64-unknown-linux-musl
```

This alone explains a lot of the CLI ecosystem's migration: `ripgrep`, `fd`, `bat`, `exa`/`eza`, `hyperfine`, `tokei`, `starship`, `difftastic`, `uv`, and `ruff` are all Rust.

### 2. Startup Time

A CLI is invoked constantly, often in loops and shell prompts. Rust starts in ~1 ms; a Python tool pays 30–80 ms of interpreter startup before doing anything, and a JVM tool much more. For a tool run in a `for` loop or a git hook, this dominates.

### 3. Throughput

CLIs are frequently I/O- and parse-heavy — exactly where Rust wins. `ripgrep` beats `grep` on large trees not by magic but by parallel directory walking, SIMD-friendly matching, and no allocation in the hot loop.

### 4. `clap` Is Excellent

Argument parsing, subcommands, validation, shell completions, and `--help` generation come from one derive:

```rust
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "mytool", version, about = "Does a thing")]
struct Cli {
    #[arg(short, long, global = true)]
    verbose: bool,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Build { #[arg(short, long, default_value_t = 4)] jobs: usize },
    Clean { #[arg(long)] force: bool },
}

fn main() {
    let cli = Cli::parse();
    match cli.command {
        Commands::Build { jobs } => println!("building with {jobs} jobs"),
        Commands::Clean { force } => println!("cleaning (force={force})"),
    }
}
```

Generate completions for bash/zsh/fish/PowerShell with `clap_complete`.

### 5. Cross-Platform Distribution

One codebase, builds for Linux/macOS/Windows on x86-64 and ARM. `cargo-dist` will produce installers and GitHub releases for all of them; users can also `cargo install`, or get it from Homebrew/apt.

### 6. Correctness Under Weird Input

CLIs eat untrusted paths, encodings, and streams. `Result` forces the error paths to be handled, `OsString`/`Path` handle non-UTF-8 filenames correctly, and there are no buffer overflows in the parsing.

### 7. The Supporting Ecosystem

| Need | Crate |
|---|---|
| Args | **clap** |
| Errors for humans | **anyhow** (+ `color-eyre` for pretty reports) |
| Progress bars | **indicatif** |
| Colour / styling | **owo-colors**, **console** |
| Prompts | **dialoguer**, **inquire** |
| Full TUI | **ratatui** |
| Config files | **config**, **figment** |
| Parallelism | **rayon** |
| CLI testing | **assert_cmd**, **trycmd**, **insta** |

### When *Not* to Choose Rust

- The tool is 50 lines of glue — a shell script or Python is done in ten minutes.
- It must be edited in place by users on the target machine.
- Your team has no Rust experience and the tool is one-off.
- It depends heavily on a library that only exists in another ecosystem.

Compile times are the real day-to-day cost; for a small CLI they stay tolerable, and `cargo check` covers the fast feedback loop.

<br>

## 62. Describe how you would implement _file I/O operations_ in _Rust_.

### Whole-File Convenience

```rust
use std::fs;

let text = fs::read_to_string("input.txt")?;    // String — must be valid UTF-8
let bytes = fs::read("image.png")?;             // Vec<u8> — any content

fs::write("output.txt", "hello")?;              // create or truncate, then write
fs::write("out.bin", &bytes)?;
```

Perfect for config files. Do **not** use them on files that might be gigabytes.

### Streaming — Always Buffer

```rust
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};

let file = File::open("large.log")?;
let reader = BufReader::new(file);              // without this, one syscall per read

for line in reader.lines() {
    let line = line?;                           // each yields io::Result<String>
    if line.contains("ERROR") { println!("{line}"); }
}

let out = File::create("report.txt")?;
let mut writer = BufWriter::new(out);
for i in 0..1_000_000 {
    writeln!(writer, "line {i}")?;
}
writer.flush()?;                                // flush explicitly — Drop swallows errors
```

`BufWriter` flushes on drop, but the error is discarded. Call `flush()?` yourself when the data matters.

### Reusing the Line Buffer

`lines()` allocates a `String` per line. In a hot loop:

```rust
let mut buf = String::new();
while reader.read_line(&mut buf)? != 0 {
    process(&buf);
    buf.clear();                                // reuse the allocation
}
```

### `OpenOptions` — Full Control

```rust
use std::fs::OpenOptions;

let mut f = OpenOptions::new()
    .read(true)
    .append(true)
    .create(true)          // create if missing
    // .create_new(true)   // fail if it already exists (atomic guard)
    // .truncate(true)
    .open("app.log")?;

writeln!(f, "started at {:?}", std::time::SystemTime::now())?;
```

### Paths

```rust
use std::path::{Path, PathBuf};

let mut p = PathBuf::from("/var/log");
p.push("app");
p.set_extension("log");                 // /var/log/app.log

if p.exists() && p.is_file() {
    println!("{:?} {:?}", p.file_stem(), p.extension());
}

let joined = Path::new("data").join("2026").join("report.csv");   // portable separators
```

Use `Path`/`PathBuf`, never string concatenation — it is what makes the code work on Windows, and it handles non-UTF-8 filenames that would break a `String`.

### Directories

```rust
fs::create_dir_all("out/reports/2026")?;

for entry in fs::read_dir("data")? {
    let entry = entry?;
    let meta = entry.metadata()?;
    println!("{:?} {} bytes, dir={}", entry.path(), meta.len(), meta.is_dir());
}

fs::copy("a.txt", "b.txt")?;
fs::rename("old.txt", "new.txt")?;      // atomic within a filesystem
fs::remove_file("temp.txt")?;
fs::remove_dir_all("build")?;           // recursive — be careful
```

Recursive traversal is easier with the **walkdir** crate.

### Seeking and Random Access

```rust
use std::io::{Seek, SeekFrom, Read};

let mut f = File::open("data.bin")?;
f.seek(SeekFrom::Start(1024))?;
let mut buf = [0u8; 64];
f.read_exact(&mut buf)?;                // errors on short read, unlike read()
```

### Error Handling That Helps

```rust
match File::open(path) {
    Ok(f) => f,
    Err(e) if e.kind() == std::io::ErrorKind::NotFound => create_default(path)?,
    Err(e) => return Err(e),
}
```

Add context, since a bare `io::Error` never says *which* file:

```rust
use anyhow::Context;
let text = fs::read_to_string(&path)
    .with_context(|| format!("reading {}", path.display()))?;
```

### Async and Advanced

```rust
// tokio — only worth it when I/O concurrency matters; the OS has no true async file I/O
let text = tokio::fs::read_to_string("config.toml").await?;
```

- **memmap2** — memory-map huge files for random access.
- **tempfile** — safe temporary files and directories that clean themselves up.
- **Atomic writes** — write to `file.tmp`, then `fs::rename` over the target, so a crash never leaves a half-written file.

<br>

## 63. What are some challenges you might face when integrating _Rust_ in a larger, language-diverse codebase?

### 1. The FFI Boundary Is Where Safety Stops

Every call across the boundary is `unsafe`, and the guarantees Rust makes internally do not extend past it. You must decide and document, for every pointer that crosses: who allocates, who frees, and how long it stays valid. Getting this wrong reintroduces exactly the bugs Rust was adopted to eliminate.

Panics must not unwind into C — catch them at the boundary and convert to error codes.

### 2. Type and Error Impedance Mismatch

Rust's expressive types have no equivalent on the other side:

| Rust | Crossing the boundary |
|---|---|
| `Result<T, E>` | error codes + out-params, or exceptions |
| `Option<T>` | nullable pointer |
| `String` (UTF-8, not null-terminated) | `CString`/`CStr` conversion, possible allocation |
| enums with data | tagged struct or union |
| traits / generics | erased entirely — monomorphize a concrete set |
| lifetimes | gone; ownership becomes convention |

Every conversion is a place to leak or double-free.

### 3. Build System Integration

Cargo assumes it owns the build. CMake, Bazel, Gradle, Maven, setuptools, and webpack all assume the same. Making them cooperate is real work: `corrosion` (CMake), `rules_rust` (Bazel), `maturin` (Python), `napi-rs` (Node). Add cross-compilation for every target platform, and CI gets complicated — you now need a Rust toolchain everywhere the other language builds.

### 4. Compile Times

A large Rust workspace can take minutes to build clean. In a polyglot repo where other teams expect a fast edit-compile-test loop, this becomes a political problem as much as a technical one. Mitigations: `cargo check`, `sccache`, splitting into smaller crates, fewer proc macros, `lld`/`mold` as linker.

### 5. Async Runtime Mismatch

Rust async does not compose with another language's event loop for free. Bridging Tokio to Python's asyncio or Node's libuv means blocking calls, callback shims, or `pyo3-asyncio`. Also: holding Python's GIL across an `.await` is a deadlock waiting to happen.

### 6. Team Learning Curve

Ownership, lifetimes, and the trait system take weeks to internalize. In a mixed codebase this creates a bus-factor problem — the Rust component becomes "the thing only Ana can change". Realistically you need at least two or three people fluent enough to review before Rust is a safe dependency.

### 7. Debugging and Observability Across the Boundary

Stack traces stop at the FFI line. Profilers show one opaque frame. Logs come from two systems with different formats and levels. You need deliberate work to unify tracing (`tracing` → OpenTelemetry) and to keep symbols available in release builds:

```toml
[profile.release]
debug = 1        # keep line-table info for profilers
```

### 8. Dependency and Toolchain Duplication

Two package managers, two lockfiles, two audit surfaces, two sets of licence obligations. Version skew (`openssl` in Rust vs. the system one) causes confusing runtime failures. A shared binary may end up linking two TLS stacks.

### 9. ABI Instability

Rust has **no stable ABI**. Two Rust libraries built with different compiler versions cannot safely exchange non-`#[repr(C)]` types. Anything crossing a dynamic-library boundary must be `extern "C"` + `#[repr(C)]`, or you must build everything together.

### What Actually Works

Start with a **narrow, high-value component** — a hot parser, a crypto routine, an encoding loop — with a small, well-defined interface. Keep the FFI surface tiny and hand-audited, and let it grow only when the boundary proves stable. Prefer a high-level binding generator (`pyo3`, `napi-rs`, `cxx`, `wasm-bindgen`) over hand-written `extern "C"`; they generate the marshalling and get the ownership rules right. Write the integration tests from the *calling* language's side, and run the Rust side under Miri and ASan in CI. This is the path Dropbox, Discord, Figma, and the `ruff`/`uv` projects all took.

<br>

## 64. How does _Rust_ handle default parameter values in functions?

**Rust has no default parameter values, and no function overloading.** Every function takes exactly the arguments in its signature. This is deliberate: it keeps call resolution unambiguous and type inference tractable.

Here are the idiomatic substitutes, roughly in order of how often you should reach for them.

### 1. `Option<T>` Parameters

Simple, explicit, and honest — at the cost of a noisy call site:

```rust
fn connect(host: &str, port: Option<u16>, timeout: Option<Duration>) -> Connection {
    let port = port.unwrap_or(8080);
    let timeout = timeout.unwrap_or(Duration::from_secs(30));
    // ...
}

connect("localhost", None, None);
connect("localhost", Some(9000), None);
```

Good for one or two optional arguments; unpleasant beyond that.

### 2. The Builder Pattern — the Standard Answer

For anything with several optional settings, this is what idiomatic Rust does:

```rust
pub struct Server { host: String, port: u16, workers: usize, tls: bool }

pub struct ServerBuilder { host: String, port: u16, workers: usize, tls: bool }

impl Server {
    pub fn builder(host: impl Into<String>) -> ServerBuilder {
        ServerBuilder { host: host.into(), port: 8080, workers: 4, tls: false }
    }
}

impl ServerBuilder {
    pub fn port(mut self, p: u16) -> Self { self.port = p; self }
    pub fn workers(mut self, n: usize) -> Self { self.workers = n; self }
    pub fn tls(mut self, on: bool) -> Self { self.tls = on; self }
    pub fn build(self) -> Server {
        Server { host: self.host, port: self.port, workers: self.workers, tls: self.tls }
    }
}

let s = Server::builder("0.0.0.0").port(3000).tls(true).build();
```

Required arguments go in `builder(...)`; optional ones become methods. The `derive_builder` and `bon` crates generate this for you.

### 3. `Default` + Struct Update Syntax

```rust
#[derive(Debug)]
pub struct Config { pub host: String, pub port: u16, pub retries: u32, pub verbose: bool }

impl Default for Config {
    fn default() -> Self {
        Config { host: "localhost".into(), port: 8080, retries: 3, verbose: false }
    }
}

let cfg = Config { port: 9000, ..Default::default() };
```

Concise and readable — the closest thing Rust has to Python's keyword arguments.

### 4. Multiple Named Constructors

```rust
impl Buffer {
    pub fn new() -> Self { Self::with_capacity(1024) }
    pub fn with_capacity(n: usize) -> Self { /* ... */ }
}
```

`Vec::new` / `Vec::with_capacity` and `HashMap::new` / `HashMap::with_hasher` follow exactly this convention.

### 5. `impl Into<T>` for Flexible Argument Types

Not a default, but it removes the most common reason people want overloads:

```rust
fn greet(name: impl Into<String>) { println!("Hello, {}", name.into()); }

greet("Alice");
greet(String::from("Bob"));
```

### 6. A Macro, if You Truly Need Variadics

```rust
macro_rules! connect {
    ($host:expr) => { connect_impl($host, 8080, 30) };
    ($host:expr, $port:expr) => { connect_impl($host, $port, 30) };
    ($host:expr, $port:expr, $timeout:expr) => { connect_impl($host, $port, $timeout) };
}
```

Works, but costs you type checking and IDE support. Use it last.

### Choosing

| Situation | Approach |
|---|---|
| 1–2 optional args | `Option<T>` |
| Many optional args, complex construction | builder |
| A settings struct | `Default` + `..Default::default()` |
| A few common presets | named constructors |
| Flexible input types | `impl Into<T>` / `AsRef<T>` |

<br>

## 65. Discuss _Rust's release channels_ and the _stability guarantee_.

### The Three Channels

| Channel | Cadence | Purpose |
|---|---|---|
| **stable** | every 6 weeks | production; only stabilized features |
| **beta** | every 6 weeks | release candidate for the next stable |
| **nightly** | every night | unstable features, behind `#![feature(...)]` gates |

### The Train Model

Everything merges into **nightly** (branched from master). Every six weeks, nightly branches to **beta**; six weeks later that beta becomes **stable**. So a feature merged today reaches stable in 6–12 weeks, and every change has spent 12 weeks being tested before it reaches production users.

```
master → nightly → (6 weeks) → beta → (6 weeks) → stable
```

Version numbers march linearly: 1.86, 1.87, 1.88 — there is no Rust 2.0 planned, ever.

### The Stability Guarantee

> **Code that compiles on stable Rust 1.x will continue to compile on 1.y for y > x.**

This is the core promise, and it is taken seriously: the release team runs **Crater**, which compiles *every crate on crates.io* plus a large sample of public GitHub repositories against a candidate compiler and reports any new failures. A regression is a release blocker.

**What the guarantee does not cover:**

- New lints and warnings (warnings are not breaking).
- Bug fixes for behaviour that was always incorrect or unsound.
- Type inference changes that break code relying on ambiguity.
- The standard library adding a method that makes a call ambiguous with your trait's method.
- Anything on nightly, by definition.
- **ABI** — that is not stable at all, between versions or even between builds.

### Nightly and Feature Gates

Unstable features are usable only on nightly, and only with an explicit opt-in:

```rust
#![feature(generic_const_exprs)]
```

```bash
rustup toolchain install nightly
cargo +nightly build
rustup override set nightly       # pin this directory
```

This is the key design decision: instead of letting unfinished features leak into production, they are gated. Users who want them accept the risk explicitly; everyone else is unaffected.

Some tooling is still nightly-only: Miri, `-Z` compiler flags, some `rustfmt` options, and `cargo expand`'s nicer output.

### Editions — Opt-In Breaking Changes

Editions (2015, 2018, 2021, 2024) allow *syntactic* changes that would otherwise be breaking, without splitting the ecosystem:

```toml
[package]
edition = "2021"
```

Crates of **different editions interoperate freely**, because editions only affect how a crate's own source is parsed — they all compile to the same internal representation. That is how `async` and `dyn` could become keywords without breaking 2015 code. `cargo fix --edition` automates most migrations.

### Pinning and MSRV

```toml
# rust-toolchain.toml — pins the toolchain for everyone and for CI
[toolchain]
channel = "1.86.0"
components = ["rustfmt", "clippy"]
```

```toml
# Cargo.toml — the minimum supported Rust version
[package]
rust-version = "1.75"
```

Cargo refuses to build with an older toolchain than `rust-version`, giving a clear error instead of a confusing compile failure. Library authors are expected to treat an MSRV bump as at least a minor version change.

### Practical Guidance

Use **stable** for everything you ship. Run **beta** in CI as an early-warning signal for upcoming regressions. Use **nightly** only for tooling (Miri, formatting options) or when a specific unstable feature is genuinely required — and pin the exact nightly date if you do, because nightly can break daily.
