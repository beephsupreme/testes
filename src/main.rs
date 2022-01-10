#![allow(unused)]
use anyhow::{Context, Result};

mod maxloot;
mod input;

fn main() -> Result<()> {
    let ml = match maxloot::driver(){
        Ok(ml)=>{println!("{}", ml)},
        Err(e)=>{println!("{}", e)}
    };
    Ok(())
}
