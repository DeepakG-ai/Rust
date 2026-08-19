pub struct Employee {
    pub name: String,
    pub monthly_salary: f64,
    pub years: u32,
}

impl Employee {
    // new is similiar to constructor, but it will not call automatically.
    pub fn new(name: &str, monthly_salary: f64,years:u32) -> Employee {
        Employee {
            name: name.to_string(),
            monthly_salary,
            years,
        
        }
    }
}

