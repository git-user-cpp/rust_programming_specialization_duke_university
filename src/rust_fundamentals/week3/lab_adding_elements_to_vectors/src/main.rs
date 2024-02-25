fn insert_beg_end(v: &mut Vec<i32>, var: i32) {
    v.insert(0, var);
    v.push(var);
}

fn merge_two_vectors(v1: &mut Vec<i32>, v2: Vec<i32>) {
    v1.extend(v2);
}

fn main() {
    let mut v: Vec<i32> = vec![1, 2, 3];
    v.push(4);
    println!("{:?}", v);

    let more_numbers: Vec<i32> = vec![5, 6];
    v.extend(more_numbers);
    println!("{:?}", v);

    let mut other_numbers: Vec<i32> = vec![7, 8];
    v.append(&mut other_numbers);
    println!("{:?}", v);

    v.insert(0, 0);
    println!("{:?}", v);

    insert_beg_end(&mut v, 111);
    println!("{:?}", v);

    let v2: Vec<i32> = vec![1, 2, 3];
    merge_two_vectors(&mut v, v2);
    println!("{:?}", v);
}