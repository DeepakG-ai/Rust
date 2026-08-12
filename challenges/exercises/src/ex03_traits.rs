//! # Exercise 03 — Traits, trait objects, and the standard traits
//!
//! **Scenario.** An alerting fan-out: one incident, many delivery channels
//! (email, Slack), some of which fail.
//!
//! **Python contrast.** A trait is like an ABC / Protocol, but checked at
//! compile time and with two *different* dispatch modes:
//!
//! - `fn f<N: Notifier>(n: &N)` — **static** dispatch. Monomorphised, inlined,
//!   zero cost, but every concrete type gets its own copy of the code.
//! - `fn f(n: &dyn Notifier)` — **dynamic** dispatch. One copy of the code, a
//!   vtable lookup per call. Required when you want a *heterogeneous*
//!   collection, e.g. `Vec<Box<dyn Notifier>>`.
//!
//! That is exactly the `Tool` / `ToolDyn` split you read in `xai-tool-runtime`.
//!
//! Run: `cargo test -p exercises ex03`

use std::fmt;
use std::str::FromStr;

/// A delivery channel for alerts.
pub trait Notifier {
    /// Short channel name, e.g. `"email"`.
    fn channel(&self) -> &str;

    /// Deliver `message`. Returns the delivery receipt, or the failure reason.
    fn send(&self, message: &str) -> Result<String, String>;

    /// Task 1 — a **default method**. Implementors get this for free unless
    /// they override it. Return `"<channel>"`, e.g. `"<email>"`.
    fn describe(&self) -> String {
        todo!("format the channel name in angle brackets")
    }
}

pub struct Email {
    pub to: String,
}

pub struct Slack {
    pub webhook: String,
    /// Simulates an unreachable webhook.
    pub broken: bool,
}

// Task 2 — implement `Notifier` for both.
//
//   Email::channel() -> "email"
//   Email::send(m)   -> Ok("email to {to}: {m}")
//
//   Slack::channel() -> "slack"
//   Slack::send(m)   -> Err("slack webhook {webhook} unreachable")  if broken
//                    -> Ok("slack {webhook}: {m}")                  otherwise
//
// Slack also overrides `describe()` to return "<slack:{webhook}>".

impl Notifier for Email {
    fn channel(&self) -> &str {
        todo!()
    }

    fn send(&self, message: &str) -> Result<String, String> {
        todo!()
    }
}

impl Notifier for Slack {
    fn channel(&self) -> &str {
        todo!()
    }

    fn send(&self, message: &str) -> Result<String, String> {
        todo!()
    }

    fn describe(&self) -> String {
        todo!("override the default so it reads <slack:HOOK>")
    }
}

/// Task 3 — **dynamic** dispatch over a heterogeneous list.
///
/// Returns `(receipts, failures)` in input order.
pub fn broadcast(notifiers: &[Box<dyn Notifier>], message: &str) -> (Vec<String>, Vec<String>) {
    todo!("call send on each, partition Ok from Err")
}

/// Task 4 — **static** dispatch over a homogeneous list.
///
/// Returns how many deliveries succeeded. Note the shape difference: this one
/// cannot hold a mix of `Email` and `Slack`, but it costs no vtable lookup.
pub fn count_delivered<N: Notifier>(notifiers: &[N], message: &str) -> usize {
    todo!()
}

/// Alert severity, ordered: `Info < Warning < Critical`.
///
/// Task 5 — the derives are already correct. Do not change them; just note
/// that `PartialOrd`/`Ord` on an enum use *declaration order*.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Severity {
    Info,
    Warning,
    Critical,
}

/// Task 5a — `Display` is what `{}` uses. Print `INFO` / `WARN` / `CRIT`.
///
/// (`Debug`, derived above, is what `{:?}` uses — it prints `Info`. Two
/// different traits for two different audiences: humans vs. developers.)
impl fmt::Display for Severity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!("write!(f, ...) with the short uppercase form")
    }
}

