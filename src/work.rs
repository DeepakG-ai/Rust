trait Describe {
    fn describe(&self) -> String;
}

struct Employee {
    name: String,
    monthly_salary: f64,
}

impl Describe for Employee {
    fn describe(&self) -> String {
        return format!("{} monthly salary is {}", self.name, self.monthly_salary);
    }
}

struct Product {
    name: String,
    price: f64,
}

impl Describe for Product {
    fn describe(&self) -> String {
        return format!("{} price is {}", self.name, self.price);
    }
}

fn description<T: Describe>(item: &T) {
    println!("{}", item.describe())
}

fn main() {
    let e = Employee {
        name: String::from("deepak"),
        monthly_salary: 100000.0,
    };

    let p = Product {
        name: String::from("laptop"),
        price: 50000.0,
    };

    description(&e);
    description(&p);
}
