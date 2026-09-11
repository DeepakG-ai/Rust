// ============================================================================
// ## Q30 — Threads and channels (the sync world under Tokio)
//
// You jumped from sync code straight to `async` at Q23. This is the foundation
// `async` sits on — and `Arc<Mutex<T>>` from Q25 makes far more sense afterwards.
//
// Write three programs in one file:
//
// 1. Plain threads:
//    Spawn 5 threads with `std::thread::spawn`, each printing its id, then `join()`
//    them all. Show what happens if you forget to `join`.
//
// 2. Shared counter:
//    10 threads each incrementing an `Arc<Mutex<i32>>` 1000 times. Print the final
//    value — it must be exactly 10000. Then try the same with `Arc<AtomicUsize>`
//    and `fetch_add`.
//
// 3. Channels:
//    3 producer threads sending 10 messages each over `std::sync::mpsc::channel()`,
//    with `main` collecting all 30.
//
// Questions to answer in a comment:
// 1. Why does the receiving `for` loop end on its own when the producers finish?
// 2. What happens if you keep the original `Sender` alive in `main` and never drop it?
//
// Hint:
// The loop ends when ALL senders are dropped. Each thread gets a clone;
// the original in `main` must be dropped too, or the loop hangs forever.
// ============================================================================

use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::thread;

fn part1_plain_threads() {
    todo!("Spawn 5 threads, print id, join them all")
}

fn part2_shared_counter() {
    todo!("10 threads incrementing Arc<Mutex<i32>> 1000 times (and with AtomicUsize)")
}

fn part3_channels() {
    todo!("3 producer threads sending 10 messages each over mpsc::channel(), main collects 30")
}

fn main() {
    println!("--- Part 1: Plain Threads ---");
    part1_plain_threads();

    println!("\n--- Part 2: Shared Counter ---");
    part2_shared_counter();

    println!("\n--- Part 3: Channels ---");
    part3_channels();
}
