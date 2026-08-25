# Concurrency, Async, and Tokio — From Scratch

A complete guide to understanding concurrency in Rust using a kitchen analogy.

---

## Part 1 — The Kitchen Analogy

### Synchronous (Sync)

**One chef, one dish at a time.**

The chef starts making biryani. He will NOT touch anything else until the biryani is fully done — even if the rice is boiling and he's just standing there watching it. Only after the biryani is plated does he start cutting vegetables for the next dish.

```
Chef: [make biryani....................] [cut vegetables......] [make juice....]
                                         ↑ starts only after biryani is DONE
```

**In code:**
```rust
fn main() {
    let biryani = make_biryani();       // blocks until done
    let salad = cut_vegetables();       // starts only after biryani finishes
    let juice = make_juice();           // starts only after salad finishes
}
```

---

### Asynchronous (Async)

**One chef, switching between dishes while WAITING.**

The chef puts rice on the stove to boil (takes 20 minutes). Instead of standing and watching, he starts cutting vegetables for the next dish. When the rice timer rings, he goes back and finishes the biryani.

The key word is **"while waiting"** — the chef switches tasks only during idle/waiting time.

```
Chef: [put rice on stove] → [cut vegetables] → [rice done! finish biryani] → [make juice]
       ↑ waiting for rice    ↑ uses wait time    ↑ comes back
```

**In code (Rust + Tokio):**
```rust
async fn make_biryani() -> String {
    let rice = boil_rice().await;    // ← chef yields here, does other work
    mix_with_masala(rice)            // ← comes back when rice is ready
}
```

---

### Multithreading

**Multiple chefs in the SAME kitchen (shared fridge, stove, countertop).**

Chef A makes biryani, Chef B makes salad, Chef C makes juice — all at the same time, sharing the same kitchen resources. They need to coordinate so they don't grab the same knife or bump into each other.

```
Chef A: [make biryani..........]
Chef B: [cut vegetables........]    ← all working simultaneously
Chef C: [make juice............]
```

**In code (Rust):**
```rust
use std::thread;

fn main() {
    let t1 = thread::spawn(|| make_biryani());
    let t2 = thread::spawn(|| cut_vegetables());
    let t3 = thread::spawn(|| make_juice());

    t1.join().unwrap();
    t2.join().unwrap();
    t3.join().unwrap();
}
```

**"Shared kitchen"** = shared memory. This is why Rust has `Arc<Mutex<T>>` — it's like putting a lock on the fridge so two chefs don't open it at the same time.

---

### Multiprocessing

**Separate departments — each has its OWN kitchen.**

- **Starter department**: Has its own kitchen, stove, fridge. Chefs here only make starters (fry, kebab, etc.)
- **Juice counter**: Has its own blender, fruits, fridge. Chef here only makes juice.
- **Dinner department**: Has its own kitchen. Makes biryani, curries, etc.

Each department is a **separate process** with its **own memory**. The starter chef cannot reach into the juice counter's fridge — they must send messages (like a waiter carrying orders between departments).

```
┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐
│ Starter Dept     │    │ Juice Counter    │    │ Dinner Dept      │
│ (own kitchen)    │    │ (own kitchen)    │    │ (own kitchen)    │
│                  │    │                  │    │                  │
│ Chef A: fry      │    │ Chef C: juice    │    │ Chef D: biryani  │
│ Chef B: kebab    │    │                  │    │ Chef E: curry    │
└─────────────────┘    └─────────────────┘    └─────────────────┘
      ↑                       ↑                       ↑
  Process 1               Process 2               Process 3
  (separate memory)       (separate memory)       (separate memory)
```

Each department (process) can have multiple chefs (threads), and each chef can do async or sync work — their choice.

---

### Summary Table

| Concept | Kitchen Analogy | Technical Meaning |
|:---|:---|:---|
| **Sync** | One chef, one dish at a time | Code runs sequentially, waits for each operation |
| **Async** | One chef, switches dishes while waiting | One thread, yields at `.await` points during I/O waits |
| **Multithreading** | Multiple chefs, same kitchen (shared fridge) | Multiple OS threads, shared memory space |
| **Multiprocessing** | Separate departments, each with own kitchen | Multiple OS processes, separate memory spaces |

---

## Part 2 — Concurrency vs Parallelism

