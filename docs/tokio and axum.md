# Tokio and Axum Notes for Rust Learners

This document explains Tokio and Axum in simple terms, especially for developers coming from Python.

---

## Table of Contents

1. Quick Setup
2. `async fn`, `Future`, and `.await`
3. `#[tokio::main]`
4. `tokio::spawn` and `JoinHandle`
5. `mpsc` and `oneshot` channels
6. `tokio::join!` and `tokio::select!`
7. `Arc`, `Mutex`, `Send`, and `'static`
8. Async networking and subprocesses
9. Cancellation and graceful shutdown
10. Axum after Tokio fundamentals

---

## 1. Quick Setup

Create a new Rust project:

```bash
cargo new tokio_axum_demo
cd tokio_axum_demo
```

Add dependencies:

```bash
cargo add tokio --features full
cargo add axum
cargo add serde --features derive
cargo add tokio-util --features rt
```

Your `Cargo.toml` should look roughly like this:

```toml
[package]
name = "tokio_axum_demo"
version = "0.1.0"
edition = "2021"

[dependencies]
tokio = { version = "1", features = ["full"] }
axum = "0.7"
serde = { version = "1", features = ["derive"] }
tokio-util = "0.7"
```

Notes:

- `tokio` is the async runtime.
- `axum` is the web framework.
- `serde` is used for JSON serialization/deserialization.
- `tokio-util` gives useful helpers like `CancellationToken`.

---

## 2. `async fn`, `Future`, and `.await`

In Rust, an `async fn` does not immediately run when you call it.

Instead, it returns a `Future`.

A `Future` represents a value that may not be available yet.

Simple idea:

> A `Future` is like a promise that work will happen later.

Example:

```rust
async fn say_hello() -> String {
    String::from("hello")
}
```

Calling this function does not run it yet:

```rust
let fut = say_hello();
```

The variable `fut` is a future.

To actually run it, you need `.await`:

```rust
let result = say_hello().await;
println!("{result}");
```

Important point:

> `async fn` returns a future. `.await` drives that future to completion.

Python analogy:

```python
import asyncio

async def say_hello():
    return "hello"

async def main():
    result = await say_hello()
    print(result)

asyncio.run(main())
```

Rust version:

```rust
#[tokio::main]
async fn main() {
    let result = say_hello().await;
    println!("{result}");
}

async fn say_hello() -> String {
    String::from("hello")
}
```

Common beginner mistake:

```rust
async fn do_work() {
    println!("working");
}

fn main() {
    do_work(); // This does nothing by itself.
}
```

The future is created, but never awaited.

You need a runtime and `.await`.

---

## 3. `#[tokio::main]`

Tokio provides a macro called `#[tokio::main]`.

It turns your async `main` function into a normal `main` function that starts the Tokio runtime.

Example:

```rust
#[tokio::main]
async fn main() {
    println!("Hello from Tokio");
}
```

Conceptually, this is similar to:

```rust
fn main() {
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        println!("Hello from Tokio");
    });
}
```

So:

> `#[tokio::main]` sets up the async runtime for you.

Python analogy:

```python
asyncio.run(main())
```

Rust:

```rust
#[tokio::main]
async fn main() {
    // async code here
}
```

Without Tokio, most async code cannot actually run.

---

## 4. `tokio::spawn` and `JoinHandle`

`tokio::spawn` lets you run an async task in the background.

Example:

```rust
#[tokio::main]
async fn main() {
    let handle = tokio::spawn(async {
        println!("inside spawned task");
        42
    });

    let result = handle.await.unwrap();
    println!("task returned: {result}");
}
```

When you call `tokio::spawn`, Tokio gives you a `JoinHandle`.

The `JoinHandle` lets you wait for the spawned task to finish.

Example:

```rust
let handle = tokio::spawn(async {
    1 + 1
});

let result = handle.await.unwrap();
println!("{result}");
```

Important details:

- `tokio::spawn` puts the task onto the Tokio runtime.
- The task can run concurrently with other tasks.
- `handle.await` waits for the task to finish.
- `handle.await` returns a `Result`, because the task may panic.

Python analogy:

```python
import asyncio

async def worker():
    return 42

async def main():
    task = asyncio.create_task(worker())
    result = await task
    print(result)

asyncio.run(main())
```

Rust:

