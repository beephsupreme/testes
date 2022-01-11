#![allow(unused)]
use anyhow::{Context, Result};

mod prizes;
mod input;

fn main() -> Result<()> {
    let s = match prizes::driver() {
        Ok(s)=>{},
        Err(e)=>{println!("{}", e)}
    };
    Ok(())
}
