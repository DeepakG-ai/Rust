use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    routing::{delete, get},
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
type AppState = Arc<Mutex<HashMap<u64, Todo>>>;

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
async fn delete_todo(
    State(state): State<AppState>,
    Path(id): Path<u64>,
) -> StatusCode {
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
