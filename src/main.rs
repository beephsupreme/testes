#![allow(unused)]
use anyhow::{Context, Result};

mod fueling;
mod input;

fn main() -> Result<()> {
    let mut d = input::get_int().context("Input Error".to_string())?;
    let mut r = input::get_int().context("Input Error".to_string())?;
    let mut n = input::get_int().context("Input Error".to_string())?;
    let mut v = input::get_int_vec(n as usize).context("Input Error".to_string())?;
    let stops = fueling::compute_minimum_stops(d, r, &mut v);
    println!("{}", stops);
    Ok(())
}
