use crate::employee::Employee;

pub fn total_annual(staff: &Vec<Employee>) -> f64 {
    let mut total = 0.0;
    for i in staff {
        total += i.monthly_salary * 12.0;
    }
    total
}

pub fn seniors(staff: &Vec<Employee>) -> Vec<&Employee> {
    let mut result = Vec::new();
    for emp in staff {
        if emp.years >= 5 {
            result.push(emp);
        }
    }
    result
}
