//! Integration tests — the grader for project 3.
//!
//! Notice how few `sleep`s there are. The channel gives you FIFO ordering, so
//! "record then snapshot" is deterministic without waiting for anything. The
//! one test that does care about time uses `start_paused`, which gives tokio a
//! virtual clock: timers fire instantly but report real elapsed durations.

use std::time::Duration;

use metrics_actor::MetricSummary;
use metrics_actor::MetricsHandle;
use metrics_actor::spawn;
use tokio::task::JoinHandle;
use tokio_util::sync::CancellationToken;

/// One hour, i.e. "no flush ticks will interfere with this test".
const NO_FLUSH: Duration = Duration::from_secs(3600);

fn start(flush_every: Duration) -> (MetricsHandle, JoinHandle<()>, CancellationToken) {
    let cancel = CancellationToken::new();
    let (handle, join) = spawn(flush_every, cancel.clone());
    (handle, join, cancel)
}

#[tokio::test]
async fn records_are_visible_without_sleeping() {
    let (handle, _join, cancel) = start(NO_FLUSH);

    handle.record("latency", 10);
    handle.record("latency", 30);
    handle.record("latency", 5);
    handle.record("errors", 1);

    // No sleep: the snapshot command queues behind the records, so FIFO
    // ordering guarantees they have all been applied.
    let snap = handle.snapshot().await.expect("actor should be alive");

    assert_eq!(snap.metrics.len(), 2);
    assert_eq!(
        snap.metrics["latency"],
        MetricSummary {
            count: 3,
            sum: 45,
            min: 5,
            max: 30,
        }
    );
    assert_eq!(
        snap.metrics["errors"],
        MetricSummary {
            count: 1,
            sum: 1,
            min: 1,
            max: 1,
        }
    );
    assert_eq!(snap.flushes, 0);

    cancel.cancel();
}

#[tokio::test]
async fn metrics_come_back_sorted() {
    let (handle, _join, cancel) = start(NO_FLUSH);

    for name in ["zebra", "apple", "mango"] {
        handle.record(name, 1);
    }

    let snap = handle.snapshot().await.expect("alive");
    let names: Vec<&str> = snap.metrics.keys().map(String::as_str).collect();
    assert_eq!(names, vec!["apple", "mango", "zebra"], "BTreeMap ordering");

    cancel.cancel();
}

#[tokio::test]
async fn mean_is_computed_from_the_summary() {
    let (handle, _join, cancel) = start(NO_FLUSH);

    handle.record("latency", 10);
    handle.record("latency", 20);

    let snap = handle.snapshot().await.expect("alive");
    assert_eq!(snap.metrics["latency"].mean(), 15.0);

    cancel.cancel();
}

#[tokio::test]
async fn reset_clears_metrics_but_not_the_flush_counter() {
    let (handle, _join, cancel) = start(NO_FLUSH);

    handle.record("a", 5);
    handle.reset();

    let snap = handle.snapshot().await.expect("alive");
    assert!(snap.metrics.is_empty());
    assert_eq!(snap.flushes, 0);

    // Still usable afterwards.
    handle.record("b", 1);
    let snap = handle.snapshot().await.expect("alive");
    assert_eq!(snap.metrics.len(), 1);

    cancel.cancel();
}

#[tokio::test]
async fn cloned_handles_all_reach_the_same_actor() {
    let (handle, _join, cancel) = start(NO_FLUSH);

    let tasks: Vec<_> = (0..4)
        .map(|_| {
            let handle = handle.clone();
            tokio::spawn(async move {
                for _ in 0..100 {
                    handle.record("hits", 1);
                }
            })
        })
        .collect();

    for t in tasks {
        t.await.expect("writer task panicked");
    }

    let snap = handle.snapshot().await.expect("alive");
    assert_eq!(
        snap.metrics["hits"],
        MetricSummary {
            count: 400,
            sum: 400,
            min: 1,
            max: 1,
        },
        "no lost updates — the actor is the only writer"
    );

    cancel.cancel();
}

#[tokio::test]
async fn explicit_cancellation_stops_the_actor() {
    let (handle, join, cancel) = start(NO_FLUSH);

    handle.record("a", 1);
    assert!(handle.snapshot().await.is_some());

    cancel.cancel();
    join.await.expect("actor task should exit cleanly, not panic");

    assert!(handle.is_closed(), "the receiver is gone");
    assert_eq!(
        handle.snapshot().await,
        None,
        "callers learn the actor died from a None reply"
    );
}

#[tokio::test]
async fn dropping_every_handle_stops_the_actor() {
    let (handle, join, _cancel) = start(NO_FLUSH);

    let clone = handle.clone();
    drop(handle);

    assert!(
        clone.snapshot().await.is_some(),
        "one surviving handle keeps the actor alive"
    );

    drop(clone);

    // recv() now returns None, so the loop breaks with no cancellation needed.
    join.await.expect("actor task should exit cleanly");
}

#[tokio::test(start_paused = true)]
async fn flush_ticks_fire_on_schedule() {
    let (handle, _join, cancel) = start(Duration::from_millis(100));

    let snap = handle.snapshot().await.expect("alive");
    assert_eq!(
        snap.flushes, 0,
        "tokio's Interval fires its first tick immediately — swallow it \
         before the loop or you start at 1"
    );

    // Virtual time: this returns instantly, but the actor sees ticks at
    // t=100ms and t=200ms.
    tokio::time::sleep(Duration::from_millis(250)).await;

    let snap = handle.snapshot().await.expect("alive");
    assert_eq!(snap.flushes, 2);

    tokio::time::sleep(Duration::from_millis(100)).await;
    let snap = handle.snapshot().await.expect("alive");
    assert_eq!(snap.flushes, 3);

    cancel.cancel();
}

#[tokio::test]
async fn recording_after_shutdown_is_a_silent_no_op() {
    let (handle, join, cancel) = start(NO_FLUSH);
    cancel.cancel();
    join.await.expect("clean exit");

    // Must not panic. A metrics call is never worth crashing the caller over.
    handle.record("a", 1);
    handle.reset();
    assert_eq!(handle.snapshot().await, None);
}

/// A handle must be usable from any task on a multi-threaded runtime, which
/// means it has to be `Send + Sync`. If someone puts an `Rc` in it, this stops
/// compiling.
#[test]
fn handle_is_send_and_sync() {
    fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<MetricsHandle>();
}
