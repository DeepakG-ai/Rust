// ============================================================================
// ## Q5 — struct and methods
//
// ```rust
// struct Employee {
//     name: String,
//     monthly_salary: f64,
//     years: u32,
// }
// ```
//
// Write an `impl Employee` block with:
// - `fn new(name: &str, monthly_salary: f64, years: u32) -> Employee`
// - `fn annual_salary(&self) -> f64`
// - `fn is_senior(&self) -> bool` — 5 years or more
// - `fn summary(&self) -> String` — `"Asha: 1200000 per year (senior)"`
//
// In `main`, make a `Vec<Employee>` with 3 people and print every summary.
//
// Hint: `&self` = read the struct. `Self` is shorthand for the struct's own type.
// ============================================================================

struct Employee {
    name: String,
    monthly_salary: f64,
    years: u32,
}

impl Employee {
    fn new(name: &str, monthly_salary: f64, years: u32) -> Self {
        Self {
            name: name.to_string(),
            monthly_salary,
            years,
        }
    }

    fn annual_salary(&self) -> f64 {
        self.monthly_salary * 12.0
    }

    fn is_senior(&self) -> bool {
        if self.years > 5 {
            return true;
        } else {
            return false;
        }
    }

    fn summary(&self) -> String {
        let salary = self.annual_salary();
        let label = if self.is_senior() {
            "senior"
        } else {
            "not senior"
        };
        format!("{}:{salary} per years ({label})", self.name)
    }
}

fn main() {
    let employees = vec![
        Employee::new("Asha", 100000.0, 6),
        Employee::new("Deepak", 45000.0, 8),
        Employee::new("Bob", 30000.0, 2),
    ];

    for emp in &employees {
        println!("{}", emp.summary());
    }
}
