# Rust

## Resources

- **Web Cohort 3 Notion:** https://petal-estimate-4e9.notion.site/Web-3-Cohort-d1b49c992dbf4648b185f974523d127c
- **Rust Notion:** https://petal-estimate-4e9.notion.site/Rust-for-Solana-contracts-1937dfd1073580c8a8fbc7e135e6d22a

---

## Setup & Commands

- `cargo init` — initial setup, creates `main.rs` and `Cargo.toml`
- `cargo run` — compile and run the program
- `println!("{}", ans)` — similar to `printf("%d", ans)` in C/C++ or `print(ans)` in Python

### Cargo.toml (Binary Targets)

```toml
[package]
name = "Rust"
version = "0.1.0"
edition = "2024"

[dependencies]

[[bin]]
name = "borrow"
path = "src/borrow.rs"

[[bin]]
name = "rectangle"
path = "src/rectangle.rs"

[[bin]]
name = "enum"
path = "src/enum.rs"

[[bin]]
name = "enum2"
path = "src/enum2.rs"

[[bin]]
name = "string"
path = "src/string.rs"
```

---

## 1. Variables

Variables are **immutable by default**. Use `mut` to make them mutable.

```rust
fn main() {
    let x = 5;
    println!("The value of x is: {x}");
    x = 6; // ❌ ERROR — x is immutable
    println!("The value of x is: {x}");
}
```

> We are changing `x` without `mut`, so this gives an error.

**Fix:**

```rust
let mut x = 5;
```

### `const` and `static`

Besides `let`, Rust has two more ways to declare values:

| | `let` | `const` | `static` |
|---|---|---|---|
| Mutable? | Yes (with `mut`) | ❌ Never | Yes (with `mut`, but unsafe) |
| Type annotation | Optional (inferred) | **Required** | **Required** |
| Scope | Local (inside function) | Any scope | Global (entire program) |
| Evaluated | At runtime | At **compile time** | At **compile time** |
| Lifetime | Dies when scope ends | Lives as long as needed | Lives for **entire program** |

```rust
// const — must be UPPERCASE, type required, value known at compile time
const MAX_POINTS: u32 = 100_000;

// static — similar to const but has a fixed memory address
static GREETING: &str = "Hello, world!";

fn main() {
    let x = 5;             // local variable
    println!("{MAX_POINTS}"); // const works everywhere
    println!("{GREETING}");   // static works everywhere
}
```

> **When to use which?** Use `let` for local variables. Use `const` for values that never change and are known at compile time (like math constants, config limits). Use `static` only when you need a fixed memory address that lives for the entire program.

---

## 2. Data Types

Every value in Rust has a data type. Rust is a **statically typed** language — it must know the types of all variables at compile time. The compiler can usually infer the type based on the value and usage.

### 2.1 Scalar Types

A scalar type represents a **single value**: Integer, Float, Boolean, Character.

#### Integer

Integer sizes range from 1 byte (`i8`/`u8`) to 16 bytes (`i128`/`u128`).

- `i` = **signed** → can store **both positive and negative** numbers
- `u` = **unsigned** → can store **only positive** numbers (and zero)

| Length | Signed | Unsigned |
|---|---|---|
| 8-bit | `i8` | `u8` |
| 16-bit | `i16` | `u16` |
| 32-bit | `i32` | `u32` |
| 64-bit | `i64` | `u64` |
| 128-bit | `i128` | `u128` |
| Architecture-dependent | `isize` | `usize` |

- **Signed** range: −(2ⁿ⁻¹) to 2ⁿ⁻¹ − 1 (e.g., `i8` → −128 to 127)
- **Unsigned** range: 0 to 2ⁿ − 1 (e.g., `u8` → 0 to 255)
- `isize`/`usize` depend on architecture (32-bit or 64-bit)

**How it works in binary (sign bit):**

**`u8` (unsigned, 8 bits)** — all 8 bits store the value:

```
  5 in u8  → 00000101  (all 8 bits = value)
200 in u8  → 11001000  (all 8 bits = value)
Range: 0 to 255
```

**`i8` (signed, 8 bits)** — the **leftmost bit (8th bit) is the sign bit**, only 7 bits store the value:

```
  5 in i8  → 0|0000101
               ↑ sign bit = 0 means POSITIVE
               └ remaining 7 bits = 5

 -5 in i8  → 1|1111011  (two's complement)
               ↑ sign bit = 1 means NEGATIVE
Range: -128 to 127
```

> **Why -128 to 127?** With 7 bits for value: 2⁷ = 128. So negative side goes to −128, positive side goes to 127 (because 0 takes one spot).

