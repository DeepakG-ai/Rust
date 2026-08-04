struct Rect {
    width: f32,
    height: f32,
}

impl Rect {
    fn rec_area(&self) -> f32 {
        return self.width * self.height;
    }

    fn perimeter(&self) -> f32 {
        return 2.0 * self.width * self.height;
    }

    fn print_something() {
        println!("This is static method");
    }
}

fn main() {
    let r = Rect {
        width: 10.0,
        height: 3.0,
    };

    println!("{} {}", r.width, r.height);
    println!("{}", r.rec_area());
    println!("{}", r.perimeter());
    Rect::print_something();
}
