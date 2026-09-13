# Logging and Tracing in Rust

This guide explains Rust's `tracing` ecosystem from the perspective of a developer who already knows the basics of Python logging.

The most useful mental model is:

> `tracing` is structured logging plus information about which operation produced each log.

It can produce normal log lines, but it can also describe operations that start, perform work, contain other operations, and finish.

---

## Read This First: The Beginner Path

This is a reference guide, so it intentionally contains more than you need on your first day. Do **not** try to memorize the entire file.

For today's lesson, learn only these six ideas:

1. A **level** says how important or detailed a record is.
2. An **event** says that something happened now.
3. A **span** describes an operation that takes some time.
4. **Instrumentation** means adding events and spans to your code.
5. `tracing` creates events and spans.
6. `tracing-subscriber` filters them and prints or exports them.

Read the first example below, then Sections 1 through 10. Skip every subsection labelled **Later topic** until the basic model feels natural.

### One analogy

Imagine a hospital:

- An **event** is one note: "medicine given at 10:30."
- A **span** is the whole patient visit, from admission to discharge.
- A **field** is searchable information such as `patient_id=42`.
- A **subscriber** is the records department receiving the notes.
- A **filter** decides which notes must be kept.
- A **layer** is one destination: terminal, file, or monitoring service.

You can use events without manually creating spans. Start with events; add spans around meaningful operations when you are comfortable.

---

## Your First Complete Tracing Program

This repository already contains the required dependencies:

```toml
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter"] }
```

Put this example in `src/bin/tracing_demo.rs` when you want to practise:

```rust
use tracing::{debug, info, instrument, warn};
use tracing_subscriber::EnvFilter;

fn setup_tracing() {
    let filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("info"));

    tracing_subscriber::fmt()
        .with_env_filter(filter)
        .init();
}

#[instrument]
fn process_order(order_id: u64, stock: u32) {
    info!("processing order");

    if stock == 0 {
        warn!("item is out of stock");
        return;
    }

    debug!("reserving inventory");
    info!("order accepted");
}

fn main() {
    setup_tracing();
    process_order(1001, 2);
}
```

### What exactly is `tracing_subscriber::fmt()`?

This line creates a **subscriber builder** that will turn tracing events into readable text logs:

```rust
tracing_subscriber::fmt()
```

Read the path from left to right:

```text
tracing_subscriber  the crate that collects tracing data
fmt()               a function that starts building a text-formatting subscriber
```

It does not install anything yet. The following methods configure and install it:

```rust
tracing_subscriber::fmt()       // 1. Create the subscriber builder.
    .with_env_filter(filter)    // 2. Decide which events are enabled.
    .init();                    // 3. Install it globally.
```

The name is confusing because `fmt` is both a module name and a function exported by that module. These three forms are related but have different purposes:

| Form | Meaning |
|---|---|
| `tracing_subscriber::fmt()` | Create a configurable formatting subscriber builder. |
| `tracing_subscriber::fmt::init()` | Install a simple formatting subscriber using defaults. |
| `tracing_subscriber::fmt::layer()` | Create only a formatting layer to combine with a registry and other layers. This is for later. |

For Q33, use the first form because you need to add an `EnvFilter` before calling `.init()`:

```rust
let filter = EnvFilter::try_from_default_env()
    .unwrap_or_else(|_| EnvFilter::new("info"));

tracing_subscriber::fmt()
    .with_env_filter(filter)
    .init();
```

Run it normally:

```powershell
cargo run --bin tracing_demo
```

The default filter is `INFO`, so you see the two `INFO` events. The `DEBUG` event is ignored.

Now enable debug output:

```powershell
$env:RUST_LOG = "debug"
cargo run --bin tracing_demo
```

Now the `DEBUG` event also appears.

### What happens, in exact order?

```text
1. setup_tracing() installs the subscriber.
2. process_order(...) creates a span because of #[instrument].
3. info!(...) creates an event inside that span.
4. EnvFilter checks whether the event's level is enabled.
5. The fmt subscriber formats an enabled event as a log line.
6. process_order returns, so its span ends.
```

That is the complete beginner model. Everything else in this guide extends one of those six steps.

---

## 1. Python Logging Compared with Rust Tracing

