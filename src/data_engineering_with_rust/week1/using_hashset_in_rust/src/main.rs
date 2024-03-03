use rand::seq::SliceRandom;
use rand::thread_rng;
use std::collections::HashSet;

fn generate_fruit() -> &'static str {
    let fruits = [
        "Apple",
        "Banana",
        "Cherry",
        "Date",
        "Elderberry",
        "Fig",
        "Grape",
        "Honeydew",
    ];

    let mut rng = thread_rng();
    fruits.choose(&mut rng).unwrap()
}

fn main() {
    let mut fruit_set = HashSet::new();
    println!("Generating random 100 fruits...");
    
    for _ in 0..100 {
        fruit_set.insert(generate_fruit());
    }

    println!("Number of unique fruits generated: {}", fruit_set.len());
}
