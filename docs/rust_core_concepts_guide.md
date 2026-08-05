# Rust Core Concepts 

## 1. Serde 

Serialization is the process of converting in-memory Rust data structures (structs, enums) into a format that can be stored on disk or transmitted over a network, and deserializing them back into Rust objects.

### 1. Serde (Serialization / Deserialization)

* **What it does:** Converts Rust data into flexible, standard text or structured formats like JSON, YAML, XML or TOML.
* **Analogy:** Writing a message in plain English or standard text so that any app or browser can easily read and inspect it.
* **Best used for:** Web APIs, configuration files, CLI inputs/outputs, and human-readable data.

---

### 2. Borsh (Binary Object Representation Serializer for Hashing)

* **What it does:** Converts Rust data into a compact binary stream of bytes (0s and 1s).
* **Analogy:** Packing a suitcase tightly into a custom binary box so it takes up minimal space and loads ultra-fast.
* **Best used for:** High-performance caching, state persistence, binary protocols, and security-critical systems (where exact byte-for-byte consistency is required).

Borsh converts Rust structs directly into a strict, compact binary byte array (`Vec<u8>`).

---

### Summary

* Use Serde when you need flexibility and readability (e.g., JSON/YAML).
* Use Borsh when you need maximum speed and small size (compact binary).

### Code Comparison

#### Serde Example (JSON)
```rust
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct User {
    id: u32,
    username: String,
    is_active: bool,
}

fn main() {
    let user = User { id: 42, username: "alice".into(), is_active: true };

    // Serialize struct -> JSON String
    let json_text = serde_json::to_string(&user).unwrap();
    println!("{}", json_text);
    // Output: {"id":42,"username":"alice","is_active":true}

    // Deserialize JSON String -> struct
    let restored: User = serde_json::from_str(&json_text).unwrap();
    println!("{:?}", restored);
}
```

#### Borsh Example (Binary)
```rust
use borsh::{BorshSerialize, BorshDeserialize};

#[derive(BorshSerialize, BorshDeserialize, Debug)]
struct User {
    id: u32,
    username: String,
    is_active: bool,
}

fn main() {
    let user = User { id: 42, username: "alice".into(), is_active: true };

    // Serialize struct -> Compact byte vector
    let bytes: Vec<u8> = borsh::to_vec(&user).unwrap();
    println!("{:?}", bytes);
    // Output (raw bytes): [42, 0, 0, 0, 5, 0, 0, 0, 97, 108, 105, 99, 101, 1]

    // Deserialize byte vector -> struct
    let restored: User = borsh::from_slice(&bytes).unwrap();
    println!("{:?}", restored);
}
```

---

## 2. Classes in Rust (`struct` + `impl`)

Rust does not have a traditional `class` keyword. Instead, it separates **data fields** from **methods and behavior**.

| OOP Class Concept | Rust Equivalent | Description |
| :--- | :--- | :--- |
| **Attributes / Properties** | `struct` | Defines the shape and fields of the data |
| **Methods / Functions** | `impl` block | Defines functions that operate on the `struct` |
| **`this` / `self`** | `&self` / `&mut self` | First argument passed to instance methods |

### Example Comparison

#### Python / JavaScript / Java Class Concept:

```python
class User:
    def __init__(self, username, email):
        self.username = username
        self.email = email

    def greet(self):
        print(f"Hello, {self.username}!")
```

#### Rust Equivalent (`struct` + `impl`):

```rust
// 1. Define the DATA (the fields)
struct User {
    username: String,
    email: String,
}

// 2. Define the BEHAVIOR (the methods)
impl User {
    // Constructor-like associate function (conventionally called `new`)
    fn new(username: String, email: String) -> Self {
        User { username, email }
    }

    // Method operating on an instance (&self = read-only borrow of the object)
    fn greet(&self) {
        println!("Hello, {}!", self.username);
    }
}

fn main() {
    // Creating an object / instance
    let user = User::new("Alice".into(), "alice@example.com".into());
    user.greet(); // Outputs: Hello, Alice!
}
```

