use my_lib::Person;

fn main() {
    let mut person = Person::new(String::from("Adam"), 22);
    println!("{:?}",  person);

    println!("Name: {}", person.get_name());
    println!("Age: {}", person.get_age());

    person.set_name(String::from("Madam"));
    println!("New name: {}", person.get_name());

    person.set_age(15);
    println!("New age: {}", person.get_age());
}

