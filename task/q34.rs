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
// Answer:
// The attack is SQL Injection (SQLi).
//
// When using `format!`, user input is concatenated directly into the raw SQL string.
// If the input contains SQL syntax (e.g. "'; DROP TABLE users; --"), the database
// engine treats that text as executable SQL statements.
//
// With `.bind()`, SQLx uses parameterized queries (prepared statements).
// The database engine parses and compiles the query structure first, and the bound
// values are transmitted separately as literal data. The data can never be
// evaluated as SQL commands, making SQL injection impossible.
//
// Hint:
// A `SqlitePool` is cheap to clone — it is an `Arc` internally, so clone it
// per request rather than wrapping it in a `Mutex`.
// ============================================================================

use std::fmt::{self, Display};
use sqlx::{FromRow, SqlitePool};

// Task 3: Struct mapped from database rows using FromRow
#[derive(Debug, FromRow)]
struct User {
    id: i64,
    name: String,
    email: String,
}

// Task 5: Custom error type (following the pattern from Q27)
#[derive(Debug)]
enum UserError {
    DuplicateEmail { email: String },
    Database(sqlx::Error),
}

impl Display for UserError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UserError::DuplicateEmail { email } => {
                write!(f, "user with email '{email}' already exists")
            }
            UserError::Database(err) => write!(f, "database error: {err}"),
        }
    }
}

impl std::error::Error for UserError {}

impl From<sqlx::Error> for UserError {
    fn from(err: sqlx::Error) -> Self {
        UserError::Database(err)
    }
}

// Helper to insert a user and map SQLite UNIQUE violations to UserError::DuplicateEmail
async fn insert_user(pool: &SqlitePool, name: &str, email: &str) -> Result<(), UserError> {
    let result = sqlx::query("INSERT INTO users (name, email) VALUES (?, ?)")
        .bind(name)
        .bind(email)
        .execute(pool)
        .await;

    match result {
        Ok(_) => Ok(()),
        Err(sqlx::Error::Database(db_err)) if db_err.is_unique_violation() => {
            Err(UserError::DuplicateEmail {
                email: email.to_string(),
            })
        }
        Err(err) => Err(UserError::Database(err)),
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Create the pool and a table
    let pool = SqlitePool::connect("sqlite:app.db?mode=rwc").await?;

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS users (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL,
            email TEXT NOT NULL UNIQUE
        )",
    )
    .execute(&pool)
    .await?;

    // 2. Insert a user with bound parameters (ignore error if already inserted from prior run)
    let initial_email = "deepak@gmail.com";
    let _ = insert_user(&pool, "Deepak Gowda", initial_email).await;

    // 3. Read rows back into a struct using `sqlx::FromRow` and `fetch_all`
    let all_users: Vec<User> = sqlx::query_as::<_, User>("SELECT id, name, email FROM users")
        .fetch_all(&pool)
        .await?;

    println!("--- Task 3: All Users (fetch_all) ---");
    for user in &all_users {
        println!("User #{} - Name: {}, Email: {}", user.id, user.name, user.email);
    }

    // 4. Fetch one user by id with `fetch_optional` (returns Option<T>, missing row is not error)
    println!("\n--- Task 4: Fetch Optional ---");
    let target_id: i64 = 1;
    let maybe_user: Option<User> =
        sqlx::query_as::<_, User>("SELECT id, name, email FROM users WHERE id = ?")
            .bind(target_id)
            .fetch_optional(&pool)
            .await?;

    match maybe_user {
        Some(user) => println!("Found user #{target_id}: {}, email: {}", user.name, user.email),
        None => println!("User #{target_id} does not exist."),
    }

    // Also testing with a nonexistent ID
    let missing_id: i64 = 999999;
    let missing_user: Option<User> =
        sqlx::query_as::<_, User>("SELECT id, name, email FROM users WHERE id = ?")
            .bind(missing_id)
            .fetch_optional(&pool)
            .await?;
    println!("Nonexistent user check (#{}): {:?}", missing_id, missing_user);

    // 5. Handle the duplicate-email case: insert the same email twice
    // and turn the `UNIQUE` violation into our custom error type
    println!("\n--- Task 5: Handle Duplicate Email ---");
    match insert_user(&pool, "Deepak Clone", initial_email).await {
        Ok(_) => println!("Inserted user successfully."),
        Err(UserError::DuplicateEmail { email }) => {
            println!("Caught custom error: User with email '{email}' already exists!");
        }
        Err(e) => return Err(e.into()),
    }

    Ok(())
}
