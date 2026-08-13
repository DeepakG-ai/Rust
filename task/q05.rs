struct Employee{
    name:String,
    monthly_salary:f64,
    years:u32,

}

impl Employee {
    fn new(name:&str,monthly_salary:f64,years:u32)->Employee{
        return Employee{
            name:name.to_string(),
            monthly_salary,
            years,
        }
    }

    fn deposit(&mut self,amount:f64)->f64{
        self.monthly_salary * (self.years as f64)
    }

    fn print_details(&self)->String{
        format!()
    }
}

fn main(){

}