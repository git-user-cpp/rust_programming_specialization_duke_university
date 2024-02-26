//! This is a library that provides utilities for command line tools.
//! # Examples:
//! ```
//! use documenting_your_code::read_stdin;
//! let input = read_stdin();
//! ```
//! # Panics:
//! The `read_stdin` will panic if it fails to read a line with a message "Error: Failed to read input!".

use std::io::{BufRead, BufReader};

/// This functions reads a line from stdin and returns it as a String.
/// It will panic if it fails to read a line with a message "Error: Failed to read input!".
/// # Examples:
/// ```
/// let input = read_stdin();
/// ```
pub fn read_stdin() -> String {
    let stdin = std::io::stdin();
    let mut reader = BufReader::new(stdin.lock());
    let mut line = String::new();
    reader.read_line(&mut line).expect("Error: Failed to read input!");
    line.trim().to_string()
}