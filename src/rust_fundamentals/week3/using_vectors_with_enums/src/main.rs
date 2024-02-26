enum Shape {
    Circle(f64),
    Square(f64),
}

fn main() {
    let shapes = vec![Shape::Circle(5.0), Shape::Square(3.0), Shape::Circle(2.0)];

    let total_area: f64 = shapes.iter().map(|shape| match shape {
        Shape::Circle(radius) => std::f64::consts::PI * (radius * radius),
        Shape::Square(length) => length * length,
    }).sum();

    println!("Total area: {:.2} sq. units.", total_area);
}