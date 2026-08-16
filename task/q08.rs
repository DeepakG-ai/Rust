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
}

fn find_by_name<'a>(employees: &'a Vec<Employee>, name: &str) -> Option<&'a Employee> {
    for i in employees {
        if name == i.name {
            return Some(i);
        }
    }
    return None;
}

fn highest_paid(employees: &Vec<Employee>) -> Option<&Employee> {
    let mut highest_salary: f64 = 0.0;
    let mut highest_emp: Option<&Employee> = None; // track the employee, not the name

    for i in employees {
        if i.monthly_salary > highest_salary {
            highest_salary = i.monthly_salary;
            highest_emp = Some(i); // remember this employee
        }
    }

    return highest_emp; // return after checking ALL employees
}

fn main() {
    let employees = vec![
        Employee::new("Asha", 100000.0, 6),
        Employee::new("Deepak", 45000.0, 8),
        Employee::new("Bob", 30000.0, 2),
    ];

    match highest_paid(&employees) {
        None => println!("not found"),
        Some(i) => println!("Highest paid name is {}", i.name),
    }

    let name1 = String::from("Deepak");

    match find_by_name(&employees, &name1) {
        None => println!("Name is Not found"),
        Some(i) => println!("Name Found {}", i.name),
    }

    // search for someone who doesn't exist
    match find_by_name(&employees, "Nobody") {
        None => println!("Name is Not found"),
        Some(i) => println!("Name Found {}", i.name),
    }
}
