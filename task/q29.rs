// ============================================================================
// ## Q29 — Reading and writing files
//
// Completely absent from Q1–Q25, and it is in almost every real program.
//
// Tasks:
// 1. Write `notes.txt` containing 5 lines, some of which contain the word `error`
//    in mixed case (e.g., "Error: disk full", "warning", "fatal ERROR occurred").
// 2. Write `fn count_errors(path: &str) -> std::io::Result<usize>` that opens the
//    file with `BufReader` and counts lines containing `error`, case-insensitively.
// 3. Append a timestamped line to `runs.log` every time the program runs.
// 4. Make it not load the whole file into memory (use streaming/buffered lines).
//
// Questions to answer in a comment:
// 1. What is the difference between `File::open` and `BufReader::new(File::open(..)?)`
//    in terms of syscalls?
// 2. Which of `File::create` and `OpenOptions::new().append(true)` destroys existing data?
//
// Hint:
// - `use std::io::BufRead;` for `.lines()`
// - `use std::io::Write;` for `writeln!`
// Two different traits, and forgetting the import is the most common error here.
// ============================================================================

use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, BufReader, Write};
use std::time::{SystemTime, UNIX_EPOCH};

/// Opens the file at `path` using `BufReader` and counts the number of lines
/// that contain the word "error" (case-insensitively), without loading the whole
/// file into memory at once.
fn count_errors(path: &str) -> io::Result<usize> {
    let file = File::open(path)?;
    // BufReader buffers reads (default 8 KB), streaming chunks into memory
    // rather than loading the entire file into a giant String or Vec.
    let reader = BufReader::new(file);
    let mut count = 0;

    for line_result in reader.lines() {
        let line = line_result?; // Propagate I/O error if a read fails
        if line.to_lowercase().contains("error") {
            count += 1;
        }
    }

    Ok(count)
}

fn main() -> io::Result<()> {
    println!("=== 1. Writing 5 lines to notes.txt ===");
    // Task 1: Write notes.txt containing 5 lines with mixed-case "error"
    // File::create creates the file or truncates it if it already exists.
    let mut notes_file = File::create("notes.txt")?; //truncate(true) underthe hood
    writeln!(notes_file, "Line 1: System booted successfully")?;
    writeln!(
        notes_file,
        "Line 2: Error: disk space running low on /dev/sda1"
    )?;
    writeln!(
        notes_file,
        "Line 3: Warning: high memory consumption detected"
    )?;
    writeln!(
        notes_file,
        "Line 4: fatal ERROR: database connection pool exhausted"
    )?;
    writeln!(
        notes_file,
        "Line 5: Info: temporary error resolved automatically"
    )?;
    println!("Successfully wrote 5 lines to notes.txt.\n");

    println!("=== 2. Streaming lines & counting errors with BufReader ===");
    // Task 2 & 4: Count errors case-insensitively using streaming BufReader
    let errors = count_errors("notes.txt")?;
    println!("Found {errors} lines containing 'error' (case-insensitively) in notes.txt\n");

    println!("=== 3. Appending timestamped line to runs.log ===");
    // Task 3: Append to runs.log every time the program runs using OpenOptions
    let mut log_file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("runs.log")?;

    let epoch_secs = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();

    writeln!(
        log_file,
        "[Unix Timestamp: {epoch_secs}s] Run completed: {errors} errors found in notes.txt"
    )?;
    println!("Appended entry to runs.log.\n");

    // Display current runs.log entries
    println!("=== Current contents of runs.log ===");
    let log_reader = BufReader::new(File::open("runs.log")?);
    for (idx, line) in log_reader.lines().enumerate() {
        println!("  {}: {}", idx + 1, line?);
    }

    Ok(())
}

// ============================================================================
// Q29 Questions Answered:
//
// 1. What is the difference between `File::open` and `BufReader::new(File::open(..)?)`
//    in terms of syscalls?
//
//    - `File::open` creates an UNBUFFERED file handle.
//      Every read operation (e.g., reading a byte or asking for a line) issues a
//      direct Operating System system call (`read()` on Unix/Linux, `ReadFile` on Windows).
//      Syscalls are expensive because each one requires a context switch between
//      user space and kernel space. Reading a file byte-by-byte or line-by-line
//      directly from a `File` would trigger thousands of OS context switches.
//
//    - `BufReader::new(...)` wraps the file handle with an in-memory buffer (8 KB by default).
//      When you request the first line or byte, `BufReader` makes ONE single syscall
//      to read up to 8,192 bytes into its memory buffer. Subsequent calls to `.lines()`
//      or `.read_line()` slice directly from this in-memory buffer with zero OS syscalls.
//      Another syscall is only made when the 8 KB buffer becomes empty.
//      This reduces the number of syscalls by several orders of magnitude.
//
// 2. Which of `File::create` and `OpenOptions::new().append(true)` destroys existing data?
//
//    - `File::create` DESTROYS existing data.
//      Under the hood, `File::create(path)` is shorthand for:
//      `OpenOptions::new().write(true).create(true).truncate(true).open(path)`
//      The `truncate(true)` flag causes the OS to immediately truncate the file length
//      to 0 bytes if the file already exists, completely erasing all prior contents
//      (equivalent to Python's `"w"` mode).
//
//    - `OpenOptions::new().create(true).append(true)` PRESERVES existing data.
//      It does not set `truncate(true)`. If the file already exists, existing contents
//      remain intact, and all new writes are appended at the end of the file
//      (equivalent to Python's `"a"` mode).
// ============================================================================