**Same logic for `i32` / `u32` (32 bits):**

```
  5 in u32 → 00000000 00000000 00000000 00000101  (all 32 bits = value)
             Range: 0 to 4,294,967,295

  5 in i32 → 0|0000000 00000000 00000000 00000101
              ↑ sign bit = 0 (positive)
              └ remaining 31 bits = value

 -5 in i32 → 1|1111111 11111111 11111111 11111011  (two's complement)
              ↑ sign bit = 1 (negative)
              Range: -2,147,483,648 to 2,147,483,647
```

> **Summary:** Unsigned (`u`) uses ALL bits for value → bigger positive range. Signed (`i`) uses 1 bit for sign → can go negative but smaller positive range.

#### Floating-Point

Two types: `f32` (4 bytes) and `f64` (8 bytes). Default is `f64` because on modern CPUs it's roughly the same speed as `f32` but more precise.

```rust
fn main() {
    let x = 2.0;       // f64
    let y: f32 = 3.0;  // f32
}
```

#### Boolean

Two values: `true` and `false`. Booleans are **one byte** in size. Type is `bool`.

```rust
fn main() {
    let t = true;
    let f: bool = false; // with explicit type annotation
}
```

#### Character

`char` is **4 bytes** in size and represents a Unicode scalar value. Use **single quotes** (as opposed to string literals which use double quotes).

```rust
fn main() {
    let c = 'z';
    let z: char = 'ℤ';          // with explicit type annotation
    let heart_eyed_cat = '😻';
}
```

> `char` can represent ASCII, accented letters, CJK characters, emojis, and zero-width spaces. Unicode range: U+0000 to U+D7FF and U+E000 to U+10FFFF.

---

### 2.2 Compound Types

Compound types group **multiple values** into one type. Two **primitive** compound types: **Tuple** and **Array**.

> **What about `Vec`, `HashMap`, `String`?** These are **not** primitive compound types — they are **collections** from the standard library (`std`). They live on the **heap** and can **grow/shrink** at runtime. Tuples and arrays are fixed-size and live on the **stack**. Collections are covered later.

#### Tuple

- Groups values of **different types**
- **Fixed length** — cannot grow or shrink

```rust
fn main() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);
}
```

**Destructuring:**

```rust
fn main() {
    let tup = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of y is: {y}");
}
```

#### Array

- All elements must be the **same type**
- **Fixed length**

```rust
fn main() {
    let a = [1, 2, 3, 4, 5];
    let months = ["January", "February", "March", "April", "May", "June", "July",
                  "August", "September", "October", "November", "December"];
}
```

**Type annotation & repeat syntax:**

```rust
let a: [i32; 5] = [1, 2, 3, 4, 5];  // type: i32, length: 5
let a = [3; 5];                       // same as [3, 3, 3, 3, 3]
```

**Accessing elements:**

```rust
fn main() {
    let a = [1, 2, 3, 4, 5];
    let first = a[0];
    let second = a[1];
}
```

**Out-of-bounds access:** Rust checks array bounds at runtime. Accessing an invalid index causes a **panic** (runtime error).

```rust
use std::io;

fn main() {
    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");
}
```

> Entering index `10` with a 5-element array → `index out of bounds` panic.

---

## 3. Functions

- Declared with `fn` keyword
- Use **snake_case** for names
- Rust doesn't care **where** you define functions, as long as they're in scope

```rust
fn main() {
    println!("Hello, world!");
    another_function();
}

fn another_function() {
    println!("Another function.");
}
```

### Parameters

- You **must** declare the type of each parameter
- Multiple params separated by commas

```rust
fn another_function(x: i32) {
    println!("The value of x is: {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}
```

### Statements vs Expressions

- **Statements** — perform an action, **do not** return a value (e.g., `let y = 6;`)
- **Expressions** — evaluate to a value (e.g., `5 + 6`, function calls, block `{}`)

```rust
fn main() {
    let y = {
        let x = 3;
        x + 1   // no semicolon = expression, returns 4
    };
    println!("The value of y is: {y}"); // prints 4
}
```

> ⚠️ Adding a semicolon to an expression turns it into a statement (no return value).

### Return Values

- Declare return type with `->` after parentheses
- The **last expression** (without `;`) is the return value
- Can also use `return` keyword for early return

```rust
fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

fn main() {
    let x = five();       // x = 5
    let y = plus_one(5);  // y = 6
    println!("x = {x}, y = {y}");
}
```

> ⚠️ `x + 1;` (with semicolon) inside a function returning `i32` → **mismatched types** error. Remove the semicolon to fix it.

**Implicit vs Explicit return:**

