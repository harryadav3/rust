//! This is a library that provides utilities for command line tools
//! So far it only provides a funcito to read ilne form stdin.
//! # Examples:
//! ```
//! use cli_utils::read_stdin;
//! let input = read_stdin();
//! ```
//! #Panics:
//! The `read_stdin` function will panic if it fails to read a line with a message "Failed to read line".

use std::io::{BufRead, BufReader};

/// This is function reads a line form stdin and returns it as a string.
/// It will panic if it fails to read a lne with a message "Failed to read line".
/// # Exampels::
///
/// ```:
/// let input = read_stding();
/// ```
///

struct User {}
pub fn read_stdin() -> String {
    let stdin = std::io::stdin();
    let mut reader = BufReader::new(stdin.lock());
    let mut line = String::new();
    reader
        .read_line(&mut line)
        .expect("Failed to read input line");
    line.trim().to_string()
}
