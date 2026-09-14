// ============================================================================
// ## Q35 — Capstone: Axum + sqlx + tracing + tests
//
// Everything above, in one small service. This is the graduation project.
//
// Build a users API:
// - `GET    /users`      → list all
// - `GET    /users/{id}` → one user, `404` when missing
// - `POST   /users`      → create, `409` on duplicate email
// - `DELETE /users/{id}` → delete, `404` when missing
//
// Requirements (each one a question you already answered):
// - State is a `SqlitePool`, passed with `axum::extract::State` (Q34) — NOT a `HashMap` (Q25)
// - One error enum for the whole app, with `impl IntoResponse` mapping each variant
//   to the right status code (Q27)
// - `#[tracing::instrument]` on every handler, `RUST_LOG` controlled (Q33)
// - Config from env vars with sensible defaults: `DATABASE_URL`, `PORT` (Q31)
// - Integration tests that spin up the app against an in-memory database
//   (`sqlite::memory:`) and assert real status codes (Q26)
//
// The design question to answer in a comment:
// - Where do you convert a `sqlx::Error` into your own error type — in the handler,
//   or in a separate storage layer? Write down which you chose and why.
//
// Answer:
// We convert `sqlx::Error` into our `AppError` inside the handler layer (using `?` via
// `From<sqlx::Error>` for generic DB errors, and explicit match for unique-constraint violations).
// Why:
// In a focused service like this, keeping the conversion at the handler boundary provides
// maximum transparency and zero boilerplate without redundant abstraction layers.
// However, in large multi-protocol applications (e.g. gRPC + Web + CLI), placing conversion
// inside a dedicated storage/repository layer is preferred to completely decouple database
// implementation details (like SQLite vs PostgreSQL error codes) from transport logic.
//
// Hint:
// `impl IntoResponse for AppError` is what lets a handler return
// `Result<Json<User>, AppError>` and have axum turn the error into an HTTP
// response automatically. That one impl centralizes conversion of application errors
// into HTTP responses.
// ============================================================================

use std::env;
use std::net::SocketAddr;

use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::get,
};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, SqlitePool};
use tracing::{info, instrument};
use tracing_subscriber::EnvFilter;

// --- Data models ---

#[derive(Serialize, Deserialize, FromRow, Debug, Clone, PartialEq, Eq)]
pub struct User {
    pub id: i64,
    pub name: String,
    pub email: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreateUser {
    pub name: String,
    pub email: String,
}

// --- Custom Error Handling (Q27) ---

#[derive(Debug)]
pub enum AppError {
    NotFound,
    DuplicateEmail(String),
    Database(sqlx::Error),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            AppError::NotFound => (StatusCode::NOT_FOUND, "User not found".to_string()),
            AppError::DuplicateEmail(email) => (
                StatusCode::CONFLICT,
                format!("User with email '{email}' already exists"),
            ),
            AppError::Database(err) => {
                tracing::error!(error = %err, "Database operation failed");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Internal server error".to_string(),
                )
            }
        };

        let body = Json(serde_json::json!({
            "error": message,
            "status": status.as_u16(),
        }));

        (status, body).into_response()
    }
}

// Automatic conversion for `?` operator on sqlx queries
impl From<sqlx::Error> for AppError {
    fn from(err: sqlx::Error) -> Self {
        AppError::Database(err)
    }
}

// --- Handlers (Q33: #[tracing::instrument] on every handler) ---

/// GET /users → list all users
#[instrument(skip(pool))]
pub async fn list_users(State(pool): State<SqlitePool>) -> Result<Json<Vec<User>>, AppError> {
    let users = sqlx::query_as::<_, User>("SELECT id, name, email FROM users ORDER BY id ASC")
        .fetch_all(&pool)
        .await?;

    info!(count = users.len(), "Retrieved users list");
    Ok(Json(users))
}

