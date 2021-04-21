use anyhow::Result;
use std::fs::read_to_string;
use std::{env::args, process::exit};

fn main() -> Result<()> {
    let path = match args().nth(1) {
        Some(path) => path,
        None => exit(1),
    };

    let content = read_to_string(path)?;
    let line_length = content.lines().count();
    if line_length > 10 {
        for line in content.lines().skip(line_length - 10) {
            println!("{}", line);
        }
    } else {
        for line in content.lines() {
            println!("{}", line);
        }
    }

    Ok(())
}
