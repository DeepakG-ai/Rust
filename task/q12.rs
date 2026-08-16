trait Describe{
    fn describe(&self)->String;
}

struct Employee{
    name : String,
    monthly_salary:f64,
}

impl Describe for Employee{
    fn describe(&self)->String{
        format!("{} earns per month {}",self.name, self.monthly_salary)
    }
}

struct Product{
    name:String,
    price:f64
}

impl Describe for Product{
    fn describe(&self)->String{
        format!("{} price is {}",self.name, self.price)
    }
}

fn main (){
    let employee = Employee{
        name : String::from("Deepak"),
        monthly_salary:50000.0,
    };

    let product = Product{
        name:String::from("Laptop"),
        price:52400.0,
    };

    // Q12: Put BOTH employee and product in the SAME Vec
    // Hint: Vec<Box<dyn Describe>>
    // Then loop over it and print every description

    let items: Vec<Box<dyn Describe>> = vec![
        Box::new(employee),
        Box::new(product),
    ];

    //employee and product are different types, so we will store value in heap. use the pointer, that is use of box keyword.

    for item in &items {
        println!("{}", item.describe());
    }

    // QUESTION: why does this need Box<dyn Describe> instead of Vec<T> like Q10?
    // ANSWER: (write your answer here as a comment)
}
