# Closures in Rust — `Fn`, `FnMut`, `FnOnce`

---

## What is a Closure?

A closure is a **function that can see and use variables from outside itself**. A normal function cannot do this.

```rust
fn main() {
    let name = String::from("Deepak");

    // ❌ Normal function CANNOT see `name`
    fn greet() {
        // println!("{name}"); // ERROR: `name` not found!
    }

    // ✅ Closure CAN see `name` — it "captures" it from the surrounding scope
    let greet = || {
        println!("{name}"); // Works! Closure captured `name`
    };

    greet();
}
```

**That is the entire difference.** A closure is just a function that can **capture** (grab) variables from outside its own body. The `|| { ... }` syntax is how Rust writes closures — like `lambda` in Python or `() =>` in JavaScript.

---

## The Three Ownership Rules → The Three Closure Traits

Closures can capture variables from their outer environment in the exact same 3 ways as Rust's ownership rules:

1. By immutable borrow: **`&T`** → **`Fn`**
2. By mutable borrow: **`&mut T`** → **`FnMut`**
3. By taking ownership: **`T`** → **`FnOnce`**

### The Three Traits Compared

| Trait | How it calls the closure | Can it be called more than once? | Can it mutate captured variables? |
| :--- | :--- | :--- | :--- |
| **`FnOnce`** | `call_once(self)` *(takes ownership of itself)* | **Only ONCE** | Yes (can consume/destroy variables) |
| **`FnMut`** | `call_mut(&mut self)` *(mutably borrows itself)* | **Multiple times** | **Yes** |
| **`Fn`** | `call(&self)` *(immutably borrows itself)* | **Multiple times** | **No** (read-only) |

---

## 1. `FnOnce` — Can only be called once

A closure becomes `FnOnce` if it **moves** (consumes) a captured variable. Once it runs, the captured variable is gone, so calling it a second time would cause a memory error.

```rust
let name = String::from("Deepak");

let consume_name = || {
    drop(name); // takes ownership of `name` and destroys it!
};

consume_name(); // ✅ Works
// consume_name(); // ❌ ERROR: cannot call again! `consume_name` is already consumed!
```

---

## 2. `FnMut` — Can be called multiple times and mutate state

A closure becomes `FnMut` if it **modifies** captured variables.

```rust
let mut count = 0;

let mut increment = || {
    count += 1; // mutates `count` from outside
    println!("count is {count}");
};

increment(); // count is 1
increment(); // count is 2
increment(); // count is 3
```

---

## 3. `Fn` — Can be called multiple times, read-only

A closure is `Fn` if it only **reads** captured variables (borrowing `&`), or captures nothing at all. Regular functions (e.g. `fn foo()`) also implement `Fn`.

```rust
let greeting = String::from("Hello");

let print_greeting = || {
    println!("{greeting}"); // only reads `&greeting`, doesn't mutate or drop it
};

print_greeting(); // ✅ Works
print_greeting(); // ✅ Works
```

---

## The Hierarchy (Supertraits)

```
Fn → FnMut → FnOnce
```

* Every closure that implements `Fn` also implements `FnMut` and `FnOnce`.
* Every closure that implements `FnMut` also implements `FnOnce`.

So:
* If a function expects **`FnOnce`**, you can pass **any** closure (`Fn`, `FnMut`, or `FnOnce`).
* If a function expects **`FnMut`**, you can pass `FnMut` or `Fn`.
* If a function expects **`Fn`**, you can **only** pass `Fn`.

---

## What Rust Does Under the Hood

`Fn`, `FnMut`, `FnOnce` **feel** like `&self`, `&mut self`, and `self`. That is because **they literally are the same concept**, just applied to closures instead of structs.

When you write a closure:

```rust
let mut count = 0;
let mut increment = || { count += 1; };
```

**Rust secretly converts it into a struct + method call:**

