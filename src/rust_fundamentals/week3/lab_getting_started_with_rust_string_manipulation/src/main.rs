fn the_longest(s: String) -> String {
    let words = s.split(' ').collect::<Vec<&str>>();

    let mut result = words[0];

    for w in words {
        if w.len() > result.len() {
            result = w;
        }
    }

    result.to_string()
}

fn main() {
    let sentence = "the quick brown fox jumps over the lazy dog".to_string();
    
    println!("{}", &sentence[0..=4]);

    let description = format!("Title: Quick story\n{}", sentence);
    println!("{}", description);

    let mut i = 0;
    for c in sentence.chars() {
        match c {
            'a' | 'e' | 'i' | 'o' | 'u' => {
                println!("Got a vowel!");
                i += 1
            },
            _ => continue,
        }
    }
    println!("Got {i} vowels!");

    let words = sentence.split(' ').collect::<Vec<&str>>();
    println!("{:?}", words);

    let reversed = sentence.chars().rev().collect::<String>();
    println!("{}", reversed);

    let mut line = String::new();
    std::io::stdin().read_line(&mut line).expect("Failed to read line");
    println!("{}", the_longest(line));
}
