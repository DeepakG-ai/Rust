# Production Rust Iterators: A Deep Dive Through Grok Build and Codex

This guide is the production-focused continuation of [iterator.md](iterator.md).
The earlier note introduces iterator syntax. This guide studies why real Rust
applications choose a particular iterator, what happens to ownership, where
errors go, and when a plain loop is the better design.

## Repositories studied

### Grok Build

- Repository: `C:\Users\aigroup5\PycharmProjects\grok-build`
- Product: Grok Build, a terminal AI coding agent and TUI
- Source revision: `91d8cf309110a3b879c1b8198f7525aed545dfb4`
- Workspace edition: Rust 2024
- Scale inspected: more than 2,300 Rust source files under `crates/` and `prod/`
- Scope of examples: application code, not invented toy examples

### Codex

- Repository: `C:\Users\aigroup5\PycharmProjects\codex`
- Product: Codex CLI, a local coding agent
- Git revision: `230791fd1f255b9bd5ca5228326239db980f08dd`
- Workspace edition: Rust 2024
- Scale inspected: more than 3,100 Rust source files under `codex-rs/`

The line links in each part refer to those exact local snapshots. Line numbers
may move when either repository is updated.

---

## 1. The mental model to keep in your head

An iterator is a state machine that can produce one next item:

```rust
trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;
}
```

Three details matter:

1. `next` needs `&mut self` because asking for an item changes the iterator's
   position.
2. `Some(item)` means an item was produced; `None` means this call produced no
   item.
3. Most useful methods such as `map`, `filter`, `take`, and `enumerate` return a
   new iterator instead of immediately doing all the work.

Strictly speaking, the base `Iterator` trait does not promise that an iterator
can never return `Some` after returning `None`. `FusedIterator` expresses that
stronger promise. Collection iterators commonly implement it, and the concrete
sources used by the custom iterators below naturally stay exhausted. Generic code
must still not assume fusion unless the type promises `FusedIterator`.

### Iterator versus IntoIterator

These two traits answer different questions:

- `Iterator`: "How do I produce the next item?"
- `IntoIterator`: "How can this value be converted into an iterator?"

A `for` loop operates on `IntoIterator`, conceptually like this:

```rust
let mut iterator = IntoIterator::into_iter(expression);
while let Some(item) = iterator.next() {
    // loop body
}
```

This is why all of the following work:

```rust
for value in values { /* owns each T */ }
for value in &values { /* borrows each &T */ }
for value in &mut values { /* borrows each &mut T */ }
```

The expression after `in` determines the ownership mode.

### Lazy adapters and eager consumers

An adapter changes an iterator. A consumer drives it.

| Category | Common methods | Does it drive the source? |
|---|---|---:|
| Adapter | `map`, `filter`, `chain`, `zip`, `enumerate`, `skip`, `take` | No |
| Short-circuit consumer | `find`, `find_map`, `any`, `all`, `position` | Only as far as needed |
| Full consumer | `collect`, `fold`, `sum`, `count`, `for_each` | Usually to the end |
| Control-flow consumer | `try_for_each`, `collect::<Result<...>>()` | Until the first error |

For example, this does nothing observable because no consumer asks for an item:

```rust
let pipeline = values.iter().map(|value| expensive(value));
```

The work starts when `pipeline` is used by `collect`, a `for` loop, or another
consumer.

---

## 2. Ownership is part of the pipeline's type

For a `Vec<T>`:

| Starting expression | Iterator item | Can use the Vec afterward? | Typical purpose |
|---|---|---:|---|
| `values.iter()` or `&values` | `&T` | Yes | inspect values |
| `values.iter_mut()` or `&mut values` | `&mut T` | Yes, after the borrow ends | edit in place |
| `values.into_iter()` or `values` | `T` | No | consume or transfer ownership |

Follow the `Item` type through every stage. That habit explains most iterator
compiler errors.

```rust
let names: Vec<String> = load_names();

let lengths = names.iter()        // Item = &String
    .map(|name| name.len())        // Item = usize
    .collect::<Vec<_>>();          // Vec<usize>

println!("{}", names.len());      // valid: names was only borrowed
```

Compare it with ownership transfer:

```rust
let normalized = names.into_iter() // Item = String
    .map(|name| name.to_lowercase())
    .collect::<Vec<_>>();

// names cannot be used here; its Strings moved into the iterator.
```

Important nuance: `filter` receives a reference to each iterator item because it
must test the item and still be able to yield it. If `iter()` already yields
`&T`, a filter predicate conceptually receives `&&T`. Rust's deref coercions and
pattern matching often hide that extra reference.

---

## 3. Case study: nested filtering and short-circuiting

Grok Build's settings search splits a query into words, examines every setting,
and keeps a setting only when every query word is present in its search text.

