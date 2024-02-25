fn main() {
    let mut v = vec![1, 2, 3];
    v.push(4);
    println!("{:?}", v);

    let more_numbers = vec![5, 6];
    v.extend(more_numbers);
    println!("{:?}", v);

    let mut other_numbers = vec![7, 8];
    v.append(&mut other_numbers);
    println!("{:?}", v);

    v.insert(0, 0);
    println!("{:?}", v);
}
