// ============================================================================
// ## Q34 — `sqlx` with SQLite
//
// Q25 stored users in an `Arc<Mutex<HashMap>>` — which vanishes when the process
// exits. Replace it with a real database. SQLite, not Postgres: no server to
// install, and the SQL is the same.
//
// Note: The first build after adding `sqlx` will be slow. That is normal.
//
// Tasks:
// 1. Create the pool and a table:
//    ```rust
//    let pool = sqlx::SqlitePool::connect("sqlite:app.db?mode=rwc").await?;
//
//    sqlx::query(
//        "CREATE TABLE IF NOT EXISTS users (
//            id INTEGER PRIMARY KEY,
//            name TEXT NOT NULL,
//            email TEXT NOT NULL UNIQUE
//        )",
//    )
//    .execute(&pool)
//    .await?;
//    ```
//
// 2. Insert a user with bound parameters, never string formatting:
//    ```rust
//    sqlx::query("INSERT INTO users (name, email) VALUES (?, ?)")
//        .bind(&name)
//        .bind(&email)
//        .execute(&pool)
//        .await?;
//    ```
//
// 3. Read rows back into a struct using `sqlx::FromRow` and `fetch_all`.
// 4. Fetch one user by id with `fetch_optional` — it returns `Option<T>`, so a
//    missing row is not an error.
// 5. Handle the duplicate-email case: insert the same email twice and turn the
//    `UNIQUE` violation into your own error type from Q27.
//
// Question to answer in a comment:
// - Why is `.bind(&name)` different from `format!("INSERT ... VALUES ('{name}')")`?
//   Name the attack.
//
// Hint:
// A `SqlitePool` is cheap to clone — it is an `Arc` internally, so clone it
// per request rather than wrapping it in a `Mutex`.
// ============================================================================

use sqlx::FromRow;

#[derive(Debug, FromRow)]
struct User {
    id: i64,
    name: String,
    email: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    todo!("Q34 implementation")
}
