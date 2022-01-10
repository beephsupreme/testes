#![allow(unused)]
use anyhow::{Context, Result};

mod input;

fn main() -> Result<()> {
    let n: usize = 5;
    let v = input::get_int()
        .context(format!("Input Error"))?;
    println!("{:?}", v);
    Ok(())
}