These two words sound similar but mean different things:

**Concurrency** = multiple tasks **in progress** at the same time (not necessarily running at the exact same instant).

**Parallelism** = multiple tasks **executing** at the exact same instant (needs multiple CPU cores).

```
Concurrency:  One chef juggling 3 dishes (only his hands touch one dish at a time)
Parallelism:  Three chefs, each working on a dish at the exact same moment

Concurrency is about STRUCTURE (managing multiple tasks)
Parallelism is about EXECUTION (doing multiple things simultaneously)
```

### How do they relate?

```
Concurrency (the GOAL: manage multiple tasks)
  ├── Async/Await       (1 chef, switches dishes while waiting)
  ├── Multithreading    (multiple chefs, same kitchen)
  └── Multiprocessing   (multiple kitchens)
```

- **Async** is a *form of* concurrency (1 thread, many tasks)
- **Multithreading** can give you *parallelism* (tasks run on different CPU cores at the exact same instant)
- You can have concurrency WITHOUT parallelism (1 chef, 3 dishes — only 1 dish touched at a time)
- You can have parallelism WITHOUT concurrency thinking (3 chefs, each doing 1 completely isolated task)

---

## Part 3 — What Does the Thread Do During `.await`?

This is the most important question for understanding async.

### The Setup

```rust
#[tokio::main]
async fn main() {
    let handle1 = tokio::spawn(get_data("users"));     // Task A
    let handle2 = tokio::spawn(get_data("orders"));     // Task B  
    let handle3 = tokio::spawn(addition(5, 3));         // Task C
}

async fn get_data(table: &str) -> String {
    // Sends query to database, then WAITS for response
    db.query("SELECT * FROM ...").await   // ← network I/O wait
}

fn addition(a: i32, b: i32) -> i32 {
    a + b   // ← pure CPU work, no waiting, no async needed
}
```

### Timeline on ONE Thread

```
Thread-1 (Tokio worker):

Time 0ms:     Picks up Task A (get_data "users")
              → Sends SQL query over network
              → Hits .await
              → Task A is PARKED (waiting for network response)
              → Thread is FREE now!

Time 0.1ms:   Thread picks up Task B (get_data "orders") 
              → Sends SQL query over network
              → Hits .await  
              → Task B is PARKED
              → Thread is FREE now!

Time 0.2ms:   Thread picks up Task C (addition)
              → Computes 5 + 3 = 8
              → Task C is DONE

Time 0.3ms:   Thread has nothing to do → SLEEPS
              (efficient, no CPU wasted — OS-level event wait)

Time 50ms:    OS signals: "Database response arrived for Task A!"
              → Thread wakes up, picks up Task A
              → Task A processes the response, finishes

Time 80ms:    OS signals: "Database response arrived for Task B!"
              → Thread picks up Task B, finishes it
```

### Answer: What "other work" does the thread do?

**Whatever other spawned tasks are ready to run.** Specifically:

1. **Other spawned tasks** — If Task B and Task C are pending, the thread runs those
2. **New incoming connections** — In a web server, it can accept new HTTP requests
3. **Nothing (sleeps efficiently)** — If no tasks are ready, the thread sleeps using OS-level event notification (zero CPU usage), until the OS says "hey, data arrived on this socket"

---

### Why `addition()` Doesn't Need Async

```rust
fn addition(a: i32, b: i32) -> i32 {
    a + b  // Takes ~1 nanosecond. No waiting. No I/O.
}
```

This has **no reason to be async** because:
- It never **waits** for anything external (network, disk, timer)
- It finishes in nanoseconds
- There is no "gap" where the thread could go do something else

**Rule: Use `async` only when the function WAITS for something external:**
- Network call (database, HTTP API)
- File read/write
- Timer/sleep
- Channel receive

```
fn addition(a, b) → Pure CPU math     → sync is fine
fn get_data(url)  → Waits for network → async makes sense
```

**One-liner summary:**

> **`.await` = "I'm waiting for something external. Dear thread, go serve other tasks. Come back to me when my data arrives."**

---

## Part 4 — What Happens When a Response Arrives Mid-Task?

### Cooperative Scheduling

Tokio uses **cooperative scheduling**, not preemptive. That means:

> **A task is NEVER interrupted mid-execution.** It runs until it hits an `.await` point or completes.