```rust
// What Rust ACTUALLY generates behind the scenes:
struct IncrementClosure<'a> {
    count: &'a mut i32,   // captured variable stored as a field
}

impl IncrementClosure<'_> {
    fn call(&mut self) {       // <-- this is FnMut because &mut self
        *self.count += 1;
    }
}
```

| Closure trait | Is equivalent to | On the hidden struct |
| :--- | :--- | :--- |
| `Fn` | `fn call(&self)` | Only reads captured variables |
| `FnMut` | `fn call(&mut self)` | Mutates captured variables |
| `FnOnce` | `fn call(self)` | Consumes (moves) captured variables |

**`Fn`, `FnMut`, `FnOnce` are not something you write inside closures.** They are **traits** that Rust **automatically assigns** to your closure based on what it does with captured variables. You never write `impl Fn` yourself.

---

## Why Do We Need These Traits At All?

Because when you write a function that **accepts** a closure as a parameter, you need to tell Rust: *"What kind of closure am I allowed to receive?"*

Every closure has a **unique anonymous type** that only the compiler knows. So you use the traits:

```rust
// You can't write this — closures don't have a nameable type:
fn retry(operation: ???) -> Result<String, String> { ... }

// Instead, use a trait bound:
// This says: "Give me anything callable that I can call multiple times
// and that is allowed to mutate its state"
fn retry<F>(mut operation: F) -> Result<String, String>
where
    F: FnMut() -> Result<String, String>,
{
    operation();  // call it
    operation();  // call it again — allowed because FnMut, not FnOnce
}
```

---

## Passing Functions and Closures as Parameters (Step-by-Step)

### The Core Secret: Notice there are NO parentheses `()`!

Look closely at this difference:
1. **`say_hello()`** (WITH parentheses) $\rightarrow$ **"Execute this function RIGHT NOW."**
2. **`say_hello`** (WITHOUT parentheses) $\rightarrow$ **"Don't run it. Just hand over the function itself like a value."**

Think of it like a recipe:
* **`say_hello()`** is **eating the cake**.
* **`say_hello`** is **handing the recipe book to a friend** so *they* can bake it later.

---

### Step 1: Python Example (Line by Line)

In Python, functions are first-class values that can be stored in variables and passed around:

```python
# Here is a normal function
def say_hello():
    print("Hello from Deepak!")

# Look: We assign the function to a variable WITHOUT ()
my_variable = say_hello

# Now `my_variable` IS the function!
my_variable()  # Prints: "Hello from Deepak!"
```

Since `say_hello` can be stored in a variable, it can also be passed into another function's parameter:

```python
# `action` is just a parameter name, like `x` or `y`.
# But instead of holding a number, it holds a function!
def run_twice(action):
    print("About to run the action...")
    action()  # Runs whatever function was passed in!
    action()  # Runs it again!

def say_hello():
    print("Hello!")

# We pass `say_hello` (NO parentheses!) into `run_twice`:
run_twice(say_hello)
```

**Output:**
```text
About to run the action...
Hello!
Hello!
```

---

### Step 2: The Exact Same Thing in Rust (With a Normal Function)

In Rust, you can do the exact same thing:

```rust
// A normal function
fn say_hello() {
    println!("Hello from Rust!");
}

// `action` is the parameter. Its type says: "any function that takes no args"
fn run_twice<F: Fn()>(action: F) {
    println!("Running 1st time:");
    action();

    println!("Running 2nd time:");
    action();
}

fn main() {
    // Pass `say_hello` (WITHOUT parentheses!) as the parameter:
    run_twice(say_hello);
}
```

**Output:**
```text
Running 1st time:
Hello from Rust!
Running 2nd time:
Hello from Rust!
```

---

### Step 3: Breaking Down `fn run_twice<F: Fn()>(action: F)` Piece by Piece

#### 1. What is `action: F`?
In Rust, every parameter is written as **`name: Type`**:

| Code | Parameter Name | Type | Meaning |
| :--- | :--- | :--- | :--- |
| `x: i32` | `x` | `i32` | `x` is an integer |
| `s: String` | `s` | `String` | `s` is a String |
| **`action: F`** | **`action`** | **`F`** | **`action` is a function/closure!** |

