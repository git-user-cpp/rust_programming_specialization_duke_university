fn process_numbers(numbers: &[i32]) {
    let mut sum = 0;

    for number in numbers {
        sum += number;
    }

    println!("The sum og the numbers is: {}", sum);

    if sum % 2 == 0 {
        println!("The sum is even");
    } else {
        println!("the sum is odd");
    }
}

fn main() {
    process_numbers(&[1, 2, 3]);
}
