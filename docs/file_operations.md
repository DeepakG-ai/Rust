# File Operations in Rust

This guide starts with basic file reading and writing, then introduces the patterns commonly used in real applications. All examples use stable Rust unless a third-party crate is explicitly mentioned.

## 1. The basic mental model

Rust divides file handling across three standard-library modules:

- `std::fs` provides files, directories, copying, renaming, metadata, and deletion.
- `std::io` provides reading, writing, buffering, seeking, and I/O errors.
- `std::path` provides portable paths through `Path` and `PathBuf`.

A `File` value owns an operating-system file handle. When the value goes out of scope, Rust automatically closes the handle.

```rust
use std::fs::File;
use std::io;

fn open_example() -> io::Result<()> {
    let file = File::open("notes.txt")?;

    // Use file here.

    Ok(())
} // file is closed here
```

## 2. Understanding `io::Result<()>`

These two function signatures mean exactly the same thing:

```rust
fn main() -> std::io::Result<()> {
    Ok(())
}
```

```rust
use std::io;

fn main() -> io::Result<()> {
    Ok(())
}
```

In a grouped import such as `use std::io::{self, BufRead, BufReader};`, `self` imports the `std::io` module itself as `io`; it is unrelated to the `self` used inside struct methods.

`io::Result<T>` is a shorter name for approximately:

```rust
Result<T, std::io::Error>
```

Therefore, `io::Result<()>` can contain:

```rust
Ok(())       // success, with no useful return value
Err(error)   // an I/O operation failed
```

The `()` type is called the unit type. It means that an operation succeeded but has no value to return.

```rust
use std::io;

fn save() -> io::Result<()> {
    // Return only success or an error.
    Ok(())
}

fn load() -> io::Result<String> {
    // Success contains a String.
    Ok(String::from("file contents"))
}
```

The `?` operator returns an error immediately:

```rust
let text = std::fs::read_to_string("notes.txt")?;
```

It is approximately equivalent to:

```rust
let text = match std::fs::read_to_string("notes.txt") {
    Ok(value) => value,
    Err(error) => return Err(error),
};
```

Returning `io::Result<()>` from `main` makes `?` convenient. When `main` returns an error, Rust prints an error and exits unsuccessfully.

## 3. Reading a complete text file

Use `fs::read_to_string` for a small UTF-8 text file:

```rust
use std::fs;
use std::io;

fn main() -> io::Result<()> {
    let contents = fs::read_to_string("config.txt")?;
    println!("{contents}");
    Ok(())
}
```

This is useful for:

- Small configuration files
- JSON, TOML, and YAML documents
- SQL templates
- Test data

The entire file is loaded into memory. It also fails if the file is not valid UTF-8.

## 4. Reading a complete binary file

Images, PDFs, archives, and executables contain arbitrary bytes. Use `fs::read`:

```rust
use std::fs;
use std::io;

fn main() -> io::Result<()> {
    let bytes: Vec<u8> = fs::read("photo.jpg")?;
    println!("Read {} bytes", bytes.len());
    Ok(())
}
```

Use this only when the file is small enough to fit comfortably in memory.

## 5. Writing a complete file

```rust
use std::fs;
use std::io;

fn main() -> io::Result<()> {
    fs::write("message.txt", "Hello from Rust!\n")?;
    Ok(())
}
```

`fs::write`:

- Creates the file if it is missing.
- Replaces the contents if the file already exists.
- Accepts text or bytes.

Be careful: it does not append. Existing contents are overwritten.

## 6. Using `File` directly

Use `File` when you need more control than `fs::read` or `fs::write` provides.

### Read through a `File`

```rust
use std::fs::File;
use std::io::{self, Read};

fn main() -> io::Result<()> {
    let mut file = File::open("notes.txt")?;
    let mut contents = String::new();

    file.read_to_string(&mut contents)?;
    println!("{contents}");

    Ok(())
}
```

`Read` is a trait. Importing it makes methods such as `read_to_string`, `read_to_end`, and `read_exact` available.

