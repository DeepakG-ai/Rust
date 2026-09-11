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
// - `RUST_LOG=info cargo run --bin q33`
// - `RUST_LOG=debug cargo run --bin q33`
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

#[instrument]
async fn process_job(job_id: u32) {
    info!(job_id, "starting job processing");
    // simulate work
    debug!(job_id, "fetching remote resource");
    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
    info!(job_id, "job completed successfully");
}

#[tokio::main]
async fn main() {
    todo!("Q33 implementation")
}
