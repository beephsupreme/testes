#![allow(unused)]
use anyhow::{Context, Result};

mod fibonacci;
mod input;

fn main() -> Result<()> {
    let s = match fibonacci::driver() {
        Ok(s)=>{},
        Err(e)=>{println!("{}", e)}
    };
    Ok(())
}
