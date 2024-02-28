use rand::seq::SliceRandom;
use rand::thread_rng;
use std::collections::LinkedList;

fn main() {
    let mut fruits = LinkedList::new();
    fruits.push_back("Apple");
    fruits.push_back("Banana");
    fruits.push_back("Pear");
    fruits.push_back("Watermelon");

    let mut rng = thread_rng();
    let mut fruits: Vec<_> = fruits.into_iter().collect();
    fruits.shuffle(&mut rng);

    let mut fruits: LinkedList<_> = fruits.into_iter().collect();
    fruits.push_front("Tomato");
    fruits.push_back("Melon");

    for fruit in fruits {
        print!("{} ", fruit);
    }
    println!();
}