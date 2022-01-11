#![allow(unused)]

use crate::input;

fn greater_or_equal(digit: &String, max: &String) -> bool {
    if format!("{}{}", digit, max) >= format!("{}{}", max, digit){
        return true;
    }
    false
}

pub fn largest_number(numbers: &mut Vec<String>) -> String {
    let mut result: String = String::new();
    loop {
        if numbers.len() == 0 {
            break;
        }
        let mut max = "0".to_string();
        let mut pos = 0;
        for i in 0..numbers.len() {
            if greater_or_equal(&numbers[i], &max) {
                max = numbers[i].clone();
                pos = i;
            }
        }
        result = format!("{}{}", result, max);
        numbers.remove(pos);
    }
    return result;
}

pub fn driver()  -> Result<bool, input::InputError> {
    let n = match input::get_int() {
        Ok(n) => n,
        Err(e) => {return Err(e)},
    };
    let mut numbers = match input::get_words(n as usize) {
        Ok(numbers) => numbers,
        Err(e) => return Err(e),
    };
    println!("{}", largest_number(&mut numbers));
    Ok(true)
}