/// GET /users/{id} → fetch one user, 404 when missing
#[instrument(skip(pool))]
pub async fn get_user(
    State(pool): State<SqlitePool>,
    Path(id): Path<i64>,
) -> Result<Json<User>, AppError> {
    let maybe_user = sqlx::query_as::<_, User>("SELECT id, name, email FROM users WHERE id = ?")
        .bind(id)
        .fetch_optional(&pool)
        .await?;

    match maybe_user {
        Some(user) => {
            info!(user_id = id, "Found user");
            Ok(Json(user))
        }
        None => {
            info!(user_id = id, "User not found");
            Err(AppError::NotFound)
        }
    }
}

/// POST /users → create user, 409 on duplicate email
#[instrument(skip(pool, payload), fields(name = %payload.name, email = %payload.email))]
pub async fn create_user(
    State(pool): State<SqlitePool>,
    Json(payload): Json<CreateUser>,
) -> Result<(StatusCode, Json<User>), AppError> {
    let result = sqlx::query("INSERT INTO users (name, email) VALUES (?, ?)")
        .bind(&payload.name)
        .bind(&payload.email)
        .execute(&pool)
        .await;

    match result {
        Ok(res) => {
            let id = res.last_insert_rowid();
            let new_user = User {
                id,
                name: payload.name,
                email: payload.email,
            };
            info!(user_id = id, "Created new user successfully");
            Ok((StatusCode::CREATED, Json(new_user)))
        }
        Err(sqlx::Error::Database(db_err)) if db_err.is_unique_violation() => {
            info!(email = %payload.email, "Duplicate email conflict");
            Err(AppError::DuplicateEmail(payload.email))
        }
        Err(err) => Err(AppError::Database(err)),
    }
}

/// DELETE /users/{id} → delete user, 404 when missing
#[instrument(skip(pool))]
pub async fn delete_user(
    State(pool): State<SqlitePool>,
    Path(id): Path<i64>,
) -> Result<StatusCode, AppError> {
    let result = sqlx::query("DELETE FROM users WHERE id = ?")
        .bind(id)
        .execute(&pool)
        .await?;

    if result.rows_affected() == 0 {
        info!(user_id = id, "Delete target not found");
        Err(AppError::NotFound)
    } else {
        info!(user_id = id, "Deleted user successfully");
        Ok(StatusCode::NO_CONTENT)
    }
}

// --- App Initialization & Routing ---

pub async fn init_db(pool: &SqlitePool) -> Result<(), sqlx::Error> {
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS users (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            email TEXT NOT NULL UNIQUE
        );",
    )
    .execute(pool)
    .await?;

    Ok(())
}

/// Builds the Axum router attached with SqlitePool state (Q34, Q35)
pub fn app(pool: SqlitePool) -> Router {
    Router::new()
        .route("/users", get(list_users).post(create_user))
        .route("/users/{id}", get(get_user).delete(delete_user))
        .with_state(pool)
}

