//! # Exercise 06 — Custom error types
//!
//! **Scenario.** A config loader that reports *useful* errors — the thing
//! every real service needs and every tutorial skips.
//!
//! **Python contrast.** You would subclass `Exception` and `raise`. In Rust
//! you define an `enum` of everything that can go wrong, implement three
//! traits on it, and return it. The payoff: the compiler lists every failure
//! mode at the call site, and `?` composes them automatically.
//!
//! The three traits that make an error type "real":
//! 1. `Debug`   — derive it. For `{:?}` and for `.unwrap()` panics.
//! 2. `Display` — write it by hand. The message a *user* reads.
//! 3. `std::error::Error` — the marker that unlocks `Box<dyn Error>` and `?`
//!    conversion. Usually an empty impl.
//!
//! (In production you would reach for the `thiserror` crate, which derives all
//! of this. Do it by hand once so you know what the macro generates.)
//!
//! Run: `cargo test -p exercises ex06`

use std::collections::HashMap;
use std::error::Error;
use std::fmt;

/// Every way config loading can fail.
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ConfigError {
    /// A required key was absent.
    Missing { key: String },
    /// A key was present but its value was unusable.
    Invalid { key: String, reason: String },
    /// Two individually-valid settings contradict each other.
    Conflict(String),
}

/// Task 1 — the human-facing message.
///
/// - `Missing`  -> `missing required key: port`
/// - `Invalid`  -> `invalid value for port: not a number`
/// - `Conflict` -> `conflicting settings: tls enabled on plaintext port 80`
impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!("match on self and write! the right message")
    }
}

/// Task 2 — opt into the error ecosystem.
///
/// Every method has a default, so the body is empty. This one line is what
/// lets `?` convert a `ConfigError` into a `Box<dyn Error>` in task 5.
impl Error for ConfigError {}

#[derive(Debug, PartialEq, Eq)]
pub struct AppConfig {
    pub host: String,
    pub port: u16,
    pub workers: usize,
    pub tls: bool,
}

/// Task 3 — the loader. Rules, in this order:
///
/// - `host`: **required**. Missing -> `Missing { key: "host" }`.
///   Empty or whitespace-only -> `Invalid { key: "host", reason: "must not be empty" }`.
/// - `port`: **required**. Missing -> `Missing`. Not a number ->
///   `Invalid { key: "port", reason: "not a number" }`. Outside `1..=65535` ->
///   `Invalid { key: "port", reason: "out of range" }`.
/// - `workers`: optional, default `4`. Not a number ->
///   `Invalid { key: "workers", reason: "not a number" }`. Zero ->
///   `Invalid { key: "workers", reason: "must be at least 1" }`.
/// - `tls`: optional, default `false`. Accepts `true`/`false` case-insensitively.
///   Anything else -> `Invalid { key: "tls", reason: "expected true or false" }`.
/// - finally: if `tls` is on and `port == 80` ->
///   `Conflict("tls enabled on plaintext port 80")`.
///
/// Return the **first** error you hit, in the order above.
pub fn load(raw: &HashMap<String, String>) -> Result<AppConfig, ConfigError> {
    todo!("build each field, using `?` to bail on the first failure")
}

/// Task 4 — collect **all** errors instead of stopping at the first.
///
/// Real config loaders do this so the user fixes everything in one pass.
/// Report errors in the same field order as `load`. `Ok(())` if clean.
pub fn validate_all(raw: &HashMap<String, String>) -> Result<(), Vec<ConfigError>> {
    todo!("accumulate into a Vec instead of returning early")
}