### Scenario: Task A response arrives while thread is busy with Task C

```rust
// Task C — pure CPU work
async fn addition(a: i32, b: i32) -> i32 {
    a + b   // no .await anywhere, finishes in one go
}
```

```
Time 0.2ms:  Thread starts Task C (addition)
Time 0.2ms:  While Task C runs, OS says "Task A data arrived!"
             → OS puts a notification in a queue
             → Thread does NOT stop Task C
             → Task C keeps running (a + b = 8)
Time 0.2ms:  Task C finishes (takes nanoseconds)
Time 0.2ms:  Thread checks the queue → "Oh, Task A is ready!"
             → Picks up Task A, processes the database response
```

**For small CPU work like `addition`:** No problem — it finishes in nanoseconds, Task A barely waits.

### But What If Task C Was Heavy CPU Work?

```rust
async fn heavy_computation() -> u64 {
    let mut sum = 0;
    for i in 0..10_000_000_000 {  // Takes 30 seconds! No .await!
        sum += i;
    }
    sum
}
```

```
Time 0ms:       Thread starts heavy_computation
Time 50ms:      Task A response arrives from database
                → Thread is STUCK in the for loop
                → Task A CANNOT be picked up!
Time 30,000ms:  heavy_computation finally finishes
Time 30,000ms:  NOW thread picks up Task A (30 seconds late!)
```

**This is bad!** Task A waited 30 seconds unnecessarily. This is called **"blocking the runtime"**.

### The Fix: `spawn_blocking`

```rust
// Move heavy CPU work to a separate blocking thread pool
let result = tokio::task::spawn_blocking(|| {
    // This runs on a SEPARATE thread pool
    // so it won't block the async runtime
    let mut sum = 0u64;
    for i in 0..10_000_000_000 {
        sum += i;
    }
    sum
}).await.unwrap();
```

### The Golden Rule

```
.await = "yield point" — the place where the thread CAN switch to another task

No .await = thread is locked on this task, nobody else runs
```

Summary:

```
Thread sees task ready → runs it until:
  ├── Task hits .await     → PAUSE task, pick up next ready task
  ├── Task completes       → DONE, pick up next ready task  
  └── Task does heavy CPU  → STUCK! Other tasks must wait ⚠️
                              (use spawn_blocking to fix this)
```

---

## Part 5 — Yield: `.await` in Rust vs `yield` in Python

Python's `yield` and Rust's `.await` are the **same concept** — both pause execution and hand control back to a scheduler.

### Python Generator — `yield`

```python
# Python generator — yield pauses and resumes
def counter():
    x = 1
    yield x      # pause here, remember x=1
    x += 1
    yield x      # pause here, remember x=2
```

### Rust Async — `.await`

```rust
// Rust async — .await pauses and resumes
async fn get_data() -> String {
    let query = build_query();
    let resp = db.send(query).await;  // pause here, remember query
    parse(resp)                        // resume here
}
```

Same idea: **pause, save state, resume later.**

### Every `.await` is a yield point

```rust
async fn get_data(url: &str) -> String {
    //         work          yield          work
    //        ──────    ──────────────    ──────────
    let query = build_query(url);      // ← thread runs this
    let resp = db.send(query).await;   // ← YIELD! thread goes elsewhere
    parse(resp)                        // ← thread comes back, runs this
}
```

```
Thread:  [runs build_query] → .await → [goes to serve Task B] → [comes back] → [runs parse]
```

### No `.await` = No Yield = No Switching

```rust
async fn no_yield() -> u64 {
    let mut x = 0;
    for i in 0..1_000_000 {
        x += i;            // no .await anywhere
    }
    x  // thread was locked here the entire time
}
```

---

## Part 6 — How Does It Remember? (The State Machine)

When a task yields at `.await`, the thread goes off to serve other tasks. But later it comes back. **How does it know where it left off?**

### The Compiler Transforms `async fn` Into a Struct

When you write an `async fn`, the Rust **compiler** automatically transforms it into a **struct** (called a `Future`) that holds all the local variables as fields:

