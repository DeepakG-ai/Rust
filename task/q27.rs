use std::fmt::{Debug, Display};

#[derive(Debug)]
enum ConfigError {
    NotFound { path: String },
    BadPort { value: String },
    Empty,
}

impl Display for ConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConfigError::NotFound { path } => write!(f, "config not found at {path}"),
            ConfigError::BadPort { value } => write!(f, "invalid port value '{value}'"),
            ConfigError::Empty => write!(f, "configuration is empty"),
        }
    }
}

impl std::error::Error for ConfigError {}

// 3. Converts ParseIntError automatically when using `?`
impl From<std::num::ParseIntError> for ConfigError {
    fn from(err: std::num::ParseIntError) -> Self {
        ConfigError::BadPort {
            value: err.to_string(),
        }
    }
}

// 4. Parses a line like "port=8080"
fn load(text: &str) -> Result<u16, ConfigError> {
    if text.trim().is_empty() {
        return Err(ConfigError::Empty);
    }

    // Look for "port=" prefix
    let port_str = text
        .strip_prefix("port=")
        .ok_or_else(|| ConfigError::NotFound {
            //|| defines a closure (an anonymous function / lambda) that takes no arguments.
            path: text.to_string(),
        })?;

    // The bare `?` here converts ParseIntError -> ConfigError via `From`!
    let port: u16 = port_str.trim().parse()?;
    Ok(port)
}

// Question from task.md:
// Which of your impls is what makes the bare `?` work on the parse() call?
// Answer: `impl From<std::num::ParseIntError> for ConfigError`
// The `?` operator calls `From::from(err)` under the hood before returning!

fn main() {
    // 1. Success case: parses valid port
    match load("port=8080") {
        Ok(port) => println!("1. Valid: loaded port {port}"),
        Err(e) => println!("1. Error: {e}"),
    }

    // 2. Empty case: returns ConfigError::Empty
    match load("") {
        Ok(port) => println!("2. Valid: loaded port {port}"),
        Err(e) => println!("2. Error: {e}"),
    }

    // 3. BadPort case: "abc" fails integer parsing -> auto-converted by `?`
    match load("port=abc") {
        Ok(port) => println!("3. Valid: loaded port {port}"),
        Err(e) => println!("3. Error: {e}"),
    }

    // 4. NotFound case: prefix "port=" is missing
    match load("host=127.0.0.1") {
        Ok(port) => println!("4. Valid: loaded port {port}"),
        Err(e) => println!("4. Error: {e}"),
    }
}