* **`action`** is the variable name used inside the function body.
* **`F`** is the type (by convention, `F` stands for *"Function"*).

#### 2. What does `<F: Fn()>` mean?
Just like `T` in `Vec<T>`, `F` is a generic type.

`<F: Fn()>` tells Rust:
> *"The type `F` can be anything, as long as it behaves like a function that takes no arguments `()`."*

*(Tip: Rust also lets you write `fn run_twice(action: impl Fn())` — which means the exact same thing without the `<F>` syntax!)*

#### 3. "Do I need to pass function names?" — You have 3 choices!

##### Choice 1: A function name
```rust
fn say_hello() {
    println!("hello!");
}

run_twice(say_hello); // Pass function name
```

##### Choice 2: A closure stored in a variable
```rust
let my_closure = || println!("hello from closure!");

run_twice(my_closure); // Pass closure variable
```

##### Choice 3: A closure written directly in the call (anonymous)
```rust
run_twice(|| println!("hello written directly!")); // Written inline
```

All 3 work because **all three implement `Fn()`**! Rust accepts any of them for `action: F`.

#### 4. Inside `run_twice`:
When you do:
```rust
fn run_twice<F: Fn()>(action: F) {
    action(); // <-- Calls whatever you passed in (Choice 1, 2, or 3)!
    action(); // <-- Calls it again!
}
```
Rust replaces `action()` with:
* `say_hello()` if you passed Choice 1
* `my_closure()` if you passed Choice 2
* the inline closure if you passed Choice 3

---

## How This Connects to Q28

In Q28, you are asked to make Q19's `retry` generic over an operation:

```rust
fn retry<F, T, E>(attempts: u32, mut operation: F) -> Result<T, E>
where
    F: FnMut(u32) -> Result<T, E>,
```

* **Why not `FnOnce`?**
  Because retry needs to call `operation` **multiple times** (attempt 1, attempt 2, attempt 3). `FnOnce` can only run once!
* **Why not `Fn`?**
  If you used `Fn`, the caller wouldn't be allowed to pass a closure that updates internal counters or changes its state between retries.
* **Why `FnMut`?**
  It is the perfect balance: it allows **multiple calls** *and* allows the closure to **modify captured variables** if needed. And because `Fn` automatically implements `FnMut`, read-only closures and plain functions work too!

---

## Concrete Example — All Three Side by Side

```rust
fn main() {
    let name = String::from("Deepak");
    let mut count = 0;

    // 1. This closure only READS `name` → Rust assigns: Fn
    let read_only = || {
        println!("Hello {name}");  // just reading
    };

    // 2. This closure MUTATES `count` → Rust assigns: FnMut
    let mut mutator = || {
        count += 1;                // modifying captured variable
        println!("count = {count}");
    };

    // 3. This closure MOVES `name` out → Rust assigns: FnOnce
    let consumer = || {
        let moved = name;          // takes ownership, `name` is gone after this
        println!("consumed: {moved}");
    };

    // You can call read_only and mutator multiple times:
    read_only();
    read_only();
    mutator();
    mutator();

    // But consumer can only be called ONCE:
    consumer();
    // consumer(); // ❌ ERROR: value used after move
}
```

---

## Summary

| Question | Answer |
| :--- | :--- |
| What is a closure? | A function that captures outside variables. Written with `\|\| { }` |
| Is `\|\|` a lambda? | Yes, exactly. Rust's version of lambda/arrow functions |
| Do I write `Fn`/`FnMut`/`FnOnce` inside closures? | **No.** Rust auto-detects which one your closure is |
| Where do I use `Fn`/`FnMut`/`FnOnce`? | Only when **accepting** a closure as a function parameter |
| How is `FnMut` different from `fn(&mut self)`? | **It's the same thing!** Rust converts closures into hidden structs with `&mut self` methods |
