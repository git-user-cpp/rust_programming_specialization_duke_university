use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, LinkedList, VecDeque};

fn main() {
    println!("Common Rust Collections:");

    // Sequences
    println!("\n\tSequences:");
    println!("\t\tVec: https://doc.rust-lang.org/std/vec/struct.Vec.html");
    let mut my_strct = Vec::new();
    my_strct.push(String::from("Hello"));
    my_strct.push(String::from("World"));
    println!("\t\t{:?}", my_strct);
    my_strct.remove(0);
    my_strct.remove(0);
    println!("\t\t{:?}\n", my_strct);

    println!("\t\tVecDeque: https://doc.rust-lang.org/std/collections/struct.VecDeque.html");
    let mut my_strct = VecDeque::new();
    my_strct.push_front(String::from("Hello"));
    my_strct.push_back(String::from("World"));
    println!("\t\t{:?}", my_strct);
    my_strct.pop_back();
    my_strct.pop_front();
    println!("\t\t{:?}\n", my_strct);

    println!("\t\tLinkedList: https://doc.rust-lang.org/std/collections/struct.LinkedList.html");
    let mut my_strct = LinkedList::new();
    my_strct.push_front(String::from("Hello"));
    my_strct.push_back(String::from("World"));
    println!("\t\t{:?}", my_strct);
    my_strct.pop_front();
    my_strct.pop_back();
    println!("\t\t{:?}\n", my_strct);

    // Maps
    println!("\n\tMaps:");
    println!("\t\tHashMap: https://doc.rust-lang.org/std/collections/struct.HashMap.html");
    let mut my_strct = HashMap::new();
    my_strct.insert(1, String::from("Hello"));
    my_strct.insert(2, String::from("World"));
    println!("\t\t{:?}", my_strct);
    my_strct.remove(&2);
    my_strct.remove(&1);
    println!("\t\t{:?}\n", my_strct);

    println!("\t\tBTreeMap: https://doc.rust-lang.org/std/collections/struct.BTreeMap.html");
    let mut my_strct = BTreeMap::new();
    my_strct.insert(1, String::from("Hello"));
    my_strct.insert(2, String::from("World"));
    println!("\t\t{:?}", my_strct);
    my_strct.pop_last();
    my_strct.remove(&1);
    println!("\t\t{:?}\n", my_strct);

    // Sets
    println!("\n\tSets:");
    println!("\t\tHashSet: https://doc.rust-lang.org/std/collections/struct.HashSet.html");
    let mut my_strct = HashSet::new();
    my_strct.insert(String::from("Hello"));
    my_strct.insert(String::from("World"));
    println!("\t\t{:?}", my_strct);
    my_strct.remove(&String::from("World"));
    my_strct.remove(&String::from("Hello"));
    println!("\t\t{:?}\n", my_strct);

    println!("\t\tBTreeSet: https://doc.rust-lang.org/std/collections/struct.BTreeSet.html");
    let mut my_strct = BTreeSet::new();
    my_strct.insert(String::from("Hello"));
    my_strct.insert(String::from("World"));
    println!("\t\t{:?}", my_strct);
    my_strct.pop_last();
    my_strct.remove(&String::from("Hello"));
    println!("\t\t{:?}\n", my_strct);

    // Misc
    println!("\n\tMisc:");
    println!("\t\tBinaryHeap: https://doc.rust-lang.org/std/collections/struct.BinaryHeap.html");
    let mut my_strct = BinaryHeap::new();
    my_strct.push(String::from("Hello"));
    my_strct.push(String::from("World"));
    println!("\t\t{:?}", my_strct);
    my_strct.pop();
    my_strct.pop();
    println!("\t\t{:?}\n", my_strct);
}
