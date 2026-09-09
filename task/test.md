# Rust Unit Tests for Beginners — From Scratch

This guide explains Rust unit testing step by step, assuming you are new to Rust or testing.

---

## 1. What is a unit test?

A **unit test** is a small piece of code that checks whether another small piece of code works correctly.

Example:

If you write this function:

```rust
fn add(a: i32, b: i32) -> i32 {
    a + b
}
```

You can write a test like:

```rust
#[test]
fn test_add() {
    assert_eq!(add(2, 3), 5);
}
```

This test checks:

> Does `add(2, 3)` return `5`?

If yes, the test passes.  
If no, the test fails.

---

# 2. Create a Rust project

Open your terminal.

Create a library project:

```bash
cargo new demo --lib
cd demo
```

Why `--lib`?

Because unit tests are easiest to learn in a library project.

Your project structure looks like this:

```text
demo/
  Cargo.toml
  src/
    lib.rs
```

Open `src/lib.rs`.

---

# 3. Your first Rust test

Replace the contents of `src/lib.rs` with this:

```rust
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(2, 3), 5);
    }
}
```

Then run:

```bash
cargo test
```

You should see something like:

```text
running 1 test
test tests::test_add ... ok
```

That means your test passed.

---

# 4. What does each part mean?

Let's break it down.

---

## `#[test]`

```rust
#[test]
fn test_add() {
    assert_eq!(add(2, 3), 5);
}
```

`#[test]` tells Rust:

> This function is a test.

Rust will run this function when you execute `cargo test`.

---

## `#[cfg(test)]`

```rust
#[cfg(test)]
mod tests {
    // tests go here
}
```

This means:

> Only compile this module when running tests.

So your test code is not included in your normal production build.

---

## `mod tests`

```rust
mod tests {
    // test functions
}
```

This creates a module named `tests`.

It is just a container for your test functions.

---

## `use super::*;`

```rust
use super::*;
```

`super` means the parent module.

In this case, it lets the tests use items from `lib.rs`.

So this line allows the test to use:

```rust
add(2, 3)
```

without writing a long path.

---

# 5. Assertions

Assertions are macros that check conditions.

The most common ones are:

```rust
assert!
assert_eq!
assert_ne!
```

---

## `assert!`

Checks that something is `true`.

```rust
assert!(1 == 1);
```

Example:

```rust
#[test]
fn test_true_condition() {
    assert!(2 + 2 == 4);
}
```

If the condition is false, the test fails.

---

## `assert_eq!`

Checks that two values are equal.

```rust
assert_eq!(left, right);
```

Example:

```rust
assert_eq!(2 + 2, 4);
```

Very common in Rust tests.

---

## `assert_ne!`

Checks that two values are not equal.

```rust
assert_ne!(left, right);
```

Example:

```rust
assert_ne!(2 + 2, 5);
```

---

# 6. Make a failing test

Change the test to this:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(2, 3), 6);
    }
}
```

Run:

```bash
cargo test
```

Now it fails.

You will see something like:

```text
assertion `left == right` failed
  left: 5
 right: 6
```

This is useful because Rust tells you:

- expected value
- actual value

---

# 7. Testing your own functions

Let's write a more realistic example.

```rust
pub fn is_adult(age: i32) -> bool {
    age >= 18
}
```

Test:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_adult_true() {
        assert_eq!(is_adult(18), true);
    }

    #[test]
    fn test_is_adult_false() {
        assert_eq!(is_adult(17), false);
    }
}
```

Run:

```bash
cargo test
```

Good tests usually check:

- normal cases
- edge cases
- invalid inputs

---

# 8. Arrange, Act, Assert

A common testing pattern:

```rust
#[test]
fn test_example() {
    // Arrange: prepare data
    let a = 10;
    let b = 5;

    // Act: run the function
    let result = add(a, b);

    // Assert: check result
    assert_eq!(result, 15);
}
```

This makes tests easier to read.

---

# 9. Test names should explain behavior

Not great:

```rust
#[test]
fn test1() {
    // ...
}
```

Better:

```rust
#[test]
fn add_positive_numbers_returns_sum() {
    assert_eq!(add(2, 3), 5);
}
```

Good test names answer:

> What behavior is being tested?

Examples:

