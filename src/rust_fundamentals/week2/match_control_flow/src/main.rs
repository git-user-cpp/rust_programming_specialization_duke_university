use std::io;

fn main() {
    println!("Please enter a greeting:");
    let mut greet = String::new();
    io::stdin().read_line(&mut greet).expect("Failed to read input");

    match greet.trim() {
        "Good Bye" => println!("Sorry to see you go."),
        "Hello" => println!("Hi, nice to meet you!"),
        _ => println!("I can't find a greeting, good bye."),
    }
}