```rust
// What YOU write:
async fn get_data(url: String) -> String {
    let query = build_query(&url);       // step 1
    let resp = db.send(query).await;     // YIELD (step boundary)
    let result = parse(resp);            // step 2
    result
}

// What the COMPILER generates (simplified):
struct GetDataFuture {
    state: u8,              // which step are we on? (0, 1, 2)
    url: String,            // saved local variable
    query: Option<Query>,   // saved local variable  
    resp: Option<Response>, // saved local variable
}
```

When the task yields:
1. All local variables are **stored inside the Future struct** on the heap
2. The `state` field records which `.await` point we're at
3. The struct just sits in memory, waiting

When the task resumes:
1. The runtime reads the `state` field → knows exactly which step to continue from
2. All local variables are still there in the struct fields
3. Execution continues right after the `.await`

---

## Part 7 — The Queue: How Tokio Manages Tasks

Tokio uses two key data structures:

```
┌──────────────────────────────────────────────────┐
│                 Tokio Runtime                     │
│                                                   │
│  ┌───────────────┐       ┌───────────────┐       │
│  │  Run Queue     │       │  Wait List     │       │
│  │  (ready to     │       │  (parked,      │       │
│  │   run NOW)     │       │   waiting for  │       │
│  │                │       │   I/O or timer) │       │
│  │  → Task C      │       │                │       │
│  │  → Task D      │       │  Task A         │       │
│  │                │       │   (waiting for  │       │
│  │                │       │    DB response) │       │
│  │                │       │                │       │
│  │                │       │  Task B         │       │
│  │                │       │   (waiting for  │       │
│  │                │       │    HTTP reply)  │       │
│  └───────────────┘       └───────────────┘       │
│          ↑                        │               │
│          │     OS says "data      │               │
│          │     arrived on this    │               │
│          │     socket!"           │               │
│          └────────────────────────┘               │
│               move to run queue                   │
│                                                   │
│  Worker Thread: picks from Run Queue              │
│  and executes the next ready task                 │
└──────────────────────────────────────────────────┘
```

### Step-by-Step Flow

| Step | Event | What Happens |
|:---|:---|:---|
| 1 | `tokio::spawn(task_a)` | Task A goes into the **Run Queue** |
| 2 | Thread picks up Task A | Runs it until it hits `.await` |
| 3 | Task A hits `.await` (DB call) | Task A moves to **Wait List**. Its Future struct is saved in memory |
| 4 | Thread picks next from Run Queue | Runs Task C |
| 5 | OS signals "DB response arrived" | Task A moves from Wait List → **Run Queue** |
| 6 | Thread finishes Task C | Picks up Task A from Run Queue, resumes from where it paused |

### OS Event Notification (How the OS Tells Tokio "Data Arrived")

When a task is waiting for network I/O, Tokio registers the socket with the OS kernel:

| Operating System | Mechanism |
|:---|:---|
| **Windows** | IOCP (I/O Completion Ports) |
| **Linux** | `epoll` |
| **macOS** | `kqueue` |

The OS efficiently monitors thousands of sockets simultaneously. When data arrives on any socket, the OS notifies Tokio, which moves the corresponding task from the Wait List to the Run Queue.

---

## Part 8 — Memory Cost Comparison

| Approach | Memory Per Waiting Task | Max Concurrent Tasks |
|:---|:---|:---|
| **OS Thread** (`std::thread::spawn`) | ~2–8 MB (full stack) | ~1,000–10,000 |
| **Python Coroutine** (`asyncio`) | ~1–2 KB (heap frame) | ~100,000+ |
| **Rust Future / Tokio Task** (`tokio::spawn`) | ~few hundred bytes (compiler-optimized struct) | ~1,000,000+ |

That's why Tokio can handle **millions** of concurrent connections — each paused task is just a tiny struct sitting in memory, not a full thread stack.

---

## Part 9 — `tokio::spawn` vs `std::thread::spawn`

```rust
tokio::spawn(...)       // → creates a lightweight Tokio Task (green thread)
std::thread::spawn(...)  // → creates a heavy OS Thread
```

| Feature | `tokio::spawn` | `std::thread::spawn` |
|:---|:---|:---|
| **Weight** | ~few hundred bytes | ~2–8 MB stack |
| **Scheduling** | Cooperative (yields at `.await`) | Preemptive (OS can interrupt anytime) |
| **Concurrency model** | Many tasks on few threads | One thread per task |
| **Best for** | I/O-bound work (network, DB, files) | CPU-bound work (math, encryption) |
| **Count** | Millions possible | Thousands max |

