// ============================================================================
// ## Q9 — Result and ?
//
// Write:
// - `fn parse_age(raw: &str) -> Result<u32, String>`:
//   - not a number → `Err("not a number: abc")`
//   - over 150 → `Err("age out of range: 200")`
//   - otherwise → `Ok(n)`
// - `fn parse_two(a: &str, b: &str) -> Result<u32, String>`: parses both and
//   returns their sum, using the `?` operator so the first failure stops it
//
// In `main`, test with `("30", "12")`, `("30", "abc")`, `("200", "10")`.
//
// Hint: `?` means "if this is an Err, return it from this function right now".
// It is Rust's version of letting an exception bubble up.
// ============================================================================

fn parse_age(raw: &str) -> Result<u32, String> {
    let r = match raw.parse::<u32>() {
        Ok(r) => r,
        Err(_) => return Err(format!("not a number: {}", raw)), //let _ = 5; compiler: "throw 5 away", let name = 5 --> compiler : "store 5,label it 'name' "
    };

    if r > 150 {
        return Err(format!("age out of range: {}", raw));
    }

    Ok(r)
}

fn parse_two(a: &str, b: &str) -> Result<u32, String> {
    let a_val = parse_age(a)?; // if Err, return it immediately. If Ok, unwrap the value.
    let b_val = parse_age(b)?; 
    Ok(a_val + b_val)
}

fn main() {
    println!("{:?}", parse_two("30", "12"));   // Ok(42)
    println!("{:?}", parse_two("30", "abc"));  // Err("not a number: abc")
    println!("{:?}", parse_two("200", "10"));  // Err("age out of range: 200")
}