/// Task 5b — `FromStr` is what `.parse()` uses. Accept, case-insensitively:
/// `info`; `warn` or `warning`; `crit` or `critical`.
/// Anything else: `Err("unknown severity: {s}")`.
impl FromStr for Severity {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        todo!()
    }
}

/// Routing policy for the alerting system.
#[derive(Debug, PartialEq, Eq)]
pub struct AlertPolicy {
    pub min_severity: Severity,
    pub max_retries: u32,
    pub dedupe_window_secs: u64,
}

/// Task 6 — hand-written `Default` (the derive would give `Info`, `0`, `0`).
/// Defaults: `Warning`, `3`, `300`.
impl Default for AlertPolicy {
    fn default() -> Self {
        todo!()
    }
}

/// Task 7 — put it together: should this alert be delivered under `policy`?
pub fn should_alert(policy: &AlertPolicy, severity: Severity) -> bool {
    todo!("compare against min_severity — the Ord derive makes `>=` work")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t2_notifier_impls() {
        let e = Email {
            to: String::from("ops@corp.dev"),
        };
        assert_eq!(e.channel(), "email");
        assert_eq!(
            e.send("disk full"),
            Ok(String::from("email to ops@corp.dev: disk full"))
        );
        assert_eq!(e.describe(), "<email>", "default method");

        let ok = Slack {
            webhook: String::from("hooks/1"),
            broken: false,
        };
        assert_eq!(ok.send("disk full"), Ok(String::from("slack hooks/1: disk full")));
        assert_eq!(ok.describe(), "<slack:hooks/1>", "overridden method");

        let bad = Slack {
            webhook: String::from("hooks/2"),
            broken: true,
        };
        assert_eq!(
            bad.send("disk full"),
            Err(String::from("slack webhook hooks/2 unreachable"))
        );
    }

    #[test]
    fn t3_broadcast_dynamic_dispatch() {
        let notifiers: Vec<Box<dyn Notifier>> = vec![
            Box::new(Email {
                to: String::from("a@b.c"),
            }),
            Box::new(Slack {
                webhook: String::from("h1"),
                broken: true,
            }),
            Box::new(Slack {
                webhook: String::from("h2"),
                broken: false,
            }),
        ];

        let (ok, err) = broadcast(&notifiers, "ping");
        assert_eq!(
            ok,
            vec![
                String::from("email to a@b.c: ping"),
                String::from("slack h2: ping")
            ]
        );
        assert_eq!(err, vec![String::from("slack webhook h1 unreachable")]);
    }

    #[test]
    fn t4_count_delivered_static_dispatch() {
        let slacks = vec![
            Slack {
                webhook: String::from("h1"),
                broken: false,
            },
            Slack {
                webhook: String::from("h2"),
                broken: true,
            },
            Slack {
                webhook: String::from("h3"),
                broken: false,
            },
        ];
        assert_eq!(count_delivered(&slacks, "ping"), 2);
    }

    #[test]
    fn t5_severity_display_and_parse() {
        assert_eq!(Severity::Info.to_string(), "INFO");
        assert_eq!(format!("{}", Severity::Warning), "WARN");
        assert_eq!(format!("{}", Severity::Critical), "CRIT");
        assert_eq!(format!("{:?}", Severity::Critical), "Critical", "Debug differs");

        assert_eq!("info".parse::<Severity>(), Ok(Severity::Info));
        assert_eq!("WARNING".parse::<Severity>(), Ok(Severity::Warning));
        assert_eq!("Crit".parse::<Severity>(), Ok(Severity::Critical));
        assert_eq!(
            "nope".parse::<Severity>(),
            Err(String::from("unknown severity: nope"))
        );

        assert!(Severity::Critical > Severity::Info, "declaration order");
    }

    #[test]
    fn t6_and_t7_policy() {
        let p = AlertPolicy::default();
        assert_eq!(
            p,
            AlertPolicy {
                min_severity: Severity::Warning,
                max_retries: 3,
                dedupe_window_secs: 300,
            }
        );

        assert!(!should_alert(&p, Severity::Info));
        assert!(should_alert(&p, Severity::Warning));
        assert!(should_alert(&p, Severity::Critical));
    }
}
