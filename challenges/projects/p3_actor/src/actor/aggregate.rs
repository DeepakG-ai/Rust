//! State transitions for [`MetricsActor`], split out of `mod.rs`.
//!
//! Everything here is `pub(super)`: visible to the `actor` module and its
//! descendants, invisible to the rest of the crate. This is how the real
//! hunk-tracker actor in `grok-build` carves a 9,000-line type into seven
//! readable files without leaking a single internal method.
//!
//! **Experiment worth doing:** change one `pub(super)` below to nothing at all
//! (fully private). `mod.rs` stops compiling, because a private item is
//! visible only inside `aggregate` and *its* children — not in its parent.
//! Then try `pub(crate)` and note that it compiles but now `handle.rs` could
//! call it too. `pub(super)` is the tightest thing that works.

use super::MetricsActor;
use crate::commands::MetricSummary;
use crate::commands::Snapshot;

impl MetricsActor {
    /// TODO 1 — fold one observation into the named metric.
    ///
    /// First observation of a name creates
    /// `MetricSummary { count: 1, sum: value, min: value, max: value }`.
    /// Later ones bump `count`, add to `sum`, and widen `min`/`max`.
    ///
    /// Hint: `self.metrics.entry(name.to_string()).and_modify(..).or_insert(..)`
    /// does it in one lookup.
    pub(super) fn record(&mut self, name: &str, value: u64) {
        todo!()
    }

    /// TODO 2 — a point-in-time copy of the state.
    ///
    /// Must be a clone, not a reference: it is about to be sent through a
    /// channel to another task, which needs to own it.
    pub(super) fn snapshot(&self) -> Snapshot {
        todo!()
    }

    /// TODO 3 — drop all metrics.
    ///
    /// Leaves `flushes` alone — that is a lifetime counter for the actor, not
    /// part of the metric data.
    pub(super) fn reset(&mut self) {
        todo!()
    }

    /// TODO 4 — called on every periodic tick.
    ///
    /// A real collector would ship the batch to Prometheus / Datadog here.
    /// For this project, just count it so the tests can observe that the timer
    /// branch of the `select!` is actually wired up.
    pub(super) fn on_flush(&mut self) {
        todo!()
    }
}