---

## 3. Rust Syntax Rules Quick-Sheet

### 1. Semicolon `;` vs. NOTHING at the end of a line

This is the #1 thing that confuses people in Rust:

* **NO semicolon `;`** at the last line of a function/block = RETURN THIS VALUE
* **WITH a semicolon `;`** = DO THIS ACTION AND MOVE TO THE NEXT LINE

#### Example:

```rust
fn add_one(x: i32) -> i32 {
    x + 1  // NO semicolon = Rust RETURNS this value (same as `return x + 1;`)
}

fn add_one_explicit(x: i32) -> i32 {
    return x + 1; // Explicit return also requires semicolon
}

fn do_something(x: i32) {
    println!("{}", x); // WITH semicolon = statement (does action, returns nothing)
}
```

> 💡 **Rule of thumb:** In a function or `if`/`match` block, the final line without `;` is what the block evaluates/returns.

---

### 2. Comma `,` vs. Semicolon `;`

#### Use COMMAS `,` for LISTS of things:

* Struct definitions & struct instances
* Function parameters
* Enum variants
* Array/Vector elements

```rust
// Struct Definition (uses commas)
struct Point {
    x: i32,  // comma
    y: i32,  // comma (trailing comma is allowed and recommended in Rust!)
}

// Struct Instance (uses commas)
let p = Point {
    x: 10,   // comma
    y: 20,   // comma
};

// Function Parameters (uses commas)
fn calculate(a: i32, b: i32) -> i32 {
    a + b
}
```

#### Use SEMICOLONS `;` for ACTIONS inside code blocks:

```rust
fn main() {
    let x = 5;      // Semicolon: creating a variable statement
    let y = x * 2;  // Semicolon: creating a variable statement

    println!("{}", y); // Semicolon: executing a function call statement
}
```

---

### 3. Quick Summary Cheat Sheet

| Symbol | Name | When is it used? | Example |
| :--- | :--- | :--- | :--- |
| **`;`** | Semicolon | Ends a statement / action | `let x = 5;` |
| *(nothing)* | No Semicolon | Returns a value from a block/function | `x + 1` |
| **,** | Comma | Separates fields in structs, enums, parameters, and lists | `Point { x: 1, y: 2 }` |
| **`:`** | Colon | Declares a type OR assigns a value to a struct field | `let x: i32` or `x: 10` |
| **`::`** | Double Colon | Accesses items in a module/impl (path) | `User::new()` or `std::fs::read()` |

If you ever get stuck on a line, ask: *"Is this line an action (needs `;`), a list item (needs `,`), or the final returned value (needs nothing)?"*

> 💡 **Key Rule:** The final line of a block or function without a `;` becomes the returned value of that block.

---

## 4. Generics (`<T>`)

Generics allow you to write functions, structs, or enums with **type placeholders** (such as `<T>`), eliminating code duplication across different types.

### Problem: Duplicated Code
```rust
fn sum_i32(a: i32, b: i32) -> i32 { a + b }
fn sum_f32(a: f32, b: f32) -> f32 { a + b }
```

### Solution: Generics Placeholder
```rust
struct StorageBox<T> {
    item: T,
}

let int_box = StorageBox { item: 100 };
let str_box = StorageBox { item: "Hello".to_string() };
```

---

## 5. Traits (`trait`)

A **Trait** defines a contract of capabilities (methods) that different types can choose to implement.

### 💡 Real-World Analogy: A USB-C Cable

Think of a USB-C Charger.
Your charger doesn't care if you plug in a Phone, a Laptop, or a Pair of Headphones. All it cares about is that the device has a USB-C port and knows how to accept charge.

* The **USB-C specification** is the Trait (the contract of what a device can do).
* The **Phone** or **Laptop** are the Structs that fulfill that contract.

