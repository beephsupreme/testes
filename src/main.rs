#![allow(unused)]
use anyhow::{Context, Result};

mod salary;
mod input;

fn main() -> Result<()> {
    let s = match salary::driver() {
        Ok(s)=>{println!("{}", s)},
        Err(e)=>{println!("{}", e)}
    };
    Ok(())
}
