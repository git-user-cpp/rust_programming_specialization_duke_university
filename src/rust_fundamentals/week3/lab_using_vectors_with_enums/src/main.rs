enum Shape {
    Circle(f64),
    Square(f64),
    Triangle(f64, f64),
}

fn largest(shapes: &Vec<Shape>) {
    let mut largest_area = 0.0;
    
    for shape in shapes {
        let area = match shape {
        Shape::Circle(radius) => std::f64::consts::PI * radius * radius,
        Shape::Square(length) => length * length,
        Shape::Triangle(base, height) => (base * height) / 2.0,
        };
        if area > largest_area {
            largest_area = area;
        }
    }
    println!("Largest area: {}", largest_area);
}

fn main() {
    let shapes = vec![Shape::Circle(5.0), Shape::Square(3.0), Shape::Triangle(10.0, 5.0)];

    largest(&shapes);

    let total_area: f64 = shapes
    .iter()
    .map(|shape| match shape {
        Shape::Circle(radius) => std::f64::consts::PI * radius * radius,
        Shape::Square(length) => length * length,
        Shape::Triangle(base, height) => (base * height) / 2.0,
    })
    .sum();

    println!("Total area: {} sq. units", total_area);
}