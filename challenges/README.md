# Rust challenges — a practice ladder

Ten graded exercises and three mini-projects, all test-driven. Every task is a
`todo!()` with a test suite that already knows the right answer, so you always
know whether you are done.

Written for someone who **understands the concepts and needs the reps** —
coming from Python, where none of this was your problem.

## Quick start

```bash
cd challenges

cargo test -p exercises ex01     # start here
cargo test -p exercises          # all ten
cargo test                       # everything, projects included
```

Open `exercises/src/ex01_ownership.rs`, replace the `todo!()`s, re-run. Red to
green, one function at a time.

## The ladder

Do them in order. Each one assumes the last one clicked.

| # | File | Topic | What you will stop getting wrong |
|---|------|-------|----------------------------------|
| 01 | `ex01_ownership.rs` | moves, borrows, `&str` vs `String` | when to clone (almost never) |
| 02 | `ex02_option_result.rs` | `Option`, `Result`, `?` | writing `match` for everything |
| 03 | `ex03_traits.rs` | traits, `dyn`, `Display`, `FromStr`, `Default` | static vs dynamic dispatch |
| 04 | `ex04_iterators.rs` | iterator chains, closures, generics | reaching for `for` loops |
| 05 | `ex05_collections.rs` | `HashMap`/`BTreeMap`/`VecDeque`, `entry` | double lookups |
| 06 | `ex06_errors.rs` | error enums, `Display`, `Error`, `Box<dyn Error>` | `unwrap()` everywhere |
| 07 | `ex07_lifetimes.rs` | `'a`, structs that borrow, zero-copy parsing | fear of lifetime annotations |
| 08 | `ex08_shared_state.rs` | `Rc`/`RefCell`, `Arc`/`Mutex`/`RwLock`, `Send`/`Sync` | reaching for `clone()` under pressure |
| 09 | `ex09_threads.rs` | `thread::spawn`, `mpsc`, scoped threads | fan-out/fan-in plumbing |
| 10 | `ex10_async.rs` | `async`/`await`, tokio, `select!`, RPITIT vs `BoxFuture` | accidentally-sequential async |

Then the projects, which combine everything:

| # | Crate | Build | Core skill |
|---|-------|-------|-----------|
| 1 | `projects/p1_logstats` | a CLI that summarises an access log | modules, parsing, iterators, custom errors |
| 2 | `projects/p2_kvstore` | a thread-safe cache with TTL | `Send`/`Sync`, `Arc<RwLock<_>>`, trait objects, injectable time |
| 3 | `projects/p3_actor` | a tokio actor with a cloneable handle | `mpsc` + `oneshot`, `select!`, cancellation, `pub(super)` |

Each project has its own `README.md` with a spec, a definition of done, and
stretch goals. Projects 2 and 3 are deliberate miniatures of patterns in the
`codex` and `grok-build` codebases you have been reading — after project 3,
`xai-hunk-tracker` will read like ordinary code.

## How to work

1. **Read the doc comment first.** It is the spec. The tests only check it.
2. **Run the test before you write anything** so you see the failure.
3. **Do not edit the tests.** If a test looks wrong, re-read the doc comment —
   the disagreement is information.
4. **Read every compiler error all the way through.** `rustc` errors are long
   because they usually contain the fix. This is the opposite of a Python
   traceback, where the last line is the whole story.
5. **When stuck for more than ~15 minutes, ask for a hint, not the answer.**
   "Why does the borrow checker reject this?" teaches you something.
   "Show me the solution" does not.

## Useful commands

```bash
cargo test -p exercises ex07                # one exercise
cargo test -p exercises ex07 -- --nocapture # let println! through
cargo test -p p1_logstats                   # one project
cargo clippy --all-targets -- -D warnings   # the style grader
cargo fmt                                   # never argue about formatting again
cargo run -p p1_logstats -- projects/p1_logstats/tests/fixtures/access.log
```

Two more worth knowing:

```bash
cargo test -- --test-threads=1   # deterministic ordering when debugging
cargo doc --open                 # your own doc comments, rendered
```

## Python → Rust survival table

| Python | Rust | The catch |
|--------|------|-----------|
| `x = y` | `let x = y;` | `y` may be **moved** — no longer usable |
| `def f(s: str)` | `fn f(s: &str)` | borrow for reading, don't take ownership |
| `s.length` / `len(s)` | `s.len()` | **bytes**, not characters |
| `d[k]` | `map[k]` / `map.get(k)` | indexing panics; `get` returns `Option` |
| `d.get(k, 0)` | `*map.get(k).unwrap_or(&0)` | |
| `d[k] += 1` | `*map.entry(k).or_insert(0) += 1` | one hash lookup, not two |
| `None` | `Option<T>` | must be unwrapped explicitly |
| `raise` / `try` | `Result<T, E>` + `?` | errors are values, not control flow |
| `[f(x) for x in xs]` | `xs.iter().map(f).collect()` | lazy until `collect` |
| `sorted(xs, key=f)` | `xs.sort_by_key(f)` | sorts **in place**, returns `()` |
| `isinstance` / ABC | traits | checked at compile time |
| `threading` | `std::thread` | no GIL — real parallelism |
| `queue.Queue` | `mpsc::channel` | closes when senders drop |
| `asyncio.gather` | `join_all` / `JoinSet` | futures are lazy; nothing runs unawaited |
| global mutable state | `Arc<Mutex<T>>` or an actor | the compiler will make you choose |

## Rules of thumb worth memorising early

- **Take `&str`, return `String`.** Borrow input, own output, unless you can
  prove the output borrows from the input.
- **`clone()` is not cheating, but it is a smell.** Write it to get moving,
  then come back and ask whether a borrow would do.
- **`unwrap()` in `src/` is a bug; in tests it is fine.** Use `expect("why")`
  when you genuinely know better than the compiler.
- **If the borrow checker is fighting you, the design is usually wrong.**
  Nine times out of ten the fix is smaller structs or shorter borrows, not
  more lifetime annotations.
- **Warnings are errors that have not bitten you yet.** Finish each exercise
  with `cargo clippy` clean.

## Progress

- [ ] ex01 ownership
- [ ] ex02 option / result
- [ ] ex03 traits
- [ ] ex04 iterators
- [ ] ex05 collections
- [ ] ex06 errors
- [ ] ex07 lifetimes
- [ ] ex08 shared state
- [ ] ex09 threads
- [ ] ex10 async
- [ ] p1 logstats
- [ ] p2 kvstore
- [ ] p3 actor
