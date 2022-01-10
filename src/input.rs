use std::io;
use thiserror::Error;

#[derive(Error, Debug)]
#[non_exhaustive]
pub enum InputError {
    #[error("ArgLength error: Expected {0}, Found {1}.")]
    ArgLengthError(usize, usize),
    #[error("Error reading from stdin.")]
    ReadError,
    #[error("Error parsing input.")]
    ParseError,
}

pub fn get_int_vec(n: usize) -> Result<Vec<i32>, InputError> {
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => {}
        Err(_) => { return Err(InputError::ReadError); }
    }
    let numbers: Vec<i32> = match input
        .split_whitespace()
        .map(|s| s.parse()).collect() {
        Ok(numbers) => { numbers }
        Err(_) => { return Err(InputError::ParseError); }
    };
    if n == numbers.len() {
        Ok(numbers)
    } else {
        Err(InputError::ArgLengthError(n, numbers.len()))
    }
}