Source: [settings/registry.rs:439](../../grok-build/crates/codegen/xai-grok-pager/src/settings/registry.rs#L439)

```rust
let q = query.to_lowercase();
let words: Vec<&str> = q.split_whitespace().collect();

self.entries
    .iter()
    .filter(|m| {
        let haystack = build_search_haystack(m);
        words.iter().all(|w| haystack.contains(w))
    })
    .collect()
```

Read the types from left to right:

```text
&Vec<SettingMeta>
  -> Iter<'_, SettingMeta>
  -> Item = &SettingMeta
  -> filter keeps selected &SettingMeta values
  -> collect builds Vec<&SettingMeta>
```

Production lessons:

- No `SettingMeta` is cloned. The returned vector contains borrowed references.
- `all` short-circuits on the first missing word.
- The inner `all` expresses AND search; replacing it with `any` would implement
  OR search.
- `all` on an empty iterator is `true`. The real function explicitly handles an
  empty query first, making that edge case obvious to readers.
- `build_search_haystack` allocates a `String` for each examined setting. The
  iterator itself is lazy, but code inside its closure can still allocate.

This is an important correction to the slogan "iterators do not allocate."
Adapters normally do not allocate storage for the sequence, but closures and
consumers still can.

---

## 4. Case study: combining two owned sources with `chain`

When Grok Build discovers instruction files, it combines home-level roots and
project-level roots while attaching a flag to each item.

Source: [prompt/agents_md.rs:253](../../grok-build/crates/codegen/xai-grok-agent/src/prompt/agents_md.rs#L253)

```rust
let roots = home_roots
    .into_iter()
    .map(|root| (root, false))
    .chain(project_roots.into_iter().map(|root| (root, true)));

for (root, is_project) in roots {
    // discover files under root
}
```

Why this shape is useful:

- Both vectors are finished being built, so consuming them is appropriate.
- `into_iter` moves each root instead of cloning it.
- `map` enriches the two sources with their origin.
- `chain` preserves precedence: all home roots are visited before project roots.
- No temporary combined `Vec` is created. The `for` loop pulls directly from
  the first source and then the second.

`chain` requires both sides to yield the same item type. Mapping each side to
`(root, bool)` creates that common type.

This is typical production iterator design: normalize different sources to one
item type, then join them lazily.

---

## 5. Case study: `filter_map` defines a failure policy

Later in the same discovery function, Grok Build consumes candidate files and
keeps only those it can read.

Source: [prompt/agents_md.rs:286](../../grok-build/crates/codegen/xai-grok-agent/src/prompt/agents_md.rs#L286)

```rust
candidates
    .into_iter()
    .filter_map(|candidate| {
        let content = std::fs::read_to_string(&candidate.path).ok()?;
        let file_name = candidate
            .path
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("AGENTS.md")
            .to_string();

        Some(AgentConfigFile {
            file_name,
            file_path: candidate.path.display().to_string(),
            content,
        })
    })
    .collect()
```

`filter_map` expects the closure to return `Option<U>`:

- `Some(config)` yields one output item.
- `None` yields no output item and iteration continues.

The `?` here is operating on `Option`, not `Result`. The transformation is:

```text
read_to_string: io::Result<String>
              .ok(): Option<String>
                  ?: return None from this closure on failure
```

This is not merely clever syntax. It is a product decision: unreadable
instruction files are omitted rather than failing the whole discovery operation.
In code where a missing file must be reported, converting the error to `None`
would be wrong; use a fallible pipeline instead.

Use `filter_map` when zero-or-one output item is the actual domain model, not as
a way to hide errors accidentally.

---

## 6. Case study: collecting an iterator of `Result`s

Wire-format tool entries must all be valid. Grok Build consumes the entries,
attaches each source index, converts them, and aborts on the first invalid entry.

Source: [registry/proto_convert.rs:61](../../grok-build/crates/codegen/xai-grok-tools/src/registry/proto_convert.rs#L61)

```rust
let tools = entries
    .into_iter()
    .enumerate()
    .map(|(idx, entry)| tool_config_from_entry(idx, entry))
    .collect::<Result<Vec<_>, _>>()?;
```

The type trace is the important part:

```text
entries.into_iter()                         Item = ToolConfigEntry
.enumerate()                                Item = (usize, ToolConfigEntry)
.map(tool_config_from_entry)                Item = Result<ToolConfig, Error>
.collect::<Result<Vec<_>, _>>()             Result<Vec<ToolConfig>, Error>
?                                           Vec<ToolConfig> or early return
```

`Result` implements `FromIterator` specially. Collecting results:

- pushes every `Ok(value)` into the output collection;
- stops at the first `Err(error)`;
- drops any partially built vector;
- returns that error.

The index is not just for display. `enumerate` adds source context so a conversion
error can identify the offending entry.

Compare the main failure policies:

| Pipeline | On an error |
|---|---|
| `.filter_map(|x| parse(x).ok())` | discard bad item, continue |
| `.map_while(Result::ok)` | stop at first error, discard the error |
| `.map(parse).collect::<Result<Vec<_>, _>>()` | stop and return the error |
| `.try_for_each(|x| validate(x))` | stop and return the error, no output collection |

Choosing among these is business logic, not style.

---

## 7. Case study: returning `impl Iterator` from an API

Grok Build's metadata wrapper exposes borrowed string pairs without exposing its
`BTreeMap<String, String>` representation.

Source: [workspace-types/metadata.rs:106](../../grok-build/crates/codegen/xai-grok-workspace-types/src/metadata.rs#L106)

```rust
pub fn iter(&self) -> impl Iterator<Item = (&str, &str)> {
    self.0.iter().map(|(k, v)| (k.as_str(), v.as_str()))
}
```

This design provides several benefits:

- The caller gets a lazy view, not an allocated `Vec`.
- The caller can add `filter`, `find`, `collect`, or a `for` loop.
- The concrete adapter type remains private.
- The returned references cannot outlive `&self`; the inferred lifetime connects
  them to the metadata value.
- Static dispatch lets the compiler specialize and inline the chain.

The hidden return type is still one concrete type chosen by the function. If
different branches produce unrelated iterator types, common solutions are:

1. normalize the branches with `Option::into_iter`, `empty`, `once`, `chain`, or
   `Either`-style wrappers;
2. return a collected container if the API truly needs ownership;
3. use `Box<dyn Iterator<Item = T> + 'a>` when runtime polymorphism is worth the
   allocation and dynamic dispatch.

Do not collect merely to avoid spelling an iterator type. `impl Iterator` is
usually the better public read-only view.

---

## 8. Case study: `Option` is a zero-or-one iterator

MCP initialization has a set of handshaking servers in two states and no set in
the third state. The code returns one iterator type without allocating.

Source: [mcp/servers.rs:205](../../grok-build/crates/codegen/xai-grok-mcp/src/servers.rs#L205)

```rust
match self {
    Self::Starting { handshaking } | Self::Finished { handshaking } => {
        Some(handshaking.iter())
    }
    Self::NotStarted => None,
}
.into_iter()
.flatten()
```

Work through the layers:

```text
match result             Option<hash_set::Iter<'_, McpServerName>>
.into_iter()             zero or one hash-set iterator
.flatten()               zero or many &McpServerName values
```

This pattern avoids returning a boxed trait object and avoids allocating an
empty vector. The same idea appears in simpler chains:

```rust
required
    .into_iter() // Option<T> becomes 0 or 1 T
    .chain(optional_items)
```

`Result<T, E>` can also be iterated as zero-or-one `T`, but doing that silently
discards the error. That is much less often desirable.

---

## 9. Case study: lazy precedence with `chain` and `filter`

An MCP state owns some clients and inherits others. Owned clients must come
first, and an inherited client must be omitted if its name is overridden.

Source: [mcp/servers.rs:890](../../grok-build/crates/codegen/xai-grok-mcp/src/servers.rs#L890)

```rust
pub fn all_clients(&self) -> impl Iterator<Item = (&McpServerName, &Arc<McpClient>)> {
    self.owned_clients.iter().chain(
        self.shared_clients
            .iter()
            .filter(|(name, _)| !self.owned_clients.contains_key(name.as_str())),
    )
}
```

The iterator encodes three invariants directly:

1. owned entries appear first;
2. shared entries appear second;
3. owned names win conflicts.

Notice that the filter closure borrows another field of `self`. The returned
iterator therefore borrows `self`; callers cannot mutably change these maps while
that iterator is alive.

The snapshot code then decides where ownership is required:

```rust
clients: state
    .all_clients()
    .map(|(k, v)| (k.clone(), Arc::clone(v)))
    .collect(),
```

The API stays borrowed and lazy. Cloning happens only at the snapshot boundary,
where an independent owned map is actually required. `Arc::clone` increments a
reference count; it does not clone the full client.

---

## 10. Case study: search backward with `rev().find_map()`

A conversation can contain many enum variants. Grok Build wants the most recent
assistant response, not merely the last item.

Source: [sampling-types/conversation.rs:773](../../grok-build/crates/codegen/xai-grok-sampling-types/src/conversation.rs#L773)

```rust
pub fn assistant(&self) -> Option<&AssistantItem> {
    self.items.iter().rev().find_map(|item| match item {
        ConversationItem::Assistant(a) => Some(a),
        _ => None,
    })
}

pub fn assistant_mut(&mut self) -> Option<&mut AssistantItem> {
    self.items.iter_mut().rev().find_map(|item| match item {
        ConversationItem::Assistant(a) => Some(a),
        _ => None,
    })
}
```

Why `find_map` instead of `filter(...).map(...).next()`?

- The test and extraction are one pattern match.
- It stops at the first match from the back.
- Pattern matching safely narrows the enum variant.
- The mutable version returns a reference to the object in place, with no clone.

`rev` is available because a slice iterator implements `DoubleEndedIterator`.
Not every iterator can be reversed; the compiler requires that capability.

This is a powerful domain pattern:

```rust
events.iter().rev().find_map(|event| match event {
    Event::TheVariant(data) => Some(data),
    _ => None,
})
```

---

## 11. Case study: a stateful `filter` closure

The memory search subsystem tokenizes a query, removes noise, and deduplicates
while preserving first-seen order.

Source: [memory/query_expansion.rs:168](../../grok-build/crates/codegen/xai-grok-memory/src/query_expansion.rs#L168)

```rust
let lowered = query.to_lowercase();
let mut seen = HashSet::new();

lowered
    .split(|c: char| !c.is_alphanumeric() && c != '_')
    .filter(|w| w.len() >= 2)
    .filter(|w| !STOP_WORDS.contains(w))
    .filter(|w| !w.chars().all(|c| c.is_numeric()))
    .filter(|w| seen.insert(*w))
    .map(|w| w.to_string())
    .collect()
```

The surprising line is:

```rust
.filter(|w| seen.insert(*w))
```

`HashSet::insert` returns `true` only for a new value. The closure mutates
captured state, which works because iterator predicates accept `FnMut`, not only
`Fn`.

Lifetime and allocation details:

- `lowered` must remain alive because the split words are `&str` slices into it.
- `seen` stores those borrowed slices only during this function.
- The final `to_string` is the point where each accepted word becomes owned.
- Deduplication preserves the query's order, unlike collecting into a set and
  iterating the set afterward.

Stateful closures are useful, but state hidden inside a very long chain can become
hard to reason about. Give the state a clear name and consider a loop when there
are multiple mutations or branches.

---

## 12. Case study: `scan`, `enumerate`, `skip`, and `take`

The read-file tool needs line text, one-based line numbers, and exact byte offsets.
It computes all three in one lazy traversal.

Source: [read_file/mod.rs:257](../../grok-build/crates/codegen/xai-grok-tools/src/implementations/grok_build/read_file/mod.rs#L257)

```rust
for (i, (pos, line_len, line)) in file_content
    .split_inclusive('\n')
    .scan(0, |pos, line| {
        let out = *pos;
        let line_len = line.len();
        *pos += line_len;
        Some((out, line_len, strip(line)))
    })
    .enumerate()
    .skip(skip)
    .take(take)
{
    // render the requested line range
}
```

`scan` is like a stateful `map` that may also stop:

```rust
scan(initial_state, |state, item| -> Option<output>)
```

For each line, the closure:

1. reads the current cumulative byte position;
2. advances it by this line's byte length;
3. yields the old position, length, and stripped text.

Then:

- `enumerate` supplies the zero-based line index;
- `skip` discards lines before the requested offset;
- `take` stops after the requested count;
- the `for` loop consumes the lazy pipeline.

Ordering matters. Moving `skip` before `scan` would make `pos` relative to the
visible range instead of the original file. Moving `enumerate` after `skip` would
renumber the visible range from zero. A chain is a data-flow program; adapter
order is semantics.

---

## 13. Case study: move values into two groups with `partition`

The active-session registry removes every session from its existing vector,
separates live sessions from crashed sessions, restores the live set, and returns
the crashed set.

Source: [active_sessions.rs:71](../../grok-build/crates/codegen/xai-grok-shell/src/active_sessions.rs#L71)

```rust
let (alive, dead): (Vec<_>, Vec<_>) = sessions
    .drain(..)
    .partition(|session| is_pid_alive(session.pid));

*sessions = alive;
dead
```

This is stronger than iterating by reference:

- `drain(..)` removes and yields owned `ActiveSession` values.
- `partition` consumes every value into exactly one output collection.
- No session object needs to be cloned.
- Two vectors are allocated because two owned results are required.

The explicit type annotation tells `partition` which two collection types to
build. `partition` is powered by `Default + Extend`, so it is not limited to
vectors.

Use this pattern when ownership must be split. If you only need to remove
unwanted items and do not need them afterward, `retain` is simpler.

---

## 14. Case study: `zip`, `take_while`, and a Unicode boundary

The suggestion UI computes the common byte prefix of two UTF-8 strings.

Source: [suggestion_controller/mod.rs:768](../../grok-build/crates/codegen/xai-grok-pager/src/views/suggestion_controller/mod.rs#L768)

```rust
fn common_str_prefix<'a>(a: &'a str, b: &str) -> &'a str {
    let mut n = a
        .bytes()
        .zip(b.bytes())
        .take_while(|(x, y)| x == y)
        .count();

    while n > 0 && !a.is_char_boundary(n) {
        n -= 1;
    }
    &a[..n]
}
```

What each stage does:

- `bytes()` produces the raw UTF-8 bytes.
- `zip` pairs corresponding bytes and naturally stops at the shorter string.
- `take_while` stops at the first unequal pair.
- `count` turns the matching prefix into a byte length.

The loop afterward is essential. Equal byte prefixes can end in the middle of a
multi-byte UTF-8 character, and slicing a `str` there would panic. Production
iterator code still needs domain invariants; a compact chain does not make byte
operations automatically Unicode-safe.

General `zip` warning: it silently stops when either iterator ends. If unequal
lengths indicate corrupted data, check lengths first or use a stricter strategy.

---

## 15. Case study: short-circuit side effects with `try_for_each`

The image generation client validates and inserts user-supplied HTTP headers.

Source: [image_gen/mod.rs:116](../../grok-build/crates/codegen/xai-grok-tools/src/implementations/grok_build/image_gen/mod.rs#L116)

Simplified to emphasize the iterator behavior:

```rust
extra_headers.into_iter().try_for_each(|(key, value)| {
    let header_name = HeaderName::from_bytes(key.as_bytes())
        .map_err(convert_header_error)?;
    let header_value = HeaderValue::from_str(value)
        .map_err(convert_value_error)?;
    headers.insert(header_name, header_value);
    Ok::<(), ToolError>(())
})?;
```

`try_for_each` is the fallible form of consuming a sequence for side effects. It
stops at the first error. There is no output collection because the desired
result is the mutation of `headers`.

A normal `for` loop with `?` would also be excellent here:

```rust
for (key, value) in extra_headers {
    let name = parse_name(key)?;
    let value = parse_value(value)?;
    headers.insert(name, value);
}
```

The iterator form is valuable when it keeps a short validation pipeline local.
Use the loop when the body needs several branches, logging, `continue`, or more
than one kind of state change.

One transactional warning: values inserted before a later invalid header remain
in the local `headers` map. Here that is safe because the enclosing constructor
returns an error and drops the local map. If the mutation targeted shared state,
validate everything before committing or implement rollback.

---

## 16. Case study: implementing a custom adapter

Grok Build wraps the markdown parser's event stream to change how single-tilde
strikethrough is interpreted.

Source: [markdown-core/lib.rs:33](../../grok-build/crates/codegen/xai-grok-markdown-core/src/lib.rs#L33) and
[markdown-core/lib.rs:71](../../grok-build/crates/codegen/xai-grok-markdown-core/src/lib.rs#L71)

```rust
pub fn offset_events(text: &str)
    -> impl Iterator<Item = (Event<'_>, Range<usize>)> + '_
{
    DoubleTildeOnlyStrike {
        text,
        events: Parser::new_ext(text, parser_options()).into_offset_iter(),
    }
}

impl<'a, I> Iterator for DoubleTildeOnlyStrike<'a, I>
where
    I: Iterator<Item = (Event<'a>, Range<usize>)>,
{
    type Item = (Event<'a>, Range<usize>);

    fn next(&mut self) -> Option<Self::Item> {
        let (event, range) = self.events.next()?;
        match &event {
            Event::Start(Tag::Strikethrough)
                if !is_double_tilde_strike(self.text, &range) =>
            {
                Some(strike_delim_text(self.text, &range, true))
            }
            _ => Some((event, range)),
        }
    }
}
```

The real implementation also handles the closing event. Its architecture is the
lesson:

- The adapter owns another iterator, `events`.
- Its generic bound specifies the exact upstream item type.
- `self.events.next()?` propagates upstream exhaustion: `None` from the inner
  iterator becomes `None` from the wrapper.
- Most events pass through unchanged; selected events are transformed.
- The event values and ranges remain borrowed from the original input text.
- The public function hides the concrete wrapper behind `impl Iterator`.

This is streaming transformation. It does not parse everything into a vector,
rewrite the vector, and return another vector. One call to `next` pulls one
upstream event and returns one downstream event.

### A state-machine iterator

Another custom iterator scans environment-variable references while storing a
byte cursor:

Source: [hooks/env_expand.rs:243](../../grok-build/crates/codegen/xai-grok-hooks/src/env_expand.rs#L243)

```rust
pub(crate) fn iter_env_var_references(input: &str) -> EnvVarRefIter<'_> {
    EnvVarRefIter { input, pos: 0 }
}

pub(crate) struct EnvVarRefIter<'a> {
    input: &'a str,
    pos: usize,
}

impl<'a> Iterator for EnvVarRefIter<'a> {
    type Item = EnvVarRef<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        // Advance self.pos until one complete reference is found.
        // Return Some(reference), or None at end of input.
    }
}
```

This is when a custom iterator is justified: parsing one result requires durable
state across calls, and callers benefit from stopping early. A helper returning
`Vec<EnvVarRef>` would eagerly scan and allocate storage for every match even if
the caller only wanted `.find(...)` or `.take(1)`.

When implementing a custom iterator, review:

- Does every successful `next` call advance the state?
- Can malformed input cause an infinite loop?
- Are byte indices always valid string boundaries before slicing?
- What happens after exhaustion?
- Can `size_hint`, `DoubleEndedIterator`, `ExactSizeIterator`, or `FusedIterator`
  be implemented honestly?

Only promise those extra traits when their contracts are actually true.

---

## 17. Case study: making a domain type collectable

The metadata wrapper implements `FromIterator`, which lets ordinary pipelines
collect directly into the domain type.

Source: [workspace-types/metadata.rs:132](../../grok-build/crates/codegen/xai-grok-workspace-types/src/metadata.rs#L132)

```rust
impl<K, V> FromIterator<(K, V)> for Metadata
where
    K: Into<String>,
    V: Into<String>,
{
    fn from_iter<I: IntoIterator<Item = (K, V)>>(iter: I) -> Self {
        Self(
            iter.into_iter()
                .map(|(k, v)| (k.into(), v.into()))
                .collect(),
        )
    }
}
```

Now callers can write:

```rust
let metadata: Metadata = [
    ("x-session", session_id),
    ("traceparent", trace_id),
]
.into_iter()
.collect();
```

Design details:

- `FromIterator` is what powers `Iterator::collect`.
- Accepting `K: Into<String>` and `V: Into<String>` makes the boundary flexible.
- The function itself accepts any `IntoIterator`, not only a pre-built iterator.
- The inner `BTreeMap` determines duplicate-key behavior: later insertions replace
  earlier values.
- The `BTreeMap` also gives deterministic key order, an explicit production
  requirement in this type.

Implement `FromIterator` when collecting into a domain type is natural and its
duplicate/order semantics are well defined.

---

## 18. A production codebase does not replace every loop

Grok Build often combines iterator searches with a normal loop. Its file-tool
override logic mutates each configured tool, performs nested lookup, and skips
nonmatching entries.

Source: [agent/config.rs:1434](../../grok-build/crates/codegen/xai-grok-agent/src/config.rs#L1434)

```rust
for tool in self.tool_config.tools.iter_mut() {
    let Some(slot) = FILE_TOOL_SLOTS
        .iter()
        .find(|slot| slot.contains(&tool.id.as_str()))
    else {
        continue;
    };

    if let Some(replacement) = file_tools
        .iter()
        .find(|candidate| slot.contains(&candidate.id.as_str()))
    {
        *tool = replacement.clone();
    }
}
```

The loop is clearer because it has:

- in-place mutation;
- an early `continue`;
- nested searches;
- a conditional assignment.

Trying to force all of that into `filter_map(...).for_each(...)` would not make
the code more functional or more efficient. The production lesson is:

> Use iterator chains for a readable flow of values. Use loops for readable
> control flow. They compose well together.

---

## 19. Smaller production patterns worth recognizing

### `fold` for a custom reduction

The memory re-ranker computes score extrema:

```rust
let max_score = relevance
    .iter()
    .copied()
    .fold(f64::NEG_INFINITY, f64::max);
```

Source: [memory/mmr.rs:77](../../grok-build/crates/codegen/xai-grok-memory/src/mmr.rs#L77)

`copied` changes `Item = &f64` into `Item = f64`. `fold` carries one accumulator
through the sequence. Its explicit identity also defines behavior for an empty
input. Use `reduce` instead when the first item should be the initial accumulator
and an empty input should produce `None`.

### Reverse destructive edits

Plugin discovery removes losing candidates by descending index:

```rust
to_remove.sort_unstable();
to_remove.dedup();
for idx in to_remove.into_iter().rev() {
    candidates.remove(idx);
}
```

Source: [plugins/discovery.rs:788](../../grok-build/crates/codegen/xai-grok-agent/src/plugins/discovery.rs#L788)

Removing from highest to lowest preserves the meaning of remaining indices.
`into_iter` consumes the now-finished index vector, and `rev` requires its vector
iterator to be double-ended.

### Borrowed adjacent windows

Permission code detects protected `.git/hooks` path components:

```rust
components
    .windows(2)
    .any(|pair| pair == [".git", "hooks"])
```

Source: [permission/shell_access.rs:499](../../grok-build/crates/codegen/xai-grok-workspace/src/permission/shell_access.rs#L499)

`windows(2)` yields overlapping borrowed slices and allocates nothing. It is the
right tool for adjacent pairs, byte signatures, and local ordering checks. It
panics if the window size is zero.

### Stop reading text after the first I/O error

Prompt-history loading uses:

```rust
reader
    .lines()
    .map_while(Result::ok)
    .filter(|line| !line.trim().is_empty())
    .filter_map(|line| serde_json::from_str(&line).ok())
    .collect::<Vec<_>>()
```

Source: [prompt_history.rs:75](../../grok-build/crates/codegen/xai-grok-shell/src/session/prompt_history.rs#L75)

There are two different failure policies in four lines:

- an I/O error stops iteration because `map_while` stops on `None`;
- malformed JSON is skipped because `filter_map` continues after `None`.

The errors themselves are not returned. That may be appropriate for best-effort
history recovery, but it would be dangerous in an accounting or migration path.

---

## 20. Performance: what "zero cost" does and does not mean

Iterator adapters are generic concrete types. In optimized builds, Rust can
monomorphize, inline, and fuse them into efficient loops. That is why they are
widely used in performance-sensitive production Rust.

Do not turn that into the stronger claim that every iterator chain always has
identical assembly to every hand-written loop. Check the real costs:

1. **Allocation boundaries:** `collect::<Vec<_>>()`, `to_string`, `format!`, and
   `to_lowercase` allocate. `map`, `filter`, and `take` themselves normally do
   not allocate a sequence.
2. **Cloning:** `.iter().cloned()` duplicates items; `.into_iter()` can move them.
3. **Dynamic dispatch:** `Box<dyn Iterator>` normally adds a heap allocation for
   the box and an indirect `next` call. `impl Iterator` uses static dispatch.
4. **Short-circuiting:** `find`, `any`, `all`, `take_while`, and fallible consumers
   can avoid examining the rest of the input.
5. **Collection growth:** good `size_hint` implementations let consumers reserve
   more accurately. Filters cannot know their exact final length.
6. **Data structure work:** the `HashSet` lookups, map lookups, parsing, file I/O,
   and string creation inside closures often dominate adapter overhead.
7. **Debug versus release:** judge performance with an optimized build and a
   representative benchmark.

Prefer the clearest correct form first. Measure a hot path before rewriting it.

---

## 21. Common mistakes when reading or writing production iterators

### Mistake 1: forgetting the consumer

```rust
values.iter().map(|value| log(value)); // lazy and unused
```

Use a `for` loop for side effects, or consume the iterator explicitly.

### Mistake 2: cloning because reference types were not traced

Before adding `.clone()`, write down the `Item` type at each stage. Often the
function can return references, or `into_iter` can transfer ownership.

### Mistake 3: consuming a collection too early

```rust
let selected = values.into_iter().filter(predicate).collect::<Vec<_>>();
println!("{}", values.len()); // moved value
```

Use `iter()` if later code still needs the original collection.

### Mistake 4: using `map` for filtering

- `map`: exactly one output for every input.
- `filter`: either keep the unchanged input or drop it.
- `filter_map`: zero or one transformed output.
- `flat_map`: zero or many transformed outputs.

### Mistake 5: accidentally hiding errors

`.ok()` erases an error. That can be deliberate in discovery or recovery paths,
but it should make you ask, "Where is this failure observed?"

### Mistake 6: assuming `zip` checks equal length

It does not. It stops at the shorter side.

### Mistake 7: mutating a collection while iterating over it

You may mutate an item through `iter_mut`, but you generally cannot push to or
remove from the same collection while its iterator borrows it. Use `retain`,
`drain`, collect edits first, or restructure into phases.

### Mistake 8: forcing a complicated loop into adapters

If a closure needs several mutations, `continue`-like branches, logging, and
multiple early exits, a `for` loop usually communicates the algorithm better.

### Mistake 9: confusing `Option::map` with `Iterator::map`

Production Rust uses the same method names across abstractions:

```rust
option.map(f);              // transforms zero or one optional value
result.map(f);              // transforms the Ok value
iterator.map(f);            // lazily transforms a sequence
```

Look at the receiver's type, not only the method name.

### Mistake 10: returning a lazy iterator that borrows a temporary

This cannot work:

```rust,compile_fail
fn words() -> impl Iterator<Item = &'static str> {
    String::from("temporary words").split_whitespace()
}
```

The temporary `String` is dropped when the function returns. Return owned values
or make the input borrow come from the caller.

---

## 22. How to read an unfamiliar iterator chain

Use this five-pass method on production code.

### Pass 1: identify the source

Is it a vector, slice, map, range, `Option`, database row stream, file lines, or a
custom iterator?

### Pass 2: write the `Item` type after every stage

```text
Vec<Entry>
into_iter              -> Entry
enumerate              -> (usize, Entry)
map(parse)             -> Result<Config, Error>
collect                -> Result<Vec<Config>, Error>
```

### Pass 3: mark ownership

For each stage, ask whether values are borrowed, mutably borrowed, moved, copied,
or cloned.

### Pass 4: find the consumer and stop condition

Does the chain run fully, stop at the first match, stop at a limit, or stop at an
error?

### Pass 5: mark hidden effects

Look inside closures for allocation, mutation, I/O, locking, logging, and error
suppression. Laziness describes scheduling, not purity.

If you cannot explain those five points, you do not yet understand the chain.

---

## 23. Method-selection cheat sheet

| Goal | Prefer | Important behavior |
|---|---|---|
| Borrow every item | `iter()` | yields `&T` |
| Mutate items in place | `iter_mut()` | yields exclusive `&mut T` |
| Transfer ownership | `into_iter()` | yields `T` |
| Transform one-to-one | `map` | lazy |
| Keep matching inputs | `filter` | predicate borrows each item |
| Transform and possibly omit | `filter_map` | closure returns `Option<U>` |
| Transform one-to-many | `flat_map` | flattens each returned iterable |
| First transformed match | `find_map` | short-circuits |
| Require every predicate | `all` | short-circuits on false |
| Require any predicate | `any` | short-circuits on true |
| Combine sequential sources | `chain` | first source, then second |
| Pair parallel sources | `zip` | stops at shorter source |
| Add positions | `enumerate` | positions start at zero |
| Inspect neighbors | `windows` | overlapping borrowed slices |
| Carry custom state | `scan` | stateful map that may stop |
| Reduce to one value | `fold` | explicit initial accumulator |
| Split owned inputs | `partition` | builds two collections |
| Gather fallible outputs | `collect::<Result<Vec<_>, _>>()` | first error wins |
| Apply fallible side effects | `try_for_each` | first error wins |
| Expose a lazy API | `impl Iterator<Item = T>` | concrete type hidden |
| Make a domain type collectable | `FromIterator` | defines collection semantics |

---

## 24. Exercises based on the production patterns

Do these in order. For every solution, annotate the item type after each adapter.

### Exercise 1: borrowing search results

Given `Vec<Tool>`, return `Vec<&Tool>` containing enabled tools whose names contain
a query. Do not clone a `Tool` or `String`.

### Exercise 2: owned normalization

Consume `Vec<String>` and return a `BTreeSet<String>` of trimmed, lowercase,
nonempty values. Explain exactly why the input vector is unavailable afterward.

### Exercise 3: error policy

Parse `Vec<String>` as integers in three ways:

1. skip invalid entries;
2. stop at the first invalid entry and return the error;
3. preserve one result per input as `Vec<Result<i32, _>>`.

### Exercise 4: last matching enum variant

Given a vector of `Event::{User, Assistant, Tool}`, return a reference to the most
recent `Assistant` value using `rev().find_map()`.

### Exercise 5: precedence merge

Lazily chain local and inherited key-value maps, omitting inherited keys that are
present locally. Return `impl Iterator`; do not allocate a combined vector.

### Exercise 6: custom iterator

Implement `EveryOther<I>` so it yields one upstream item, skips the next, and
repeats. Then decide whether it can implement `FusedIterator` for every `I`.

### Exercise 7: scan with offsets

Use `split_inclusive('\n').scan(...)` to produce `(byte_offset, line)` values.
Verify it with ASCII and multi-byte UTF-8 text.

### Exercise 8: loop versus chain

Rewrite a chain with two mutable captured variables and three conditional branches
as a `for` loop. Compare which version makes invariants easier to see.

### Exercise 9: strict zip

Write `strict_zip(a, b)` that returns an error when the iterables have different
lengths instead of silently truncating. Start with slices, then make it generic.

### Exercise 10: collect into a domain type

Create `Headers(BTreeMap<String, String>)`, implement `FromIterator<(K, V)>`, and
document what happens when a key occurs twice.

---

## 25. What Grok Build teaches about industry Rust

The strongest lessons from this codebase are architectural:

1. Iterator choice communicates ownership. Borrowing, mutation, and consumption
   are visible at the start of the pipeline.
2. Failure behavior is encoded in the combinator: `filter_map`, `map_while`,
   fallible `collect`, and `try_for_each` are meaningfully different.
3. Lazy iterator-returning APIs keep policy with the caller and avoid needless
   temporary collections.
4. `Option`, map views, file lines, parser events, ranges, and custom scanners all
   participate in the same iterator vocabulary.
5. Stateful algorithms can use `scan`, mutable closures, and custom `Iterator`
   implementations without exposing internal state.
6. Production code freely mixes iterator adapters and loops. Readability and
   invariants decide the form.
7. Allocation and cloning occur at deliberate ownership boundaries, not merely
   because an iterator chain ended.
8. Compact syntax never replaces domain checks such as Unicode boundaries,
   precedence rules, equal-length assumptions, and transactional mutation.

The goal is not to write the longest possible chain. The goal is to make data
flow, ownership, stopping behavior, and failure policy obvious to the next
engineer.

---

## Part II: How Codex uses iterators

Codex uses the same foundational adapters as Grok Build, but several patterns are
especially prominent:

| Codex design need | Iterator technique |
|---|---|
| Configuration precedence | lazy `DoubleEndedIterator` APIs |
| Conversation-history structure | `peekable` + `from_fn` |
| Retry protocols | `map(Some)` + `chain(once(None))` |
| Directory traversal | `successors` + `from_fn` |
| Concurrent filesystem probing | convert an iterator into an async `Stream` |
| Permission inheritance | `rev().try_fold(...)` |
| Tool capability sets | domain-specific `FromIterator` + `fold` |
| Markdown rendering | stacked custom iterator adapters |
| Runtime-dependent JSON shapes | `Box<dyn Iterator>` |
| Inclusive ancestor boundaries | stateful `scan` |

These are not isolated tricks. They show four broad architectural uses:

1. an iterator can be a public view over private state;
2. an iterator can be a protocol or state machine;
3. an iterator can be the synchronous input to asynchronous work;
4. an iterator can carry a domain invariant such as precedence, exact length, or
   short-circuit failure.

### C1. Configuration order is an API guarantee

Codex stores configuration layers in precedence order and exposes lazy views in
both directions.

Source: [config/state.rs:478](../../codex/codex-rs/config/src/state.rs#L478)

```rust
pub fn layers_low_to_high(&self) -> impl DoubleEndedIterator<Item = &ConfigLayerEntry> {
    self.all_layers_low_to_high()
        .filter(|layer| !layer.is_disabled())
}

pub fn layers_high_to_low(&self) -> impl DoubleEndedIterator<Item = &ConfigLayerEntry> {
    self.layers_low_to_high().rev()
}

pub fn all_layers_low_to_high(&self) -> impl DoubleEndedIterator<Item = &ConfigLayerEntry> {
    self.layers.iter()
}
```

This return type says more than `impl Iterator`:

- callers may consume from the front or the back;
- callers may use `rev()` without collecting first;
- the configuration data remains borrowed;
- the underlying vector remains private;
- disabled-layer filtering remains centralized.

Codex then merges layers from low to high:

```rust
let mut merged = TomlValue::Table(toml::map::Map::new());
for layer in self.layers_low_to_high() {
    merge_toml_values(&mut merged, &layer.config);
}
```

The ordering is business logic: later, higher-precedence values override earlier
values. A caller searching for the winning value can instead traverse high to
low and stop at the first match.

An opaque return type can expose the capabilities callers need without exposing
the concrete adapter type. Useful return contracts include:

```text
impl Iterator<Item = T>
impl DoubleEndedIterator<Item = T>
impl ExactSizeIterator<Item = T>
impl Iterator<Item = T> + Clone
```

Promise only capabilities the implementation can preserve.

### C2. `ExactSizeIterator` exposes known length without exposing storage

Codex's template type exposes placeholder names as borrowed strings:

Source: [utils/template/lib.rs:170](../../codex/codex-rs/utils/template/src/lib.rs#L170)

```rust
pub fn placeholders(&self) -> impl ExactSizeIterator<Item = &str> {
    self.placeholders.iter().map(String::as_str)
}
```

The slice iterator knows its exact remaining length, and one-to-one `map`
preserves that property. Callers can therefore use both iterator methods and
`ExactSizeIterator::len()`:

```rust
let placeholders = template.placeholders();
let count = placeholders.len();
```

This is stronger and cheaper than returning a new `Vec<&str>`. It also avoids
returning `&Vec<String>`, which would leak representation and expose owned string
types when callers only need string slices.

`size_hint()` and `ExactSizeIterator` are related but not identical. Any iterator
can provide lower and upper estimates through `size_hint`; `ExactSizeIterator`
promises that the exact remaining length is known.

### C3. `peekable` and `from_fn` build a grouping state machine

Remote-history compaction treats an image-resize notice as attached to the item
immediately before it. Codex groups that pair without allocating nested vectors.

Source: [core/compact_remote_history.rs:15](../../codex/codex-rs/core/src/compact_remote_history.rs#L15)

```rust
impl<T: Borrow<ResponseItem>> HistoryItemGroup<T> {
    pub(crate) fn into_items(self) -> impl Iterator<Item = T> {
        std::iter::once(self.source).chain(self.attached_notice)
    }
}

pub(crate) fn history_item_groups<I>(items: I)
    -> impl Iterator<Item = HistoryItemGroup<I::Item>>
where
    I: IntoIterator,
    I::Item: Borrow<ResponseItem>,
{
    let mut items = items.into_iter().peekable();
    std::iter::from_fn(move || {
        let source = items.next()?;
        let attached_notice = items.next_if(|notice| is_attached_notice(notice.borrow()));
        Some(HistoryItemGroup {
            source,
            attached_notice,
        })
    })
}
```

The mechanics are precise:

1. `IntoIterator` accepts a vector, slice iterator, or any other compatible
   source.
2. `peekable` supports inspecting the following item.
3. `next_if` consumes that following item only when it is an attached notice.
4. `from_fn` stores the captured `items` state between calls.
5. `?` propagates source exhaustion as `None` from the generated iterator.

Flattening a group uses another small iterator insight:

```rust
std::iter::once(self.source).chain(self.attached_notice)
```

`Option<T>` implements `IntoIterator<Item = T>`, so the chain yields the source
followed by zero or one notice.

This is a parser-like state machine implemented from standard adapters. A custom
struct implementing `Iterator` would also work. `from_fn` is attractive when the
state is local, the algorithm is short, and callers do not need to name the
iterator type.

### C4. A final sentinel turns retry policy into an iterable protocol

Codex's MCP HTTP initialization retries after each configured delay and then
makes one final attempt with no delay available afterward.

Source: [rmcp-client/streamable_http_retry.rs:43](../../codex/codex-rs/rmcp-client/src/streamable_http_retry.rs#L43)

```rust
for (attempt, retry_delay_ms) in STREAMABLE_HTTP_RETRY_DELAYS_MS
    .iter()
    .copied()
    .map(Some)
    .chain(std::iter::once(None))
    .enumerate()
{
    // Try the connection.
    // On failure, Some(delay) means sleep and retry.
    // None means this was the final attempt.
}
```

Type trace:

```text
slice.iter()                    Item = &u64
.copied()                       Item = u64
.map(Some)                      Item = Option<u64>, always Some so far
.chain(once(None))              N retry delays followed by one final sentinel
.enumerate()                    Item = (usize, Option<u64>)
```

If there are `N` retry delays, the loop performs `N + 1` connection attempts.
After a retryable failure, the body uses:

```rust
let Some(retry_delay_ms) = retry_delay_ms else {
    return Err(error);
};
```

This eliminates separate "initial attempt," "retry attempts," and "final
attempt" branches. The iterator describes the complete retry protocol and avoids
off-by-one arithmetic.

The loop is still preferable to `try_for_each`: its body awaits work, carries
transport state, updates a deadline, logs attempts, and has multiple return
paths. The iterator organizes inputs; the loop expresses control flow.

### C5. Synchronous iterators feed bounded asynchronous concurrency

Codex searches ancestors for marker files. It first creates a synchronous lazy
sequence of probes, then converts that sequence into an asynchronous stream.

Source: [file-system/find_up.rs:73](../../codex/codex-rs/file-system/src/find_up.rs#L73)

```rust
let mut ancestors = std::iter::successors(Some(start), parent);
let mut ancestor = ancestors.next();
let mut marker_index = 0;

let probes = std::iter::from_fn(move || {
    let current_ancestor = ancestor.clone()?;
    let marker = markers.get(marker_index)?;
    let marker_path = marker_path(&current_ancestor, marker);

    marker_index += 1;
    if marker_index == markers.len() {
        marker_index = 0;
        ancestor = ancestors.next();
    }

    Some((current_ancestor, marker_path))
});
```

`successors(Some(start), parent)` produces:

```text
start, parent(start), parent(parent(start)), ...
```

`from_fn` combines each ancestor with every marker. It is effectively a lazy
nested loop whose state is `ancestor + marker_index`.

The important boundary follows:

```rust
let mut results = futures::stream::iter(probes)
    .map(|(ancestor, marker_path)| async move {
        // asynchronously stat marker_path
    })
    .buffered(MAX_CONCURRENT_PROBES);
```

After `futures::stream::iter(probes)`, methods such as `map` come from
`StreamExt`, not `Iterator`. A `Stream` is the asynchronous cousin of an iterator:

```text
Iterator::next()        synchronous pull
Stream::poll_next()     asynchronous readiness-based pull
```

`.buffered(limit)` allows several filesystem checks to be in flight while
yielding their results in source order. Preserving order matters because the
nearest matching ancestor should win. An unordered concurrency adapter could
return a farther ancestor merely because its I/O finished sooner.

This is a major production pattern: build selection/order synchronously with an
iterator, then cross into async only where I/O begins.

### C6. `rev().try_fold()` implements fallible inheritance

Permission profiles can extend parent profiles. Codex first walks the parent
chain, explicitly detects cycles, and then merges from the root parent toward the
selected child.

Source: [config/permissions_toml.rs:40](../../codex/codex-rs/config/src/permissions_toml.rs#L40)

```rust
if let Some(cycle_start) = profile_names
    .iter()
    .position(|name| name == &next_profile_name)
{
    let cycle = profile_names[cycle_start..]
        .iter()
        .cloned()
        .chain(std::iter::once(next_profile_name))
        .collect::<Vec<_>>();
    return Err(PermissionProfileResolutionError::Cycle { cycle });
}

let profile = profiles
    .into_iter()
    .rev()
    .try_fold(profile, merge_permission_profiles)?;
```

There are two iterator jobs here:

- `position` short-circuits when a repeated profile name reveals a cycle;
- `try_fold` accumulates merged configuration and stops at the first invalid
  merge.

Why `rev`? The traversal discovers profiles child-to-parent, but inheritance must
be applied parent-to-child so the child wins conflicts.

Conceptually:

```text
discovered: child -> parent -> root
merge order: root -> parent -> child
```

`fold` has an accumulator type `A` and closure `FnMut(A, Item) -> A`.
`try_fold` replaces that result with `Result<A, E>` or another `Try` type, making
early failure part of the reduction.

The surrounding `loop` is a good design choice. Parent lookup, error construction,
cycle detection, and state updates are clearer as control flow; only the final
linear merge becomes an iterator reduction.

### C7. `FromIterator` can collect into a bit-field, not only a container

Codex models the surfaces where a tool may be exposed as bit flags. Collecting a
sequence of individual surfaces unions the flags.

Source: [tools/tool_executor.rs:14](../../codex/codex-rs/tools/src/tool_executor.rs#L14)

```rust
impl FromIterator<ToolExposureSurface> for ToolExposures {
    fn from_iter<T: IntoIterator<Item = ToolExposureSurface>>(surfaces: T) -> Self {
        surfaces
            .into_iter()
            .fold(Self::NONE, |exposures, surface| exposures | surface.into())
    }
}
```

This enables:

```rust
let exposures: ToolExposures = configured_surfaces.into_iter().collect();
```

The target is not a `Vec`, `HashSet`, or map. It is one small domain value.
`FromIterator` means "this type knows how to aggregate a sequence," not
necessarily "this type stores every item."

The algebra is especially clean:

- identity value: `ToolExposures::NONE`;
- combine operation: bitwise OR;
- duplicates: harmless;
- order: irrelevant.

Whenever a custom `FromIterator` uses `fold`, identify its identity, duplicate
semantics, and whether order matters.

### C8. Fallible collection validates borrowed data before an API boundary

AWS request signing needs every HTTP header to be representable as text. Codex
borrows header names and values, validates them, and returns the first invalid
value.

Source: [aws-auth/signing.rs:17](../../codex/codex-rs/aws-auth/src/signing.rs#L17)

```rust
let signable_headers = request
    .headers
    .iter()
    .map(|(name, value)| {
        Ok::<_, AwsAuthError>((
            name.as_str(),
            value.to_str().map_err(AwsAuthError::InvalidHeaderValue)?,
        ))
    })
    .collect::<Result<Vec<_>, _>>()?;

let signable_request = SignableRequest::new(
    request.method.as_str(),
    request.url.as_str(),
    signable_headers.into_iter(),
    SignableBody::Bytes(request.body.as_ref()),
)
.map_err(AwsAuthError::SigningRequest)?;
```

Ownership details:

- `headers.iter()` borrows the original header map;
- the tuple contains borrowed `&str` values, not cloned strings;
- `collect::<Result<Vec<_>, _>>()` validates the full input or returns the first
  error;
- `signable_headers.into_iter()` moves those small reference tuples into the
  signing API;
- the original request must stay alive because the strings remain borrowed.

The temporary vector is deliberate. It crosses from a fallible conversion into
an API that expects ordinary valid header pairs. Not every `collect` is waste;
sometimes it is the validation and ownership boundary.

### C9. `Box<dyn Iterator>` handles runtime-dependent concrete types

A doctor report can store checks in either a current JSON object or a legacy JSON
array. Those collection views have different concrete iterator types.

Source: [app-server/feedback_doctor_report.rs:172](../../codex/codex-rs/app-server/src/request_processors/feedback_doctor_report.rs#L172)

```rust
fn check_values(checks: &Value) -> Box<dyn Iterator<Item = &Value> + '_> {
    match checks {
        Value::Array(values) => Box::new(values.iter()),
        Value::Object(values) => Box::new(values.values()),
        _ => Box::new(std::iter::empty()),
    }
}
```

Why not `impl Iterator`? An opaque `impl Trait` return still requires one concrete
return type. These branches produce:

```text
slice::Iter<Value>
serde_json::map::Values<String, Value>
iter::Empty<&Value>
```

Boxing erases those differences behind dynamic dispatch. The tradeoffs are a
small heap allocation for the iterator object and an indirect `next` call.

This is a reasonable choice because:

- the shape is chosen at runtime;
- the path processes diagnostic JSON, not a proven CPU hotspot;
- it avoids allocating and cloning all values into a common vector;
- the compatibility logic remains localized.

Alternatives include a custom enum iterator, an `Either` type, or having each
match branch call a generic helper directly. Choose based on performance needs
and API clarity.

### C10. Codex stacks custom adapters for one-pass Markdown processing

The TUI first merges adjacent decoded text events and then tracks top-level block
boundaries. Both layers remain lazy.

Source: [tui/markdown_text_merge.rs:8](../../codex/codex-rs/tui/src/markdown_text_merge.rs#L8)

```rust
pub(crate) struct DecodedTextMerge<I: Iterator> {
    iter: Peekable<I>,
}

impl<'a, I> Iterator for DecodedTextMerge<I>
where
    I: Iterator<Item = (Event<'a>, Range<usize>)>,
{
    type Item = (Event<'a>, Range<usize>);

    fn next(&mut self) -> Option<Self::Item> {
        let (event, mut range) = self.iter.next()?;
        let Event::Text(text) = event else {
            return Some((event, range));
        };
        if !matches!(self.iter.peek(), Some((Event::Text(_), _))) {
            return Some((Event::Text(text), range));
        }

        let mut merged = text.into_string();
        while matches!(self.iter.peek(), Some((Event::Text(_), _))) {
            let Some((Event::Text(text), next_range)) = self.iter.next() else {
                break;
            };
            merged.push_str(&text);
            range.end = next_range.end;
        }
        Some((Event::Text(merged.into()), range))
    }
}
```

Unlike a one-to-one adapter, one call to this `next` may consume several upstream
events. It merges adjacent text while retaining the full combined source range.

A second adapter records block metadata while passing events through. Its core
shape is:

Source: [tui/markdown_render/streaming.rs:60](../../codex/codex-rs/tui/src/markdown_render/streaming.rs#L60)

```rust
struct TopLevelBlockTracker<I> {
    iter: I,
    depth: usize,
    block_count: usize,
    last_start: usize,
    first_is_html: bool,
}

impl<'a, I> Iterator for TopLevelBlockTracker<I>
where
    I: Iterator<Item = (Event<'a>, Range<usize>)>,
{
    type Item = (Event<'a>, Range<usize>);

    fn next(&mut self) -> Option<Self::Item> {
        let (event, range) = self.iter.next()?;
        // Update depth and block metadata.
        Some((event, range))
    }
}
```

The renderer composes them:

```rust
let parser = TopLevelBlockTracker {
    iter: DecodedTextMerge::new(parser.into_offset_iter()),
    // state fields...
};
let mut writer = Writer::new(input, parser, width, cwd, &never_hide_link_destination);
writer.run();
```

Data flow:

```text
Markdown parser
  -> decoded-text merger
  -> top-level block tracker
  -> terminal writer
```

There is one parser pass and no intermediate event vector. The outer code can
inspect tracker state after the writer finishes because the writer owns the
iterator adapter.

This demonstrates an advanced use of `Iterator`: adapters may collect side-channel
metadata while preserving the original item stream. Document such side effects
carefully because a caller must consume the iterator for the metadata to become
complete.

### C11. `scan` includes a boundary that `take_while` would exclude

Project configuration discovery walks from the current directory upward through
the project root, then reverses that list to apply layers from root to leaf.

Source: [config/loader/mod.rs:1415](../../codex/codex-rs/config/src/loader/mod.rs#L1415)

```rust
let mut dirs = cwd
    .ancestors()
    .scan(false, |done, ancestor| {
        if *done {
            None
        } else {
            if &ancestor == project_root {
                *done = true;
            }
            Some(ancestor)
        }
    })
    .collect::<Vec<_>>();

dirs.reverse();
```

The state flag means:

- yield ancestors normally;
- when the project root is encountered, yield it and mark completion;
- on the next call, return `None`.

A naive `.take_while(|ancestor| ancestor != project_root)` would exclude the
project root. The domain requires an inclusive stopping condition, so `scan`
fits.

Collection is also intentional. `ancestors()` runs leaf-to-root, but configuration
must be processed root-to-leaf. Collecting and reversing creates that order.
Could `.rev()` be used directly? Only if the source iterator implements
`DoubleEndedIterator`; an opaque ancestor iterator may not promise that property,
and `scan` itself does not generally preserve it.

### C12. Optional iterators and loops cooperate in MCP tool registration

Codex registers normal MCP tools first and conditionally appends app tools.

Source: [core/mcp_tool_exposure.rs:80](../../codex/codex-rs/core/src/mcp_tool_exposure.rs#L80)

```rust
let non_app_tools = filter_non_codex_apps_mcp_tools_only(all_mcp_tools);
let app_tools = apps_enabled
    .then(|| filter_codex_apps_mcp_tools(all_mcp_tools, config))
    .into_iter()
    .flatten();

for tool in non_app_tools.chain(app_tools) {
    // Build or reuse a handler.
    // Enforce per-tool and total byte budgets.
    // Mutate the registry and registered-name set.
    // Continue when a tool specification cannot be built.
}
```

The first half is a clean lazy pipeline:

- `bool::then` creates `Option<AppToolIterator>`;
- `Option::into_iter` produces zero or one inner iterator;
- `flatten` produces zero or many app tools;
- `chain` keeps ordinary tools before app tools.

The second half is a loop because registration involves:

- mutable maps and a registry;
- a cumulative byte budget;
- handler construction that may fail;
- conditional exposure;
- `continue` and several branches.

This is the same industry lesson seen in Grok Build: iterators express the input
sequence and ordering; a loop expresses complex stateful effects.

### C13. Iterator methods and Stream methods can look identical

Codex uses both `Iterator` and `futures::StreamExt` extensively. Method names such
as `map`, `filter`, `take`, `skip`, `chain`, and `for_each` can appear on either.

To identify which abstraction you are reading, inspect the receiver:

| Receiver | Execution model | Typical consumer |
|---|---|---|
| `values.iter()` | synchronous | `collect`, `for`, `find` |
| `std::iter::from_fn(...)` | synchronous state machine | `collect`, `for` |
| `futures::stream::iter(...)` | asynchronous stream wrapper | `.next().await` |
| network/event stream | asynchronously produced values | `.next().await`, `try_collect().await` |

An ordinary iterator does not drive asynchronous work. Mapping with an async
closure only produces futures; something else must poll and await them. Common
strategies are:

1. build synchronous work descriptions with an iterator, then convert to a
   stream;
2. use a `for` loop and `await` sequentially;
3. map stream items to futures and apply `buffered` or `buffer_unordered` with an
   explicit concurrency bound.

Be deliberate about ordering. `buffered` preserves input order;
`buffer_unordered` yields whichever future completes first.

### C14. What Codex adds to the production iterator picture

The Codex codebase reinforces and extends the Grok Build lessons:

1. **Return traits describe capabilities.** `DoubleEndedIterator` and
   `ExactSizeIterator` are useful API promises, not implementation trivia.
2. **Small iterator constructors can encode protocols.** `once`, `successors`,
   `from_fn`, and `peekable` model retries, hierarchies, and grouped records.
3. **Iterators compose across architecture layers.** A synchronous probe sequence
   becomes an async stream at the I/O boundary.
4. **Fallible reduction expresses precedence.** `rev().try_fold()` says both which
   direction values override and where processing stops.
5. **`FromIterator` is domain extensibility.** Collecting can produce a bit-field
   or another aggregate rather than a conventional container.
6. **Trait objects solve genuinely dynamic shapes.** `Box<dyn Iterator>` is used
   selectively when runtime branches cannot share one concrete type.
7. **Custom adapters enable single-pass systems.** Markdown parsing, merging,
   metadata tracking, and rendering remain a streaming pipeline.
8. **Loops remain central.** Iterator construction handles selection and order;
   loops handle async work, mutation, budgets, logging, and multi-exit control
   flow.

Across both repositories, the production standard is not "use iterators
everywhere." It is:

> Choose a sequence abstraction whose type makes ownership, order, capabilities,
> stopping behavior, and failure policy visible.

