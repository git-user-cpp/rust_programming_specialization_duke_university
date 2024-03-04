fn main() {
    let fruit_salad = vec![
        "apple",
        "banana",
        "cherry",
        "dates",
        "elderberries",
    ];
    println!("Original fruit salad {:?}", fruit_salad);

    let mut fruit_salad = fruit_salad;
    fruit_salad.push("fig");
    println!("Mutable fruit salad {:?}", fruit_salad);
}