```rust
let task = tokio::spawn(async {
    42
});

let result = task.await.unwrap();
println!("{result}");
```

Important rule:

> The future passed to `tokio::spawn` usually needs to own its data.

This will usually not work:

```rust
let message = String::from("hello");

tokio::spawn(async {
    println!("{message}");
});
```

Because the spawned task borrows `message`, but the task may outlive it.

Instead, move ownership into the task:

```rust
let message = String::from("hello");

tokio::spawn(async move {
    println!("{message}");
});
```

The `async move` block moves captured variables into the future.

---

## 5. `mpsc` and `oneshot` channels

Channels are used to send messages between async tasks.

Tokio provides several channel types.

Two common ones:

- `mpsc`: many producers, one consumer
- `oneshot`: one producer, one consumer, one message

---

### `mpsc` channel

`mpsc` means:

> Multiple producers, single consumer.

Example:

```rust
use tokio::sync::mpsc;

#[tokio::main]
async fn main() {
    let (tx, mut rx) = mpsc::channel::<i32>(10);

    tokio::spawn(async move {
        tx.send(1).await.unwrap();
        tx.send(2).await.unwrap();
        tx.send(3).await.unwrap();
    });

    while let Some(value) = rx.recv().await {
        println!("received: {value}");
    }
}
```

Explanation:

- `tx` is the sender.
- `rx` is the receiver.
- `channel::<i32>(10)` creates a bounded channel with capacity 10.
- `tx.send(...).await` sends a value.
- `rx.recv().await` receives values until all senders are dropped.

If you want multiple producers, clone the sender:

```rust
use tokio::sync::mpsc;

#[tokio::main]
async fn main() {
    let (tx, mut rx) = mpsc::channel::<i32>(10);

    let tx2 = tx.clone();

    tokio::spawn(async move {
        tx.send(1).await.unwrap();
    });

    tokio::spawn(async move {
        tx2.send(2).await.unwrap();
    });

    while let Some(value) = rx.recv().await {
        println!("received: {value}");
    }
}
```

---

### `oneshot` channel

A `oneshot` channel sends exactly one value from one task to another.

Example:

```rust
use tokio::sync::oneshot;

#[tokio::main]
async fn main() {
    let (tx, rx) = oneshot::channel::<String>();

    tokio::spawn(async move {
        tx.send(String::from("done")).unwrap();
    });

    let result = rx.await.unwrap();
    println!("received: {result}");
}
```

Explanation:

- `tx.send(...)` sends one value.
- `rx.await` waits for that value.
- Once the value is sent, the channel is finished.

Python-ish analogy:

- `mpsc` is like a queue where many workers can push items and one consumer reads them.
- `oneshot` is like waiting for a single result from one background task.

---

## 6. `tokio::join!` and `tokio::select!`

Tokio gives macros to run futures together.

The two most important ones are:

- `tokio::join!`
- `tokio::select!`

---

### `tokio::join!`

`tokio::join!` runs multiple futures concurrently and waits for all of them to finish.

Example:

```rust
async fn task_a() -> String {
    tokio::time::sleep(std::time::Duration::from_millis(50)).await;
    String::from("A")
}

async fn task_b() -> String {
    tokio::time::sleep(std::time::Duration::from_millis(50)).await;
    String::from("B")
}

#[tokio::main]
async fn main() {
    let (a, b) = tokio::join!(task_a(), task_b());

    println!("{a}");
    println!("{b}");
}
```

This waits for both futures.

Important idea:

> `join!` waits for all futures to complete.

Python analogy:

```python
await asyncio.gather(task_a(), task_b())
```

---

### `tokio::select!`

`tokio::select!` waits for multiple futures, but continues as soon as one of them finishes.

Example:

```rust
use std::time::Duration;

async fn slow_task() -> String {
    tokio::time::sleep(Duration::from_secs(2)).await;
    String::from("slow task finished")
}

async fn timeout_task() -> String {
    tokio::time::sleep(Duration::from_millis(500)).await;
    String::from("timeout happened")
}

#[tokio::main]
async fn main() {
    tokio::select! {
        result = slow_task() => {
            println!("slow: {result}");
        }
        result = timeout_task() => {
            println!("timeout: {result}");
        }
    }
}
```

Here, the timeout finishes first, so `select!` chooses that branch.

Important idea:

> `select!` waits for the first future to complete.

Python analogy:

