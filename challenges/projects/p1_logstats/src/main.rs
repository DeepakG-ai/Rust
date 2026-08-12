//! CLI entry point. This file is **already written** — you do not need to
//! change it. It exists so that the moment your library compiles, you have a
//! real program to run.
//!
//! Worth reading anyway for two idioms:
//!
//! - `fn main() -> ExitCode` — how a Rust binary reports failure without
//!   `std::process::exit`, which skips destructors.
//! - errors go to stderr, data goes to stdout, so `logstats x.log | grep` works.

use std::process::ExitCode;

use logstats::parse_log;
use logstats::render;
use logstats::summarize;

const USAGE: &str = "usage: logstats <logfile> [--top N]";

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();

    let (path, top_n) = match parse_args(&args) {
        Ok(parsed) => parsed,
        Err(message) => {
            eprintln!("{message}");
            eprintln!("{USAGE}");
            return ExitCode::from(2);
        }
    };

    let text = match std::fs::read_to_string(&path) {
        Ok(text) => text,
        Err(err) => {
            eprintln!("cannot read {path}: {err}");
            return ExitCode::FAILURE;
        }
    };

    let (records, failures) = parse_log(&text);
    print!("{}", render(&summarize(&records, top_n)));

    if !failures.is_empty() {
        eprintln!();
        eprintln!("{} malformed line(s):", failures.len());
        for (line_no, err) in &failures {
            eprintln!("  line {line_no}: {err}");
        }
    }

    ExitCode::SUCCESS
}

fn parse_args(args: &[String]) -> Result<(String, usize), String> {
    let mut path: Option<String> = None;
    let mut top_n: usize = 3;
    let mut i = 0;

    while i < args.len() {
        match args[i].as_str() {
            "--top" => {
                let raw = args
                    .get(i + 1)
                    .ok_or_else(|| String::from("--top needs a number"))?;
                top_n = raw
                    .parse()
                    .map_err(|_| format!("--top expects a number, got {raw}"))?;
                i += 2;
            }
            "-h" | "--help" => return Err(String::from("logstats — access-log analytics")),
            other if other.starts_with('-') => {
                return Err(format!("unknown flag: {other}"));
            }
            other => {
                if path.is_some() {
                    return Err(String::from("only one log file at a time"));
                }
                path = Some(other.to_string());
                i += 1;
            }
        }
    }

    let path = path.ok_or_else(|| String::from("no log file given"))?;
    Ok((path, top_n))
}