The file is mutable because reading changes its cursor position.

### Create and write through a `File`

```rust
use std::fs::File;
use std::io::{self, Write};

fn main() -> io::Result<()> {
    let mut file = File::create("report.txt")?;

    file.write_all(b"Sales report\n")?;
    file.write_all(b"Total: 42\n")?;

    Ok(())
}
```

`File::create` creates a missing file, but it truncates an existing file to zero bytes.

## 7. Text strings and byte strings

Normal strings contain UTF-8 text:

```rust
let text: &str = "hello";
```

Byte strings contain bytes:

```rust
let bytes = b"hello";
```

`write_all` accepts bytes:

```rust
file.write_all(b"fixed text\n")?;
```

Convert a dynamic `String` or `&str` with `as_bytes()`:

```rust
let message = String::from("dynamic text\n");
file.write_all(message.as_bytes())?;
```

For formatted text, use `write!` or `writeln!`:

```rust
use std::io::Write;

writeln!(file, "user={}, score={}", "alice", 95)?;
```

`writeln!` writes formatted text followed by a newline.

## 8. Opening files with `OpenOptions`

`OpenOptions` lets you choose exactly how a file is opened.

### Append to a log file

```rust
use std::fs::OpenOptions;
use std::io::{self, Write};

fn main() -> io::Result<()> {
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open("application.log")?;

    writeln!(file, "user logged in")?;
    file.write_all(b"raw bytes\n")?;

    Ok(())
}
```

This opens the file for appending and creates it if it is missing.

### Common Python-to-Rust mode mapping

| Python mode | Rust equivalent |
|---|---|
| `"r"` | `File::open(path)` |
| `"w"` | `File::create(path)` |
| `"a"` | `.append(true).create(true)` |
| `"r+"` | `.read(true).write(true)` |
| `"w+"` | `.read(true).write(true).create(true).truncate(true)` |
| `"a+"` | `.read(true).append(true).create(true)` |
| `"x"` | `.write(true).create_new(true)` |

Full examples:

```rust
use std::fs::OpenOptions;

// "r": read only; file must exist
let file = OpenOptions::new()
    .read(true)
    .open("data.txt")?;

// "w": write; create or truncate
let file = OpenOptions::new()
    .write(true)
    .create(true)
    .truncate(true)
    .open("data.txt")?;

// "a": append; create if missing
let file = OpenOptions::new()
    .append(true)
    .create(true)
    .open("data.txt")?;

// "r+": read and write; file must exist
let file = OpenOptions::new()
    .read(true)
    .write(true)
    .open("data.txt")?;

// "w+": read and write; create or truncate
let file = OpenOptions::new()
    .read(true)
    .write(true)
    .create(true)
    .truncate(true)
    .open("data.txt")?;

// "a+": read and append; create if missing
let file = OpenOptions::new()
    .read(true)
    .append(true)
    .create(true)
    .open("data.txt")?;

// "x": create a new file; fail if it already exists
let file = OpenOptions::new()
    .write(true)
    .create_new(true)
    .open("data.txt")?;
```

This mapping covers access, creation, appending, and truncation. Rust files still read and write bytes; Rust does not automatically add Python-style text encoding behavior.

## 9. Where `self` is being used

`main` is a free function, so it has no `self`:

```rust
fn main() -> std::io::Result<()> {
    Ok(())
}
```

`self` exists in methods defined inside an `impl` block:

```rust
use std::fs::File;
use std::io::{self, Write};

struct Logger {
    file: File,
}

impl Logger {
    fn write_message(&mut self, message: &str) -> io::Result<()> {
        writeln!(self.file, "{message}")?;
        Ok(())
    }
}
```

When calling:

```rust
logger.write_message("user logged in")?;
```

`logger` becomes `self` inside `write_message`.

`OpenOptions::new()` has no `self` because it creates a new value. It is an associated function:

```rust
let mut options = OpenOptions::new();
```