| Python logging | Rust `tracing` |
|---|---|
| `logger.debug(...)` | `tracing::debug!(...)` |
| `logger.info(...)` | `tracing::info!(...)` |
| `logger.warning(...)` | `tracing::warn!(...)` |
| `logger.error(...)` | `tracing::error!(...)` |
| `logger.critical(...)` | Usually `error!`, followed by shutdown or error propagation |
| `extra={"user_id": 42}` | `info!(user_id = 42, "...")` |
| Logger name such as `app.database` | Target, normally the Rust module path |
| Handler, formatter, and filter | Subscriber and layers |
| Request context | Span |

Rust `tracing` has five levels:

```text
ERROR
WARN
INFO
DEBUG
TRACE
```

`TRACE` is more detailed and usually more verbose than `DEBUG`.

There is no `CRITICAL` level. A fatal problem is usually recorded with `error!`, after which the program decides whether to return an error, shut down, or panic. Do not panic merely because you want to produce a critical log. A panic should normally represent a broken program invariant or an unrecoverable programming defect.

### Level filtering

If the configured level is `INFO`, these levels are enabled:

```text
INFO, WARN, ERROR
```

These levels are disabled:

```text
DEBUG, TRACE
```

The more verbose the configured level, the more information is collected.

---

## 2. Rust's `log` Crate Versus Its `tracing` Crate

Rust has two commonly encountered producer APIs. They are alternatives with compatibility bridges between them, not two layers that every program must install.

### The `log` crate

`log` is a small logging facade. Code emits flat log records through macros:

```rust
log::error!("database connection failed");
log::warn!("retrying request");
log::info!("application started");
log::debug!("cache miss");
log::trace!("polling socket");
```

The `log` crate does not print anything by itself. An executable installs a logger implementation such as `env_logger`:

```toml
[dependencies]
log = "0.4"
env_logger = "0.11"
```

```rust
fn main() {
    env_logger::init();
    log::info!("application started");
}
```

`log` is still valid and widely used. It can be enough for a small synchronous program or for a library that wants the smallest common logging interface.

### The `tracing` crate

`tracing` is a richer diagnostics framework. It adds spans, structured fields, parent-child relationships, and async-aware context.

It is sometimes described as a "superset of `log`". That is a useful first approximation, but not literally true: they are different facades with different record models. Compatibility adapters can move records between them.

For the async Rust applications in this repository, learning `tracing` directly is the better default. A tiny CLI may use either API; using `tracing` is not inherently wrong or excessively complicated if the rest of the project already uses it.

### Later topic: Compatibility in both directions

A dependency may still use `log` even when your application uses `tracing`:

```text
dependency's log::info!
        |
        | LogTracer compatibility adapter
        v
tracing Event
        |
        v
your tracing Subscriber
```

With its default `tracing-log` feature, `tracing-subscriber` initialization installs this compatibility adapter. This is why many applications can collect `log` records from older dependencies through the same subscriber.

The other direction is also possible. Enabling `tracing`'s `log` feature makes tracing instrumentation emit a `log` record when no tracing subscriber is active. This is useful for libraries whose callers may use either ecosystem.

Do not enable `log-always` while also converting `log` records back into tracing events unless you deliberately want both outputs; that configuration can create duplicate events.

### What should you choose?

For this learning path:

- Use `tracing` for Tokio, Axum, servers, background workers, and applications with concurrent operations.
- Understand `log` because many dependencies and existing codebases use it.
- Do not install both `env_logger` and a tracing subscriber without designing how they interact.
- In a reusable library, emit diagnostics but let the final application choose the logger or subscriber and output destination.

---

## 3. Why Not Use Only Traditional Log Messages?

A traditional log describes one fact:

```text
database query failed
```

In a synchronous program, nearby lines may provide enough context. That becomes unreliable in an asynchronous server because many tasks are making progress at the same time:

```text
request started
request started
loading user
loading user
request completed
database failed
```

Which request had the database failure?

Tokio can move between tasks whenever a future yields. Multiple tasks may use the same thread, so a thread ID is not a request ID. `tracing` solves this by associating events with spans representing the operations in progress.

---

## 4. Events: Something Happened

An **event** represents one point in time. It is the closest equivalent to a normal Python log record.

