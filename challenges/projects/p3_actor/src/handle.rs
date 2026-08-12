//! The caller-facing API.
//!
//! A handle is *nothing but a channel sender*. That is what makes it cheap to
//! clone, `Send + Sync` for free, and impossible to deadlock — there is no
//! lock to hold.

use tokio::sync::mpsc;
use tokio::sync::oneshot;

use crate::commands::Command;
use crate::commands::Snapshot;

/// A cheap, cloneable handle to a running `MetricsActor`.
///
/// Every clone talks to the same actor. When the last clone is dropped, the
/// actor's `recv()` returns `None` and it shuts itself down — reference
/// counting doing lifecycle management for free.
#[derive(Clone)]
pub struct MetricsHandle {
    tx: mpsc::UnboundedSender<Command>,
}

impl MetricsHandle {
    /// `pub(crate)`: only `actor::spawn` may mint a handle, because only it
    /// has a receiver to pair with. A `pub` constructor here would let callers
    /// build handles pointing at nothing.
    pub(crate) fn new(tx: mpsc::UnboundedSender<Command>) -> Self {
        Self { tx }
    }

    /// TODO 1 — fire-and-forget.
    ///
    /// Not `async`, and it does not return a `Result`: sending on an unbounded
    /// channel never blocks, and a caller recording a metric has nothing
    /// useful to do if the actor is gone. Swallow the error deliberately
    /// (`let _ = ...`) — that is a decision, not sloppiness.
    pub fn record(&self, name: &str, value: u64) {
        todo!("send Command::Record")
    }

    /// TODO 2 — request/response over a one-way channel.
    ///
    /// The shape, which you will use again and again:
    ///
    /// 1. `oneshot::channel()` for the reply
    /// 2. send the command with the sender half inside it
    /// 3. `.await` the receiver half
    ///
    /// Returns `None` if the actor is gone — either the send failed, or the
    /// actor died before replying (which drops the sender and errors the
    /// receiver). `Option` rather than `Result` because there is exactly one
    /// reason to fail.
    ///
    /// Hint: `self.tx.send(..).ok()?` then `rx.await.ok()`.
    pub async fn snapshot(&self) -> Option<Snapshot> {
        todo!()
    }

    /// TODO 3 — fire-and-forget reset.
    pub fn reset(&self) {
        todo!()
    }

    /// TODO 4 — has the actor shut down?
    ///
    /// An `mpsc::UnboundedSender` knows when its receiver has been dropped.
    pub fn is_closed(&self) -> bool {
        todo!()
    }
}
