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
// Hint:
// `impl IntoResponse for AppError` is what lets a handler return
// `Result<Json<User>, AppError>` and have axum turn the error into an HTTP
// response automatically. That one impl removes every `match` from your handlers.
// ============================================================================

use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{delete, get, post},
};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, SqlitePool};
use tracing::instrument;

#[derive(Serialize, Deserialize, FromRow, Debug, Clone)]
struct User {
    id: i64,
    name: String,
    email: String,
}

#[derive(Deserialize)]
struct CreateUser {
    name: String,
    email: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    todo!("Q35 Capstone implementation")
}