```rust
fn user_is_adult_when_age_is_18()
fn user_is_not_adult_when_age_is_17()
fn add_negative_numbers_returns_correct_sum()
```

---

# 10. Testing panics

Sometimes your code should panic when something invalid happens.

Example:

```rust
pub fn divide(a: i32, b: i32) -> i32 {
    if b == 0 {
        panic!("Cannot divide by zero");
    }

    a / b
}
```

To test that it panics, use:

```rust
#[test]
#[should_panic(expected = "Cannot divide by zero")]
fn divide_by_zero_panics() {
    divide(10, 0);
}
```

`#[should_panic]` means:

> This test passes only if the code panics.

`expected = "..."` checks the panic message.

---

# 11. Testing functions that return `Option`

Example:

```rust
pub fn first_word(text: &str) -> Option<&str> {
    let words: Vec<&str> = text.split_whitespace().collect();

    if words.is_empty() {
        None
    } else {
        Some(words[0])
    }
}
```

Test:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_word_returns_first_word() {
        assert_eq!(first_word("hello world"), Some("hello"));
    }

    #[test]
    fn first_word_returns_none_for_empty_string() {
        assert_eq!(first_word(""), None);
    }
}
```

---

# 12. Testing functions that return `Result`

Sometimes functions return:

```rust
Result<T, E>
```

Example:

```rust
pub fn parse_age(input: &str) -> Result<i32, String> {
    match input.parse::<i32>() {
        Ok(age) if age >= 0 => Ok(age),
        Ok(_) => Err("Age cannot be negative".to_string()),
        Err(_) => Err("Invalid age".to_string()),
    }
}
```

Test successful case:

```rust
#[test]
fn parse_age_success() {
    assert_eq!(parse_age("25"), Ok(25));
}
```

Test error case:

```rust
#[test]
fn parse_age_negative_error() {
    assert_eq!(
        parse_age("-5"),
        Err("Age cannot be negative".to_string())
    );
}
```

Test invalid input:

```rust
#[test]
fn parse_age_invalid_input() {
    assert_eq!(parse_age("abc"), Err("Invalid age".to_string()));
}
```

---

# 13. Unit tests can test private functions

In Rust, unit tests inside the same file can access private functions.

Example:

```rust
fn secret_helper(x: i32) -> i32 {
    x * 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_secret_helper() {
        assert_eq!(secret_helper(5), 10);
    }
}
```

Even though `secret_helper` is private, the test can use it because tests are inside the same module tree.

This is useful for testing internal logic.

---

# 14. Unit tests vs integration tests

Rust has two common test types:

## Unit tests

- Usually inside `src/lib.rs` or `src/main.rs`
- Test small pieces of code
- Can test private functions
- Fast

Example location:

```text
src/lib.rs
```

---

## Integration tests

- Located in the `tests/` folder
- Test your library from outside
- Can only use public API
- Useful for testing larger behavior

Example:

```text
demo/
  src/
    lib.rs
  tests/
    integration_test.rs
```

Example `tests/integration_test.rs`:

```rust
use demo;

#[test]
fn test_add_from_outside() {
    assert_eq!(demo::add(2, 3), 5);
}
```

For now, focus mostly on unit tests.

---

# 15. Running tests

Run all tests:

```bash
cargo test
```

Run only tests whose names contain a word:

```bash
cargo test add
```

Example:

```bash
cargo test adult
```

This runs tests with `adult` in their name.

---

## Run tests with output

By default, Rust captures print output.

If you use `println!` in a test and want to see it:

```rust
#[test]
fn test_with_print() {
    println!("This is visible");
    assert_eq!(2 + 2, 4);
}
```

Run:

```bash
cargo test -- --nocapture
```

---

## Run ignored tests

You can ignore a test:

```rust
#[test]
#[ignore]
fn slow_test() {
    assert_eq!(1, 1);
}
```

Run ignored tests only:

```bash
cargo test -- --ignored
```

---

# 16. Example: full beginner project

Here is a complete small example.

`src/lib.rs`:

```rust
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

pub fn is_even(n: i32) -> bool {
    n % 2 == 0
}