### Tokio's Worker Thread Pool

Even though you `tokio::spawn` many tasks, Tokio typically uses only **2–4 OS worker threads** (defaults to number of CPU cores). All your tasks are multiplexed across these few threads.

```
                    Tokio Runtime
┌────────────────────────────────────────────┐
│                                             │
│  Worker Thread 1:  Task A → Task D → Task G│
│  Worker Thread 2:  Task B → Task E → Task H│
│  Worker Thread 3:  Task C → Task F → Task I│
│  Worker Thread 4:  (idle, sleeping)         │
│                                             │
│  Total tasks: 1,000,000                     │
│  Total OS threads: 4                        │
└────────────────────────────────────────────┘
```

---

## Part 10 — Complete Example: Q23 from task.md

```rust
use std::time::Instant;

// This function simulates an I/O-bound operation (like a DB call)
// It sleeps asynchronously — during the sleep, the thread is FREE
async fn fetch_user_data(user_id: u32, delay_ms: u64) -> String {
    tokio::time::sleep(tokio::time::Duration::from_millis(delay_ms)).await;
    // ↑ YIELD POINT: thread goes to serve other tasks
    // ↓ When timer fires, thread comes back and runs this line
    format!("User {user_id} data loaded")
}

#[tokio::main]
async fn main() {
    let start = Instant::now();

    // All 3 tasks are spawned nearly instantly (at ~0ms)
    // They all start sleeping concurrently
    let handle1 = tokio::spawn(fetch_user_data(1, 300));  // sleeps 300ms
    let handle2 = tokio::spawn(fetch_user_data(2, 100));  // sleeps 100ms
    let handle3 = tokio::spawn(fetch_user_data(3, 200));  // sleeps 200ms

    // .await on JoinHandle — waits for each task to complete
    let result1 = handle1.await.unwrap();  // by 300ms, already done
    let result2 = handle2.await.unwrap();  // finished at 100ms, already done
    let result3 = handle3.await.unwrap();  // finished at 200ms, already done

    let elapsed = start.elapsed();

    println!("{result1}");
    println!("{result2}");
    println!("{result3}");
    println!("Total elapsed: {elapsed:.2?}");
    // Output: ~300ms (NOT 600ms) — because all 3 ran concurrently!
}
```

### Timeline Visualization

```
0ms         100ms       200ms       300ms
|___________|___________|___________|
|                                   |
|  Task 1: sleeping 300ms...........✓  "User 1 data loaded"
|  Task 2: sleeping 100ms..✓          "User 2 data loaded"  
|  Task 3: sleeping 200ms......✓       "User 3 data loaded"
|                                   |
start                            elapsed ≈ 300ms
```

All 3 tasks overlap — total time is **~300ms** (the longest single task), not 300+100+200 = 600ms.

---

## Quick Reference Cheat Sheet

```
┌─────────────────────────────────────────────────────────────┐
│                     WHEN TO USE WHAT                         │
├─────────────────────────────────────────────────────────────┤
│                                                              │
│  fn addition(a, b) → a + b                                  │
│  ├── Pure CPU math, finishes instantly                       │
│  └── Use: regular sync function ✅                           │
│                                                              │
│  async fn get_data(url) → db.query().await                  │
│  ├── Waits for external I/O (network, disk, timer)          │
│  └── Use: async function with .await ✅                      │
│                                                              │
│  async fn heavy_math() → for i in 0..10_billion { ... }     │
│  ├── Heavy CPU work, no .await, blocks the runtime ❌        │
│  └── Use: tokio::task::spawn_blocking() ✅                   │
│                                                              │
├─────────────────────────────────────────────────────────────┤
│                                                              │
│  .await    = yield point (thread CAN switch)                │
│  no .await = thread is locked (nobody else runs)            │
│                                                              │
│  tokio::spawn       = lightweight task (millions possible)  │
│  std::thread::spawn = heavy OS thread (thousands max)       │
│                                                              │
│  Concurrency = the GOAL (manage multiple tasks)             │
│  Async       = one TECHNIQUE (to achieve that goal)         │
│  Parallelism = simultaneous EXECUTION (needs multiple cores)│
│                                                              │
└─────────────────────────────────────────────────────────────┘
```
