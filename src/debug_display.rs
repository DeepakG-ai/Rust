// this is manual code for debug macro underhood #[derive(Debug)]

use std::fmt::{Debug, Display};

struct User {
    username: String,
    age: i32,
}

impl Display for User {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "This is the user struct with age {}", self.age)
    }
}

impl Debug for User {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "This is the user struct age {},{}",
            self.age, self.username
        )
    }
}

fn main() {
    let u = User {
        username: String::from("Deepak"),
        age: 23,
    };
    println!("Display: {}", u);   // uses Display
    println!("Debug: {:?}", u);    // uses Debug
 }