pub fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_works() {
        assert_eq!(add(2, 3), 5);
    }

    #[test]
    fn add_negative_numbers() {
        assert_eq!(add(-2, -3), -5);
    }

    #[test]
    fn is_even_true() {
        assert!(is_even(4));
    }

    #[test]
    fn is_even_false() {
        assert!(!is_even(5));
    }

    #[test]
    fn greet_returns_correct_message() {
        assert_eq!(greet("Alice"), "Hello, Alice!");
    }
}
```

Run:

```bash
cargo test
```

Expected:

```text
running 5 tests
test tests::add_works ... ok
test tests::add_negative_numbers ... ok
test tests::is_even_true ... ok
test tests::is_even_false ... ok
test tests::greet_returns_correct_message ... ok
```

---

# 17. Common beginner mistakes

## Mistake 1: Forgetting `#[test]`

This will not run as a test:

```rust
fn test_something() {
    assert_eq!(1, 1);
}
```

You need:

```rust
#[test]
fn test_something() {
    assert_eq!(1, 1);
}
```

---

## Mistake 2: Forgetting `use super::*;`

Without this, you may need to write full paths.

Usually write:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        // ...
    }
}
```

---

## Mistake 3: Writing one giant test

Bad:

```rust
#[test]
fn test_everything() {
    // 100 lines checking many things
}
```

Better:

```rust
#[test]
fn add_positive_numbers() {
    // ...
}

#[test]
fn add_negative_numbers() {
    // ...
}
```

Small tests are easier to debug.

---

## Mistake 4: Testing implementation instead of behavior

Instead of only checking internal details, test what the function should do.

Example:

For `is_adult`, test:

```rust
assert!(is_adult(18));
assert!(!is_adult(17));
```

That is the behavior users care about.

---

# 18. Good testing habits

For junior developers, these habits are very useful:

## 1. Write tests for simple functions first

Example:

```rust
fn add
fn multiply
fn is_valid_age
fn format_name
```

---

## 2. Test edge cases

Examples:

```rust
0
-1
empty string
very large input
boundary values
```

For age:

```rust
17
18
0
-1
```

---

## 3. One test checks one behavior

If a test fails, you should quickly know what broke.

---

## 4. Use clear failure messages

You can add a custom message:

```rust
assert!(age >= 0, "Age should not be negative, got {}", age);
```

Example:

```rust
#[test]
fn age_is_valid() {
    let age = 25;
    assert!(age >= 0, "Age should not be negative, got {}", age);
}
```

---

# 19. Quick test template

You can copy this template often:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_name_of_behavior() {
        // Arrange
        let input = something;

        // Act
        let result = function_to_test(input);

        // Assert
        assert_eq!(result, expected);
    }
}
```

---

# 20. Useful `cargo test` commands

| Command | Meaning |
|---|---|
| `cargo test` | Run all tests |
| `cargo test add` | Run tests with `add` in name |
| `cargo test -- --nocapture` | Show `println!` output |
| `cargo test -- --ignored` | Run ignored tests |
| `cargo test -- --test-threads=1` | Run tests one by one |

---

# 21. Mental model

Think of unit tests as small safety checks.

Before:

```rust
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}
```

You hope it works.

After tests:

```rust
#[test]
fn add_works() {
    assert_eq!(add(2, 3), 5);
}
```

You can prove it works for that case.

Every time you change code, you run:

```bash
cargo test
```

If tests still pass, you probably did not break existing behavior.

---

# 22. Beginner exercise

Try writing tests for these functions:

```rust
pub fn multiply(a: i32, b: i32) -> i32 {
    a * b
}

pub fn is_positive(n: i32) -> bool {
    n > 0
}

pub fn full_name(first: &str, last: &str) -> String {
    format!("{} {}", first, last)
}
```

Example tests to write:

```rust
multiply(2, 3) == 6
multiply(-2, 3) == -6
multiply(0, 5) == 0

is_positive(5) == true
is_positive(-5) == false
is_positive(0) == false

full_name("John", "Doe") == "John Doe"
```

---

# 23. Simple summary

Rust unit testing basics:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_something() {
        assert_eq!(2 + 2, 4);
    }
}
```

Important points:

- `#[test]` marks a test function
- `cargo test` runs tests
- `assert!` checks true conditions
- `assert_eq!` checks equality
- `assert_ne!` checks inequality
- `#[should_panic]` tests expected panics
- `#[ignore]` skips a test
- Unit tests usually live inside `src`
- Integration tests live in the `tests/` folder
