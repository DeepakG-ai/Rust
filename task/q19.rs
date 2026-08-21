use std::thread::sleep;
use std::time::Duration;

fn call_api(url: &str) -> Result<String, String> {
    for attempt in 1..=3 {
        match fake_request(url, attempt) {
            Ok(body) => return Ok(body),
            Err(e) => {
                println!("attempt {}/3 failed: {} — retrying in 2s", attempt, e);
                if attempt < 3 {
                    sleep(Duration::from_secs(2));
                }
            }
        }
    }
    Err(String::from("all 3 attempts failed"))
}

fn fake_request(url:&str,attempt:u32)->Result<String,String> {
    if attempt <3{
        Err(String::from("connection refused"))
    } else{
        Ok(format!("200 ok from {url}"))
    }
}

fn main() {
    match call_api("https://api.example.com/data") {
        Ok(body) => println!("Success: {}", body),
        Err(e) => println!("Error: {}", e),
    }
}
