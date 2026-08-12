//! The actor task: owns all state, processes commands one at a time.
//!
//! Submodules are private. `aggregate` holds the state transitions and marks
//! them `pub(super)`, which means "visible in `actor` and everything under
//! it" — so this file can call them, and `crate::handle` cannot.

mod aggregate;

use std::collections::BTreeMap;
use std::time::Duration;

use tokio::sync::mpsc;
use tokio::task::JoinHandle;
use tokio_util::sync::CancellationToken;

use crate::commands::Command;
use crate::commands::MetricSummary;
use crate::handle::MetricsHandle;

/// The state nobody else can touch.
///
/// The struct and every field are fully private to the `actor` module — no
/// `pub` of any kind. Yet `aggregate.rs` can still read and mutate them,
/// because it is a *child* module and children see their ancestors' private
/// items. That is how one type's implementation gets split across two files
/// without widening its visibility by one inch.
struct MetricsActor {
    metrics: BTreeMap<String, MetricSummary>,
    flushes: u64,
    cmd_rx: mpsc::UnboundedReceiver<Command>,
    cancel: CancellationToken,
    flush_every: Duration,
}

/// TODO 1 — start the actor and hand back a handle.
///
/// Steps:
/// 1. `mpsc::unbounded_channel()` for commands
/// 2. build a `MetricsActor` owning the **receiver**
/// 3. `tokio::spawn(actor.run())`
/// 4. return `(MetricsHandle::new(tx), join_handle)`
///
/// Critical: do **not** keep a clone of `tx` anywhere in here. The actor shuts
/// down when the last sender is dropped, so a stray sender held by the library
/// means the actor never exits and `dropping_every_handle_stops_the_actor`
/// hangs forever.
///
/// Also note what `tokio::spawn` demands: `Future + Send + 'static`. The actor
/// owns everything it needs — no borrows — which is exactly why it satisfies
/// `'static`.
pub fn spawn(flush_every: Duration, cancel: CancellationToken) -> (MetricsHandle, JoinHandle<()>) {
    todo!()
}

impl MetricsActor {
    /// TODO 2 — the main loop. This is the heart of the project.
    ///
    /// ```text
    /// let mut ticker = tokio::time::interval(self.flush_every);
    /// ticker.tick().await;          // interval fires immediately — swallow it
    ///
    /// loop {
    ///     tokio::select! {
    ///         biased;
    ///         _ = self.cancel.cancelled() => break,
    ///         _ = ticker.tick()           => self.on_flush(),
    ///         cmd = self.cmd_rx.recv()    => {
    ///             let Some(cmd) = cmd else { break };   // all handles dropped
    ///             self.handle_command(cmd);
    ///         }
    ///     }
    /// }
    /// ```
    ///
    /// Three things to understand rather than copy:
    ///
    /// - **`biased;`** polls branches top-down. Without it the order is
    ///   random, and a saturated command channel could starve shutdown.
    /// - **`recv()` returning `None`** means every sender is gone. That is the
    ///   implicit shutdown path, and it costs you nothing to support.
    /// - **the losing futures get dropped** each time round the loop. Both of
    ///   these branches are cancellation-safe, so re-creating them next
    ///   iteration loses nothing. A partially-consumed socket read would not
    ///   be, and that is the bug class to watch for in real code.
    async fn run(mut self) {
        todo!()
    }

    /// TODO 3 — dispatch one command.
    ///
    /// Delegate to the `pub(super)` helpers in `aggregate.rs`. For `Snapshot`,
    /// send the reply and **ignore a send failure** — it just means the caller
    /// gave up and dropped the receiver, which is not the actor's problem.
    ///
    /// Keep the `match` exhaustive with no wildcard arm, so adding a variant
    /// to `Command` later becomes a compile error instead of a silent no-op.
    fn handle_command(&mut self, cmd: Command) {
        todo!()
    }
}
