#![feature(array_windows)]
mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
use anyhow::Result;


fn main() -> Result<()> {
    day01::run()?;
    day02::run()?;
    day03::run()?;
    day04::run()?;
    day05::run()?;
    Ok(())
}
