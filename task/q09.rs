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