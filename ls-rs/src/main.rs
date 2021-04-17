use std::fs;
use std::io;
use std::path;
use std::process;

fn main() {
    let result = read_dir("./");
    match result {
        Ok(files) => {
            println!("{}", files.join("  "))
        }
        Err(error) => {
            eprintln!("[ls-rs error]: {}", error);
            process::exit(1);
        }
    }
}

fn read_dir<P: AsRef<path::Path>>(path: P) -> io::Result<Vec<String>> {
    Ok(fs::read_dir(path)?
        .filter_map(|entry| {
            let entry = entry.ok()?;
            let file_name = entry.file_name().to_string_lossy().to_owned().to_string();
            Some(file_name)
        })
        .collect())
}
