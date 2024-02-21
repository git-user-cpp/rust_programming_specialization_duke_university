use std::io;

fn main() {
    println!("Please enter a greeting:");
    let mut greet = String::new();
    io::stdin().read_line(&mut greet).expect("Failed to read input");

    match greet.to_lowercase().trim() {
        "good bye" => println!("Sorry to see you go."),
        "hello" => println!("Hi, nice to meet you!"),
        _ => println!("I can't find a greeting, good bye."),
    }
}
