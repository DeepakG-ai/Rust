use axum::{
    Json, Router,
    extract::Path,
    http::StatusCode,
    routing::{get, post},
};
use serde::{Deserialize, Serialize};

// --- Data model ---

#[derive(Serialize, Deserialize, Debug, Clone)]
struct User {
    id: u64,
    name: String,
    role: String,
}

impl User {
    fn new(id: u64, name: String, role: String) -> User {
        User { id, name, role }
    }
}

// --- Handlers ---

// GET / → plain text welcome message
async fn hello() -> &'static str {
    "Welcome to Rust Axum Server!"
}

// GET /users/:id → extracts path param, returns JSON
async fn get_user(Path(id): Path<u64>) -> Json<User> {
    let user = User::new(id, "Alice".into(), "Admin".into());
    Json(user)
}

// POST /users → receives JSON body, prints it, returns 201 Created + JSON
async fn create_user(Json(user): Json<User>) -> (StatusCode, Json<User>) {
    println!("Created user: {user:?}");
    (StatusCode::CREATED, Json(user))
}

// --- Main ---

#[tokio::main]
async fn main() {
    // Build router with three routes
    let app = Router::new()
        .route("/", get(hello))
        .route("/users/{id}", get(get_user))
        .route("/users", post(create_user));

    // Bind to port 3000
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    println!("Server running on http://127.0.0.1:3000");

    // Start serving
    axum::serve(listener, app).await.unwrap();
}
