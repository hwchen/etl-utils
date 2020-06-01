//! add a number to the beginning of a csv line

use anyhow::{Context, Result};
use std::io::BufRead;

fn main() -> Result<()> {
    let stdin = std::io::stdin();
    let stdin = stdin.lock();

    for (i, line) in stdin.lines().enumerate() {
        let line = line.context("could not get line")?;
        println!("{},{}", i, line);
    }
    Ok(())
}
