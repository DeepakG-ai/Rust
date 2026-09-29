# Rust Learning Notes

## Functions inside `impl` vs outside `impl`

### Inside `impl` — Associated Functions
```rust
impl Employee {
    fn new(name: &str, monthly_salary: f64, years: u32) -> Employee { ... }
    fn find_by_name(employees: &Vec<Employee>, name: &str) -> Option<&Employee> { ... }
    fn highest_paid(employees: &Vec<Employee>) -> Option<&Employee> { ... }
}
```
**Call with `Employee::`** prefix:
```rust
Employee::find_by_name(&employees, "Deepak")
Employee::highest_paid(&employees)
```

### Outside `impl` — Free/Standalone Functions
```rust
fn find_by_name(employees: &Vec<Employee>, name: &str) -> Option<&Employee> { ... }
fn highest_paid(employees: &Vec<Employee>) -> Option<&Employee> { ... }
```
**Call directly** — no prefix needed:
```rust
find_by_name(&employees, "Deepak")
highest_paid(&employees)
```

### Methods vs Associated Functions (inside `impl`)
- **Method** — takes `&self` → call on instance: `emp.summary()`
- **Associated function** — no `&self` → call on type: `Employee::new(...)`

---

## `Option<T>` — Rust's replacement for `null`/`None`

Rust has no `null`. Use `Option` to represent "maybe a value, maybe nothing":
```rust
Option<&Employee> = Some(&employee)   // found something
                  | None              // found nothing
```

### Handling with `match`:
```rust
match Employee::find_by_name(&employees, "Deepak") {
    Some(emp) => println!("Found: {} with salary {}", emp.name, emp.monthly_salary),
    None => println!("Not found"),
}

match highest_paid(&employees) {
    Some(i) => println!("Highest paid: {}", i.name),
    None => println!("No employees"),
}
```

---

## Enum Variants — Two Styles

### Struct-like (named fields):
```rust
Suspended { reason: String }
// In match → MUST use the field name
Status::Suspended { reason } => ...
```

### Tuple-like (unnamed, just type):
```rust
Closed(String)
// In match → you CHOOSE any variable name
Status::Closed(date) => ...    // "date" is your choice
Status::Closed(x) => ...       // "x" also works
```

---