The other calls are methods and do use `self` internally:

```rust
options.append(true);
options.create(true);
let file = options.open("log.txt")?;
```

The chained version means the same thing:

```rust
let file = OpenOptions::new()
    .append(true)
    .create(true)
    .open("log.txt")?;
```

Similarly:

```rust
file.write_all(data)?;
```

is method syntax. The `file` value becomes the `self` receiver used by `write_all`.

## 10. `write` versus `write_all`

`write` is allowed to write only part of a buffer:

```rust
use std::io::Write;

let number_written = file.write(b"hello world")?;
println!("Wrote {number_written} bytes");
```

Its return type is `io::Result<usize>`. The returned number may be smaller than the buffer length.

`write_all` keeps writing until all bytes are written or an error occurs:

```rust
file.write_all(b"hello world")?;
```

Use `write_all` for most application code. Use `write` only when you intentionally want low-level control.

If `write_all` returns an error, some bytes might already have been written. It does not roll the file back automatically.

## 11. Reading large files with `BufReader`

Do not load a huge file into one `String` or `Vec<u8>`. Process it incrementally:

```rust
use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn main() -> io::Result<()> {
    let file = File::open("server.log")?;
    let reader = BufReader::new(file);

    for line_result in reader.lines() {
        let line = line_result?;
        println!("{line}");
    }

    Ok(())
}
```

This pattern is used for:

- Log processing
- CSV imports
- Newline-delimited JSON
- Large text datasets

`lines()` removes the newline. When the exact line ending matters, use `read_line`:

```rust
use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn main() -> io::Result<()> {
    let file = File::open("input.txt")?;
    let mut reader = BufReader::new(file);
    let mut line = String::new();

    while reader.read_line(&mut line)? != 0 {
        print!("{line}");
        line.clear(); // Reuse the same allocation.
    }

    Ok(())
}
```

## 12. Writing many records with `BufWriter`

Use `BufWriter` when performing many small writes:

```rust
use std::fs::File;
use std::io::{self, BufWriter, Write};

fn main() -> io::Result<()> {
    let file = File::create("results.csv")?;
    let mut writer = BufWriter::new(file);

    writeln!(writer, "id,name,score")?;

    for id in 1..=100_000 {
        writeln!(writer, "{id},user-{id},{}", id * 10)?;
    }

    writer.flush()?;
    Ok(())
}
```

Buffering combines many small writes into fewer operating-system calls. Explicitly call `flush` so any flushing error can be returned.

## 13. Working with paths

Do not build paths by concatenating strings:

```rust
// Avoid this:
let path = base + "/" + filename;
```

Use `Path` and `PathBuf`:

```rust
use std::path::{Path, PathBuf};

fn report_path(base: &Path, name: &str) -> PathBuf {
    base.join("reports").join(name)
}

fn main() {
    let base = Path::new("data");
    let path = report_path(base, "sales.csv");
    println!("{}", path.display());
}
```

Think of them like this:

- `Path` is a borrowed path, similar to `str`.
- `PathBuf` is an owned, growable path, similar to `String`.

Reusable functions commonly accept `impl AsRef<Path>`:

```rust
use std::fs;
use std::io;
use std::path::Path;

fn load_file(path: impl AsRef<Path>) -> io::Result<Vec<u8>> {
    fs::read(path)
}
```

Paths are not guaranteed to contain valid UTF-8. Prefer `Path` over `&str` for filesystem APIs.

## 14. Directory operations

### Create nested directories

```rust
use std::fs;
use std::io;

fn main() -> io::Result<()> {
    fs::create_dir_all("output/reports/2026")?;
    Ok(())
}
```

`create_dir_all` creates missing parent directories and succeeds if the directory already exists.

### List a directory

```rust
use std::fs;
use std::io;

fn main() -> io::Result<()> {
    for entry_result in fs::read_dir("data")? {
        let entry = entry_result?;
        let path = entry.path();
        let metadata = entry.metadata()?;

        if metadata.is_file() {
            println!("File: {}", path.display());
        } else if metadata.is_dir() {
            println!("Directory: {}", path.display());
        }
    }

    Ok(())
}
```

