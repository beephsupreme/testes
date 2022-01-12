#![allow(unused)]
use std::io;
use thiserror::Error;

#[derive(Error, Debug)]
#[non_exhaustive]
pub enum InputError {
    #[error("ArgLength: Expected: {0}, Got: {1}")]
    ArgLength(usize, usize),
    #[error("Error reading from stdin.")]
    ReadError,
    #[error("Could not parse {0}")]
    ParseError(String),
    #[error("Out-Of-Range: {0}")]
    OutOfRange(String),
}

pub fn get_int_vec(n: usize) -> Result<Vec<i32>, InputError> {
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => {}
        Err(_) => {
            return Err(InputError::ReadError);
        }
    }
    let numbers: Vec<i32> = match input.split_whitespace().map(|s| s.parse()).collect() {
        Ok(numbers) => numbers,
        Err(_) => {
            return Err(InputError::ParseError(input));
        }
    };
    if n == numbers.len() {
        Ok(numbers)
    } else {
        Err(InputError::ArgLength(n, numbers.len()))
    }
}

pub fn get_words(n: usize) -> Result<Vec<String>, InputError> {
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => {}
        Err(_) => {
            return Err(InputError::ReadError);
        }
    }
    let mut tokens = input.split_whitespace();
    let mut words: Vec<String> = vec![];
    for t in tokens {
        words.push(t.to_string());
    }
    if n == words.len() {
        Ok(words)
    } else {
        Err(InputError::ArgLength(n, words.len()))
    }
}

pub fn get_float_vec(n: usize) -> Result<Vec<f64>, InputError> {
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => {}
        Err(_) => {
            return Err(InputError::ReadError);
        }
    }
    let numbers: Vec<f64> = match input.split_whitespace().map(|s| s.parse()).collect() {
        Ok(numbers) => numbers,
        Err(_) => {
            return Err(InputError::ParseError(input));
        }
    };
    if n == numbers.len() {
        Ok(numbers)
    } else {
        Err(InputError::ArgLength(n, numbers.len()))
    }
}

pub fn get_int() -> Result<i32, InputError> {
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => {}
        Err(_) => {
            return Err(InputError::ReadError);
        }
    }
    let numbers: Vec<i32> = match input.split_whitespace().map(|s| s.parse()).collect() {
        Ok(numbers) => numbers,
        Err(_) => {
            return Err(InputError::ParseError(input));
        }
    };
    if numbers.len() == 1 {
        Ok(numbers[0])
    } else {
        Err(InputError::ArgLength(1, numbers.len()))
    }
}

pub fn get_float() -> Result<f64, InputError> {
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => {}
        Err(_) => {
            return Err(InputError::ReadError);
        }
    }
    let numbers: Vec<f64> = match input.split_whitespace().map(|s| s.parse()).collect() {
        Ok(numbers) => numbers,
        Err(_) => {
            return Err(InputError::ParseError(input));
        }
    };
    if numbers.len() == 1 {
        Ok(numbers[0])
    } else {
        Err(InputError::ArgLength(1, numbers.len()))
    }
}
