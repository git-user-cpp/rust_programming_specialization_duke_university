use std::io::BufRead;

fn main() {
    let file = std::fs::File::open("non_existent_file.txt");
    match file {
        Ok(file) => {
            let reader = std::io::BufReader::new(file);
            for line in reader.lines() {
                println!("{}", line.unwrap());
            }
        }
        Err(error) => match error.kind() {
            std::io::ErrorKind::NotFound => {
                panic!("File not found: {}", error);
            }
            _ => {
                panic!("Error opening file: {}", error);
            }
        },
    };
}
