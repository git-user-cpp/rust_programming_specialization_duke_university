use std::collections::HashMap;

fn init_languages() -> HashMap<String, i32> {
    let mut languages = HashMap::new();
    languages.insert(String::from("JavaScript"), 1995);
    languages.insert(String::from("HTML/CSS"), 1990);
    languages.insert(String::from("Pyhon"), 1991);
    languages.insert(String::from("SQL"), 1974);
    languages.insert(String::from("TypeScript"), 2012);
    languages.insert(String::from("Bash/Shell"), 1989);
    languages.insert(String::from("Java"), 1995);
    languages.insert(String::from("C#"), 2000);
    languages.insert(String::from("C++"), 1985);
    languages.insert(String::from("C"), 1972);
    languages.insert(String::from("PHP"), 1995);
    languages.insert(String::from("PowerShell"), 2006);
    languages.insert(String::from("Go"), 2007);
    languages.insert(String::from("Rust"), 2010);

    languages
}

fn calculate_weights(years_active: &mut HashMap<String, i32>) -> HashMap<String, i32> {
    for year in years_active.values_mut() {
        *year = 2024 - *year;
    }

    let min_year = *years_active.values().min().unwrap_or(&0);
    let max_year = *years_active.values().max().unwrap_or(&0);

    let mut weights = HashMap::new();

    for (lang, year) in years_active.iter() {
        let normalized_year = (year - min_year) as f64 / (max_year - min_year) as f64;
        let weight = (normalized_year * 99.0) as i32 + 1;
        weights.insert(String::from(lang), weight);
    }

    weights
}

fn main() {
    let mut languages = init_languages();
    let weights = calculate_weights(&mut languages);

    println!("Languages weighting from 1 to 100 by age (1 is the newest, 100 is the oldest");
    for (language, weight) in &weights {
        println!("{} {}", language, weight);
    }
}
