use rand::seq::SliceRandom;
use rand::thread_rng;
use std::collections::VecDeque;

fn main() {
    let mut fruit = VecDeque::new();
    fruit.push_back("Apple");
    fruit.push_back("Pear");
    fruit.push_back("Banana");

    let rng = thread_rng;
    let mut fruit: Vec<_> = fruit.into_iter().collect();
    fruit.shuffle(&mut rng());

    let mut fruit: VecDeque<_> = fruit.into_iter().collect();
    fruit.push_front("Tomato");
    fruit.push_back("Watermelon");
    fruit.push_back("Quaso");

    for fr in fruit {
        print!("{} ", fr);
    }
    println!();
}