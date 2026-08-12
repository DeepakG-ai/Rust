# Project 3 — `metrics_actor`

The capstone. A metrics collector built as a **tokio actor**: one task owns all
the state, and everyone else talks to it through a cheap cloneable handle.

This is a deliberate miniature of
`grok-build/crates/codegen/xai-hunk-tracker/` — same command enum, same
`mpsc` + `oneshot` split, same `select!` loop with `biased;` and a
`CancellationToken`, same `pub(super)` module carving. Once you have written
this, that 9,000-line actor will read like ordinary code.

**Skills exercised:** `tokio::spawn`, `mpsc`, `oneshot`, `select!`,
`CancellationToken`, `Interval`, `Send + 'static`, `pub(super)` visibility,
async testing with a virtual clock.

## Why an actor instead of `Arc<Mutex<State>>`?

| | `Arc<Mutex<State>>` | Actor |
|---|---|---|
| Concurrency bug surface | lock ordering, poisoning, holding a guard across `.await` | none — one owner |
| Backpressure | invisible | the channel is the queue |
| Ordering | whoever grabs the lock | strict FIFO |
| Cost of a call | lock contention | one channel send |

Notice the FIFO guarantee — it is why the tests can send `record()` and then
immediately `snapshot()` with no sleeping, and still see the records. Getting
that for free is a big part of why this pattern wins.

## Architecture

```
   your code
       │  MetricsHandle (Clone — just an mpsc::Sender)
       │
       │  record()    ──► Command::Record        fire-and-forget
       │  snapshot()  ──► Command::Snapshot{reply} ──► oneshot ──► back to you
       │  reset()     ──► Command::Reset
       ▼
  mpsc::UnboundedSender
       │
       ▼
  ┌──────────────────────────────────────────┐
  │ MetricsActor  (one tokio task)           │
  │   owns BTreeMap<String, MetricSummary>   │
  │                                          │
  │   loop { select! {  biased;              │
  │     cancel.cancelled()  => break         │
  │     ticker.tick()       => on_flush()    │
  │     cmd_rx.recv()       => handle(cmd)   │
  │   }}                                     │
  └──────────────────────────────────────────┘
```

## Your job

| File                    | Status   | What to do                                     |
|-------------------------|----------|------------------------------------------------|
| `src/commands.rs`       | complete | read it — the message protocol                 |
| `src/handle.rs`         | 4 TODOs  | the caller-facing API                          |
| `src/actor/mod.rs`      | 3 TODOs  | spawning and the `select!` loop                |
| `src/actor/aggregate.rs`| 4 TODOs  | the state transitions, behind `pub(super)`     |

## Three traps this project is built around

**1. Never `.await` while holding the state.** You do not have to think about
it here, because the actor *is* the only owner. That is the point.

**2. `Interval`'s first tick fires immediately.** `tokio::time::interval(d)`
completes its first `tick()` at once, not after `d`. Swallow it before the
loop or your flush count will be off by one — the tests check this.

**3. `select!` drops the losing futures.** Every branch must be
cancellation-safe. `mpsc::recv()` and `CancellationToken::cancelled()` are.
`biased;` makes the branches poll top-down instead of in random order, so
shutdown always beats a busy command queue.

## Shutdown, two ways

Both must work:

- **explicit** — `cancel.cancel()`, the `cancelled()` branch wins, loop breaks
- **implicit** — every `MetricsHandle` is dropped, so `recv()` returns `None`

After either, `handle.snapshot().await` returns `None` because the send fails.
That is how a caller learns the actor is gone.

## Run it

```bash
cargo test -p p3_actor
cargo test -p p3_actor -- --nocapture
```

## Definition of done

- [ ] `cargo test -p p3_actor` passes
- [ ] `cargo clippy -p p3_actor -- -D warnings` is clean
- [ ] both shutdown paths work
- [ ] changing `pub(super)` to private in `aggregate.rs` breaks the build in
      `mod.rs` — confirm that, then change it back. That is you *feeling*
      the module boundary rather than reading about it.

## Stretch goals

1. Add `Command::Percentile { name, p, reply }` that returns p95 for one
   metric. You will need to keep the raw samples, not just min/max/sum.
2. Add command coalescing like the real hunk tracker: on wake, drain the
   channel with `try_recv()` and merge all pending `Record`s for the same
   metric into one update before touching state.
3. Swap `mpsc::unbounded_channel` for a **bounded** `mpsc::channel(64)` and
   make `record` async. Now a slow actor applies backpressure to producers —
   which is usually what you want in production, and is why unbounded
   channels are a common source of memory leaks.
4. Add `#[tracing::instrument]` to the actor loop and print a span per command.
