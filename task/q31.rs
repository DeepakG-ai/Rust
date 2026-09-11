// ============================================================================
// ## Q31 — serde on its own
//
// You already use serde in q24 and q25 via `#[derive(Serialize)]`, but never
// learned it directly.
//
// Model:
// ```rust
// #[derive(Serialize, Deserialize, Debug)]
// struct Settings {
//     model: String,
//     temperature: f64,
//     #[serde(default)]
//     verbose: bool,
// }
// ```
//
// Tasks:
// 1. Write `settings.json` to disk with `serde_json::to_string_pretty`.
// 2. Read it back into a `Settings`, change `model`, write it out again.
// 3. Delete the `verbose` field from the JSON file by hand and re-run. Why does it
//    still parse?
// 4. Add an extra unknown field to the JSON by hand. Why does it *also* still
//    parse? Now make that an error.
//
// Question to answer in a comment:
// - You deploy this, and next month the API adds a new field to its response.
//   Which of your settings breaks the deploy — and is breaking what you actually want?
//
// Hint:
// - `#[serde(default)]` for step 3
// - `#[serde(deny_unknown_fields)]` for step 4
// ============================================================================

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Settings {
    model: String,
    temperature: f64,
    #[serde(default)]
    verbose: bool,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    todo!("Q31 implementation")
}
