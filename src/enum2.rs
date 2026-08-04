use std::f32::consts::PI;

enum Shape {
    Square(f32),
    Circle(f32),
    Rectangle(f32, f32),
}

fn main() {
    let shape = Shape::Square(10.0);
    let shape_circle = Shape::Circle(9.34);
    let shape_rect = Shape::Rectangle(6.5, 7.5);
    print!(
        "{} {} {}",
        calculate_area(shape),
        calculate_area(shape_rect),
        calculate_area(shape_circle)
    )
}

fn calculate_area(s: Shape) -> f32 {
    return match s {
        Shape::Circle(radius) => PI * radius * radius,
        Shape::Square(side) => side * side,
        Shape::Rectangle(width, height) => height * width,
    };
}