`read_dir` is not recursive. The `walkdir` crate is commonly used for recursive traversal.

### Copy, rename, and remove

```rust
use std::fs;
use std::io;

fn main() -> io::Result<()> {
    fs::copy("source.txt", "backup.txt")?;
    fs::rename("old.txt", "new.txt")?;
    fs::remove_file("temporary.txt")?;
    fs::remove_dir("empty-directory")?;

    Ok(())
}
```

To remove a directory tree:

```rust
fs::remove_dir_all("temporary-tree")?;
```

This is destructive. Always validate the exact path before calling it.

## 15. File metadata

```rust
use std::fs;
use std::io;

fn main() -> io::Result<()> {
    let metadata = fs::metadata("data.txt")?;

    println!("Size: {} bytes", metadata.len());
    println!("Is file: {}", metadata.is_file());
    println!("Is directory: {}", metadata.is_dir());
    println!("Read-only: {}", metadata.permissions().readonly());

    Ok(())
}
```

Use `symlink_metadata` instead of `metadata` when you need information about a symbolic link itself rather than its target.

## 16. Seeking inside a file

Seeking moves the file cursor to a byte position:

```rust
use std::fs::File;
use std::io::{self, Read, Seek, SeekFrom};

fn main() -> io::Result<()> {
    let mut file = File::open("archive.bin")?;

    file.seek(SeekFrom::Start(100))?;

    let mut bytes = [0_u8; 16];
    file.read_exact(&mut bytes)?;

    println!("{bytes:?}");
    Ok(())
}
```

Available seek positions include:

```rust
SeekFrom::Start(100)   // 100 bytes from the beginning
SeekFrom::Current(20)  // 20 bytes forward
SeekFrom::Current(-10) // 10 bytes backward
SeekFrom::End(-32)     // 32 bytes before the end
```

Seek positions count bytes, not UTF-8 characters. Seeking is common in binary formats, archives, databases, and media files.

## 17. Error handling in applications

Avoid `unwrap()` for normal filesystem failures:

```rust
// Avoid in production paths:
let text = std::fs::read_to_string("config.txt").unwrap();
```

Return the error instead:

```rust
use std::fs;
use std::io;
use std::path::Path;

fn load_config(path: &Path) -> io::Result<String> {
    fs::read_to_string(path)
}
```

Handle expected error types when necessary:

```rust
use std::fs;
use std::io::{self, ErrorKind};

fn main() -> io::Result<()> {
    match fs::read_to_string("config.txt") {
        Ok(contents) => println!("{contents}"),

        Err(error) if error.kind() == ErrorKind::NotFound => {
            println!("Configuration is missing; using defaults");
        }

        Err(error) if error.kind() == ErrorKind::PermissionDenied => {
            eprintln!("Permission denied: {error}");
            return Err(error);
        }

        Err(error) => return Err(error),
    }

    Ok(())
}
```

Applications often use `anyhow` to attach context:

```rust
use anyhow::{Context, Result};
use std::fs;
use std::path::Path;

fn load_config(path: &Path) -> Result<String> {
    fs::read_to_string(path)
        .with_context(|| format!("failed to read {}", path.display()))
}
```

## 18. Simple production patterns

Production code uses the same APIs, but adds safety based on the importance of the data.

### A bounded read

Never trust the size of an external file. The following reads at most one byte beyond the limit so it can detect oversized input:

```rust
use std::fs::File;
use std::io::{self, Read};
use std::path::Path;

const MAX_SIZE: u64 = 16 * 1024;

fn read_small_file(path: &Path) -> io::Result<Vec<u8>> {
    let file = File::open(path)?;
    let mut bytes = Vec::new();

    file.take(MAX_SIZE + 1).read_to_end(&mut bytes)?;

    if bytes.len() as u64 > MAX_SIZE {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "file is too large",
        ));
    }

    Ok(bytes)
}
```

