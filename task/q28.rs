// ============================================================================
// ## Q28 — Closures (Fn, FnMut, FnOnce)
//
// Rewrite Q19's retry so the operation is a parameter instead of hardcoded:
// ```rust
// fn retry<F, T, E>(attempts: u32, mut operation: F) -> Result<T, E>
// where
//     F: FnMut(u32) -> Result<T, E>,
// ```
//
// Call it three ways:
// 1. with a closure that fails twice then succeeds
// 2. with a closure that captures a counter from enclosing scope and mutates it
// 3. with a plain function passed by name (no closure at all)
//
// Questions answered in comments below:
// - Why is the bound FnMut and not Fn?
// - What breaks if you change it to Fn?
// - What breaks if you change it to FnOnce?
// ============================================================================

fn retry<F, T, E>(attempts: u32, mut operation: F) -> Result<T, E>
where
    F: FnMut(u32) -> Result<T, E>, //FnMut is the Trait."I will accept any type F, WHERE F implements the FnMut trait — meaning it is callable with a u32 argument and returns a Result<T, E>."
{
    for attempt in 1..=attempts {
        match operation(attempt) {
            Ok(value) => return Ok(value),
            Err(e) => {
                println!("Attempt {attempt}/{attempts} failed");
                if attempt == attempts {
                    return Err(e);
                }
            }
        }
    }
    unreachable!();
}

// Plain function for Case 3 (no closure at all)
fn ping_service(attempt: u32) -> Result<String, String> {
    if attempt < 2 {
        Err(String::from("service unavailable"))
    } else {
        Ok(String::from("200 OK from server"))
    }
}

// |attempt| is just the parameter list (like (attempt) in a normal function) closure( anonymous funtion) which is similar to Lamda function in python
//attempt| {
//  if attempt < 3 { ... }}

fn main() {
    println!("=== 1. Closure that fails twice, then succeeds ===");
    let result1 = retry(3, |attempt| {
        // |attempt| is just the parameter list (like (attempt) in a normal function)
        if attempt < 3 {
            Err(String::from("connection timeout"))
        } else {
            Ok(String::from("data fetched successfully"))
        }
    });
    match result1 {
        Ok(val) => println!("Success: {val}\n"),
        Err(err) => println!("Error: {err}\n"),
    }

    println!("=== 2. Closure that captures a counter and mutates it ===");
    let mut call_count = 0;
    let result2 = retry(3, |attempt| {
        call_count += 1; ////  Modifies an outside variable!
        if attempt < 2 {
            Err(String::from("rate limited"))
        } else {
            Ok(format!("recovered after {call_count} attempts"))
        }

        //If retry had used F: Fn(...), Rust would say:"Error: you passed a closure that modifies call_count, but retry only accepts read-only (Fn) closures!"
        //if retry had used FnOnce(...), Rust would say: "Error: retry is calling operation 3 times in a loop, but FnOnce can only be called once!"
    });
    match result2 {
        Ok(val) => println!("Success: {val}\n"),
        Err(err) => println!("Error: {err}\n"),
    }

    println!("=== 3. Plain function passed by name (no closure at all) ===");
    let result3 = retry(3, ping_service);
    match result3 {
        Ok(val) => println!("Success: {val}\n"),
        Err(err) => println!("Error: {err}\n"),
    }
}
// ============================================================================
// Q28 Questions Answered:
//
// 1. Why is the bound `FnMut` and not `Fn`?
//    `FnMut` allows the closure to be called repeatedly AND permits it to mutate
//    variables captured from its enclosing scope (like `call_count` in Case 2).
//    `Fn` only allows immutable borrows (`&self`), which forbids modifying captured state.
//
// 2. What breaks if you change it to `Fn`?
//    Case 2 breaks! Closures that mutate captured state implement `FnMut`, but NOT `Fn`.
//    If `retry` required `F: Fn(...)`, passing any mutating closure would fail to compile with:
//    `cannot borrow captured outer variable in an Fn closure as mutable`.
//
// 3. What breaks if you change it to `FnOnce`?
//    The `retry` function itself breaks! `FnOnce` takes ownership of the closure (`self`),
//    meaning it can be called at most once. Because `retry` calls `operation(attempt)`
//    inside a loop across multiple attempts, an `FnOnce` bound produces a compiler error:
//    `use of moved value: operation`.
//
// Trait Hierarchy:
//   Fn (immutable borrow) -> FnMut (mutable borrow) -> FnOnce (takes ownership)
// Every `Fn` implements `FnMut`, and plain functions (`fn`) implement `Fn`.
// Therefore, `FnMut` is the ideal bound: it accepts plain functions, read-only closures,
// and mutating closures alike!
// ============================================================================
