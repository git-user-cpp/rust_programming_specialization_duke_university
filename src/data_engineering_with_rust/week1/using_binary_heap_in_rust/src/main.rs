use rand::seq::SliceRandom;
use rand::thread_rng;
use std::cmp::Ord;
use std::collections::BinaryHeap;

#[derive(Eq, PartialEq, Debug)]
enum Fruit {
    Fig,
    Other(String),
}

impl Ord for Fruit {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (Fruit::Fig, Fruit::Fig) => std::cmp::Ordering::Equal,
            (Fruit::Fig, Fruit::Other(_)) => std::cmp::Ordering::Greater,
            (Fruit::Other(_), Fruit::Fig) => std::cmp::Ordering::Less,
            (Fruit::Other(_), Fruit::Other(_)) => std::cmp::Ordering::Equal,
        }
    }
}

impl PartialOrd for Fruit {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn generate_fruit_salad() -> BinaryHeap<Fruit> {
    let mut rng = thread_rng();
    let fruits = vec![
        "Apple",
        "Orrange",
        "Pear",
        "Banana",
        "Fig",
        "Fig",
        "Fig",
        "Fig",
    ];

    let mut fruit_salad = BinaryHeap::new();

    let mut figs_count = 0;
    while figs_count < 2 {
        let fruit = fruits.choose(&mut rng).unwrap();

        if *fruit == "Fig" {
            figs_count += 1;
            fruit_salad.push(Fruit::Fig);
        } else {
            fruit_salad.push(Fruit::Other(fruit.to_string()));
        }
    }

    fruit_salad
}

fn main() {
    println!("{:?}", generate_fruit_salad());
}
