fn get_item(index: usize) {
    let vec = vec![1, 2, 3, 4, 5];

    if vec.is_empty() {
        println!("The vector is empty!");
        return;
    }

    let value = match vec.get(index) {
        Some(value) => value,
        None => {
            println!("Index {} is out of bounds for the vector", index);
            return;
        }
    };

    println!("The value at index {} is {}", index, value);
}

fn calculate_sum(vec: &Vec<i32>) -> i32 {
    vec.iter().sum()
}

fn main() {
    let vec = vec![1, 2, 3, 4, 5];
    get_item(3);

    match vec.first() {
        Some(first_value) => println!("The first value in the vector is: {}", first_value),
        None => println!("The vector is empty!"),
    }

    let sum = calculate_sum(&vec);
    println!("The sum of the elements in the vector is: {}", sum);
}
