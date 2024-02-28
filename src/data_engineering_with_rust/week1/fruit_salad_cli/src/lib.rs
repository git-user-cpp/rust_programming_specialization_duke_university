use rand::seq::SliceRandom;
use rand::thread_rng;

pub fn create_fruit_salad(num_fruits: usize) -> Vec<String> {
    let mut fruits = vec![
        String::from("Banana"),
        String::from("Apple"),
        String::from("Pineapple"),
        String::from("Strawberry"),
        String::from("Fig"),
        String::from("Orange"),
        String::from("Mango"),
        String::from("Clementine"),
    ];

    let mut rng = thread_rng();
    fruits.shuffle(&mut rng);

    fruits.into_iter().take(num_fruits).collect()
}