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
use std::fs;

// Model from prompt
#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct Settings {
    model: String,
    temperature: f64,
    #[serde(default)]
    verbose: bool,
}

// Strict version for Task 4: rejects unknown fields
#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(deny_unknown_fields)]
struct StrictSettings {
    model: String,
    temperature: f64,
    #[serde(default)]
    verbose: bool,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file_path = "settings.json";

    // ------------------------------------------------------------------------
    // Task 1: Write `settings.json` to disk with `serde_json::to_string_pretty`
    // ------------------------------------------------------------------------
    println!("=== Task 1: Write settings.json to disk ===");
    let setting = Settings {
        model: String::from("claude-opus-5"),
        temperature: 0.5,
        verbose: true,
    };

    let serialized_string = serde_json::to_string_pretty(&setting)?;
    fs::write(file_path, &serialized_string)?;
    println!("Successfully wrote settings to '{file_path}':\n{serialized_string}\n");

    // ------------------------------------------------------------------------
    // Task 2: Read it back into a `Settings`, change `model`, write it out again
    // ------------------------------------------------------------------------
    println!("=== Task 2: Read back into Settings, change model, and re-write ===");
    let json_data = fs::read_to_string(file_path)?;
    let mut loaded_setting: Settings = serde_json::from_str(&json_data)?;
    println!("Loaded from '{file_path}': {loaded_setting:?}");

    // Change model
    loaded_setting.model = String::from("gpt-6-astra");
    let updated_json = serde_json::to_string_pretty(&loaded_setting)?;
    fs::write(file_path, &updated_json)?;
    println!("Wrote updated settings back to '{file_path}':\n{updated_json}\n");

    // ------------------------------------------------------------------------
    // Task 3: Delete `verbose` field and re-run. Why does it still parse?
    // ------------------------------------------------------------------------
    println!("=== Task 3: Missing 'verbose' field ===");
    // Simulating the JSON file without the `verbose` key (as if deleted by hand):
    let json_without_verbose = r#"{
  "model": "gpt-4o",
  "temperature": 0.5
}"#;

    let parsed_without_verbose: Settings = serde_json::from_str(json_without_verbose)?;
    println!("Parsed JSON without 'verbose' successfully: {parsed_without_verbose:?}");
    println!(
        "-> `verbose` was automatically populated with its default value: {}\n",
        parsed_without_verbose.verbose
    );
    assert_eq!(parsed_without_verbose.verbose, false);

    // ------------------------------------------------------------------------
    // Task 4: Add extra unknown field. Why does it still parse? Make it an error.
    // ------------------------------------------------------------------------
    println!("=== Task 4: Extra unknown field ===");
    // Simulating JSON with an extra unknown key (e.g. "max_tokens"):
    let json_with_extra_field = r#"{
  "model": "gpt-4o",
  "temperature": 0.5,
  "verbose": true,
  "max_tokens": 4096
}"#;

    // 4a. Standard Settings ignores extra unknown fields by default
    let parsed_extra_default: Settings = serde_json::from_str(json_with_extra_field)?;
    println!("Standard Settings parsed without error (extra field ignored): {parsed_extra_default:?}");

    // 4b. StrictSettings with #[serde(deny_unknown_fields)] treats extra fields as error
    let parsed_extra_strict: Result<StrictSettings, _> = serde_json::from_str(json_with_extra_field);
    match parsed_extra_strict {
        Ok(res) => println!("Unexpected success: {res:?}"),
        Err(err) => {
            println!("StrictSettings produced an error as expected:\n   => {err}\n");
        }
    }

    println!("All Q31 tasks completed successfully!");
    Ok(())
}

// ============================================================================
// Q31 Questions Answered:
//
// 1. Why does it still parse when you delete `verbose` from the JSON?
//    - It still parses because of the `#[serde(default)]` attribute on `verbose: bool`.
//    - Without this attribute, Serde strictly requires every field present in the
//      struct to exist in the input JSON; missing any field triggers `missing field '<name>'`.
//    - With `#[serde(default)]`, if Serde does not find the key in the JSON, it calls
//      the type's `Default::default()` implementation.
//    - For `bool`, `Default::default()` returns `false`.
//
// 2. Why does it *also* still parse when you add an extra unknown field?
//    - By default, Serde follows the robustness principle ("be liberal in what you accept"):
//      it silently ignores and skips any JSON fields that don't match fields on the struct.
//    - To change this behavior and reject unknown keys, you add the container attribute:
//      `#[serde(deny_unknown_fields)]`.
//    - With `deny_unknown_fields`, Serde throws an error like:
//      `unknown field 'max_tokens', expected one of 'model', 'temperature', 'verbose'`.
//
// 3. Question to answer:
//    "You deploy this, and next month the API adds a new field to its response.
//     Which of your settings breaks the deploy — and is breaking what you actually want?"
//
//    - WHICH SETTING BREAKS:
//      `#[serde(deny_unknown_fields)]` breaks the deploy.
//
//    - WHY IT BREAKS:
//      In web development and microservices, APIs continuously evolve. Adding a new
//      field to a JSON response payload is standard practice for backward-compatible
//      API updates. If your client deserializes with `#[serde(deny_unknown_fields)]`,
//      the moment the upstream service introduces a new key (e.g. `"rate_limit"`,
//      `"created_at"`), all deserialization calls fail immediately, crashing or
//      breaking your running service.
//
//    - IS BREAKING WHAT YOU ACTUALLY WANT?
//      NO, breaking is almost NEVER what you want when consuming external APIs.
//      For external APIs, you want forward-compatibility: ignore unfamiliar fields
//      and parse only the data your application needs.
//
//    - WHEN WOULD YOU ACTUALLY WANT `deny_unknown_fields`?
//      `deny_unknown_fields` is ideal for local user configuration files (e.g.,
//      `settings.json`, `config.toml`, CLI inputs). If a human makes a typo
//      (e.g., writing `"temprature": 0.8` or `"verbos": true`), you want the program
//      to loudly fail with an error rather than silently ignoring the typo and using
//      wrong or default values.
// ============================================================================
