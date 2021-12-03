#![feature(array_windows)]
mod day01;
mod day02;
mod day03;
use anyhow::Result;


fn main() -> Result<()> {
    //day01::run()?;
    //day02::run()?;
    day03::run()?;
    Ok(())
}