```rust
use tracing::{debug, error, info, warn};

let user_id = 42;

info!(user_id, "login succeeded");

warn!(
    user_id,
    attempts = 3,
    "user has been temporarily throttled"
);

error!(
    user_id,
    error = "connection refused",
    retryable = true,
    "database operation failed"
);

debug!(query = "find_user", "executing database query");
```

Values such as `user_id`, `attempts`, and `retryable` are structured fields. They are separate from the human-readable message.

This is better than building one large string:

```rust
// Less useful to log-analysis systems:
info!("user 42 logged in from mobile");

// Structured and searchable:
info!(user_id = 42, client = "mobile", "login succeeded");
```

A JSON log processor can search for `user_id = 42` without parsing an English sentence.

### Display and Debug values

Use `%` to record a value through its `Display` implementation:

```rust
error!(error = %err, "request failed");
```

Use `?` to record a value through its `Debug` implementation:

```rust
debug!(response = ?response, "provider response received");
```

As a simple rule:

- Use `%` for errors and human-readable values.
- Use `?` for structs and detailed debugging values.

---

## 5. Spans: An Operation Is Happening

A **span** represents a period of time with a beginning and an end.

```text
HTTP request starts
    authentication starts
        password lookup
        login succeeds
    authentication ends
HTTP request ends
```

Events created inside a span inherit its context. Conceptually, the result may look like this:

```text
request{request_id="abc123"}
  authenticate{user_id=42}
    DEBUG checking password
    INFO login succeeded
```

Now `checking password` does not need to repeat the request ID and user ID in its message. The enclosing spans provide that context.

### Later topic: Creating a span manually

```rust
use tracing::{info, info_span};

let span = info_span!("process_order", order_id = 1001);

span.in_scope(|| {
    info!("validating order");
    info!("order accepted");
});
```

Use `in_scope` for synchronous work. Be careful not to hold the guard returned by `span.enter()` across an `.await`, because other tasks may execute while that guard is active.

For async functions, prefer `#[instrument]` or the `Instrument` trait.

---

## 6. What Instrumentation Means

**Instrumentation** means adding code that exposes what the application is doing.

All of these are instrumentation:

```rust
info!("server started");
debug!(user_id, "loading user");
let span = tracing::info_span!("process_order", order_id);
```

The `#[instrument]` attribute is a convenient way to create a span around a function call:

```rust
use tracing::{debug, info, instrument, warn};

#[instrument(skip(password))]
fn authenticate(user_id: u64, password: &str) -> bool {
    debug!("checking credentials");

    if password != "correct" {
        warn!(reason = "invalid_credentials", "login rejected");
        return false;
    }

    info!("login succeeded");
    true
}
```

Calling this function creates a span similar to:

```text
authenticate{user_id=42}
```

The events inside the function are associated with that span.

By default, `#[instrument]` records function arguments using `Debug`. Therefore, skipping sensitive or unnecessarily large arguments is important:

```rust
#[instrument(skip(password, request_body))]
fn create_user(
    email: &str,
    password: &str,
    request_body: &[u8],
) {
    // ...
}
```

Never record:

- Passwords
- Access or refresh tokens
- API keys
- Authorization headers
- Session cookies
- Private encryption keys
- Full payment-card information
- Unnecessary personally identifiable information

`#[instrument]` also works with async functions and keeps their tracing context associated with the future as it is polled:

```rust
#[tracing::instrument(skip(client))]
async fn load_user(client: &DatabaseClient, user_id: u64) {
    tracing::debug!("loading user from database");
    // client.load(user_id).await;
}
```

### Later topic: Instrumenting an async block or future manually

Sometimes you cannot put `#[instrument]` on a function, or you want a span around only part of it. Attach a span to the future with `.instrument(...)`:

```rust
use tracing::{info, info_span, Instrument};

async fn handle_request(request_id: u64) {
    let span = info_span!("request", request_id);

    async {
        info!("calling dependency");
        fetch_data().await;
    }
    .instrument(span)
    .await;
}
```

This enters the span each time the future is polled and exits it when the future yields. That is why it is safe across `.await`.

### Later topic: Spawned tasks need their own attached context

Do not assume that a new Tokio task automatically belongs to the current span. Attach the intended span when spawning it:

```rust
use tracing::{info_span, Instrument};

let task_span = info_span!("refresh_cache", cache = "users");

tokio::spawn(
    async move {
        refresh_cache().await;
    }
    .instrument(task_span),
);
```

