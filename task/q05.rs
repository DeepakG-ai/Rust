struct Employee {
    name: String,
    monthly_salary: f64,
    years: u32,
}

impl Employee {
    // new is similiar to constructor, but it will not call automatically.
    fn new(name: &str, monthly_salary: f64, years: u32) -> Employee {
        Employee {
            name: name.to_string(),
            monthly_salary,
            years,
        }
    }

    
    // Methods borrow `self` via `&self` to read struct fields
    fn annual_salary(&self) -> f64 {
        self.monthly_salary * 12.0
    }

    // Returns true if 5 years or more
    fn is_senior(&self) -> bool {
        self.years >= 5
    }

    // Formats summary string combining name, annual salary, and seniority label
    fn summary(&self) -> String {
        let seniority = if self.is_senior() { "senior" } else { "not senior" };
        format!("{}: {} per year ({})", self.name, self.annual_salary(), seniority)
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