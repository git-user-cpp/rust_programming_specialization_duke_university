fn borrow_vec(vector: &mut Vec<i32>) {
    vector.push(10);
}

fn borrow_integer(x: &mut i32) {
    *x += 1;
}

fn borrow_string(s: &mut String) {
    *s += "hello";
}

fn main() {
    let mut my_vec = vec![1, 2, 3, 4, 5];
    let mut my_int = 10;
    let mut my_string = String::from("Hello, world!");

    borrow_integer(&mut my_int);
    println!("{}", my_int);

    borrow_string(&mut my_string);
    println!("{}", my_string);

    borrow_vec(&mut my_vec);
    println!("{:?}", my_vec);
}