```python
done, pending = await asyncio.wait(
    [task_a(), task_b()],
    return_when=asyncio.FIRST_COMPLETED,
)
```

Also important:

When one branch wins in `select!`, the other futures are dropped.

That is useful for cancellation and timeouts.

---

## 7. `Arc`, `Mutex`, `Send`, and `'static`

These are not Tokio-only concepts, but they are very common in async Rust.

---

### `Arc`

`Arc` means Atomic Reference Counted pointer.

It lets multiple tasks share ownership of the same data.

Example:

```rust
use std::sync::Arc;

#[tokio::main]
async fn main() {
    let value = Arc::new(String::from("shared"));

    let value2 = value.clone();

    tokio::spawn(async move {
        println!("{value2}");
    });

    println!("{value}");
}
```

Simple idea:

> `Arc` allows multiple owners of the same data.

If you have used Python, this is a bit like multiple references to the same object, but Rust needs explicit shared ownership for cross-task data.

---

### `Mutex`

A `Mutex` protects shared data so only one task can modify it at a time.

For async code, if you need to hold a lock across `.await`, use `tokio::sync::Mutex`.

Example:

```rust
use std::sync::Arc;
use tokio::sync::Mutex;

#[tokio::main]
async fn main() {
    let counter = Arc::new(Mutex::new(0));

    let mut handles = vec![];

    for _ in 0..5 {
        let counter = counter.clone();

        let handle = tokio::spawn(async move {
            let mut guard = counter.lock().await;
            *guard += 1;
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.await.unwrap();
    }

    let final_value = counter.lock().await;
    println!("final counter: {final_value}");
}
```

Explanation:

- `Arc` lets tasks share the counter.
- `Mutex` makes sure only one task updates it at a time.
- `counter.lock().await` gives access to the protected value.

Important note:

If the locked section is very short and does not cross `.await`, `std::sync::Mutex` can also be okay.

But for beginners, using `tokio::sync::Mutex` in async code is often safer.

---

### `Send`

`Send` is a trait that means:

> This value can be transferred between threads.

Tokio tasks may move between threads, so spawned futures usually need to be `Send`.

This is why some types cannot be safely used across `await` points.

Simple idea:

> `Send` means Rust trusts this value to move safely to another thread.

---

### `'static`

`'static` means:

> The value does not contain short-lived references to data owned elsewhere.

When you use `tokio::spawn`, the task may run for a long time, so it usually cannot borrow local data.

Bad idea:

```rust
let message = String::from("hello");

tokio::spawn(async {
    println!("{message}");
});
```

Better:

```rust
let message = String::from("hello");

tokio::spawn(async move {
    println!("{message}");
});
```

Or use `Arc` if sharing is needed.

Simple idea:

> Spawned tasks usually need to own their data.

---

## 8. Async networking and subprocesses

Tokio is very useful for networking.

It can handle:

- TCP servers
- TCP clients
- UDP
- async file I/O
- async subprocesses

---

### Simple async TCP server

This is a basic TCP echo server.

It receives data and sends the same data back.

```rust
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

async fn handle_connection(mut socket: TcpStream) {
    let mut buffer = [0u8; 1024];

    match socket.read(&mut buffer).await {
        Ok(bytes_read) => {
            let _ = socket.write_all(&buffer[..bytes_read]).await;
        }
        Err(e) => {
            eprintln!("read error: {e}");
        }
    }
}

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    println!("TCP server listening on 127.0.0.1:3000");

    loop {
        let (socket, _) = listener.accept().await.unwrap();

        tokio::spawn(async move {
            handle_connection(socket).await;
        });
    }
}
```

Explanation:

- `TcpListener::bind` listens on a port.
- `listener.accept().await` waits for a connection.
- Each connection is handled in a spawned task.
- `socket.read` reads bytes.
- `socket.write_all` sends bytes back.

This is low-level networking.

Axum builds on top of this kind of foundation.

---

### Async subprocess

Tokio can also run external commands asynchronously.

Example:

```rust
use tokio::process::Command;

#[tokio::main]
async fn main() {
    let output = Command::new("ls")
        .arg("-la")
        .output()
        .await
        .expect("failed to execute command");

    println!("stdout:");
    println!("{}", String::from_utf8_lossy(&output.stdout));
}
```

On Windows, you might use:

