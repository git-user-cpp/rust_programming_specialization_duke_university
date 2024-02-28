//! This program creates a fruit salad.

use rand::seq::SliceRandom;
use rand::thread_rng;

fn main() {
    let mut fruits = vec![
        "Orange",
        "Fig",
        "Pomegrante",
        "Cherry",
        "Apple",
        "Pear",
        "Peach",
    ];

    let mut rng = thread_rng();
    fruits.shuffle(&mut rng);

    println!("Fruit Salad:");
    for(i, item) in fruits.iter().enumerate() {
        if i != fruits.len() - 1 {
            print!("{}, ", item);
        } else {
            println!("{}", item);
        }
    }

    //much better for this task.
    for fruit in fruits {
        print!("{}, ", fruit);
    }
    println!();
}