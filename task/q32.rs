// ============================================================================
// ## Q32 — Pattern matching beyond a basic `match`
//
// Q7 only did a simple enum match. These four forms are everywhere in real code:
//
// 1. `if let` — one arm, ignore the rest:
//    if let Some(name) = maybe_name { println!("{name}"); }
//
// 2. `let ... else` — bind or bail, no rightward drift:
//    let Some(name) = maybe_name else { return Err("no name".into()); };
//
// 3. `while let` — loop until the pattern stops matching:
//    while let Some(top) = stack.pop() { println!("{top}"); }
//
// 4. match guards and bindings (`@`):
//    match age {
//        n if n < 0   => "impossible",
//        0..=17       => "minor",
//        n @ 18..=64  => { println!("adult aged {n}"); "adult" }
//        _            => "senior",
//    }
//
// Tasks:
// 1. Write one program that uses all four on real data.
// 2. Destructure a slice:
//    match nums {
//        []                => "empty",
//        [x]               => "one item",
//        [first, .., last] => "many",
//    }
//
// Question to answer in a comment:
// - Rewrite this nested mess using `let ... else` and say which version you would
//   rather debug:
//   ```rust
//   if let Some(user) = get_user(id) {
//       if let Ok(cfg) = load_config() {
//           if cfg.enabled { do_work(user, cfg); }
//       }
//   }
//   ```
// ============================================================================

struct User {
    name: String,
    age: i32,
}

struct Config {
    enabled: bool,
}

fn get_user(id: u32) -> Option<User> {
    match id {
        1 => Some(User {
            name: "Ferris".to_string(),
            age: 25,
        }),
        _ => None,
    }
}

fn load_config(text: &str) -> Result<Config, std::str::ParseBoolError> {
    // "true" and "false" are valid; anything else returns Err.
    let enabled = text.parse::<bool>()?;
    Ok(Config { enabled })
}

fn do_work(user: User, cfg: Config) {
    println!("Processing {} (enabled={})", user.name, cfg.enabled);
}

fn run_user_job(id: u32, config_text: &str) {
    // Answer: I prefer this version to the nested if-let version because
    // each failure has its own early exit and the success path stays flat.
    // The else branch must leave this path (here, by returning).
    let Some(user) = get_user(id) else {
        println!("Skipping user {id}: user not found");
        return;
    };

    let Ok(cfg) = load_config(config_text) else {
        println!("Skipping user {id}: invalid configuration");
        return;
    };

    if !cfg.enabled {
        println!("Skipping user {id}: processing is disabled");
        return;
    }

    do_work(user, cfg);
}

fn age_group(age: i32) -> &'static str {
    match age {
        n if n < 0 => "impossible", // Guard: an extra condition on a match.
        0..=17 => "minor",
        n @ 18..=64 => {
            // @ both matches the range and binds the value to n.
            println!("Adult aged {n}");
            "adult"
        }
        _ => "senior",
    }
}

fn describe_slice(nums: &[i32]) {
    match nums {
        [] => println!("Empty slice"),
        [x] => println!("One item: {x}"),
        [first, .., last] => {
            // .. matches everything between the first and last elements.
            println!("Many items: first={first}, last={last}");
        }
    }
}

fn main() {
    // 1. if let: use the user only when lookup succeeds.
    if let Some(user) = get_user(1) {
        println!("Found {}: {}", user.name, age_group(user.age));
    }

    // 2. let ... else: demonstrate success and each early exit.
    run_user_job(1, "true");
    run_user_job(99, "true");
    run_user_job(1, "invalid");
    run_user_job(1, "false");

    // 3. while let: pop returns Some until the stack is empty.
    // A Vec used as a stack processes the last item first: 103, 102, 101.
    let mut pending_orders = vec![101, 102, 103];
    while let Some(order_id) = pending_orders.pop() {
        println!("Processing order {order_id}");
    }

    // 4. Exercise the guard, ranges, and @ binding, including boundaries.
    for age in [-1, 0, 17, 18, 64, 65] {
        println!("Age {age}: {}", age_group(age));
    }

    // Slice patterns: empty, single item, two items, and more than two.
    describe_slice(&[]);
    describe_slice(&[10]);
    describe_slice(&[10, 20]);
    describe_slice(&[10, 20, 30, 40]);
}
