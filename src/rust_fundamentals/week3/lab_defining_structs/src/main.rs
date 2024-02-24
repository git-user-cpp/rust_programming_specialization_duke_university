use std::ops::Add;

#[derive(Debug)]
struct Person {
    first_name: String,
    last_name: String,
    age: u8,
    email: String,
    phone_number: u64,
}

impl Person {
    fn display_full_name(&self) {
        println!("{}", self.first_name.clone().add(&self.last_name));
    }
}

fn main() {
    let person = Person {
        first_name: "John".to_string(),
        last_name: "Doe".to_string(),
        age: 25,
        email: "sillyemail@gmail.com".to_string(),
        phone_number: 1111111111
    };

    person.display_full_name();

    println!("{:?}", person);
}
