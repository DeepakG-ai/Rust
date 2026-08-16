fn main() {
    let temps = [-10.0, 0.0, 25.0, 37.5];
    for c in temps {
        println!("{}", describe(c));
    }
    
}

fn celsius_to_fahrenheit(c: f64) -> f64 {
    c * (9.0 / 5.0) + 32.0
}

fn is_freezing(c: f64) -> bool {
    c <= 0.0
}

fn describe(c: f64) -> String {
    let f = celsius_to_fahrenheit(c);
    let label = if is_freezing(c) { "freezing" } else { "not freezing" };
    format!("{c}C = {f}F ({label})")
}
 