// ============================================================================
// ## Q8 — Option
//
// Using the Employee struct from Q5, write:
// - `fn find_by_name<'a>(employees: &'a Vec<Employee>, name: &str) -> Option<&'a Employee>`
// - `fn highest_paid(employees: &Vec<Employee>) -> Option<&Employee>` — None when empty
//
// In `main`, handle both results with `match` — print the person when found,
// print `"not found"` when not.
//
// Hint: this is Rust's replacement for `None` in Python. There is no `null`.
// ============================================================================

struct Employee {
    name: String,
    monthly_salary: f64,
    years: u32,
}

impl Employee {
    fn new(name: &str, monthly_salary: f64, years: u32) -> Employee {
        Employee {
            name: name.to_string(),
            monthly_salary,
            years,
        }
    }

    fn find_by_name<'a>(employee: &'a Vec<Employee>, name: &str) -> Option<&'a Employee> {
        for i in employee {
            if name == i.name {
                return Some(i);
            }
        }
        return None;
    }

    fn highest_paid(employees: &Vec<Employee>) -> Option<&Employee> {
        let mut highest_salary: f64 = 0.0;
        let mut highest_emp: Option<&Employee> = None;
        for i in employees {
            if i.monthly_salary > highest_salary {
                highest_salary = i.monthly_salary;
                highest_emp = Some(i);
            }
        }
        return highest_emp;
    }
}

fn main() {
    let employees = vec![
        Employee::new("Asha", 100000.0, 6),
        Employee::new("Deepak", 45000.0, 8),
        Employee::new("Bob", 30000.0, 2),
    ];
    match Employee::find_by_name(&employees, "Deepak") {
        Some(emp) => println!("Found:{} with salary{}", emp.name, emp.monthly_salary),
        None => println!("Not Found"),
    }
}
