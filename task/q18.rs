// ============================================================================
// ## Q18 — splitting code across files
//
// This one is about project structure, not logic. Make three files:
// - task/q18/main.rs        <- fn main()
// - task/q18/employee.rs    <- the Employee struct and its impl
// - task/q18/payroll.rs     <- functions that work on Vec<Employee>
//
// Rules to discover:
// - anything you want visible from another file needs `pub`
// - `pub struct` is not enough — each field you access from outside also needs `pub`
// - in `payroll.rs`, reach the other module with `use crate::employee::Employee;`
//
// Functions:
// - `fn total_annual(staff: &Vec<Employee>) -> f64`
// - `fn seniors(staff: &Vec<Employee>) -> Vec<&Employee>`
//
// Build it with: `rustc q18/main.rs -o q18.exe`
// ============================================================================

fn main() {
    // See task/q18/main.rs for the multi-file module implementation.
}
