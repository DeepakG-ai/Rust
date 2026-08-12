//! # metrics_actor — a tokio actor, built from scratch
//!
//! See `README.md` for the architecture diagram and the spec.
//!
//! Module layout, and why:
//!
//! ```text
//! lib.rs
//!   ├── commands   (pub)     the wire protocol — callers construct these
//!   ├── handle     (pub)     the caller-facing API
//!   └── actor      (PRIVATE) the implementation; only `spawn` escapes
//!        └── aggregate       state transitions, `pub(super)` to `actor`
//! ```
//!
//! `mod actor;` has no `pub`, so nothing inside it is reachable from outside
//! this crate except what is re-exported below. That is the "prefer private
//! modules and an explicitly exported public API" rule from the codex
//! codebase, applied in miniature.

#![allow(unused_variables, dead_code, unused_imports, unused_mut)]

mod actor;
pub mod commands;
pub mod handle;

pub use actor::spawn;
pub use commands::Command;
pub use commands::MetricSummary;
pub use commands::Snapshot;
pub use handle::MetricsHandle;
