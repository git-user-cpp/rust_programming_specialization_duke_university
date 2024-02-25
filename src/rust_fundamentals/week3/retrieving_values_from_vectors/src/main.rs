fn get_item(index: usize) {
    let vec = vec![1, 2, 3, 4, 5];

    let value = vec.get(index).unwrap();

    println!("The value at index {} is {}", index, value);
}

fn main() {
    let vec = vec![1, 2, 3, 4, 5];

    let third_value = vec[2];

    println!("The third value in the vector is: {}", third_value);

    let last_value = vec.last().unwrap();
    println!("The last value in the vector is: {}", last_value);

    match vec.first() {
        Some(first_value) => println!("The first value in the vector is: {}",
                                        first_value),
        None => println!("The vector is empty!"),
    }

    get_item(3);
}