If the task should simply inherit whichever span is current at the spawn site, use `.in_current_span()`:

```rust
tokio::spawn(async move {
    refresh_cache().await;
}.in_current_span());
```

### Later topic: Recording returned errors automatically

The `err` option emits an `ERROR` event when a function returns `Err`:

```rust
#[tracing::instrument(skip(repository), err)]
async fn load_order(
    repository: &OrderRepository,
    order_id: u64,
) -> Result<Order, OrderError> {
    repository.load(order_id).await
}
```

The error type must support the requested formatting. `err` uses `Display`; `err(Debug)` uses `Debug`.

Similarly, `ret` records successful return values:

```rust
#[tracing::instrument(ret)]
fn double(value: u64) -> u64 {
    value * 2
}
```

Use `ret` carefully: return values can be large or sensitive. In production code, recording a small explicit result field is often safer.

### Later topic: Declaring a field now and recording it later

Sometimes a span field is not known when the operation begins:

```rust
use tracing::{field, info, info_span, Instrument};

let span = info_span!("request", request_id = 42, status_code = field::Empty);

async {
    let status_code = call_service().await;
    tracing::Span::current().record("status_code", status_code);
    info!("request completed");
}
.instrument(span)
.await;
```

A field must be declared when the span is created before it can be recorded later.

---

## 7. Why `tracing` and `tracing-subscriber` Are Separate

The complete pipeline is:

```text
Application and library code
        |
        | tracing::info!, warn!, #[instrument]
        v
Tracing events and spans
        |
        v
Subscriber
        +-- filters unwanted events
        +-- prints human-readable logs
        +-- produces JSON logs
        +-- exports spans to an observability backend
```

### The `tracing` crate

The `tracing` crate is used by application and library code to describe what happened:

```rust
tracing::info!(order_id, "order accepted");
```

It provides:

- Event macros such as `info!` and `error!`
- Spans
- The `#[instrument]` attribute
- Structured fields
- The subscriber interface

It does not decide how information is formatted or where it is sent.

### The `tracing-subscriber` crate

The `tracing-subscriber` crate receives events and spans. It decides:

- Which events are enabled
- How events are formatted
- Whether the output is human-readable or JSON
- Whether timestamps, file names, targets, or thread IDs are shown
- Whether data is sent to another system

A useful analogy is:

```text
tracing             = producer and facade
tracing-subscriber  = filters, handlers, and formatters
```

This separation is especially important for reusable libraries:

- A library should emit events using `tracing`.
- A library should not install a global subscriber.
- The final executable owns the output and filtering policy.

The subscriber is normally initialized once, near the beginning of `main`.

---

## 8. Subscribers, Registries, and Layers

A subscriber may be composed from multiple **layers**:

```text
Registry
  + EnvFilter layer
  + console formatting layer
  + JSON/file layer
  + OpenTelemetry export layer
```

The registry stores span data and relationships. A layer adds some behavior.

For example:

- `EnvFilter` decides what is enabled.
- `fmt::layer()` formats events as log lines.
- An OpenTelemetry layer can export spans to another service.

You do not need to write custom layers while learning. Start with one filter and one formatting layer.

### Later topic: Spans do not automatically print start and end lines

A span provides context even when no separate line is printed for its creation or completion. The default formatting layer does not emit span lifecycle events.

You can request them explicitly:

```rust
use tracing_subscriber::fmt::format::FmtSpan;

let format_layer = tracing_subscriber::fmt::layer()
    .with_span_events(FmtSpan::NEW | FmtSpan::CLOSE);
```

The choices are:

- `NEW`: the span was created.
- `ENTER`: execution entered the span.
- `EXIT`: execution left the span.
- `CLOSE`: the final span handle was dropped; timing information may be included.
- `ACTIVE`: `ENTER | EXIT`.
- `FULL`: all lifecycle events.
- `NONE`: no lifecycle events; this is the default.

`ENTER` and `EXIT` can be extremely noisy for async futures because a future may be entered and exited every time it is polled. `NEW | CLOSE` is often easier to read.

### Later topic: Per-layer filtering

Different destinations often need different verbosity:

```rust
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt};
use tracing_subscriber::{EnvFilter, Layer};

let console_layer = fmt::layer()
    .with_filter(EnvFilter::new("debug"));

// `.json()` requires tracing-subscriber's `json` feature; see Section 13.
let json_layer = fmt::layer()
    .json()
    .with_writer(std::io::stderr)
    .with_filter(EnvFilter::new("info"));

tracing_subscriber::registry()
    .with(console_layer)
    .with(json_layer)
    .init();
```

The console receives `DEBUG` and above. The JSON destination receives only `INFO` and above.

### Later topic: Global and scoped subscribers

`init()` installs a process-wide default and panics if another global subscriber or logger conflicts with it:

```rust
subscriber.init();
```

`try_init()` reports the conflict as a `Result` instead:

```rust
subscriber.try_init()?;
```

Tests often need a temporary scoped subscriber rather than a second global one:

```rust
let subscriber = tracing_subscriber::fmt().with_test_writer().finish();
let _guard = tracing::subscriber::set_default(subscriber);

// Events in this scope use the temporary subscriber.
```

The guard restores the previous scoped default when dropped. A scoped default applies to the current thread; async tests and spawned threads require additional care.

---

## 9. Basic Project Setup

Add the dependencies to `Cargo.toml`:

```toml
[dependencies]
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter"] }
```

This repository already has these dependencies.

Create the initialization function:

```rust
use tracing_subscriber::{
    fmt,
    layer::SubscriberExt,
    util::SubscriberInitExt,
    EnvFilter,
};

fn init_tracing() {
    let filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("info"));

    tracing_subscriber::registry()
        .with(filter)
        .with(fmt::layer())
        .init();
}
```

Call it once when the program starts:

```rust
fn main() {
    init_tracing();

    tracing::info!("application started");
}
```

For a tiny learning program, a shorter setup is possible:

```rust
fn main() {
    tracing_subscriber::fmt::init();
    tracing::info!("application started");
}
```

The layered version is more flexible when you later add custom filtering, JSON, or trace exporting.

---

## 10. What `EnvFilter` Does

`EnvFilter` allows the logging level to be changed while starting the application, without modifying or recompiling the code.

It normally reads the `RUST_LOG` environment variable.

### PowerShell examples

Enable debug information globally:

```powershell
$env:RUST_LOG = "debug"
cargo run
```

Enable only warnings and errors:

```powershell
$env:RUST_LOG = "warn"
cargo run
```

Use `INFO` globally but enable `DEBUG` for one application:

```powershell
$env:RUST_LOG = "info,my_app=debug"
cargo run
```

Enable detailed tracing for one module:

```powershell
$env:RUST_LOG = "info,my_app::database=trace"
cargo run
```

Replace `my_app` with the crate or target name used by your program. Cargo package names containing `-` normally appear in Rust module targets with `_` instead.

### Linux and macOS examples

```bash
RUST_LOG=debug cargo run
```

```bash
RUST_LOG=info,my_app::database=trace cargo run
```

### Hierarchical filtering

Targets normally use Rust module paths:

```text
my_app
my_app::authentication
my_app::database
my_app::database::queries
```

This filter:

```text
info,my_app::database=trace
```

means:

- Use `INFO` for everything by default.
- Include `DEBUG` and `TRACE` inside `my_app::database`.

This is similar to configuring Python loggers by names such as `app.database`.

### Why the explicit fallback is useful

The setup used earlier contains:

```rust
let filter = EnvFilter::try_from_default_env()
    .unwrap_or_else(|_| EnvFilter::new("info"));
```

This means:

1. Try to read a valid `RUST_LOG` value.
2. If the variable is missing or invalid, use `INFO`.

`EnvFilter::from_default_env()` by itself falls back to `ERROR` when the environment variable is missing or invalid. An explicit `INFO` fallback is often less surprising for an application.

---

## 11. Choosing the Correct Level

### `ERROR`

Use when an operation unexpectedly failed and may require investigation.

```rust
error!(error = %err, order_id, "payment request failed");
```

Examples:

- Database migration failed
- Required dependency is unavailable
- Data could not be persisted
- A background job permanently failed after retries

### `WARN`

Use when something abnormal happened but the application recovered or continued.

```rust
warn!(attempt = 2, "request failed; retrying");
```

Examples:

- A request will be retried
- A deprecated configuration was used
- A cache is unavailable, so the database will be used
- A rate limit is close to being reached

### `INFO`

Use for important normal lifecycle and business events.