### An append-only record file

```rust
use std::fs::OpenOptions;
use std::io::{self, Write};
use std::path::Path;

fn append_record(path: &Path, record: &str) -> io::Result<()> {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }

    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)?;

    writeln!(file, "{record}")?;
    Ok(())
}
```

This is useful for logs, audit events, and newline-delimited JSON.

### Atomic replacement of important files

Writing directly to an important configuration or state file can leave incomplete contents if the process crashes. The safer pattern is:

1. Create a temporary file in the destination directory.
2. Write the complete contents.
3. Flush the writer.
4. Sync when crash durability matters.
5. Rename the temporary file over the destination.

The `tempfile` crate makes temporary-file creation safer:

```rust
use std::io::{self, Write};
use std::path::Path;
use tempfile::NamedTempFile;

fn atomic_write(path: &Path, contents: &[u8]) -> io::Result<()> {
    let parent = path.parent().ok_or_else(|| {
        io::Error::new(io::ErrorKind::InvalidInput, "path has no parent")
    })?;

    std::fs::create_dir_all(parent)?;

    let mut temporary = NamedTempFile::new_in(parent)?;
    temporary.write_all(contents)?;
    temporary.flush()?;
    temporary.as_file().sync_all()?;
    temporary.persist(path).map_err(|error| error.error)?;

    Ok(())
}
```

Keeping the temporary file in the same directory also keeps it on the same filesystem, which is important for rename-based replacement.

### `flush`, `sync_data`, and `sync_all`

These operations solve different problems:

- `BufWriter::flush()` sends Rust's user-space buffer to the operating system.
- `File::sync_data()` asks the operating system to persist file data.
- `File::sync_all()` asks it to persist file data and metadata.

Syncing every write can be expensive. Use it when losing recent data after a crash would be unacceptable.

### File locking

When cooperating processes could update the same file, a file lock can coordinate them:

```rust
use std::fs::OpenOptions;
use std::io;

fn main() -> io::Result<()> {
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open("state.db")?;

    file.lock()?;

    // Read or update protected state.

    file.unlock()?;
    Ok(())
}
```

These standard-library locking methods require Rust 1.89 or newer. Lock behavior has platform-specific details, and every cooperating program must follow the same locking rules.

## 19. Async file operations with Tokio

Use normal `std::fs` for command-line programs, batch processing, startup configuration, and dedicated worker threads.

Inside a Tokio web server or other async application, use `tokio::fs` so blocking file operations are moved away from async worker tasks:

```rust
use tokio::fs;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let contents = fs::read_to_string("config.txt").await?;
    println!("{contents}");
    Ok(())
}
```

For streamed async writing:

```rust
use tokio::fs::File;
use tokio::io::AsyncWriteExt;

async fn save() -> std::io::Result<()> {
    let mut file = File::create("output.txt").await?;
    file.write_all(b"hello\n").await?;
    file.flush().await?;
    Ok(())
}
```

Regular filesystem operations are usually blocking at the operating-system level. Tokio generally performs them through a blocking thread pool. Avoid converting every tiny operation into a separate async operation; batch work where reasonable.

## 20. Common supporting crates

The standard library supplies the main filesystem building blocks. Real applications commonly add:

- `serde_json` for JSON serialization and parsing
- `toml` for configuration files
- `csv` for streaming CSV records
- `tempfile` for secure temporary files
- `walkdir` for recursive directory traversal
- `anyhow` for application error context
- `thiserror` for structured library errors
- `memmap2` for memory-mapped files
- `notify` for watching filesystem changes
- `tokio` for file work in async applications

## 21. Mistakes to avoid

### Mistake 1: accidentally truncating an existing file

These replace the previous contents:

```rust
File::create(path)?;
fs::write(path, data)?;
OpenOptions::new().write(true).truncate(true).open(path)?;
```

Use append mode when the old contents must remain.

### Mistake 2: using `write` and assuming everything was written

