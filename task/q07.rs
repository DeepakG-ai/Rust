// ============================================================================
// ## Q7 — enum and match
//
// ```rust
// enum Status {
//     Active,
//     Suspended { reason: String },
//     Closed(String),   // closing date
// }
// ```
//
// Write `fn status_message(status: &Status) -> String` using a `match` that
// returns:
// - `Active`    → `"Account is active"`
// - `Suspended` → `"Suspended: payment overdue"`
// - `Closed`    → `"Closed on 2026-01-15"`
//
// In `main`, put all three in a `Vec<Status>` and print each message.
//
// Hint: `match` must cover every variant — the compiler enforces it. That is a
// feature, not a nuisance.
// ============================================================================

enum Status {
    Active,
    Suspended { reason: String }, //Suspended(reason:String) not like this. it is curly. so status_message-> Status::Suspended { reason }
    Closed(String),
}

fn status_message(status: &Status) -> String { //status_message(Status::Active) so it match with Active,active is excuted in iter 1.
    match status {
        Status::Active => format!("Account is active"),
        Status::Suspended { reason } => format!("Suspended: {}", reason),
        Status::Closed(date) => format!("Closed on {}", date),
    }
}

fn main() {
    let statuses = vec![
        Status::Active,
        Status::Suspended { reason: String::from("payment overdue") },
        Status::Closed(String::from("2026-01-15")),
    ];

    for s in &statuses { //borrowing statuses
        println!("{}", status_message(s));
    }
}
