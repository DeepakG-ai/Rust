# SQLx Notes and Beginner Guide for Rust

A comprehensive, step-by-step guide to working with **`sqlx`** and **SQLite** in Rust, covering database connections, schema management, CRUD operations, safe parameter binding, struct mapping, error handling, and web framework integration.

---

## Table of Contents

1. [Introduction to `sqlx`](#1-introduction-to-sqlx)
2. [Dependencies and Setup](#2-dependencies-and-setup)
3. [Connection Pools (`SqlitePool`)](#3-connection-pools-sqlitepool)
4. [Executing Commands (DDL & Writes)](#4-executing-commands-ddl--writes)
5. [Safe Parameter Binding vs. SQL Injection](#5-safe-parameter-binding-vs-sql-injection)
6. [Mapping Rows to Structs (`FromRow`)](#6-mapping-rows-to-structs-fromrow)
7. [Query Methods: `fetch_all`, `fetch_optional`, `fetch_one`](#7-query-methods-fetch_all-fetch_optional-fetch_one)
8. [Handling Database Errors](#8-handling-database-errors)
9. [Integration with Axum (`State<SqlitePool>`)](#9-integration-with-axum-statesqlitepool)
10. [Testing with In-Memory SQLite](#10-testing-with-in-memory-sqlite)
11. [Complete Standalone Example](#11-complete-standalone-example)

---

## 1. Introduction to `sqlx`

When building services, storing state in memory (for instance, using `Arc<Mutex<HashMap<...>>>`) is simple, but all data is lost the moment your program stops or restarts. A relational database provides persistent, durable storage.

**`sqlx`** is an async, pure-Rust SQL toolkit designed to interact with databases like PostgreSQL, MySQL, and SQLite.

### Why `sqlx`?
- **Async-First**: Built natively for async runtimes like Tokio.
- **Raw SQL Control**: Unlike heavy ORMs (Object-Relational Mappers), you write standard SQL queries directly, giving you complete visibility and control.
- **Strong Typing**: Easily deserializes database query results into Rust structs using traits like `FromRow`.
- **Compile-Time or Runtime**: Offers runtime queries (flexible, no build-time database requirement) and compile-time checked queries (via macros).

### Why SQLite?
SQLite requires **no external server or daemon**. The entire database is stored in a single local file (e.g. `app.db`) or held purely in memory (`sqlite::memory:`).

---

## 2. Dependencies and Setup

In your `Cargo.toml`, add `sqlx` with Tokio runtime and SQLite driver support:

```toml
[dependencies]
tokio = { version = "1", features = ["full"] }
sqlx = { version = "0.8", features = ["runtime-tokio", "sqlite"] }
```

> **Note**: The first build after adding `sqlx` can take a few minutes as SQLite is compiled from source. This is normal.

---

## 3. Connection Pools (`SqlitePool`)

Opening a brand new database connection on every incoming request is slow and wasteful. Instead, applications use a **Connection Pool**.

- A connection pool manages an active set of database connections, handing them out when queries run and taking them back when queries complete.
- **`SqlitePool` is cheap to clone**: Internally, it is wrapped in an `Arc`. Cloning a `SqlitePool` merely clones an atomic pointer to the same underlying pool. You do **not** need to wrap it inside a `Mutex`.

### Establishing a Connection

```rust
use sqlx::SqlitePool;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    // Connect to a local SQLite file:
    let pool = SqlitePool::connect("sqlite:app.db?mode=rwc").await?;
    
    Ok(())
}
```

### Connection URL Parameters:
- `"sqlite:app.db"`: Database file path.
- `?mode=rwc`: Stands for **R**ead, **W**rite, **C**reate. If `app.db` does not exist yet, SQLite will automatically create the file.

---

## 4. Executing Commands (DDL & Writes)

When running operations that **do not return table rows** (like `CREATE TABLE`, `INSERT`, `UPDATE`, or `DELETE`), use `sqlx::query(...)` with `.execute(&pool)`.

```rust
sqlx::query(
    "CREATE TABLE IF NOT EXISTS users (
        id INTEGER PRIMARY KEY,
        name TEXT NOT NULL,
        email TEXT NOT NULL UNIQUE
    )"
)
.execute(&pool)
.await?;
```

`.execute(&pool).await?` returns a `QueryResult` with helpful metadata:
- `result.rows_affected()`: The number of rows modified/deleted/inserted.
- `result.last_insert_rowid()`: The auto-generated primary key ID of the row inserted.

---

## 5. Safe Parameter Binding vs. SQL Injection

When inserting user-supplied data, **never construct queries with string formatting or string concatenation**:

```rust
// ❌ CRITICAL SECURITY VULNERABILITY: SQL Injection!
let bad_query = format!("INSERT INTO users (name) VALUES ('{}')", user_input);
```

If an attacker supplies `' OR '1'='1` or `'; DROP TABLE users; --`, your application will execute their code.

### The Correct Way: Prepared Statements & `.bind(...)`

In SQLite, placeholders are denoted by `?`. Chain `.bind(&val)` calls in the exact order the placeholders appear:

```rust
let name = "Alice";
let email = "alice@example.com";

// ✅ 100% Safe: sqlx sends the query structure and parameters separately
sqlx::query("INSERT INTO users (name, email) VALUES (?, ?)")
    .bind(name)   // Replaces 1st '?'
    .bind(email)  // Replaces 2nd '?'
    .execute(&pool)
    .await?;
```

The database treats bound values strictly as literal values, never as executable SQL commands.

---

## 6. Mapping Rows to Structs (`FromRow`)

To read database rows directly into strongly typed Rust structs, derive `sqlx::FromRow`.

### Field Mapping Rules
- Struct field names must match the SQL column names (or SQL aliases via `AS`).
- Field types must correspond to SQLite data types:
  - `INTEGER PRIMARY KEY` / `INTEGER` $\rightarrow$ `i64` (or `i32`)
  - `TEXT` $\rightarrow$ `String`
  - `BOOLEAN` $\rightarrow$ `bool`
  - Nullable columns $\rightarrow$ `Option<T>`

```rust
use sqlx::FromRow;

#[derive(Debug, FromRow)]
struct User {
    id: i64,
    name: String,
    email: String,
}
```

---

## 7. Query Methods: `fetch_all`, `fetch_optional`, `fetch_one`

Use `sqlx::query_as::<_, TargetStruct>(...)` to fetch rows converted into your struct. The `<_, User>` parameter tells Rust:
1. `_`: Infer the database driver (`Sqlite`).
2. `User`: Deserialize rows into the `User` struct.

| Method | Return Type | Typical Use Case |
| :--- | :--- | :--- |
| `.fetch_all(&pool)` | `Result<Vec<T>, Error>` | Retrieving multiple items (e.g. `SELECT * FROM users`). Returns empty `Vec` if no records match. |
| `.fetch_optional(&pool)` | `Result<Option<T>, Error>` | Looking up a single item by an ID. Returns `Ok(Some(item))` if found, or `Ok(None)` if no record matches. |
| `.fetch_one(&pool)` | `Result<T, Error>` | When the record **must** exist. Returns `Err(sqlx::Error::RowNotFound)` if 0 rows are found. |

### Examples

#### Fetch All Rows
```rust
let users: Vec<User> = sqlx::query_as::<_, User>("SELECT id, name, email FROM users")
    .fetch_all(&pool)
    .await?;
```

#### Fetch Optional Row (by ID)
```rust
let target_id: i64 = 42;

let maybe_user: Option<User> = sqlx::query_as::<_, User>(
    "SELECT id, name, email FROM users WHERE id = ?"
)
.bind(target_id)
.fetch_optional(&pool)
.await?;

match maybe_user {
    Some(user) => println!("Found user: {:?}", user),
    None => println!("User with ID {target_id} does not exist."),
}
```

---

## 8. Handling Database Errors

Database calls return `Result<T, sqlx::Error>`.

### Detecting Constraint Violations (e.g., `UNIQUE` duplicate emails)
When an `INSERT` violates a `UNIQUE` constraint, SQLite returns an error. You can inspect `sqlx::Error::Database`:

```rust
let insert_res = sqlx::query("INSERT INTO users (name, email) VALUES (?, ?)")
    .bind("Bob")
    .bind("alice@example.com") // Already exists
    .execute(&pool)
    .await;

match insert_res {
    Ok(_) => println!("User inserted successfully!"),
    Err(sqlx::Error::Database(db_err)) => {
        if db_err.is_unique_violation() {
            eprintln!("Conflict: A user with this email already exists.");
        } else {
            eprintln!("Database error: {}", db_err.message());
        }
    }
    Err(other_err) => eprintln!("System/IO error: {}", other_err),
}
```

---

## 9. Integration with Axum (`State<SqlitePool>`)

In web APIs, the database pool is registered as global application state and passed into handlers.

### 1. Initialize Pool & Build Router
```rust
use axum::{Router, routing::get};
use sqlx::SqlitePool;

let pool = SqlitePool::connect("sqlite:app.db?mode=rwc").await?;

let app = Router::new()
    .route("/users", get(list_users))
    .with_state(pool); // Pool is cloned and stored in application state
```

### 2. Extract Pool in Handler
```rust
use axum::extract::State;
use axum::Json;

async fn list_users(
    State(pool): State<SqlitePool>
) -> Result<Json<Vec<User>>, StatusCode> {
    let users = sqlx::query_as::<_, User>("SELECT id, name, email FROM users")
        .fetch_all(&pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(users))
}
```

---

## 10. Testing with In-Memory SQLite

For unit and integration tests, you can use an in-memory SQLite database (`"sqlite::memory:"`).

- The database is created instantly in RAM.
- Each test gets an isolated, clean database without leaving temporary `.db` files on disk.
- Runs with zero setup or teardown scripts required.

```rust
#[tokio::test]
async fn test_user_creation() {
    let pool = SqlitePool::connect("sqlite::memory:").await.unwrap();

    // Run schema
    sqlx::query("CREATE TABLE users (id INTEGER PRIMARY KEY, name TEXT NOT NULL)")
        .execute(&pool)
        .await
        .unwrap();

    // Insert test record
    sqlx::query("INSERT INTO users (name) VALUES (?)")
        .bind("Tester")
        .execute(&pool)
        .await
        .unwrap();

    // Verify
    let count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM users")
        .fetch_one(&pool)
        .await
        .unwrap();

    assert_eq!(count.0, 1);
}
```

---

## 11. Complete Standalone Example

Here is a full, runnable example demonstrating all fundamental concepts:

```rust
use sqlx::{FromRow, SqlitePool};

#[derive(Debug, FromRow)]
struct Product {
    id: i64,
    name: String,
    sku: String,
    price: i64,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Connect (in-memory for this demo)
    let pool = SqlitePool::connect("sqlite::memory:").await?;

    // 2. Create table
    sqlx::query(
        "CREATE TABLE products (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            sku TEXT NOT NULL UNIQUE,
            price INTEGER NOT NULL
        )"
    )
    .execute(&pool)
    .await?;

    // 3. Insert with bound parameters
    sqlx::query("INSERT INTO products (name, sku, price) VALUES (?, ?, ?)")
        .bind("Mechanical Keyboard")
        .bind("KB-001")
        .bind(99)
        .execute(&pool)
        .await?;

    // 4. Query all records
    let products: Vec<Product> = sqlx::query_as::<_, Product>("SELECT * FROM products")
        .fetch_all(&pool)
        .await?;
    println!("All Products: {:?}", products);

    // 5. Query one optional record
    let found = sqlx::query_as::<_, Product>("SELECT * FROM products WHERE sku = ?")
        .bind("KB-001")
        .fetch_optional(&pool)
        .await?;
    println!("Found by SKU: {:?}", found);

    // 6. Handle duplicate constraint
    let duplicate_res = sqlx::query("INSERT INTO products (name, sku, price) VALUES (?, ?, ?)")
        .bind("Another Keyboard")
        .bind("KB-001") // Duplicate SKU!
        .bind(120)
        .execute(&pool)
        .await;

    if let Err(sqlx::Error::Database(err)) = duplicate_res {
        if err.is_unique_violation() {
            println!("Handled duplicate SKU properly!");
        }
    }

    Ok(())
}
```
