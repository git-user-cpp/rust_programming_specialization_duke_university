fn main() {
    let sentence = "the quick brown fox jumps over the lazy dog".to_string();
    println!("{}", &sentence[0..=4]);

    let description = format!("Title: Quick story\n{}", sentence);
    println!("{}", description);

    for c in sentence.chars() {
        match c {
            'a' | 'e' | 'i' | 'o' | 'u' => println!("Got a vowel!"),
            _ => continue,
        }
    }

    let words: Vec<&str> = sentence.split_whitespace().collect();
    println!("{:?}", words);

    let words = sentence.split(' ').collect::<Vec<&str>>();
    println!("{:?}", words);

    let reversed = sentence.chars().rev().collect::<String>();
    println!("{}", reversed);
}
