#![feature(array_windows)]
mod day01;
use anyhow::Result;
use crate::day01::day01;
fn main() -> Result<()> {
    day01()?;
    Ok(())
}
