//use q05::Employee
struct Employee{
    name:String,
    monthly_salary:f64,
    years:u32,
}

impl Employee{
    fn new(name:&str,monthly_salary:f64,years:u32)->Employee{
        Employee{
            name:name.to_string(),
            monthly_salary,
            years,
        }
    }
}

//in struct User{ username:String} and main(){ let u = User(username:String::from("deepak"))} , to access username, use u.username

fn highest_paid(employees:&Vec<Employee>)->String{
    let mut highest_salary:f64 = 0.0;
    let mut employee_name = String::new();   // creates an actual empty string value

    for i in employees{ // entire vec ["Asha", 100000.0, 6]=employees , i.monthly_salary = 100000,i.name="Asha"
        if i.monthly_salary>highest_salary{
            highest_salary=i.monthly_salary;
            employee_name=i.name.clone();
        }
    }

    return employee_name;
}

fn main() {
    let employees = vec![
        Employee::new("Asha", 100000.0, 6),
        Employee::new("Deepak", 45000.0, 8),
        Employee::new("Bob", 30000.0, 2),
    ];

    let result = highest_paid(&employees); //&employees = [[asha,10,6],[deepak,45,8],[bob,30,2]]
    println!("Highest paid: {}", result);
}

