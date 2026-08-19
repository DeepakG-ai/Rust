### Return in rust

```rust 
fn some(text:&str)->String{
    return text;
}

fn some(text:&str)->String{
    text // it also returns same text as above. no semicolon
}
``` 


### Borrowing and accessing rules

Stack types (i32, f64, char, bool) → auto-copied, no & needed Heap types (String, Vec, HashMap) → must borrow or 

In Rust:

Primitive types (i32, f64, char, bool) and fixed-size arrays/tuples of them → automatically copied. No need to worry about borrowing.
Heap-allocated types (String, Vec, HashMap, custom structs) → must choose: either borrow (&) to let someone look at it, or move to give it away permanently.
Why? Copying a number is instant (few bytes on the stack). Copying a Vec of 1 million items would be expensive and unexpected, so Rust forces you to be explicit.


```rust

fn main (){
    let prices = vec![10.5,20.0, 3.25];
    let ans1 = total_borrowed(&prices);
    println!("borrowed:{}",ans1);
    println!("borrowed prices:{:?}",prices);

    let ans = total_owned(prices);
    println!("owned:{}",ans);
    //println!("owned prices:{:?}",prices);
// ERROR: borrow of moved value: `prices`
// value moved into total_owned(), can't use it after
}

// BORROW: "hey function, LOOK at my prices, then give back"
fn total_borrowed(prices:& Vec<f64>)->f64{
    let mut total:f64 = 0.0;
    for i in prices{
        total = total +i
    }
    return total;
}
// MOVE: "hey function, TAKE my prices, they're yours now" 
fn total_owned(prices:Vec<f64>)->f64{
    let mut total=0.0;
    for i in prices{
        total = total+i
    }
    return total;
}

//borrowed values will come back to variable again. but moving values to function will not come back. it belongs to function.. not variables..
```

### Box

Box<T> = "I want to force this value onto the heap. Keep only a pointer on the stack." 

```rust
// Stack: value lives directly in the variable
let x: i32 = 42;

// Heap: value is stored on the heap, x just holds a pointer to it
let x: Box<i32> = Box::new(42);
```
1. When the size is unknown at compile time (recursive types):
```rust
// ❌ ERROR: Rust can't know how big this is (infinite nesting)
enum List {
    Item(i32, List),  // List contains a List contains a List...
    End,
}

// ✅ Box has a fixed size (it's just a pointer)
enum List {
    Item(i32, Box<List>),  // now Rust knows: i32 + pointer size
    End,
}
```

2. When data is large and you don't want to copy it on the stack:
// This puts a huge array on the heap instead of blowing up the stack
let big_data = Box::new([0u8; 1_000_000]);

### Calling the Struct 

It is similar to python calling class and object

```rust
struct Employee{ //class Employee

}

struct Product{ //class Product

}
fn main (){
    let e = Employee{    //e = Employee, then e.shortlabel
        name : String::from("Deepak"),
        monthly_salary:50000.0,

    };  //e.name =Deepak, p.name = Laptop. 

    let p = Product{
        name:String::from("Laptop"),
        price:52400.0,
    };

    println!("{}", e.short_label()); 
    println!("{}", p.short_label());

}


```

### Generics, Traits, Trait Bounds, and Lifetimes

These are 4 separate tools. They don't always need each other.

**Generics** = "write one function for any type T, instead of writing same function twice"

```rust
// ❌ Without generics: writing same thing twice
fn area_i32(w: i32, h: i32) -> i32 { w * h }
fn area_f64(w: f64, h: f64) -> f64 { w * h }

// ✅ With generics: write it ONCE
fn area<T>(w: T, h: T) -> T { w * h }
// but wait... not every type can multiply. string * f64 won't work. bool * bool won't work.
// so we need trait bounds ↓
```

**Traits** = "what ability/character does a type have?"

Think of it like: Rust has built-in traits (under the hood library) that check if a type has certain characters.
`std::ops::Mul` checks "can this type multiply?" — i32 yes, f64 yes, String no, bool no.
`PartialOrd` checks "can this type compare with > < ?" — i32 yes, f64 yes.

We can also create our OWN custom traits. Like `Describe` — "can this type describe itself?"
Built-in or custom — same mechanism, same thing.

```rust
// Rust defined this inside std:
trait Mul { fn mul(self, other: Self) -> Self; }
// implemented for i32, f64 etc. NOT for String, bool

// WE defined this:
trait Describe { fn describe(&self) -> String; }
// implemented for Employee, Product. NOT for i32, f64
```

**Trait Bounds** = "Generic + Trait together. T can be any type, BUT only if it has this ability"

```rust
fn area<T: std::ops::Mul<Output = T>>(w: T, h: T) -> T {
    w * h  // T must have the character of multiplying
}

fn print_description<T: Describe>(item: &T) {
    println!("{}", item.describe());  // T must have the character of describing
}
```

