use anyhow::Result;
use std::fs::read_to_string;
use std::{env::args, process::exit};

fn main() -> Result<()> {
    let path = match args().nth(1) {
        Some(path) => path,
        None => exit(1),
    };

    let content = read_to_string(path)?;
    for line in content.lines().take(10) {
        println!("{}", line);
    }

    Ok(())
}
