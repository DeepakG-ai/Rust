macro_rules! say_hello {
    () => {
        println!("Hello World");
    };
}
fn main() {
    say_hello!();
}

//custom macros similiar to vec![],println! etc.
//use std::{fmt::fromat,path::Display};
//#[derive(Debug)]  <- this is also a procedure  macro
//in underhood, it will like
//impl Debug for User{
//    fn  debug(&self){
//        println!("{} {}",self.username,self.age);
//    }
//} by using #[derive(Debug)] automatically impl will call.
struct User {
    username: String,
    password: String,
    age: u32,
}

fn main() {
    let u = User {
        username: String::from("Deepak"),
        password: String::from("password"),
        age: 32,
    };

    print!("{:?}", u); //debug u.debug_to_string
    print!("{}", u); //Display trait, u.to_string
}