**Lifetimes** = SEPARATE concept. Nothing to do with generics or traits.
"If I return a reference (&), how long should it live?"

```rust
// only needed when returning references, not owned types
fn longest<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() > b.len() { a } else { b }
}
// 'a means: the returned reference lives as long as both inputs
```

**Summary — they don't always need each other:**
- Trait without generics → `impl Describe for Employee` (Q10)
- Generics without traits → `struct Pair<T> { first: T, second: T }`
- Generics WITH trait bounds → `fn largest<T: PartialOrd>(list: &[T])` (Q13)
- Lifetimes without any of the above → `fn longest<'a>(a: &'a str, b: &'a str)` (Q15)
- All together → `fn foo<'a, T: Describe>(item: &'a T) -> &'a str`

### T means ANY type — including structs!

I thought T means only data types like int, float, string, vec, tuple.
WRONG. T also means struct, enum — everything is a type in Rust.

```rust
i32         → a type
f64         → a type
String      → a type (it's actually a struct too!)
Vec<f64>    → a type
Employee    → a type
Product     → a type
```

Even `String` is a struct inside Rust's std library. So `Vec<Employee>` works same as `Vec<f64>`.

In Python we already do this without thinking:
```python
def print_info(item):     # item can be int, str, Employee, anything
    print(item)
```
Python just doesn't make you declare the type. Rust does — `T` is Rust's way of saying "anything".

### How trait bounds actually work — step by step trace

```rust
fn print_description<T: Describe>(item: &T) {
    println!("{}", item.describe());
}
```

T is NOT String or f64. T is the whole struct we pass in.

```rust
// When we call:
print_description(&e);  // T becomes Employee (the struct)
print_description(&p);  // T becomes Product  (the struct)

// Rust replaces T with Employee:
fn print_description(item: &Employee) {
    println!("{}", item.describe());
    //             calls Employee's describe()
    //             returns: "Deepak earns per month 50000"
}

// Rust replaces T with Product:
fn print_description(item: &Product) {
    println!("{}", item.describe());
    //             calls Product's describe()
    //             returns: "Laptop price is 52400"
}
```

The flow:
```
print_description(&e)
    → T = Employee (the struct, not String, not f64)
    → Does Employee implement Describe? YES ✅
    → So call e.describe() → returns String → print it

print_description(&p)
    → T = Product
    → Does Product implement Describe? YES ✅
    → So call p.describe() → returns String → print it
```

T = the type you pass in (Employee, Product). The String return is just what describe() gives back — it's not T.

### Python vs Rust — the trade-off

Python = automatically handles types, memory, everything. Easy to write, bugs at runtime.
Rust = you implement everything explicitly. Harder to write, but bugs caught at compile time.

| Python | Rust |
|---|---|
| Types → automatic | Types → you declare them |
| Memory → garbage collector | Memory → ownership/borrowing |
| "Any type" → just works | "Any type" → you write `<T>` |
| "Can it multiply?" → crashes at runtime | "Can it multiply?" → compiler checks with trait bounds |
| "How long does it live?" → garbage collector | "How long does it live?" → you specify with lifetimes |

If it compiles in Rust, it works. No surprises at runtime.

### Box<dyn Trait> — putting different types in same Vec (Q12)

**The Problem:** A Vec needs all elements to be the same size in memory.

```rust
Vec<i32>       // ✅ every element is 4 bytes
Vec<f64>       // ✅ every element is 8 bytes
Vec<Employee>  // ✅ every Employee is same size
```

But Employee and Product are different structs, different sizes. Can't put both in same Vec:
```rust
// ❌ What type goes in the < >? Employee? Product? Can't be both!
let items: Vec<???> = vec![employee, product];
```

**The Solution:** `Box<dyn Describe>`

```rust
let items: Vec<Box<dyn Describe>> = vec![
    Box::new(employee),  // put Employee on heap, keep pointer (8 bytes)
    Box::new(product),   // put Product on heap, keep pointer (8 bytes)
];
```

`Box` = store the value on heap, keep a pointer on stack (already know this)
`dyn` = "dynamic" — "I don't know which type this is at compile time, but I know it implements Describe"

So `Box<dyn Describe>` means:
"A pointer to something on the heap that has a describe() method. I don't care if it's Employee or Product."

**Why it works — all pointers are same size:**
```
Vec<Box<dyn Describe>>

   Stack (Vec)              Heap
   ┌──────────┐
   │ pointer ──────→  Employee { name: "Deepak", salary: 50000 }
   │ pointer ──────→  Product  { name: "Laptop", price: 52400 }
   └──────────┘
   
   Every element in Vec = pointer = same size (8 bytes) ✅
```

**dyn vs <T> — when to use which:**

