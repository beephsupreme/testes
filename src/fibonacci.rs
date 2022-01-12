#![allow(unused)]

use std::f32::consts::E;
use crate::input;
use crate::input::InputError;

pub fn fibonacci(n: u64) -> Result(u64, InputError) {
    if n == 0 || n == 1 {
        return Ok(n);
    } else if n > 93 {
        return Err(input::InputError::OutOfRange(n.to_string()));
    } else {
        let mut p: u64 = 0;
        let mut c: u64 = 1;
        let mut t: u64 = 0;
        for i in 0..n - 1 {
            t = c;
            c = p + c;
            p = t;
        }
        Ok(c)
    }
}

pub fn driver() -> Result<u64, input::InputError> {
    let n = match input::get_int() {
        Ok(n) => n,
        Err(e) => return Err(e),
    };
    let f = match fibonacci(n as u64) {
        Ok(f) => println!("{}", f),
        Err(e) => Err(e),
    };
    Ok(f)
}