// --- Main (Q31: env vars, Q33: tracing subscriber) ---

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Initialise tracing subscriber controlled by RUST_LOG env var (Q33)
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")),
        )
        .init();

    // 2. Read configuration from env vars with sensible defaults (Q31)
    let database_url =
        env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite:app.db?mode=rwc".to_string());
    let port: u16 = env::var("PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(3000);

    info!(database_url = %database_url, port = port, "Starting Capstone server");

    // 3. Connect to SQLite pool and run schema migration (Q34)
    let pool = SqlitePool::connect(&database_url).await?;
    init_db(&pool).await?;

    // 4. Build router
    let app = app(pool);

    // 5. Run Axum server
    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    info!("Server listening on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

// --- Integration Tests (Q26: in-memory database tests) ---

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::io::{AsyncReadExt, AsyncWriteExt};
    use tokio::net::{TcpListener, TcpStream};

    /// Spawns the app against an in-memory SQLite database on an ephemeral port
    async fn setup_test_server() -> SocketAddr {
        let pool = SqlitePool::connect("sqlite::memory:").await.unwrap();
        init_db(&pool).await.unwrap();

        let router = app(pool);
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();

        tokio::spawn(async move {
            axum::serve(listener, router).await.unwrap();
        });

        addr
    }

    /// Helper to send HTTP requests over TCP stream and return (status_code, body)
    async fn send_request(
        addr: SocketAddr,
        method: &str,
        path: &str,
        body: Option<&str>,
    ) -> (u16, String) {
        let mut stream = TcpStream::connect(addr).await.unwrap();
        let request_str = match body {
            Some(b) => format!(
                "{method} {path} HTTP/1.1\r\n\
                 Host: localhost\r\n\
                 Content-Type: application/json\r\n\
                 Content-Length: {}\r\n\
                 Connection: close\r\n\r\n\
                 {b}",
                b.len()
            ),
            None => format!(
                "{method} {path} HTTP/1.1\r\n\
                 Host: localhost\r\n\
                 Connection: close\r\n\r\n"
            ),
        };

        stream.write_all(request_str.as_bytes()).await.unwrap();

        let mut response = Vec::new();
        stream.read_to_end(&mut response).await.unwrap();
        let response_str = String::from_utf8_lossy(&response).to_string();

        let status_code: u16 = response_str
            .lines()
            .next()
            .and_then(|line| line.split_whitespace().nth(1))
            .and_then(|code| code.parse().ok())
            .unwrap_or(0);

        (status_code, response_str)
    }

    #[tokio::test]
    async fn test_full_users_crud_lifecycle() {
        let addr = setup_test_server().await;

        // 1. GET /users initially returns 200 OK and empty array []
        let (status, resp) = send_request(addr, "GET", "/users", None).await;
        assert_eq!(status, 200);
        assert!(resp.ends_with("[]"));

        // 2. POST /users creates a user -> 201 Created
        let new_user_json = r#"{"name":"Alice","email":"alice@example.com"}"#;
        let (status, resp) = send_request(addr, "POST", "/users", Some(new_user_json)).await;
        assert_eq!(status, 201);
        assert!(resp.contains(r#""name":"Alice""#));
        assert!(resp.contains(r#""email":"alice@example.com""#));

        // 3. GET /users includes the newly created user
        let (status, resp) = send_request(addr, "GET", "/users", None).await;
        assert_eq!(status, 200);
        let (_, body) = resp.split_once("\r\n\r\n").unwrap();
        let users: Vec<User> = serde_json::from_str(body).unwrap();
        assert_eq!(
            users,
            vec![User {
                id: 1,
                name: "Alice".to_string(),
                email: "alice@example.com".to_string(),
            }]
        );

        // 4. POST /users with same email -> 409 Conflict
        let duplicate_user_json = r#"{"name":"Alice Clone","email":"alice@example.com"}"#;
        let (status, resp) = send_request(addr, "POST", "/users", Some(duplicate_user_json)).await;
        assert_eq!(status, 409);
        assert!(resp.contains("already exists"));

        // 5. GET /users/{id} for existing user -> 200 OK
        let (status, resp) = send_request(addr, "GET", "/users/1", None).await;
        assert_eq!(status, 200);
        assert!(resp.contains(r#""email":"alice@example.com""#));

        // 6. GET /users/{id} for nonexistent user -> 404 Not Found
        let (status, _) = send_request(addr, "GET", "/users/999", None).await;
        assert_eq!(status, 404);

        // 7. DELETE /users/{id} for existing user -> 204 No Content
        let (status, _) = send_request(addr, "DELETE", "/users/1", None).await;
        assert_eq!(status, 204);

        // 8. GET /users/{id} after deletion -> 404 Not Found
        let (status, _) = send_request(addr, "GET", "/users/1", None).await;
        assert_eq!(status, 404);

        // 9. DELETE /users/{id} for already deleted user -> 404 Not Found
        let (status, _) = send_request(addr, "DELETE", "/users/1", None).await;
        assert_eq!(status, 404);
    }
}
