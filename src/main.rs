#![allow(unused)]
use anyhow::{Context, Result};

mod segments;
mod input;

fn main() -> Result<()> {
    let s = match segments::signatures(){
        Ok(s)=>{println!("{}", s)},
        Err(e)=>{println!("{}", e)}
    };
    Ok(())
}