| `<T: Describe>` (Q10) | `dyn Describe` (Q12) |
|---|---|
| Compiler knows the exact type | Compiler doesn't know the type |
| One type at a time | Mix different types together |
| Faster (no pointer lookup) | Slightly slower (pointer lookup) |

`<T>` = "I'll tell you the type" → compiler generates separate code for each type
`dyn` = "figure it out at runtime" → compiler just follows the pointer

Use `Box<dyn Trait>` when you want different types in one Vec. Box makes them all same size (a pointer).


&[T] is called a slice (borrowed view of a list/array of items).

Here is the difference:

Syntax	What it means	Example
&T	Borrow ONE single item	&5, &employee, &"apple"
&[T]	Borrow a LIST / SEQUENCE of items of type T	&[10, 20, 30], &["a", "b"]

### `impl` Blocks, Struct Names, and the `Self` Keyword

Why do we see the struct name multiple times in an `impl` block?

```rust
impl Employee {                                      // 1. WHICH struct we are adding methods to
    fn new(name: &str, ...) -> Employee {           // 2. RETURN TYPE: this function returns an Employee
        Employee {                                   // 3. CONSTRUCTOR: creating the struct instance
            name: name.to_string(),
            monthly_salary,
            years,
        }
    }
}
```

#### Shortcut with `Self`:
`Self` (capital **S**) is shorthand for *"whatever struct this impl block belongs to"*.

```rust
impl Employee {
    fn new(name: &str, monthly_salary: f64, years: u32) -> Self {
        Self {
            name: name.to_string(),
            monthly_salary,
            years,
        }
    }
}
```

- `-> Self` means `-> Employee`
- `Self { ... }` builds and returns the new instance!

#### In Generic Structs (`Pair<T>` in Q14):
Instead of repeating `Pair<T>` everywhere:
```rust
struct Pair<T> {
    first: T,
    second: T,
}

impl<T> Pair<T> {
    fn new(first: T, second: T) -> Self {
        Self { first, second }
    }
}
```

### Lifetimes & Scopes: Can a variable survive outside its block `{}`?

```rust
{
    let player2 = String::from("Bartholomew");
    winner = pick_winner(&player1, &player2);
} // 💥 player2 is destroyed here!

println!("{}", winner); // ❌ FAILS! player2 is dead, so winner is unsafe to read!
```

**Direct Answer:** In the code above, **No, you can NEVER access `player2` after `}`.**
The moment execution hits the closing curly brace `}`, Rust drops and frees `player2`'s heap memory.

---

#### 3 Ways to make the data survive outside the block:

**Option 1: Declare `player2` in the outer scope (Most common)**
```rust
fn main() {
    let player1 = String::from("Alice");
    let player2 = String::from("Bartholomew"); // ✅ Lives in outer scope

    let winner = pick_winner(&player1, &player2);
    println!("Winner is: {}", winner); // ✅ Works! Both are alive!
}
```

**Option 2: Return ownership out of the inner block**
```rust
fn main() {
    let player1 = String::from("Alice");

    let player2 = {
        let name = String::from("Bartholomew");
        name // 👈 Returns owned String out of the block
    };

    let winner = pick_winner(&player1, &player2);
    println!("Winner is: {}", winner); // ✅ Works!
}
```

**Option 3: Convert the borrowed reference to an OWNED String before the block ends**
```rust
fn main() {
    let player1 = String::from("Alice");
    let winner: String; // Owned String

    {
        let player2 = String::from("Bartholomew");
        let result_ref = pick_winner(&player1, &player2);
        winner = result_ref.to_string(); // ✅ Makes a real owned copy
    } // player2 dies, but winner owns its own heap data!

    println!("Winner is: {}", winner); // ✅ Works!
}
```

---

#### Python vs Rust Scope Boundary:
- **Python:** Variables created inside `if` or `for` blocks leak and stay alive throughout the whole function.
- **Rust:** Every `{ }` is a strict memory boundary. When the block ends `}`, everything declared inside is automatically freed unless moved out.

---

### Dereference (`*`) and Destructure (`&` in patterns)

#### What you already know
```rust
let n = 10;        // owned value (i32)
let r = &n;        // reference to n (&i32)
```

Most of the time, Rust **automatically follows references** for you — printing, method calls, comparisons all just work:
```rust
println!("{}", r);       // ✅ Rust auto-handles it
println!("{}", n);       // ✅ same output

if r > &5 { }           // ✅ Rust auto-handles comparison
r.is_positive();        // ✅ Rust auto-handles method calls
```

#### When Rust can't auto-handle it — you need dereference (`*`)

**Dereference = "follow the reference, give me the real value"**

```rust
let n = 10;
let r = &n;         // r is &i32, not i32

let result = r + 5;   // ❌ ERROR: can't add &i32 + i32
let result = *r + 5;  // ✅ *r gives you the i32 value → 10 + 5 = 15
```

