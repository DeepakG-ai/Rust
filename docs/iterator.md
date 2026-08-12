# Iterators in Rust

## What is an Iterator?

An iterator is simply a thing that gives you **one item at a time** from a collection.

---

## First Principles: Before Iterators

Without iterators, you loop with an index:

```rust
let names = vec!["Deepak", "Ravi", "Gowda"];

// Manual way — YOU manage the index
let mut i = 0;
while i < names.len() {
    println!("{}", names[i]);
    i += 1;
}
```

**Problems with this:**
- You manage `i` yourself → easy to make off-by-one bugs (`i <= names.len()` 💀)
- You directly access memory by index → risk of going out of bounds
- Verbose and repetitive

---

## With Iterators

```rust
let names = vec!["Deepak", "Ravi", "Gowda"];

// Iterator way — IT manages the position
for name in names.iter() {
    println!("{}", name);
}
```

**You just say "give me the next item" and it handles everything.** No index, no bounds checking, no bugs.

---

## How it works internally

Every iterator in Rust implements one method: **`next()`**

```rust
let names = vec!["Deepak", "Ravi", "Gowda"];
let mut iter = names.iter();

iter.next()  // → Some("Deepak")   ← first item
iter.next()  // → Some("Ravi")     ← second item
iter.next()  // → Some("Gowda")    ← third item
iter.next()  // → None             ← done, no more items
```

That's it. An iterator is just something with a `next()` method that returns `Some(item)` or `None`.

---

## The Real Power: Chaining

This is where iterators become amazing:

```rust
let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

// Get sum of squares of even numbers
let result: i32 = numbers.iter()
    .filter(|n| *n % 2 == 0)     // keep only even: [2, 4, 6, 8, 10]
    .map(|n| n * n)               // square them: [4, 16, 36, 64, 100]
    .sum();                        // add them: 220
```

Without iterators, this would be:
```rust
let mut result = 0;
let mut i = 0;
while i < numbers.len() {
    if numbers[i] % 2 == 0 {
        result += numbers[i] * numbers[i];
    }
    i += 1;
}
```

---

## Three Types of Iteration

| Method | What you get | Original collection |
|:---|:---|:---|
| `.iter()` | `&item` (reference) | ✅ Kept (borrowed) |
| `.iter_mut()` | `&mut item` (mutable ref) | ✅ Kept (can modify items) |
| `.into_iter()` | `item` (owned) | ❌ Consumed (moved) |

```rust
let names = vec!["Deepak", "Ravi"];

for n in names.iter()      { }  // borrows — names still usable after
for n in names.into_iter() { }  // moves — names is GONE after this
```

---

## `for` Loop IS an Iterator in Disguise

```rust
let names = vec!["Deepak", "Ravi"];

// These two are EXACTLY the same:
for name in names { }              // sugar for...
for name in names.into_iter() { }  // this (consumes the vec)

// If you want to borrow:
for name in &names { }             // sugar for names.iter()
for name in &mut names { }        // sugar for names.iter_mut()
```

---

## Key Iterator Methods

### Transform & Filter

| Method | What it does |
|:---|:---|
| `.map(fn)` | Transform each item |
| `.filter(fn)` | Keep items that match a condition |
| `.flat_map(fn)` | Map + flatten nested results |
| `.enumerate()` | Get `(index, item)` pairs |
| `.zip(other)` | Pair items from two iterators |
| `.chain(other)` | Join two iterators together |
| `.take(n)` | Take only first n items |
| `.skip(n)` | Skip first n items |
| `.rev()` | Reverse the iterator |

### Consume & Collect

| Method | What it does |
|:---|:---|
| `.collect()` | Gather results into a Vec, String, etc. |
| `.sum()` | Add all items together |
| `.count()` | How many items |
| `.fold(init, fn)` | Reduce all items to one value (custom accumulator) |

### Search & Check

| Method | What it does | Example |
|:---|:---|:---|
| `.find(fn)` | First matching item | `v.iter().find(|x| **x > 3)` → `Some(&4)` |
| `.any(fn)` | Is ANY item matching? | `v.iter().any(|x| *x == 5)` → `true` |
| `.all(fn)` | Do ALL items match? | `v.iter().all(|x| *x > 0)` → `true` |
| `.position(fn)` | Index of first match | `v.iter().position(|x| *x == 3)` → `Some(2)` |

---

## Examples of Less Common Methods

```rust
// fold — like sum() but custom (manual accumulator)
let sum = vec![1, 2, 3].iter().fold(0, |acc, x| acc + x);  // 6

// flat_map — flatten nested structures
let words = vec!["hello world", "foo bar"];
let split: Vec<&str> = words.iter()
    .flat_map(|s| s.split(' '))
    .collect();  // ["hello", "world", "foo", "bar"]

// chain — combine two iterators
let a = vec![1, 2];
let b = vec![3, 4];
let all: Vec<_> = a.iter().chain(b.iter()).collect();  // [1, 2, 3, 4]

// enumerate — get index + item
for (i, name) in vec!["Deepak", "Ravi"].iter().enumerate() {
    println!("{}: {}", i, name);  // 0: Deepak, 1: Ravi
}

// zip — pair two iterators
let names = vec!["Deepak", "Ravi"];
let ages = vec![25, 30];
let pairs: Vec<_> = names.iter().zip(ages.iter()).collect();
// [("Deepak", 25), ("Ravi", 30)]
```

---

## `collect()` Turbofish Syntax

`collect()` can build different types — you tell it WHAT to build:

```rust
let v: Vec<i32> = (1..5).collect();          // [1, 2, 3, 4]
let v = (1..5).collect::<Vec<i32>>();        // same thing, turbofish syntax ::<>
let s: String = vec!['h','i'].into_iter().collect();  // "hi"
```

---

## Ranges are Iterators

```rust
for i in 0..5 { }         // 0, 1, 2, 3, 4
for i in 0..=5 { }        // 0, 1, 2, 3, 4, 5  (inclusive)
for i in (0..5).rev() { }  // 4, 3, 2, 1, 0  (reversed)
```

---

## Iterators are LAZY

Nothing happens until you **consume** the iterator:

```rust
let v = vec![1, 2, 3];

v.iter().map(|x| x * 2);                    // ⚠️ does NOTHING! Just creates an iterator
v.iter().map(|x| x * 2).collect::<Vec<_>>(); // ✅ now it actually runs
```

Consumers that trigger execution: `.collect()`, `.sum()`, `.for_each()`, `.count()`, `for` loop

---

## Zero-Cost Abstraction

**Iterators are NOT slower than manual loops.** Rust compiles iterator chains into the same machine code as a hand-written `while` loop. No overhead.

```rust
// These compile to the SAME assembly:
numbers.iter().filter(|n| *n % 2 == 0).map(|n| n * n).sum()
// vs
let mut sum = 0;
for n in numbers { if n % 2 == 0 { sum += n * n; } }
```

---

## Creating Your Own Iterator

You can make any struct iterable by implementing the `Iterator` trait:

```rust
struct Counter {
    count: u32,
}

impl Iterator for Counter {
    type Item = u32;         // what type does next() return?

    fn next(&mut self) -> Option<u32> {
        self.count += 1;
        if self.count <= 5 {
            Some(self.count)
        } else {
            None             // stop iterating
        }
    }
}

// Now you can use it like any iterator:
let c = Counter { count: 0 };
for num in c {
    println!("{}", num);  // prints 1, 2, 3, 4, 5
}

// And chain methods on it:
let sum: u32 = Counter { count: 0 }
    .filter(|n| n % 2 == 0)
    .sum();  // 2 + 4 = 6
```
