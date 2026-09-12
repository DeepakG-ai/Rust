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

/// Part 1: Spawning OS threads and understanding join()
fn part1_plain_threads() {
    println!("--- Spawning 5 threads and joining them ---");
    let mut handles = Vec::new();

    for i in 0..5 {
        let handle = thread::spawn(move || {
            let current_id = thread::current().id();
            println!("  Thread index {i} running on OS thread id: {current_id:?}");
        });

        handles.push(handle);
    }

    // join() waits for each thread to finish.
    // unwrap() propagates any panic if a thread panicked.
    for handle in handles {
        handle.join().unwrap();
    }
    println!("All 5 threads joined successfully.\n");

    // --- What happens if you forget to join? ---
    // If you do NOT join, the thread is "detached". When `main` finishes,
    // the entire process exits immediately and abruptly terminates all
    // background threads, whether they completed their work or not!
    let _unjoined_handle = thread::spawn(|| {
        // Without join(), this thread might not even get CPU time before main exits!
        println!("  (Detached unjoined thread executed)");
    });
}

/// Part 2: Shared counter with Mutex vs Atomic
fn part2_shared_counter() {
    // -------------------------------------------------------------
    // Part 2a: Arc<Mutex<i32>>
    // -------------------------------------------------------------
    println!("--- Part 2a: Shared counter with Arc<Mutex<i32>> ---");
    let counter = Arc::new(Mutex::new(0));
    let mut mutex_handles = Vec::new();

    for _ in 0..10 {
        let counter_clone = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            for _ in 0..1000 {
                let mut guard = counter_clone.lock().unwrap();
                *guard += 1;
            }
        });
        mutex_handles.push(handle);
    }

    for handle in mutex_handles {
        handle.join().unwrap();
    }

    let final_mutex_val = *counter.lock().unwrap();
    println!("Final Mutex counter value: {final_mutex_val}");
    assert_eq!(final_mutex_val, 10000, "Mutex counter must be exactly 10,000");

    // -------------------------------------------------------------
    // Part 2b: Arc<AtomicUsize> and fetch_add
    // -------------------------------------------------------------
    println!("\n--- Part 2b: Shared counter with Arc<AtomicUsize> ---");
    let atomic_counter = Arc::new(AtomicUsize::new(0));
    let mut atomic_handles = Vec::new();

    for _ in 0..10 {
        let atomic_clone = Arc::clone(&atomic_counter);
        let handle = thread::spawn(move || {
            for _ in 0..1000 {
                atomic_clone.fetch_add(1, Ordering::SeqCst);
            }
        });
        atomic_handles.push(handle);
    }

    for handle in atomic_handles {
        handle.join().unwrap();
    }

    let final_atomic_val = atomic_counter.load(Ordering::SeqCst);
    println!("Final Atomic counter value: {final_atomic_val}");
    assert_eq!(final_atomic_val, 10000, "Atomic counter must be exactly 10,000");
}

/// Part 3: Channels with std::sync::mpsc
fn part3_channels() {
    let (tx, rx) = mpsc::channel();
    let mut handles = Vec::new();

    // 3 producer threads, sending 10 messages each
    for producer_id in 0..3 {
        let tx_clone = tx.clone();
        let handle = thread::spawn(move || {
            for msg_idx in 0..10 {
                let message = format!("Producer {producer_id} -> Message {msg_idx}");
                tx_clone.send(message).unwrap();
            }
            // tx_clone is dropped here when the thread closure ends
        });
        handles.push(handle);
    }

    // CRITICAL: Drop the original sender in main!
    // Each producer thread holds a clone of `tx`. If the original `tx` in main
    // is kept alive, the channel will never see all senders close, causing
    // the receiver loop below to hang forever waiting for more messages.
    drop(tx);

    let mut total_received = 0;
    // rx implements IntoIterator: it receives until ALL senders are dropped
    for msg in rx {
        total_received += 1;
        println!("  [{total_received:02}/30] Received: {msg}");
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Total messages collected in main: {total_received}");
    assert_eq!(total_received, 30, "Main must collect exactly 30 messages!");
}

fn main() {
    println!("=== Part 1: Plain Threads ===");
    part1_plain_threads();

    println!("\n=== Part 2: Shared Counter ===");
    part2_shared_counter();

    println!("\n=== Part 3: Channels ===");
    part3_channels();

    println!("\nAll Q30 tests passed successfully!");
}

// ============================================================================
// Q30 Questions Answered:
//
// 1. Why does the receiving `for` loop end on its own when the producers finish?
//
//    - In Rust, `std::sync::mpsc::Receiver` implements `IntoIterator`.
//    - The `for msg in rx` loop repeatedly calls `rx.recv()` under the hood.
//    - `rx.recv()` returns `Err(RecvError)` ONLY when all active `Sender` instances
//      connected to the channel have been dropped.
//    - When the 3 producer threads finish, each of their cloned senders (`tx_clone`)
//      is dropped.
//    - Because we also explicitly dropped the original `tx` in `main` via `drop(tx)`,
//      the total count of active senders drops to zero.
//    - When zero senders remain, the channel is officially disconnected; `rx.recv()`
//      returns `Err`, the iterator yields `None`, and the `for` loop cleanly terminates.
//
// 2. What happens if you keep the original `Sender` alive in `main` and never drop it?
//
//    - If the original `tx` in `main` is NOT dropped, the channel's sender reference count
//      never reaches zero, because `tx` is still alive in the `main` stack frame.
//    - `rx` has no way of knowing whether `main` intends to send another message later.
//    - Therefore, after receiving all 30 messages from the worker threads, the `for msg in rx`
//      loop will block (hang) indefinitely waiting for message #31 that never arrives.
//    - The program deadlocks and never terminates.
// ============================================================================
