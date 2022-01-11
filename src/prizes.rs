#![allow(unused)]

use crate::input;

pub fn prizes(n: i32) {
    let mut i = 1;
    let mut sum = 0;
    let mut output: Vec<i32> = vec![];
    while 2 * i + sum - n < 0 {
        sum += i;
        output.push(i);
        i+=1;
    }
    while sum+i !=n {
        i+=1;
    }
    output.push(i);
    println!("{}", output.len());
    for o in output {
        print!("{} ", o);
    }
    println!("\n");
}

pub fn driver() -> Result<bool, input::InputError> {
    let n = match input::get_int() {
        Ok(n) => n,
        Err(e) => return Err(e),
    };
    prizes(n);
    Ok(true)
}