/// Task 5 — `Box<dyn Error>`: the "I don't care which error type" escape hatch.
///
/// Call `load`, then format the result as
/// `"{host}:{port} (workers={workers}, tls={tls})"`.
///
/// Note the return type: `?` on a `ConfigError` works here *only* because you
/// implemented `Error` for it in task 2. Try commenting out that impl and
/// watch this function stop compiling.
pub fn summarize(raw: &HashMap<String, String>) -> Result<String, Box<dyn Error>> {
    todo!("let cfg = load(raw)?;  then format")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn map(pairs: &[(&str, &str)]) -> HashMap<String, String> {
        pairs
            .iter()
            .map(|(k, v)| (k.to_string(), v.to_string()))
            .collect()
    }

    #[test]
    fn t1_display() {
        assert_eq!(
            ConfigError::Missing {
                key: String::from("port")
            }
            .to_string(),
            "missing required key: port"
        );
        assert_eq!(
            ConfigError::Invalid {
                key: String::from("port"),
                reason: String::from("not a number"),
            }
            .to_string(),
            "invalid value for port: not a number"
        );
        assert_eq!(
            ConfigError::Conflict(String::from("tls enabled on plaintext port 80")).to_string(),
            "conflicting settings: tls enabled on plaintext port 80"
        );
    }

    #[test]
    fn t3_load_happy_path() {
        let raw = map(&[("host", "0.0.0.0"), ("port", "8443"), ("tls", "TRUE")]);
        assert_eq!(
            load(&raw),
            Ok(AppConfig {
                host: String::from("0.0.0.0"),
                port: 8443,
                workers: 4,
                tls: true,
            })
        );
    }

    #[test]
    fn t3_load_defaults() {
        let raw = map(&[("host", "localhost"), ("port", "80")]);
        assert_eq!(
            load(&raw),
            Ok(AppConfig {
                host: String::from("localhost"),
                port: 80,
                workers: 4,
                tls: false,
            })
        );
    }

    #[test]
    fn t3_load_errors() {
        assert_eq!(
            load(&map(&[("port", "80")])),
            Err(ConfigError::Missing {
                key: String::from("host")
            })
        );
        assert_eq!(
            load(&map(&[("host", "  ")])),
            Err(ConfigError::Invalid {
                key: String::from("host"),
                reason: String::from("must not be empty"),
            })
        );
        assert_eq!(
            load(&map(&[("host", "h")])),
            Err(ConfigError::Missing {
                key: String::from("port")
            })
        );
        assert_eq!(
            load(&map(&[("host", "h"), ("port", "eighty")])),
            Err(ConfigError::Invalid {
                key: String::from("port"),
                reason: String::from("not a number"),
            })
        );
        assert_eq!(
            load(&map(&[("host", "h"), ("port", "70000")])),
            Err(ConfigError::Invalid {
                key: String::from("port"),
                reason: String::from("out of range"),
            })
        );
        assert_eq!(
            load(&map(&[("host", "h"), ("port", "1"), ("workers", "0")])),
            Err(ConfigError::Invalid {
                key: String::from("workers"),
                reason: String::from("must be at least 1"),
            })
        );
        assert_eq!(
            load(&map(&[("host", "h"), ("port", "1"), ("tls", "yes")])),
            Err(ConfigError::Invalid {
                key: String::from("tls"),
                reason: String::from("expected true or false"),
            })
        );
        assert_eq!(
            load(&map(&[("host", "h"), ("port", "80"), ("tls", "true")])),
            Err(ConfigError::Conflict(String::from(
                "tls enabled on plaintext port 80"
            )))
        );
    }

    #[test]
    fn t4_validate_all() {
        assert_eq!(
            validate_all(&map(&[("host", "h"), ("port", "443")])),
            Ok(())
        );

        let errs = validate_all(&map(&[("port", "nope"), ("workers", "0")]))
            .expect_err("should have failed");
        assert_eq!(
            errs,
            vec![
                ConfigError::Missing {
                    key: String::from("host")
                },
                ConfigError::Invalid {
                    key: String::from("port"),
                    reason: String::from("not a number"),
                },
                ConfigError::Invalid {
                    key: String::from("workers"),
                    reason: String::from("must be at least 1"),
                },
            ]
        );
    }

    #[test]
    fn t5_summarize() {
        let raw = map(&[("host", "api.corp.dev"), ("port", "443"), ("tls", "true")]);
        assert_eq!(
            summarize(&raw).unwrap(),
            "api.corp.dev:443 (workers=4, tls=true)"
        );

        let err = summarize(&map(&[])).expect_err("should have failed");
        assert_eq!(err.to_string(), "missing required key: host");
    }
}
