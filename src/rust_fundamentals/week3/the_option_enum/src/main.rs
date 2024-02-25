fn divide(x: i32, y: i32) -> Option<i32> {
    if y == 0 {
        None
    } else {
        Some(x / y)
    }
}

fn main() {
    let a = 10;
    let b = 2;

    let result: Option<i32> = divide(a, b);

    match result {
        Some(x) => println!("Result: {}", x),
        None => println!("Error: division by zero"),
    }

    println!("Result: {}", result.unwrap());
}