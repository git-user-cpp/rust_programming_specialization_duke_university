use std::fs::{File, OpenOptions};
use std::io::{Error, ErrorKind, Write, BufReader, BufRead};

fn read_from_file(file: &File) -> Result<(), Error> {
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let line = line?;
        println!("{}", line);
    }
    Ok(())
}

fn write_to_file(file: &mut File) -> Result<(), Error> {
    write!(file, "This is a new line written to the file.\n")?;
    Ok(())
}

fn main() {
    let file_path = "non_existing_file.txt";
    match File::open(file_path) {
        Ok(file) => {
            match read_from_file(&file) {
                Ok(_) => println!("Read from file successfully"),
                Err(error) => println!("Error reading file: {}", error),
            }
        },
        Err(error) => match error.kind() {
            ErrorKind::NotFound => println!("File not found: {}", error),
            _ => println!("Error opening file: {}", error),
        },
    }
}