```rust
// ✅ Implicit return — NO semicolon on last expression
fn add(a: i32, b: i32) -> i32 {
    a + b        // this is the return value (no semicolon!)
}

// ✅ Explicit return — USE semicolon because `return` is a statement
fn add_early(a: i32, b: i32) -> i32 {
    if a == 0 {
        return b;   // early return WITH semicolon
    }
    a + b           // implicit return, no semicolon
}

// ❌ WRONG — semicolon without return keyword
fn broken(a: i32, b: i32) -> i32 {
    a + b;       // semicolon turns it into a statement → returns () not i32
}
```

> **Rule:** Without `return` keyword → no semicolon (implicit return). With `return` keyword → use semicolon (it's a statement).

---

## 4. Comments

```rust
// Two slashes for single-line comments

// For multiple lines, use // on each line:
// So we're doing something complicated here, long enough that we need
// multiple lines of comments to do it! Whew! Hopefully, this comment will
// explain what's going on.
```

---

## 5. Control Flow

### `if` Expressions

The condition **must** be a `bool` — Rust will NOT auto-convert integers to booleans (unlike JavaScript/Ruby).

```rust
fn main() {
    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }
}
```

> ⚠️ `if number { ... }` won't work if `number` is an integer. Use `if number != 0 { ... }` instead.

### `else if` — Multiple Conditions

Rust executes **only the first true branch**, then skips the rest.

```rust
fn main() {
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");    // ← this prints
    } else if number % 2 == 0 {
        println!("number is divisible by 2");    // skipped even though true!
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
}
```

> 💡 Too many `else if`? Use `match` instead (covered later).

### `if` in a `let` Statement

`if` is an **expression**, so it can return a value. Both arms must return the **same type**.

```rust
fn main() {
    let condition = true;
    let number = if condition { 5 } else { 6 };  // number = 5
    println!("The value of number is: {number}");
}
```

```rust
// ❌ WRONG — mismatched types
let number = if condition { 5 } else { "six" };  // i32 vs &str → ERROR
```

---

## 6. Loops

Rust has **three** kinds of loops: `loop`, `while`, and `for`.

### `loop` — Infinite Loop

Runs forever until you use `break` to stop it.

```rust
fn main() {
    loop {
        println!("again!");  // prints forever until ctrl+C
    }
}
```

- `break` — exits the loop
- `continue` — skips remaining code in current iteration, goes to next

### Returning Values from `loop`

You can return a value from a loop using `break value;`

```rust
fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;  // returns 20
        }
    };

    println!("The result is {result}");  // prints 20
}
```

### Loop Labels

For **nested loops**, `break`/`continue` affect the **innermost** loop by default. Use labels (`'label_name`) to target an outer loop.

```rust
fn main() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;              // exits inner loop only
            }
            if count == 2 {
                break 'counting_up; // exits OUTER loop
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End count = {count}");  // prints 2
}
```

### `while` — Conditional Loop

Runs while the condition is `true`. Cleaner than `loop` + `if` + `break`.

```rust
fn main() {
    let mut number = 3;

    while number != 0 {
        println!("{number}!");
        number -= 1;
    }

    println!("LIFTOFF!!!");  // 3! 2! 1! LIFTOFF!!!
}
```

### `for` — Loop Over a Collection

**Safest and most commonly used** loop in Rust. No risk of index out-of-bounds.

```rust
// Looping through an array
fn main() {
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }
}
```

Compare with `while` version (error-prone — wrong index = panic!):

```rust
// ⚠️ while loop — you have to manage the index manually
let mut index = 0;
while index < 5 {
    println!("the value is: {}", a[index]);
    index += 1;
}
```

### `for` with Range

Use `(start..end)` for a range. Use `.rev()` to reverse.

```rust
fn main() {
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");  // 3! 2! 1! LIFTOFF!!!
}
```

> `1..4` generates 1, 2, 3 (end is **exclusive**). `1..=4` generates 1, 2, 3, 4 (end is **inclusive**).

## String Types: `&str` vs `String`

In Rust, understanding the difference between the two main string types, `&str` (string slice) and `String`, is very important.

### 1. `let s = "hello";` (String Slice - `&str`)

When you write this, `s` is of type `&str` (specifically `&'static str`).

- **Memory Location:** The text `"hello"` is hardcoded directly into the final executable binary file (in the read-only memory section). It will point to the binary, **not** the heap.
- **Immutability:** It is fundamentally immutable. You cannot change the characters inside it or grow its size.
- **Size:** Its size is known at compile time and is fixed.
- **Speed:** It is very fast and lightweight because it's just a reference (a pointer and a length) to data that already exists in the binary.

```rust
// Variable `s` points to the binary memory
// Stack representation:
// [ Pointer ] ----------> (Points to the binary memory)
// [ Length: 5 ]

fn main() {
    let mut s = "hello";
    println!("{}", s); // Prints: hello

    // This works! We are reassigning the variable 's' to point to a new fixed string.
    s = "world"; 
    println!("{}", s); // Prints: world
    
    // s.push_str("world"); // COMPILE ERROR! This will NOT work because the underlying data is immutable.
}
```

### 2. `let s = String::from("hello");` (Heap-Allocated String)

When you write this, `s` is of type `String`.

- **Memory Location:** The data for the string is allocated on the **heap** at runtime. The `String::from` function takes the hardcoded `"hello"` and copies it into a newly allocated chunk of heap memory.
- **Mutability:** Because it's on the heap, a `String` can be modified (if you declare it as `let mut s`). You can append characters to it, change it, or shrink it.
- **Size:** Its size can change dynamically at runtime. It has a capacity that can grow.
- **Ownership:** The variable `s` owns this heap memory. When `s` goes out of scope, Rust will automatically deallocate that heap memory (drop it) to prevent memory leaks.

```rust
// Variable `s` points to the heap
// Stack representation:
// [ Pointer ] ----------> (Points to the HEAP)
// [ Length: 5 ]
// [ Capacity: 5 ]

fn main() {
    // 1. Create a heap-allocated String, and make it mutable
    let mut s = String::from("hello"); 
    
    // 2. Now this works perfectly!
    s.push_str(" world"); 
    
    println!("{}", s); // Prints: hello world
}
```
##Macros : 2 types
1. declarative macros: 
      
        println!,panic,vec!

2. procedural macro:
   1. custom macro
   2. attirbute macro
   3. function macro


fn main(){
    let user1= String::from("Deepak");
    let user2 = user1
    print!("{},{}",user1,user2) // this will raise error because user 1 and user 2. it is printing user1 which values is assigned to user2. owenership rules. let user2=&user1. it works because references
}

fn main(){
    let user1= 2;
    let user2=user1; //it will create copy for integer will value because they fixed and their size will not changed at runtime. string as stored in heap and size may be changed. 
}

heap is slow.stack is fast. and copying data from heap makes slow. that is why rust doesn't not copy of heap. it will allows only for stack. 

##copy and clone traits
#[derive(Debug)]

struct User{
    is_male:bool,
    age:i32,
}

fn main(){
    let u1=user{
        is_male:true,
        age:23,
    };

    let u2=u1;
    print("{:?},{:?}",u1,u2)  //we can print like print("{}",u1.ismale) it works. but u1 is not works. we have implement impl Debug for User like this.. after only it works..otherwise not. so that is why we have called #[derive(Debug)].. 
}

problem : both is_male is boolean,and age is int. both will be stack. why it is not compiling. since not in heap. 

answer: how do the rust knows in struct, they is bool and int present. 
we have call #[derive(Copy,Clone)] here. by using Copy, it will know it has to copy the struct.. 

#[derive(Debug,Copy,Clone)]
struct User{
    is_male:bool
    age:u32
    name:String // so here Copy will not work becuase string is on heap. not in stack. so we need to remove copy from macros #[derive(Debug,Clone)]
}

clone is help for creating another copy. 
let u2=u1.clone; both will two variable. 

#[derive(Debug, Copy, Clone)]  // ← tell Rust: this struct is safe to copy
struct User {
    is_male: bool,  // stack ✅
    age: i32,       // stack ✅
}

fn main() {
    let u1 = User { is_male: true, age: 23 };
    let u2 = u1;    // ✅ COPIES (because we derived Copy)
    println!("{:?}, {:?}", u1, u2); // ✅ both valid
}

Why Copy fails with String
#[derive(Debug, Copy, Clone)]  // ❌ COMPILE ERROR
struct User {
    is_male: bool,
    age: u32,
    name: String,  // ← String is on HEAP, can't Copy!
}

Fix: remove Copy, keep Clone, and use .clone():
#[derive(Debug, Clone)]  // ✅ only Clone, no Copy
struct User {
    is_male: bool,
    age: u32,
    name: String,
}

fn main() {
    let u1 = User { is_male: true, age: 23, name: String::from("Deepak") };
    let u2 = u1.clone();  // ← note: clone() with parentheses, it's a method call
    println!("{:?}, {:?}", u1, u2); // ✅ both valid, two separate copies
}

 Rust can't just look inside a struct and figure out "oh all fields are stack types, let me copy." You have to explicitly tell Rust with #[derive(Copy, Clone)]. This is by design — it makes copies visible and intentional, not accidental.


