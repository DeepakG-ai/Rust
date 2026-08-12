//! The message protocol between handles and the actor.
//!
//! **This file is complete — nothing to implement here.**
//!
//! Everything crossing the channel must be `Send + 'static`, because the actor
//! task may run on a different worker thread. Note that `String` is owned, not
//! `&str`: a borrowed message could not satisfy `'static`.

use std::collections::BTreeMap;

use tokio::sync::oneshot;

/// Rolled-up statistics for one metric.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MetricSummary {
    pub count: u64,
    pub sum: u64,
    pub min: u64,
    pub max: u64,
}

impl MetricSummary {
    pub fn mean(&self) -> f64 {
        if self.count == 0 {
            0.0
        } else {
            self.sum as f64 / self.count as f64
        }
    }
}

/// A point-in-time view of everything the actor knows.
///
/// `BTreeMap` rather than `HashMap` so iteration order is sorted and tests are
/// deterministic — worth the tiny cost for anything you print or assert on.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Snapshot {
    pub metrics: BTreeMap<String, MetricSummary>,
    /// How many periodic flush ticks have fired since startup.
    pub flushes: u64,
}

/// Everything a handle can ask the actor to do.
///
/// `Snapshot` carries its own reply channel — that is how a request/response
/// call rides on a one-way message queue. The actor answers by sending into
/// `reply`; the caller is meanwhile awaiting the matching receiver.
#[derive(Debug)]
pub enum Command {
    /// Fire-and-forget: fold `value` into the metric named `name`.
    Record { name: String, value: u64 },
    /// Request/response: send the current state back through `reply`.
    Snapshot { reply: oneshot::Sender<Snapshot> },
    /// Fire-and-forget: drop all recorded metrics.
    Reset,
}
