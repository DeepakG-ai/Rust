use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]

struct User {
    username: String,
    password: String,
}

fn main() {
    let u = User {
        username: String::from("Deepak"),
        password: String::from("123"),
    };

    let serialized_string = serde_json::to_string(&u); //The word to_string literally means: "convert this Rust object TO a String (in JSON format)".


    // let user_detial = serialized_string.unwarp(); we can use it instead of match cases
    match serialized_string {
        Ok(str) => println!("{}", str),
        Err(_) => println!("Error while converting to string"),
    }

    // Deserialization: JSON string → Rust object
    let json_str = r#"{"username":"Ravi","password":"456"}"#; // r#"..."# is a raw string (no need to escape quotes)

    let deserialized_user: Result<User, _> = serde_json::from_str(json_str); // from_str literally means: "create a Rust object FROM a String"

    match deserialized_user {
        Ok(user) => println!("Username: {}, Password: {}", user.username, user.password),
        Err(e) => println!("Error while converting from string: {}", e),
    }
}
