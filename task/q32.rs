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

fn main() {
    todo!("Q32 implementation")
}