```rust
info!(port = 8080, "server listening");
```

Examples:

- Service started or stopped
- Configuration loaded
- Order accepted
- Background job completed
- Database connection established

### `DEBUG`

Use for information developers need during diagnosis.

```rust
debug!(user_id, "loading user permissions");
```

Examples:

- Which implementation or branch was selected
- Cache hit or miss
- Query or operation being performed
- Retry timing

### `TRACE`

Use for extremely detailed, high-volume execution information.

```rust
tracing::trace!(bytes = body.len(), "received response body");
```

Examples:

- Individual loop iterations
- Protocol messages
- Fine-grained state-machine transitions
- Detailed polling behavior

`TRACE` is usually disabled in production unless investigating a specific problem.

---

## 12. Where to Put Spans and Events

Good span boundaries include:

- One incoming HTTP request
- One background job
- One database operation
- One call to another service
- One command handled by the application
- One significant business operation, such as `place_order`

Inside those spans, create events for:

- Important decisions
- Success or failure
- Retries
- Unexpected states
- Meaningful state transitions

Example:

```rust
#[tracing::instrument(skip(repository), fields(order.id = order_id))]
async fn process_order(
    repository: &OrderRepository,
    order_id: u64,
) -> Result<(), OrderError> {
    tracing::debug!("loading order");
    let order = repository.load(order_id).await?;

    tracing::debug!(status = ?order.status, "validating order state");

    repository.mark_processed(order_id).await?;
    tracing::info!("order processed");

    Ok(())
}
```

Avoid an `INFO` event for every loop iteration or every tiny internal function call. Excessive logs make useful information harder to find and increase storage and processing costs.

---

## 13. Later Topic: Development and Production Output

Human-readable output is convenient during development:

```rust
tracing_subscriber::registry()
    .with(filter)
    .with(tracing_subscriber::fmt::layer())
    .init();
```

Production systems frequently use JSON because log collectors can index its fields reliably. Enable the `json` feature:

```toml
tracing-subscriber = {
    version = "0.3",
    features = ["env-filter", "json"]
}
```

Then configure the formatting layer:

```rust
tracing_subscriber::registry()
    .with(filter)
    .with(tracing_subscriber::fmt::layer().json())
    .init();
```

Applications running in containers commonly write logs to standard output. The deployment platform then collects and sends them to a log backend.

JSON is common, not a universal requirement. Journald, OpenTelemetry, and hosted observability SDKs can preserve structured fields without requiring your application to print JSON. Choose the format expected by the system collecting the events.

### Standard output versus standard error

Choose the destination based on the executable's interface:

- A normal service often writes logs to `stdout` for a container runtime to collect.
- A CLI often writes user-requested data to `stdout` and diagnostics to `stderr`.
- A program using `stdout` for JSON-RPC or another machine protocol must send logs to `stderr` or a file. One accidental log line on `stdout` could corrupt the protocol stream.

```rust
let format_layer = tracing_subscriber::fmt::layer()
    .with_writer(std::io::stderr);
```

### Non-blocking file logging

Writing to a file is blocking I/O. In a high-throughput async application, move those writes off the request path with `tracing-appender`:

```toml
[dependencies]
tracing-appender = "0.2"
```

```rust
use tracing_subscriber::prelude::*;

fn init_file_tracing() -> tracing_appender::non_blocking::WorkerGuard {
    let file = tracing_appender::rolling::daily("./logs", "app.log");
    let (writer, guard) = tracing_appender::non_blocking(file);

    let file_layer = tracing_subscriber::fmt::layer()
        .with_writer(writer)
        .with_ansi(false);

    tracing_subscriber::registry()
        .with(file_layer)
        .init();

    guard
}

fn main() {
    let _log_guard = init_file_tracing();
    // Keep _log_guard alive until the program finishes.
}
```

The `WorkerGuard` is not decorative. The non-blocking writer sends events to a background thread. Dropping the guard early can lose buffered records, and abrupt termination through `std::process::exit` can still bypass normal cleanup.

The default non-blocking writer is lossy when its bounded queue is full. This protects application latency but means logs may be dropped under sustained overload. The builder can select backpressure instead, trading application latency for lossless delivery. This is an operational decision, not merely a syntax choice.

Disabling ANSI for file and machine-readable output prevents terminal color codes from being stored in the file:

```rust
.with_ansi(false)
```

---

## 14. Later Topic: Logging, Tracing, Metrics, and Distributed Tracing

These terms are related but not identical.

### Logs

Logs describe individual occurrences:

```text
payment failed
```

### Traces

Traces show the path and timing of an operation:

```text
HTTP request
  -> authentication
  -> database query
  -> payment service
```

### Metrics

Metrics aggregate numerical behavior over time:

```text
requests_per_second = 350
error_rate = 1.2%
p95_latency = 240 ms
```

There are also three easily confused uses of the word "trace":

- `TRACE` is the most verbose logging level.
- A span records one operation or portion of an operation.
- A distributed trace connects spans across several services using trace IDs.

The `tracing` crate provides events and spans inside Rust applications. It does not automatically create a complete distributed-tracing system. Additional integration, commonly through OpenTelemetry, exports the spans to systems such as Jaeger or Grafana Tempo.

Adding an OpenTelemetry layer exports local spans, but that alone does not connect service A's span to service B's span. Distributed tracing also needs **context propagation**:

```text
Service A current span
        |
        | inject traceparent/tracestate into request headers
        v
HTTP request
        |
        | Service B extracts headers and sets its request span's parent
        v
Service B request span
```

W3C `traceparent` and `tracestate` headers commonly carry that identity. HTTP middleware may perform injection and extraction for you, but it must be configured; the Rust compiler and `tracing` crate do not do it automatically.

Telemetry exporters also buffer data. Production applications should explicitly flush or shut down their providers during graceful shutdown when the integration exposes such an API.

Start with console logs and local spans. Add OpenTelemetry when the application communicates with other services or when cross-service latency and failures need to be investigated.

---

## 15. Production Guidelines

1. Initialize the subscriber once in the executable, not inside a library.
2. Prefer structured fields over putting every value in the message.
3. Use stable field names such as `request_id`, `user_id`, and `order_id`.
4. Create spans around meaningful units of work.
5. Do not record secrets or unnecessary personal information.
6. Avoid logging the same error at every level of the call stack.
7. Log an error where it is handled or where useful context is available.
8. Keep default production output around `INFO`; enable more detail for specific modules when diagnosing a problem.
9. Use `DEBUG` and `TRACE` carefully in loops and high-traffic paths.
10. Prefer JSON when machines consume the logs and human-readable text during local development.

### Avoid duplicate error logs

This can create the same failure three times:

```text
repository: database failed
service: could not load user
handler: request failed
```

Often it is better for lower layers to return a contextual error and for one appropriate boundary to record it:

```rust
match service.load_user(user_id).await {
    Ok(user) => handle_user(user),
    Err(err) => {
        tracing::error!(
            error = %err,
            user_id,
            "request failed while loading user"
        );
    }
}
```

---

## 16. Later Topic: Real Codebase Case Study from `codex-rs`

The repository inspected at `C:\Users\deepa\Projects\codex\codex-rs` demonstrates how the concepts above appear in a large Rust workspace. These examples are useful as evidence of the architecture, but they are not a template that a beginner must copy in full.

### Libraries emit; executable entry points assemble

Many workspace crates depend only on `tracing` and emit events. Executable crates such as the TUI and app server also depend on `tracing-subscriber` because entry points own the output policy.

This is the intended separation:

```text
core/library crates
    emit events and spans
           |
           v
TUI, CLI, or app-server entry point
    chooses files, stderr, filters, database, feedback, and OTLP layers
```

### The TUI composes several independent destinations

In `codex-rs/tui/src/startup_orchestration.rs`, startup creates a file layer with:

- A non-blocking writer and retained guard
- An `EnvFilter` with application-specific defaults
- Targets enabled so the source module remains visible
- ANSI disabled for the file
- `NEW | CLOSE` span lifecycle events
- A per-layer filter

The final registry composes that optional file layer with feedback, metadata, a local log database, an OpenTelemetry logger, and an OpenTelemetry tracing layer.

This is the practical meaning of `.with(...)`: one emitted event can be offered to several independent consumers, each with its own purpose and filter.

### The app server protects its protocol stream

In `codex-rs/app-server/src/lib.rs`, formatted logs are written to `stderr`, while configuration can select text or JSON. This matters because an app server may use `stdout` for structured protocol messages. Logging to the protocol channel would break clients.