Each `*` peels off one `&`:
```
&i32   →  *r   →  i32       (one reference, one *)
&&i32  →  **r  →  i32       (two references, two *)
```

#### Why `&&` (reference to a reference) happens inside `.filter()`

```rust
let numbers = vec![10, 20, 30];

// Step 1: .iter() already gives &i32 (borrows each element)
// Step 2: .filter() adds ANOTHER & (it borrows the iterator item to test it)
// Result: closure receives &&i32

numbers.iter().filter(|n| {
    // n is &&i32 here (two layers of reference)
    **n > 5    // peel both layers to get the plain i32
});
```

Think of it as following two arrows:
```
n  →  points to  →  &i32  →  points to  →  i32 (the actual 10)
       first *                  second *
```

#### Destructuring — same thing, different place

Instead of using `*` inside the body, you can unwrap references **right in the parameter**:

```rust
let r = &10;

// Way 1: receive the reference, use * inside
let val = *r;       // val is i32 = 10

// Way 2: unwrap it when receiving (destructure)
let &val = r;       // val is i32 = 10 (the & in the pattern removes one layer)
```

**Destructuring is just dereferencing done at the parameter.** Same result.

#### Three ways to write the same `.filter()` — all identical

```rust
// The closure receives &&i32. All three approaches produce the same output:

// Approach 1: peel nothing at parameter, peel both in body
.filter(|n| **n > 10)
//       n = &&i32
//       *n = &i32    (peeled one)
//       **n = i32    (peeled both) → compare with 10 ✅

// Approach 2: peel one at parameter, peel one in body
.filter(|&n| *n > 10)
//       n = &i32     (peeled one at the door)
//       *n = i32     (peeled the second inside) ✅

// Approach 3: peel both at parameter, nothing in body
.filter(|&&n| n > 10)
//       n = i32      (peeled both at the door)
//       just use n directly ✅
```

#### Quick reference: when do you need `*`?

| Situation | Need `*`? |
|---|---|
| `println!("{}", r)` | No, Rust handles it |
| `r.len()`, `r.is_empty()` | No, Rust handles it |
| `if r > &5` | No, Rust handles it |
| `r + 5` (math operations) | **Yes** → `*r + 5` |
| `&&i32` inside `.filter()` | **Yes** → `**n > 5` or `\|&&n\| n > 5` |

**Simple rule:** if the compiler says "expected `i32`, found `&i32`" — add `*`. If it says `&&i32` — add `**`.

#### Recommendation for beginners

Just use `**` inside the body. No fancy patterns needed:
```rust
.filter(|n| **n > 10)      // simple and clear
.filter(|n| **n % 2 == 0)  // same pattern
```

---

### HashMap (Rust vs Python `dict`)

A `HashMap` in Rust is a key-value store, exactly like a dictionary (`dict`) in Python.

#### 1. Python vs Rust Method Cheat Sheet

| Operation | Python (`dict`) | Rust (`HashMap`) |
|---|---|---|
| **Import** | Built-in | `use std::collections::HashMap;` |
| **Create** | `d = {}` | `let mut map = HashMap::new();` |
| **Insert / Update** | `d["apple"] = 3` | `map.insert("apple".to_string(), 3);` |
| **Safe Get** | `d.get("apple")` | `map.get("apple")` → returns `Option<&V>` (`Some(&3)` or `None`) |
| **Check key** | `"apple" in d` | `map.contains_key("apple")` → returns `bool` |
| **Iterate keys** | `d.keys()` | `map.keys()` |
| **Iterate values** | `d.values()` | `map.values()` |
| **Iterate pairs** | `for k, v in d.items():` | `for (k, v) in &map { ... }` |
| **Remove** | `d.pop("apple")` | `map.remove("apple")` |

#### 2. The Big Difference: `entry()` API (The Word Count Superpower)

In Python, to count occurrences you usually write:
```python
counts[word] = counts.get(word, 0) + 1
```

In Rust, you use the **`.entry()`** API:
```rust
*map.entry(word).or_insert(0) += 1;
```

**How it works step-by-step:**
1. `map.entry(word)` — checks if `word` exists in the map.
2. `.or_insert(0)` — if the key doesn't exist, it inserts `0`. It returns a **mutable reference `&mut usize`** to the value in the map.
3. `*` — **dereferences** that reference so you can directly modify the number: `+= 1`.

#### 3. Key Details to Watch in Rust:
1. **Types must be uniform:** All keys must be one type (`K`), and all values must be one type (`V`) $\rightarrow$ e.g., `HashMap<String, usize>`.
2. **Order is random:** Keys are not in sorted order when iterating. If you need them sorted (like in Q17), collect into a `Vec` and call `.sort()`.