```rust
// Potentially partial:
file.write(data)?;

// Usually wanted:
file.write_all(data)?;
```

### Mistake 3: forgetting the required trait import

Methods such as `read_to_string`, `write_all`, `lines`, and `seek` come from traits:

```rust
use std::io::{Read, Write, BufRead, Seek};
```

Import only the traits you actually use.

### Mistake 4: forgetting `mut`

Reading and writing move the file cursor:

```rust
let mut file = File::open(path)?;
```

### Mistake 5: reading a huge file completely into memory

Avoid this for unbounded input:

```rust
let contents = fs::read(path)?;
```

Use a size limit, `BufReader`, or chunked processing.

### Mistake 6: using `read_to_string` for binary data

`read_to_string` requires valid UTF-8. Use `fs::read` or `read_to_end` for binary data.

### Mistake 7: using `unwrap()` for expected failures

Files can be missing, locked, corrupted, or inaccessible. Return errors with `?`, handle expected `ErrorKind` values, and add useful context.

### Mistake 8: assuming `flush()` means physically saved

`flush()` only pushes language-level buffered data toward the operating system. Use `sync_data()` or `sync_all()` when crash durability is required.

### Mistake 9: directly overwriting critical state

For important configuration, metadata, or progress state, write a temporary file and rename it into place.

### Mistake 10: checking existence before creating

This has a race condition:

```rust
if !path.exists() {
    File::create(path)?;
}
```

Use atomic creation:

```rust
OpenOptions::new()
    .write(true)
    .create_new(true)
    .open(path)?;
```

Then handle `ErrorKind::AlreadyExists`.

### Mistake 11: constructing paths with string concatenation

Use `Path::join` or `PathBuf::push` so the code works correctly across platforms.

### Mistake 12: trusting user-supplied paths

Input such as `../../important.txt` may escape an intended directory. Avoid using user-provided names directly. Reject absolute paths and parent (`..`) components, or generate filenames yourself.

### Mistake 13: deleting an unchecked path

Before `remove_file` or especially `remove_dir_all`, verify the resolved target is exactly within the intended directory.

### Mistake 14: assuming append makes a whole record atomic

Append mode positions individual writes at the end, but several processes can still produce interleaved logical records if one record requires multiple writes. Build each record first and write it in as few operations as possible. Use locking when records must be coordinated.

### Mistake 15: using blocking file work carelessly in async code

Large `std::fs` operations inside a Tokio task can block an async worker. Use `tokio::fs` or `tokio::task::spawn_blocking` for substantial blocking work.

## 22. Quick decision guide

| Situation | Recommended approach |
|---|---|
| Small UTF-8 file | `fs::read_to_string` |
| Small binary file | `fs::read` |
| Replace a small file | `fs::write` |
| Read a large text file | `BufReader` with `lines` or `read_line` |
| Write many small records | `BufWriter` with `write_all` or `writeln!` |
| Append a log entry | `OpenOptions::append(true)` |
| Create only when absent | `OpenOptions::create_new(true)` |
| Build filesystem paths | `Path` and `PathBuf` |
| Inspect file information | `fs::metadata` or `fs::symlink_metadata` |
| Process a binary format | `Read` and `Seek` |
| Safely replace important state | Temporary file, flush, sync, rename |
| Coordinate cooperating processes | File locking |
| Work inside a Tokio application | `tokio::fs` or `spawn_blocking` |
| Traverse directories recursively | `walkdir` |

## 23. Suggested learning order

Learn these topics in this order:

1. `fs::read_to_string`, `fs::read`, and `fs::write`
2. `File::open` and `File::create`
3. `Read` and `Write`
4. `OpenOptions`
5. `BufReader` and `BufWriter`
6. `Path` and `PathBuf`
7. Directories and metadata
8. `Seek` and binary files
9. Error context and size limits
10. Atomic writes, syncing, locking, and async file operations

These concepts cover the majority of file-handling work in practical Rust applications.
