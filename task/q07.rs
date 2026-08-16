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
