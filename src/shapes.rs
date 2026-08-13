trait Shape {
    fn area(&self) -> u32;
}

struct Rect {
    width: u32,
    height: u32,
}
impl Shape for Rect {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

struct Circle {
    radius: u32,
}
impl Shape for Circle {
    fn area(&self) -> u32 {
        self.radius * self.radius
    }
}

fn main() {
    let r = Rect {
        width: 10,
        height: 20,
    };
    let c = Circle { radius: 10 };

    println!("Area of rectangle: {}", r.area());
    println!("Area of circle: {}", c.area());
    //println!("Area of rectangle: {}", get_area(r));
    //println!("Area of circle: {}", get_area(c));
}
// get_area(s: impl Shape) -> u32 {
// return s.area();
//}