```rust
let output = Command::new("cmd")
    .arg("/C")
    .arg("dir")
    .output()
    .await
    .expect("failed to execute command");
```

Important idea:

> Tokio can run subprocesses without blocking the whole program.

---

## 9. Cancellation and graceful shutdown

In async Rust, tasks do not usually stop automatically.

You need to design cancellation yourself.

Common tools:

- `tokio::select!`
- `tokio::signal::ctrl_c`
- `CancellationToken` from `tokio_util`

---

### Using `CancellationToken`

Add import:

```rust
use tokio_util::sync::CancellationToken;
use std::time::Duration;
```

Example:

```rust
use std::time::Duration;
use tokio_util::sync::CancellationToken;

#[tokio::main]
async fn main() {
    let token = CancellationToken::new();
    let child_token = token.clone();

    let worker = tokio::spawn(async move {
        loop {
            tokio::select! {
                _ = child_token.cancelled() => {
                    println!("worker received cancellation signal");
                    break;
                }
                _ = tokio::time::sleep(Duration::from_secs(1)) => {
                    println!("worker doing work...");
                }
            }
        }

        println!("worker stopped");
    });

    tokio::time::sleep(Duration::from_secs(3)).await;

    println!("main is cancelling the worker");
    token.cancel();

    let _ = worker.await;
}
```

Explanation:

- The worker loop keeps running.
- It checks if cancellation was requested.
- When `token.cancel()` is called, the worker exits.

Simple idea:

> Cancellation in Tokio is usually cooperative.

That means tasks must check for cancellation and stop themselves.

---

### Graceful shutdown

Graceful shutdown means:

> Stop accepting new work, but allow current work to finish.

For web servers, this is very important.

With Axum, you can use:

```rust
.with_graceful_shutdown(shutdown_signal())
```

Example:

```rust
async fn shutdown_signal() {
    tokio::signal::ctrl_c()
        .await
        .expect("failed to listen for ctrl+c");

    println!("shutdown signal received");
}
```

When you press `Ctrl+C`, Axum can stop accepting new connections and finish current requests.

---

## 10. Axum after Tokio fundamentals

Axum is a web framework built to work nicely with Tokio.

If you understand:

- `async fn`
- `.await`
- Tokio runtime
- `tokio::spawn`
- shared state
- async handlers

then Axum becomes much easier.

Axum is responsible for:

- routing
- HTTP methods
- request parsing
- JSON handling
- middleware
- responses

Tokio is responsible for:

- running async tasks
- network I/O
- concurrency
- timers
- background work

---

### Very simple Axum server

```rust
use axum::{routing::get, Router};

async fn hello() -> &'static str {
    "Hello from Axum"
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(hello));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    println!("listening on http://127.0.0.1:3000");

    axum::serve(listener, app).await.unwrap();
}
```

Run it:

```bash
cargo run
```

Visit:

```text
http://127.0.0.1:3000
```

Explanation:

- `Router::new()` creates a router.
- `.route("/", get(hello))` maps GET `/` to the `hello` handler.
- `TcpListener::bind` binds to a port.
- `axum::serve` starts the web server.

---

### Axum with JSON

Add Serde imports:

```rust
use serde::Deserialize;
use axum::{Json, routing::post, Router};
```

Example:

```rust
use axum::{Json, routing::post, Router};
use serde::Deserialize;

#[derive(Deserialize)]
struct CreateUser {
    name: String,
    email: String,
}

async fn create_user(Json(payload): Json<CreateUser>) -> String {
    format!("Created user: {}, email: {}", payload.name, payload.email)
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/users", post(create_user));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    println!("listening on http://127.0.0.1:3000");

    axum::serve(listener, app).await.unwrap();
}
```

Test it:

```bash
curl -X POST http://127.0.0.1:3000/users \
  -H "Content-Type: application/json" \
  -d '{"name":"Alice","email":"alice@example.com"}'
```

Explanation:

- `#[derive(Deserialize)]` lets Serde convert JSON into a Rust struct.
- `Json(payload): Json<CreateUser>` extracts the JSON body.
- The handler returns a `String`.

---

### Axum with shared state

Web servers often need shared state.

For example:

- database pool
- in-memory cache
- TODO list
- configuration

In Axum, shared state is often stored in `Arc`.

Example:

