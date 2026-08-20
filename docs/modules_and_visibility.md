# Rust Modules, Visibility & `pub struct` Guide

A complete, practical guide to Rust's module system, visibility rules, and struct encapsulation — written for developers coming from Python.

---

## Table of Contents
1. [The Core Mental Model: Rust vs Python](#1-the-core-mental-model-rust-vs-python)
2. [`mod` vs `use`: The #1 Confusion](#2-mod-vs-use-the-1-confusion)
3. [Visibility Levels (`pub`, private, `pub(crate)`, `pub(super)`)](#3-visibility-levels)
4. [`pub struct` Deep Dive: Why Structs and Fields are Different](#4-pub-struct-deep-dive)
5. [`pub enum` vs `pub struct`](#5-pub-enum-vs-pub-struct)
6. [Multi-File Project Structure (Modern Rust 2018/2021/2024)](#6-multi-file-project-structure)
7. [Step-by-Step Architecture Example (Q18 Style)](#7-step-by-step-architecture-example)
8. [Paths: `crate::`, `super::`, and `self::`](#8-paths-crate-super-and-self)
9. [Python vs Rust Comparison Cheat Sheet](#9-python-vs-rust-comparison-cheat-sheet)
10. [Top 5 Mistakes & Compiler Error Solutions](#10-top-5-mistakes--compiler-error-solutions)

---

## 1. The Core Mental Model: Rust vs Python

In **Python**:
- Any `.py` file on your disk is automatically a module.
- You write `import foo` anywhere, and Python finds `foo.py` and runs it.
- Privacy is just a naming convention (`_private_func()`), but anyone can still access it.

In **Rust**:
- Files on disk are **NOT** automatically part of your program.
- You must explicitly tell Rust: *"Hey compiler, include this file as part of my tree."*
- **Everything in Rust is PRIVATE by default.** If you want something visible outside its file/module, you **must** prefix it with `pub`.

```
                    ┌─────────────────────────┐
                    │    crate root (main.rs) │
                    └───────────┬─────────────┘
                                │
               ┌────────────────┴────────────────┐
               ▼                                 ▼
      mod employee; (employee.rs)       mod payroll; (payroll.rs)
      ├── pub struct Employee           └── pub fn total_annual()
      └── pub fn new()
```

---

## 2. `mod` vs `use`: The #1 Confusion

Beginners often mix up `mod` and `use`. They do two completely different jobs:

| Keyword | What it does | Python Analogue | How many times to write? |
|---|---|---|---|
| **`mod foo;`** | **LOADS & COMPILES** the file `foo.rs` into the program tree | (Creating/loading the module) | **Exactly ONCE** in `main.rs` (or `lib.rs`) |
| **`use foo::Bar;`** | Creates a **SHORTCUT** so you can type `Bar` instead of `foo::Bar` | `from foo import Bar` | As many times as you want in any file |

### ❌ The Common Mistake:
```rust
// In file: payroll.rs
mod employee; // ❌ ERROR! Do not load employee.rs twice!
```

### ✅ The Correct Way:
```rust
// In file: main.rs (the root loads both files ONCE)
mod employee; // Loads employee.rs
mod payroll;  // Loads payroll.rs

// In file: payroll.rs (use crate path shortcut)
use crate::employee::Employee; // ✅ Bring Employee into scope
```

---

## 3. Visibility Levels

Rust gives you fine-grained control over what is visible where:

| Visibility Syntax | Who can see and call it? | When to use? |
|---|---|---|
| *(no keyword)* | **Private:** Only code inside the **current module** and its child submodules. | Helper functions, internal data, secret state. |
| **`pub`** | **Public:** Anyone inside or outside the crate. | Public API, methods you want callers to use. |
| **`pub(crate)`** | **Internal to project:** Any file in your whole project, but hidden if published as a library. | Shared utilities between modules. |
| **`pub(super)`** | **Parent only:** Only the immediate parent module. | Nested submodules talking to their direct parent. |

### Visualizing Scopes:

```rust
mod outer {
    pub fn public_fn() {}       // Visible everywhere
    fn private_fn() {}          // Visible ONLY inside `outer`
    pub(crate) fn crate_fn() {} // Visible anywhere in this project

    mod inner {
        pub(super) fn for_parent_only() {
            // Can be called by `outer`, but not outside `outer`
        }
    }
}
```

---

## 4. `pub struct` Deep Dive: Why Structs and Fields are Different

This is where most developers get stuck. 

> ⚠️ **CRITICAL RULE:**
> Making a struct public (`pub struct`) does **NOT** make its fields public!
> Each field is private by default and needs its own `pub` if you want outside code to read/write it directly.

---

### Pattern A: Fully Public Struct (Data Transfer Object / Config)
Use this when a struct is just a transparent bag of data (like a 2D Point or Config).

```rust
// file: models.rs
pub struct Point {
    pub x: f64, // Public field
    pub y: f64, // Public field
}

// file: main.rs
mod models;
use models::Point;

fn main() {
    // ✅ Can instantiate directly with field names:
    let mut pt = Point { x: 10.0, y: 20.0 };
    
    // ✅ Can read and mutate fields directly:
    pt.x = 15.0;
    println!("x = {}", pt.x);
}
```

---

### Pattern B: Encapsulated Struct (Private Fields + Public Methods)
Use this when you have invariants (rules that must not be broken), like a Bank Account balance that cannot drop below zero.

```rust
// file: bank.rs
pub struct BankAccount {
    owner: String,  // 🔒 PRIVATE field
    balance: f64,   // 🔒 PRIVATE field
}

impl BankAccount {
    // 1. You MUST provide a `pub` constructor, because outside code 
    // cannot write `BankAccount { owner: ..., balance: ... }` directly!
    pub fn new(owner: &str) -> Self {
        Self {
            owner: owner.to_string(),
            balance: 0.0,
        }
    }

    // 2. Public method to mutate safely:
    pub fn deposit(&mut self, amount: f64) {
        if amount > 0.0 {
            self.balance += amount;
        }
    }

    // 3. Public getter method (read-only borrow):
    pub fn balance(&self) -> f64 {
        self.balance
    }
}
```

#### What happens if you try to access private fields from outside?
```rust
// file: main.rs
mod bank;
use bank::BankAccount;

fn main() {
    let mut acc = BankAccount::new("Alice"); // ✅ Works (pub constructor)
    acc.deposit(100.0);                      // ✅ Works (pub method)

    // ❌ COMPILER ERROR:
    // acc.balance = 5000.0;
    // ^^^^^^^^^^^ field `balance` of struct `BankAccount` is private!
    
    println!("Balance: {}", acc.balance());  // ✅ Works (pub getter)
}
```

---

## 5. `pub enum` vs `pub struct`

There is an important asymmetry between structs and enums in Rust:

| Type | When you mark `pub`... | Fields / Variants Visibility |
|---|---|---|
| **`pub struct`** | Struct name is public | Fields are **PRIVATE** by default (must add `pub` per field) |
| **`pub enum`** | Enum name is public | **ALL variants are automatically PUBLIC!** |

### Why are enum variants automatically public?
Because an enum represents a choice. If you couldn't see the variants, you could never `match` on it!

```rust
// file: status.rs
pub enum PaymentStatus {
    Pending,                      // Automatically public!
    Completed { tx_id: String },  // Automatically public!
    Failed(String),               // Automatically public!
}

// In main.rs:
use status::PaymentStatus;

fn handle(p: PaymentStatus) {
    match p {
        PaymentStatus::Pending => println!("Waiting..."),
        PaymentStatus::Completed { tx_id } => println!("Done: {tx_id}"),
        PaymentStatus::Failed(err) => println!("Error: {err}"),
    }
}
```

---

## 6. Multi-File Project Structure (Modern Rust 2018/2021/2024)

### Method 1: Flat Modules (Small to Medium Projects)
All module files sit directly next to `main.rs`:

```text
src/
├── main.rs          <-- Root: declares `mod employee; mod payroll;`
├── employee.rs      <-- Contains `Employee` struct
└── payroll.rs       <-- Contains `payroll` functions
```

### Method 2: Nested Submodules (Larger Projects)
When a module has its own submodules:

```text
src/
├── main.rs              <-- Root: declares `mod network;`
├── network.rs           <-- Parent module (declares `pub mod http; pub mod tcp;`)
└── network/             <-- Folder with same name as network.rs
    ├── http.rs          <-- Submodule: `network::http`
    └── tcp.rs           <-- Submodule: `network::tcp`
```

---

## 7. Step-by-Step Architecture Example (Q18 Style)

Here is a complete, working 3-file system demonstrating:
1. Public structs with private & public fields.
2. `pub fn new` constructor.
3. Cross-module imports using `crate::`.

### File 1: `task/q18/employee.rs`
```rust
pub struct Employee {
    pub name: String,         // Public: anyone can read/modify name
    monthly_salary: f64,      // Private: salary changes must go through methods
    years_of_service: u32,    // Private
}

impl Employee {
    // Constructor must be `pub`
    pub fn new(name: &str, monthly_salary: f64, years: u32) -> Self {
        Self {
            name: name.to_string(),
            monthly_salary,
            years_of_service: years,
        }
    }

    pub fn annual_salary(&self) -> f64 {
        self.monthly_salary * 12.0
    }

    pub fn is_senior(&self) -> bool {
        self.years_of_service >= 5
    }
}
```

---

### File 2: `task/q18/payroll.rs`
```rust
// Reach across modules starting from the root with `crate::`
use crate::employee::Employee;

pub fn total_annual_payout(staff: &[Employee]) -> f64 {
    let mut total = 0.0;
    for emp in staff {
        total += emp.annual_salary();
    }
    total
}

pub fn get_senior_staff<'a>(staff: &'a [Employee]) -> Vec<&'a Employee> {
    let mut seniors = Vec::new();
    for emp in staff {
        if emp.is_senior() {
            seniors.push(emp);
        }
    }
    seniors
}
```

---

### File 3: `task/q18/main.rs`
```rust
// 1. Declare and load the module files ONCE
mod employee;
mod payroll;

// 2. Bring items into scope with `use`
use employee::Employee;

fn main() {
    let staff = vec![
        Employee::new("Asha", 100_000.0, 6),
        Employee::new("Deepak", 75_000.0, 3),
    ];

    let total = payroll::total_annual_payout(&staff);
    println!("Total Annual Payout: ${:.2}", total);

    let seniors = payroll::get_senior_staff(&staff);
    println!("Seniors count: {}", seniors.len());
    for s in seniors {
        println!(" - Senior: {}", s.name);
    }
}
```

---

## 8. Paths: `crate::`, `super::`, and `self::`

When referencing types and functions from other modules, you use paths:

```
crate/
├── auth.rs           (crate::auth)
└── api/
    ├── v1.rs         (crate::api::v1)
    └── v2.rs         (crate::api::v2)
```

| Path Prefix | What it means | Example |
|---|---|---|
| **`crate::`** | Starts from the **project root** (`main.rs` or `lib.rs`). Absolute path. | `use crate::auth::User;` |
| **`super::`** | Goes up **one parent module** (like `..` in a filesystem). Relative path. | `use super::helpers;` |
| **`self::`** | Refers to the **current module** (like `.` in a filesystem). | `use self::types::*;` |

**Best Practice:** Prefer **`crate::`** for clarity. It never breaks when you move code within submodules!

---

## 9. Python vs Rust Comparison Cheat Sheet

| Concept | Python | Rust |
|---|---|---|
| **Load file into project** | Automatic (place `.py` in directory) | `mod file_name;` in root |
| **Import specific item** | `from math_utils import add` | `use crate::math_utils::add;` |
| **Import entire module** | `import math_utils` | `mod math_utils;` (or `use crate::math_utils;`) |
| **Rename on import** | `from foo import bar as my_bar` | `use foo::bar as my_bar;` |
| **Export/Re-export item** | `from .sub import Item` in `__init__.py` | `pub use sub::Item;` |
| **Privacy default** | Everything is public | Everything is private |
| **Enforce private state** | Prefix `_var` (honor system) | Compiler error (enforced at compile time) |
| **Package entry point** | `__main__.py` or root script | `src/main.rs` (or `src/lib.rs`) |

---

## 10. Top 5 Mistakes & Compiler Error Solutions

### 1. `error[E0603]: struct `Foo` is private`
- **Cause:** You wrote `struct Foo` without `pub`.
- **Fix:** Change to `pub struct Foo`.

### 2. `error[E0616]: field `bar` of struct `Foo` is private`
- **Cause:** You made the struct `pub struct Foo`, but the field inside is `bar: i32`.
- **Fix:** Either make the field `pub bar: i32`, or provide a `pub fn bar(&self) -> i32` getter.

### 3. `error[E0432]: unresolved import `employee``
- **Cause:** You tried to write `use employee::Employee;` in a submodule without `crate::`.
- **Fix:** Write `use crate::employee::Employee;`.

### 4. `error[E0583]: file not found for module `xyz``
- **Cause:** You wrote `mod xyz;` but neither `xyz.rs` nor `xyz/mod.rs` exists in that directory.
- **Fix:** Create `xyz.rs` in the directory relative to the file declaring `mod xyz;`.

### 5. `warning: field is never read` / `dead_code`
- **Cause:** You declared private fields in a struct in another file that are never accessed within that module.
- **Fix:** If the field is meant for outside callers, mark it `pub` or read it inside methods.
