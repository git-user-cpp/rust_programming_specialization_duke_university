fn print_type_of<T>(_: &T) {
    println!("Type = {}", std::any::type_name::<T>());
}

fn ownership() {
    let numbers = vec![1, 2, 3];
    print_type_of(&numbers);

    let slice = &numbers[..];
    print_type_of(&slice);
    
    println!("slice = {:?}", &slice);
}

fn modifiable() {
    let mut numbers = vec![1, 2, 3];
    print_type_of(&numbers);

    let slice = &mut numbers[..];
    slice[0] = 10;
    print_type_of(&slice);

    println!("slice = {:?}", slice);
}

fn main() {
    ownership();
    modifiable();
}