```rust
use axum::{
    extract::State,
    routing::{get, post},
    Json, Router,
};
use serde::Deserialize;
use std::sync::Arc;
use tokio::sync::Mutex;

type AppState = Arc<Mutex<Vec<String>>>;

#[derive(Deserialize)]
struct AddItemRequest {
    item: String,
}

async fn root() -> &'static str {
    "Use GET /items or POST /items"
}

async fn list_items(State(state): State<AppState>) -> Json<Vec<String>> {
    let items = state.lock().await.clone();
    Json(items)
}

async fn add_item(
    State(state): State<AppState>,
    Json(payload): Json<AddItemRequest>,
) -> &'static str {
    state.lock().await.push(payload.item);
    "item added"
}

async fn shutdown_signal() {
    tokio::signal::ctrl_c()
        .await
        .expect("failed to listen for ctrl+c");

    println!("shutdown signal received");
}

#[tokio::main]
async fn main() {
    let state: AppState = Arc::new(Mutex::new(Vec::new()));

    let app = Router::new()
        .route("/", get(root))
        .route("/items", get(list_items).post(add_item))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    println!("listening on http://127.0.0.1:3000");

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();
}
```

Test `GET /items`:

```bash
curl http://127.0.0.1:3000/items
```

Test `POST /items`:

```bash
curl -X POST http://127.0.0.1:3000/items \
  -H "Content-Type: application/json" \
  -d '{"item":"learn tokio"}'
```

Then check again:

```bash
curl http://127.0.0.1:3000/items
```

Explanation:

- `AppState` is `Arc<Mutex<Vec<String>>>`.
- `Arc` allows sharing across tasks.
- `Mutex` protects the vector.
- `State(state): State<AppState>` gives the handler access to shared state.
- `with_graceful_shutdown` lets the server stop cleanly on `Ctrl+C`.

---

## Big Picture

Here is the simplest way to think about Tokio and Axum:

```text
Tokio = async runtime
Axum  = web framework
```

More detailed:

```text
Tokio:
- runs async tasks
- handles sockets
- handles timers
- handles concurrency
- provides spawn, channels, select, join, etc.

Axum:
- handles HTTP requests
- routes URLs to handler functions
- extracts JSON, path params, query params, etc.
- returns responses
```

Python analogy:

```text
Tokio  ≈ asyncio
Axum   ≈ FastAPI / async web framework
```

---

## Recommended Learning Order

For Tokio:

1. `async fn` and `.await`
2. `#[tokio::main]`
3. `tokio::spawn`
4. `JoinHandle`
5. Channels
6. `join!` and `select!`
7. Shared state with `Arc` and `Mutex`
8. Graceful shutdown

For Axum:

1. Basic routing
2. Path parameters
3. Query parameters
4. JSON bodies
5. Shared state
6. Middleware
7. Graceful shutdown
8. Real project structure

---

## Common Mistakes

### 1. Forgetting `.await`

This creates a future but does not run it:

```rust
async fn do_work() {}

fn main() {
    do_work();
}
```

You need:

```rust
do_work().await;
```

and a runtime.

---

### 2. Borrowing local data in spawned tasks

This usually fails:

```rust
let message = String::from("hello");

tokio::spawn(async {
    println!("{message}");
});
```

Use `async move`:

```rust
let message = String::from("hello");

tokio::spawn(async move {
    println!("{message}");
});
```

---

### 3. Blocking inside async handlers

Avoid long blocking operations inside async handlers.

Bad:

```rust
async fn handler() {
    std::thread::sleep(std::time::Duration::from_secs(10));
}
```

Better for CPU/blocking work:

```rust
tokio::task::spawn_blocking(|| {
    std::thread::sleep(std::time::Duration::from_secs(10));
});
```

---

### 4. Thinking tasks cancel automatically

Tokio tasks usually stop when they finish or when you explicitly design cancellation.

Use:

- `CancellationToken`
- `select!`
- channels
- shutdown signals

---

## Final Simple Summary

If you remember only these lines:

```text
async fn returns a Future.
.await runs the Future.
#[tokio::main] starts the Tokio runtime.
tokio::spawn runs background tasks.
JoinHandle lets you wait for a spawned task.
Channels let tasks communicate.
join! waits for multiple futures.
select! waits for the first future.
Arc shares data across tasks.
Mutex protects shared data.
Send means safe to move across threads.
'static means no short-lived borrows.
Axum is the web framework.
Tokio is the runtime underneath it.
```