trait Describe {
    fn describe(&self)->String;
}

struct Employee {
    name: String,
    monthly_salary: f64,
    
}

impl Describe for Employee{
    fn describe(&self)->String{
        format!("{} earns {} per month",self.name, self.monthly_salary)

    }

}
struct Product {
    name: String,
    price: f64,
}

impl Describe for Product{
    fn describe(&self)->String{
        format!("{} costs {}",self.name, self.price)
    }

}


fn print_description<T: Describe>(item: &T){
    println!("{}",item.describe());
}


fn main (){
    let e = Employee{
        name : String::from("Deepak"),
        monthly_salary:50000.0,

    };

    let p = Product{
        name:String::from("Laptop"),
        price:52400.0,
    };

    print_description(&e);
    print_description(&p);

}


