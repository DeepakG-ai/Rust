# Project 1 — `logstats`

A CLI that reads an access log and prints the summary an on-call engineer
actually wants: request count, error rate, latency percentiles, and the
busiest endpoints.

**Skills exercised:** file I/O, `&str` vs `String`, iterator pipelines, the
`HashMap` entry API, a custom error enum, sorting with tie-breaks, module
organisation, integration tests.

## Input format

Five whitespace-separated fields per line:

```
2026-01-15T10:00:00Z GET /api/users 200 45
└─ timestamp         └── method
                         └───────── path
                                    └── status
                                        └── latency in ms
```

Blank lines are skipped. Malformed lines are **reported, not fatal** — a log
analyser that dies on one bad line is useless.

## Target output

```
$ cargo run -p p1_logstats -- projects/p1_logstats/tests/fixtures/access.log

requests: 8
errors:   2 (25.0%)
latency:  p50=10ms p95=200ms p99=200ms

TOP PATHS
path             count  errors  p95_ms
/api/users           3       1      90
/health              3       0       3
/api/orders          2       1     200
```

Plus, on stderr:

```
2 malformed line(s):
  line 9: expected 5 fields, got 4
  line 10: invalid latency: abc
```

## Your job

Fill in every `todo!()` in:

| File            | What lives there                                  |
|-----------------|---------------------------------------------------|
| `src/record.rs` | `LogRecord`, `ParseError`, line parsing           |
| `src/stats.rs`  | percentiles, per-path aggregation, `Summary`      |
| `src/report.rs` | rendering the report as a `String`                |

`src/main.rs` is already written — it is just argument handling and printing.
Once the library compiles and works, the binary works.

## Run it

```bash
cargo test -p p1_logstats            # the grader
cargo run  -p p1_logstats -- projects/p1_logstats/tests/fixtures/access.log
cargo run  -p p1_logstats -- <file> --top 5
```

## Definition of done

- [ ] `cargo test -p p1_logstats` passes
- [ ] `cargo clippy -p p1_logstats -- -D warnings` is clean
- [ ] the binary prints exactly the output above for the fixture
- [ ] no `.unwrap()` anywhere in `src/` except in tests

## Stretch goals

1. Add a `--min-status 500` filter that keeps only failures.
2. Add a `--json` flag that emits the summary as JSON (hand-roll it, or add
   `serde_json` to `Cargo.toml`).
3. Stream the file line by line with `BufReader` instead of reading it all
   into memory, so a 10 GB log works. This changes your function signatures —
   notice which ones and why.
