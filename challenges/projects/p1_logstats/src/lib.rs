//! # logstats — access-log analytics
//!
//! See `README.md` for the full spec. Fill in the `todo!()`s in the three
//! modules below; `main.rs` is already written against this API.
//!
//! Note the module layout: `record` -> `stats` -> `report` is a one-way
//! dependency chain, which is what keeps each piece testable on its own.

#![allow(unused_variables, dead_code, unused_imports)]

pub mod record;
pub mod report;
pub mod stats;

pub use record::LogRecord;
pub use record::ParseError;
pub use record::parse_log;
pub use report::render;
pub use stats::Summary;
pub use stats::summarize;