```rust
trait Speak {
    fn speak(&self);
}

struct Dog;
struct Robot;

impl Speak for Dog {
    fn speak(&self) { println!("Woof!"); }
}

impl Speak for Robot {
    fn speak(&self) { println!("Beep boop!"); }
}
```

---

## 6. Trait Bounds (`<T: Add>`)

### Why does `fn sum<T>(a: T, b: T) -> T { a + b }` fail?
Plain `<T>` tells Rust that `T` could be **literally any type** (including booleans, files, or database connections). Since you cannot add two database connections with `+`, the compiler rejects `a + b`.

### 💡 Real-World Analogy:

> "I have a charger that charges any device `T` (Generic), as long as that device has a USB-C port (Trait)."

### Solution: Trait Bounds
A Trait Bound restricts `<T>` to **only types that implement a specific trait**:

```rust
use std::ops::Add;

// T: Add<Output = T> guarantees T supports the `+` operator
fn sum<T: Add<Output = T>>(a: T, b: T) -> T {
    a + b
}

fn main() {
    sum(5, 10);     //  i32 implements Add
    sum(2.5, 3.5);  //  f64 implements Add
    // sum(true, false); //  Compilation Error! bool does NOT implement Add
}
```

---

## 7. Derives (`#[derive(...)]`)

The `#[derive(...)]` macro automatically generates standard trait implementations for custom structs:

* **`#[derive(Debug)]`**: Allows formatting and printing with `println!("{:?}", my_struct)`.
* **`#[derive(PartialEq)]`**: Enables equality comparison with `p1 == p2` and `p1 != p2`.

```rust
#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}
```

---

## 8. Custom Wrapper Implementation (Newtype Pattern)

If a type (like `bool`) does not implement a trait (like `Add`) by default, you can wrap it in a custom struct and implement the trait yourself:

```rust
use std::ops::Add;

// Your custom struct
#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

// 1. You teach Rust how to "+" two Points together
impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

// 2. Your generic sum function!
fn sum<T: Add<Output = T>>(a: T, b: T) -> T {
    a + b
}

fn main() {
    let p1 = Point { x: 1, y: 2 };
    let p2 = Point { x: 3, y: 4 };

    // Point now works in sum() because it implements Add!
    let p3 = sum(p1, p2);

    println!("{:?}", p3); // Outputs: Point { x: 4, y: 6 }
}
```

Your logic step-by-step is flawless:

1. **The Problem:** Writing `sum_i32` and `sum_f32` is repetitive and bad practice.
2. **Generics:** `<T>` fixes repetition so you write one function for all types.
3. **The Trap:** Plain `<T>` allows literally anything (even booleans, sockets, images). So Rust won't let you use `a + b` because `true + false` makes no sense.
4. **The Solution (Traits):** Trait bounds `<T: Add>` restrict `<T>` to only types that know how to add.
5. **The Analogy:**
   * Laptop / Phone / Headset = `i32`, `f32`, `String` (The types).
   * Banana = `bool` (A type without USB-C).
   * USB-C Cable = `Add` Trait (The contract / capability).
   * Trying `sum(true, false)` is like trying to plug a banana into a USB-C charger. The compiler stops you!

---

### The ONE tiny thing to tweak in your mind:

You said:
> "ADD is class which has the allowed types for adding values. this is hardcoded values..."

It's not hardcoded inside `Add`. Think of `Add` as a **License Bureau**:

* `i32` went to the bureau and got an `Add` license.
* `f32` went to the bureau and got an `Add` license.
* `String` went to the bureau and got an `Add` license.
* `bool` never applied for an `Add` license.

So when your `sum` function demands `<T: Add>`, it is asking:
> "Show me your Add license!"

* `sum(1, 4)` ➡️ `i32` shows its `Add` license 🟢 APPROVED
* `sum(3.4, 5.1)` ➡️ `f32` shows its `Add` license 🟢 APPROVED
* `sum(true, false)` ➡️ `bool` has no `Add` license 🔴 REJECTED BY COMPILER