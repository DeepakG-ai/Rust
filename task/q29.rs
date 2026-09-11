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

//fn count_errors(path: &str) -> io::Result<usize> {
//    todo!("Open file with BufReader and count lines containing 'error' case-insensitively")
//}

fn main() -> io::Result<()> {
    //let mut f = File::create("notes.txt")?; //underhood truncate(true) is also activate which destroy the data
    let mut f = OpenOptions::new()
        .append(true)
        .create(true)
        .open("notes.txt")?;
    writeln!(f, "user logged in")?;
    f.write_all(b"raw bytes is this new one\n")?;
    Ok(())
}
