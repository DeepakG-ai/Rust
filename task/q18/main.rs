mod employee;
mod payroll;

use employee::Employee;

fn main() {
    let staff = vec![
        Employee::new("Asha", 100000.0, 6),
        Employee::new("Ravi", 80000.0, 3),
        Employee::new("Deepak", 120000.0, 8),
    ];

    println!("Total annual payroll: {}", payroll::total_annual(&staff));

    let senior_staff = payroll::seniors(&staff);
    println!("\nSenior employees (5+ years):");
    for emp in senior_staff {
        println!("  - {} ({} years)", emp.name, emp.years);
    }
}