# Rust Syntax Cheat Sheet: Struct, Impl, Trait, Generic, Trait Bounds

---

## 1. STRUCT — holds data (fields)

```rust
// Syntax:
struct StructName {
    field1: Type,
    field2: Type,
}

// Example:
struct Rect {
    width: f64,
    height: f64,
}

// Usage:
let r = Rect { width: 10.0, height: 5.0 };
println!("{}", r.width);   // access field directly
```

---

## 2. IMPL (for struct) — adds methods to a struct

```rust
// Syntax:
impl StructName {
    fn method_name(&self) -> ReturnType {
        // can access self.field1, self.field2
    }
}

// Example:
impl Rect {
    fn area(&self) -> f64 {
        self.width * self.height
    }
}

// Usage:
let r = Rect { width: 10.0, height: 5.0 };
r.area();   // calls the method → 50.0
```

**This is what you already know from rectangle.rs. No trait needed.**

---

## 3. TRAIT — declares a behavior contract (just method signatures, NO fields)

```rust
// Syntax:
trait TraitName {
    fn method_name(&self) -> ReturnType;   // no body = required
    fn optional_method(&self) { }          // has body = default (optional)
}

// Example:
trait Shape {
    fn area(&self) -> f64;     // every struct implementing Shape MUST define this
}
```

**⚠️ Common confusion: traits do NOT hold fields like `width: f64`. Only methods.**

---

## 4. IMPL TRAIT FOR STRUCT — connects trait to struct

```rust
// Syntax:
impl TraitName for StructName {
    fn method_name(&self) -> ReturnType {
        // provide the actual logic here
    }
}

// Example:
impl Shape for Rect {
    fn area(&self) -> f64 {
        self.width * self.height    // Rect's version
    }
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        3.14 * self.radius * self.radius   // Circle's version
    }
}

// Usage:
let r = Rect { width: 10.0, height: 5.0 };
let c = Circle { radius: 7.0 };
r.area();   // calls Rect's area → 50.0
c.area();   // calls Circle's area → 153.86
```

---

## 5. TRAIT BOUND — "this function accepts any type that implements a trait"

This is where `impl Shape` in a function parameter comes from.

```rust
// ┌─────────────────────────────────────────────────────────────┐
// │  Normal function (you already know this):                   │
// │  fn get_area(s: f64) → s is a number                       │
// │                                                             │
// │  Trait bound function (new concept):                        │
// │  fn get_area(s: impl Shape) → s is ANY struct with area()  │
// └─────────────────────────────────────────────────────────────┘

// 3 ways to write the SAME thing:

// Style A: impl Trait (simplest)
fn get_area(s: impl Shape) -> f64 {
    s.area()
}

// Style B: Generic <T> with trait bound (most common)
fn get_area<T: Shape>(s: T) -> f64 {
    s.area()
}

// Style C: where clause (for complex cases)
fn get_area<T>(s: T) -> f64
where T: Shape
{
    s.area()
}

// All three: "s can be Rect, Circle, or ANY type with impl Shape"
```

### Comparison — normal param vs trait bound:

```rust
fn double(x: i32) -> i32 { x * 2 }
//            ^^^
//   "x must be i32, nothing else"

fn get_area(s: impl Shape) -> f64 { s.area() }
//                 ^^^^^^^^^
//   "s must be any type that has impl Shape for ..."
//   so Rect ✅, Circle ✅, i32 ❌ (i32 has no impl Shape)
```

---

## 6. GENERIC STRUCT — struct with flexible field types

```rust
// Syntax:
struct StructName<T> {
    field: T,
}

// Example:
struct Point<T> {
    x: T,
    y: T,
}

// Usage — compiler figures out T from the value:
let int_point = Point { x: 5, y: 10 };         // T = i32
let float_point = Point { x: 1.5, y: 3.7 };    // T = f64
```

---

## 7. GENERIC TRAIT — trait where return type is flexible

```rust
// Syntax:
trait TraitName<T> {
    fn method(&self) -> T;
}

// Example:
trait Measurable<T> {
    fn measure(&self) -> T;
}

// Can implement for different types:
impl Measurable<f64> for Rect {
    fn measure(&self) -> f64 { self.width * self.height }
}

impl Measurable<i32> for Rect {
    fn measure(&self) -> i32 { (self.width * self.height) as i32 }
}
```

---

## ALL SYNTAXES SIDE BY SIDE

```
┌──────────────────────────┬─────────────────────────────────────────┐
│ Concept                  │ Syntax                                  │
├──────────────────────────┼─────────────────────────────────────────┤
│ Struct                   │ struct Rect { width: f64 }              │
│                          │                                         │
│ Impl (methods on struct) │ impl Rect { fn area(&self) -> f64 }    │
│                          │                                         │
│ Trait (behavior contract)│ trait Shape { fn area(&self) -> f64; }  │
│                          │                                         │
│ Impl Trait for Struct    │ impl Shape for Rect { fn area()... }    │
│                          │                                         │
│ Trait Bound (param type) │ fn foo(s: impl Shape)                   │
│   same as                │ fn foo<T: Shape>(s: T)                  │
│   same as                │ fn foo<T>(s: T) where T: Shape          │
│                          │                                         │
│ Generic Struct           │ struct Point<T> { x: T, y: T }          │
│                          │                                         │
│ Generic Trait            │ trait Measurable<T> { fn m(&self) -> T } │
└──────────────────────────┴─────────────────────────────────────────┘
```

---

## THE BUILDING BLOCKS — How they connect

```
Step 1: struct Rect { width, height }         ← just data, no behavior

Step 2: impl Rect { fn area() }              ← now Rect has area()
        DONE! This works alone. (your rectangle.rs)

Step 3: trait Shape { fn area() }            ← a PROMISE: "I will have area()"
        impl Shape for Rect { fn area() }    ← Rect keeps the promise
        impl Shape for Circle { fn area() }  ← Circle also keeps the promise

Step 4: fn get_area(s: impl Shape)           ← accepts ANYONE who kept the promise
        get_area(rect)   ✅ Rect kept the promise
        get_area(circle) ✅ Circle kept the promise
        get_area(42)     ❌ i32 never promised
```

Source: [shapes.rs](file:///c:/Users/aigroup5/PycharmProjects/Rust/src/shapes.rs) | [rectangle.rs](file:///c:/Users/aigroup5/PycharmProjects/Rust/src/rectangle.rs)