The app server uses `FmtSpan::FULL`, showing that span lifecycle display is a subscriber policy. Other Codex entry points choose only `NEW | CLOSE`, demonstrating that there is no universal correct span-output setting.

### Request spans use stable semantic fields

In `codex-rs/app-server/src/app_server_tracing.rs`, a request span has a stable name such as `app_server.request` and fields including:

```text
rpc.system
rpc.method
rpc.transport
rpc.request_id
app_server.connection_id
app_server.client_name
app_server.client_version
```

Some fields begin as `field::Empty` and are recorded later when client or session information becomes available. This is more useful than encoding all context inside an English sentence.

The same code extracts incoming W3C trace context and assigns it as the request span's parent. That is the missing network step that turns isolated service spans into a distributed trace.

### Async work is explicitly instrumented

Across the app server, core, and exec server, futures and spawned tasks use patterns such as:

```rust
request_future.instrument(request_span).await;
tokio::spawn(task.instrument(task_span));
```

This confirms an important rule: the programmer deliberately associates independently running async work with the correct span.

### `skip_all` is common in large codebases

The Codex code frequently uses:

```rust
#[tracing::instrument(
    name = "model_client.stream_responses_api",
    level = "info",
    skip_all,
    fields(model = %model_name, transport = "responses_http")
)]
```

Large function arguments may contain clients, buffers, requests, credentials, or expensive `Debug` output. `skip_all` followed by an allowlist of safe fields is a strong production default.

Some internal functions also use `#[instrument(level = "trace", skip_all, err)]`: low-level spans stay disabled normally, but when `TRACE` is enabled they record failures automatically.

### Tests use scoped subscribers

Codex tests often construct a subscriber and retain the guard returned by `set_default`. This avoids repeatedly trying to install a process-global subscriber and lets a test capture only the events it needs.

### Security advisory correction

CVE-2025-58160 concerns untrusted ANSI escape sequences in terminal log output. The upstream advisory says versions **before 0.3.20** are affected and **0.3.20** is the patched version. The claim that 0.3.23 is the first safe version is incorrect.

The inspected Codex workspace declares and currently resolves `tracing-subscriber = "0.3.22"`, which is newer than the patched version. This learning project's `Cargo.toml` declares the compatible range `"0.3"` and its current `Cargo.lock` resolves version `0.3.23`. `Cargo.lock` determines the exact version used in a build, and `cargo audit` is the appropriate tool for checking known RustSec advisories.

---

## 17. Recommended Learning Order

Learn and practise these topics in this order:

1. Events and the five levels
2. Structured fields
3. `Display` (`%`) and `Debug` (`?`) recording
4. Spans and parent-child context
5. `#[instrument]`
6. `EnvFilter` and module-specific filtering
7. Subscriber layers
8. Async `.instrument(...)` and spawned-task context
9. Output destinations and non-blocking writers
10. JSON or another production transport
11. OpenTelemetry and distributed tracing

The main idea to remember is:

> Application code uses `tracing` to describe what happened and within which operation. `tracing-subscriber` decides what to keep, how to format it, and where to send it.

---

## Official References

- [`tracing` crate documentation](https://docs.rs/tracing/latest/tracing/)
- [`#[instrument]` documentation](https://docs.rs/tracing/latest/tracing/attr.instrument.html)
- [`tracing-subscriber` documentation](https://docs.rs/tracing-subscriber/latest/tracing_subscriber/)
- [`EnvFilter` documentation](https://docs.rs/tracing-subscriber/latest/tracing_subscriber/filter/struct.EnvFilter.html)
- [`fmt` subscriber documentation](https://docs.rs/tracing-subscriber/latest/tracing_subscriber/fmt/)
- [`tracing` and `log` compatibility](https://docs.rs/tracing/latest/tracing/#log-compatibility)
- [`FmtSpan` lifecycle-event documentation](https://docs.rs/tracing-subscriber/latest/tracing_subscriber/fmt/format/struct.FmtSpan.html)
- [`tracing-appender` non-blocking writer](https://docs.rs/tracing-appender/latest/tracing_appender/non_blocking/)
- [CVE-2025-58160 upstream advisory](https://github.com/tokio-rs/tracing/security/advisories/GHSA-xwfj-jgwm-7wp5)
