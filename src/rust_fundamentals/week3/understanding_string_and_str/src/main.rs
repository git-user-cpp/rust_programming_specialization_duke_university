fn print_str(s: &str) {
    let mut new_string = s.to_string();
    new_string.push_str("some other string");
    println!("{}", new_string);

    let new_string = format!("{}! other stuff here", s);
    println!("{}", new_string);
}

fn print_string(s: String) {
    println!("{}", s);
}

fn main() {
    let s = "hello, world";
    print_str(s);

    let salutation = String::from("hello");
    print_string(salutation);
}
