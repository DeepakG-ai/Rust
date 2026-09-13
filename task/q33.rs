// ============================================================================
// ## Q33 — `tracing` (structured logging)
//
// `println!` does not survive contact with production. `tracing` is what real Rust
// services use — and unlike `log`, it understands concurrency.
//
// Dependencies (already in Cargo.toml):
// - tracing = "0.1"
// - tracing-subscriber = { version = "0.3", features = ["env-filter"] }
//
// Tasks:
// 1. Initialise a subscriber in `main` that reads the `RUST_LOG` env var:
//    ```rust
//    tracing_subscriber::fmt()
//        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
//        .init();
//    ```
// 2. Replace prints with `info!`, `warn!`, `error!`, `debug!`.
// 3. Log structured fields, not formatted strings:
//    `info!(user_id = 42, attempt = 2, "retrying request");`
// 4. Put `#[tracing::instrument]` on a function and call it. Look at what appears.
// 5. Now make it concurrent: `tokio::spawn` three instrumented tasks at once and
//    look at the output again.
//
// Run it three ways and compare:
// - `cargo run --bin q33`
// - PowerShell: `$env:RUST_LOG = "info"` then `cargo run --bin q33`
// - PowerShell: `$env:RUST_LOG = "debug"` then `cargo run --bin q33`
// To clear the setting: `Remove-Item Env:RUST_LOG -ErrorAction SilentlyContinue`
//
// Question to answer in a comment:
// - With three tasks logging at once, how do you tell which line belongs to which task?
//   That is what a span gives you that a plain log line cannot.
//
// Hint:
// A span is a period of time with a name and fields; an event is a single moment.
// `#[instrument]` wraps the whole function in a span automatically.
// ============================================================================

use tracing::{debug, error, info, instrument, warn};
use tracing_subscriber::EnvFilter;

#[instrument]
async fn process_job(job_id: u32) {
    // #[instrument] creates process_job{job_id=...} for each call.
    // Events already carry this context, so we need not repeat job_id.
    info!("starting job processing");
    debug!(resource = "demo-service", "fetching remote resource");

    // Simulated I/O: other jobs can run while this task waits.
    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

    if job_id == 2 {
        warn!(attempt = 1, "simulated timeout; retrying request");
        tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;
        debug!(attempt = 2, "retry succeeded");
    }

    if job_id == 3 {
        error!(reason = "invalid_payload", "simulated job failure");
        return;
    }

    info!("job completed successfully");
}

#[tokio::main]
async fn main() {
    // Use the exact filter requested by Q33:
    // unset RUST_LOG -> ERROR; info -> INFO/WARN/ERROR; debug -> all but TRACE.
    let filter = EnvFilter::from_default_env();

    tracing_subscriber::fmt() // Build a subscriber that formats text logs.
        .with_env_filter(filter) // Choose which levels are enabled.
        .init(); // Install it once before emitting events.

    info!("running one job first");
    process_job(0).await;

    info!(job_count = 3, "starting concurrent jobs");
    let mut handles = Vec::new();
    for job_id in 1..=3 {
        handles.push(tokio::spawn(process_job(job_id)));
    }

    // All three tasks have already been spawned. Awaiting these handles
    // waits for completion; it does not make the jobs run sequentially.
    for handle in handles {
        if let Err(err) = handle.await {
            error!(error = %err, "task panicked or was cancelled");
        }
    }

    info!("all tasks finished");

    // Answer: look at process_job{job_id=1}, process_job{job_id=2}, etc.
    // These span fields identify which job produced each interleaved event.
    // The async #[instrument] attribute enters the right span on each poll.
    // At ERROR-only filtering, the INFO-level spans are disabled too, so
    // enable INFO or DEBUG to see that job context in this exercise.
}
