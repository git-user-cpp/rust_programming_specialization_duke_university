fn sum(numbers: &[i32]) -> i32 {
    let mut result = 0;
    for number in numbers {
        result += number;
    }
    result
}

fn average(numbers: &[i32]) -> f32 {
    let sum: i32 = numbers.iter().sum();
    let sum_f32: f32 = sum as f32;
    sum_f32 / numbers.len() as f32
}

fn main() {
    println!("Enter the number of elements: ");
    let mut num_elements = String::new();
    std::io::stdin().read_line(&mut num_elements).expect("Error reading input");
    let num_elements = num_elements.trim().parse::<i32>().expect("Invalid input");

    let mut numbers: Vec<i32> = Vec::with_capacity(num_elements as usize);

    for i in 0..num_elements {
        println!("Enter element {}: ", i + 1);
        let mut num_str = String::new();
        std::io::stdin().read_line(&mut num_str).expect("Error reading input");
        let num = num_str.trim().parse::<i32>().expect("Invalid input");
        numbers.push(num);
    }

    let avg = average(&numbers);
    println!("The average is {}", avg);
    let sum = sum(&numbers);
    println!("The sum is {}", sum);
}
