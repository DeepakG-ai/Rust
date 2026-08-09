// this is manual code for debug macro underhood #[derive(Debug)]

use std::fmt::{Debug, Display};
#[derive(Debug)] //debug macro

struct User {
    username: String,
    age: i32,
}

//impl Debug for User{} --> this is debug trait. 
fn main() {
    let u = User {
        username: String::from("Deepak"),
        age: 23,
    };
    println!("Display: {}", u); // uses Display
    println!("Debug: {:?}", u); // uses Debug
}

//there is no display macro
