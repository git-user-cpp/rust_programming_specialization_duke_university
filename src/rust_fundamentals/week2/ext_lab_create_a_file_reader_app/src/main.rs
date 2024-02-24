use std::fs::File;
use std::io::{stdin, BufRead, BufReader, ErrorKind};

fn main() {
    loop {
        let mut file_name = String::new();
        println!("Enter file name (or 'q' to quit):");
        stdin().read_line(&mut file_name).expect("Failed to read line");

        if file_name.trim().to_lowercase() == "q" {
            break;
        }

        let file = File::open(file_name.clone().trim());
        match file {
            Ok(file) => {
                let reader = BufReader::new(file);
                for line in reader.lines() {
                    match line {
                        Ok(line) => println!("{}", line),
                        Err(error) => println!("Error reading line: {}", error),
                    }
                }
                break;
            },
            Err(error) => match error.kind() {
                ErrorKind::NotFound => println!("File not found: {}", error),
                ErrorKind::PermissionDenied => {
                    println!("Permission denied: {}", error);
                    println!("Try opening a different file? (y/n)");
                    let mut input = String::new();
                    stdin().read_line(&mut input).expect("Failed to read line");
                    if input.trim().to_lowercase() == "y" {
                        continue;
                    }
                },
                _ => println!("Error opening file: {}", error),
            },
        }
    }
}
