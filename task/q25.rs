use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    routing::{delete, get}, //note we did not use post method, it is already comes with get.
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, Mutex}; 

// --- Data models ---

#[derive(Serialize, Deserialize, Clone, Debug)]
struct Todo {
    id: u64,
    title: String,
    completed: bool,
}

#[derive(Deserialize)]
struct CreateTodo {
    title: String,
}

// Shared thread-safe in-memory database
// Arc  = lets multiple threads hold a pointer to the same data
// Mutex = only one thread can read/write the HashMap at a time
type AppState = Arc<Mutex<HashMap<u64, Todo>>>; //just alias

// --- Handlers ---

// GET /todos → return all todos as a JSON array
async fn list_todos(State(state): State<AppState>) -> Json<Vec<Todo>> {
    let db = state.lock().unwrap();
    let todos: Vec<Todo> = db.values().cloned().collect();
    Json(todos)
}

// POST /todos → create a new todo, auto-generate id
async fn create_todo(
    State(state): State<AppState>,
    Json(input): Json<CreateTodo>,
) -> (StatusCode, Json<Todo>) {
    let mut db = state.lock().unwrap();

    // Generate next id: max existing id + 1, or 1 if empty
    let next_id = db.keys().max().unwrap_or(&0) + 1;

    let todo = Todo {
        id: next_id,
        title: input.title,
        completed: false,
    };

    db.insert(todo.id, todo.clone());

    println!("Created todo: {todo:?}");

    (StatusCode::CREATED, Json(todo))
}

// DELETE /todos/:id → remove a todo by id
async fn delete_todo(State(state): State<AppState>, Path(id): Path<u64>) -> StatusCode {
    let mut db = state.lock().unwrap();

    if db.remove(&id).is_some() {
        println!("Deleted todo with id: {id}");
        StatusCode::NO_CONTENT // 204
    } else {
        println!("Todo with id {id} not found");
        StatusCode::NOT_FOUND // 404
    }
}

// --- Main ---

#[tokio::main]
async fn main() {
    // Create shared state: an empty HashMap wrapped in Arc<Mutex<>>
    let state: AppState = Arc::new(Mutex::new(HashMap::new()));

    // Build router and attach state
    let app = Router::new()
        .route("/todos", get(list_todos).post(create_todo))
        .route("/todos/{id}", delete(delete_todo))
        .with_state(state);

    // Bind to port 3000
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    println!("Server running on http://127.0.0.1:3000");

    // Start serving
    axum::serve(listener, app).await.unwrap();
}

/*
================================================================================
                    DETAILED EXPLANATION OF Q25 CONCEPTS
================================================================================

1. SHARED IN-MEMORY STATE (`AppState`):
   ------------------------------------
   - `type AppState = Arc<Mutex<HashMap<u64, Todo>>>;` is a TYPE ALIAS (shortcut).
   - `HashMap`: The "database" here is just a standard Rust HashMap living in RAM.
     `db.insert()` and `db.remove()` are standard HashMap methods.
   - `Mutex`: Mutual Exclusion. Guarantees that only ONE thread/request can read
     or mutate the HashMap at any given millisecond, preventing data races.
   - `Arc`: Atomic Reference Counted pointer. Allows multiple concurrent HTTP
     worker threads to share ownership of the single Mutex on the heap.
   - `state.lock().unwrap()`:
     * Acquires the lock and returns a `MutexGuard`.
     * `.unwrap()`: If another thread panicked while holding the lock, the mutex
       is "poisoned", and this thread will panic too instead of touching corrupted data.
     * When the guard variable (`db`) goes out of scope at the end of the function,
       Rust's RAII automatically releases the lock!

2. ROUTING & METHOD CHAINING:
   ---------------------------
   - `.route("/todos", get(list_todos).post(create_todo))`
   - Why didn't we import `post` in `use axum::routing::{delete, get}`?
     Because `get(list_todos)` returns a `MethodRouter` object, and that object
     has `.post(...)` built-in as a method!
   - Why on one line?
     Both `GET /todos` (fetch all) and `POST /todos` (create new) share the SAME
     URL path. In Axum, calling `.route("/todos", ...)` twice will PANIC at runtime
     due to duplicate path registration. Methods on the same path must be chained
     or merged.

3. AXUM EXTRACTORS:
   -----------------
   - Handlers take "Extractors" as parameters:
     * `State(state)`: Axum injects a clone of the shared `Arc<Mutex<...>>`.
     * `Json(input)`: Parses the incoming JSON body into the `CreateTodo` struct.
     * `Path(id)`: Extracts `{id}` from the URL path (`/todos/{id}`) and parses it
       into `u64`. (Note: This is an HTTP URL path, NOT Python's filesystem `pathlib`).

4. SERDE (Serialize / Deserialize):
   ---------------------------------
   - Tokio does NOT need Serde (Tokio only manages async tasks/timers).
   - Axum DOES need Serde whenever sending or receiving JSON:
     * `#[derive(Serialize)]`: Converts Rust struct -> JSON text (outgoing response).
     * `#[derive(Deserialize)]`: Converts JSON text -> Rust struct (incoming request).

5. HTTP STATUS CODES:
   --------------------
   - `StatusCode::CREATED` (201), `StatusCode::NO_CONTENT` (204), `StatusCode::NOT_FOUND` (404)
     are type-safe constants defined in the `http` crate (`status.rs`). They prevent
     typos compared to raw magic numbers like `201` or `404`.

6. TESTING WITH CURL (PowerShell):
   -------------------------------
   - Create Todo:
     curl.exe -X POST http://127.0.0.1:3000/todos -H "Content-Type: application/json" -d "{\"title\": \"Learn Rust\"}"
   - Get Todos:
     curl.exe http://127.0.0.1:3000/todos
   - Delete Todo #1:
     curl.exe -X DELETE http://127.0.0.1:3000/todos/1
================================================================================
*/
