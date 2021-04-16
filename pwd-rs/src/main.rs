use std::env;

const DEFAULT_MODE: Mode = Mode::Physical;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();

    let mode = get_mode(args);

    let current_dir = match mode {
        Mode::Physical => get_physical_current_dir()?,
        Mode::Logical => get_logical_current_dir()?,
    };

    println!("{}", current_dir);
    Ok(())
}

enum Mode {
    Physical,
    Logical,
}

fn get_mode(args: Vec<String>) -> Mode {
    let mode: Mode = if args.contains(&"-P".to_string()) {
        Mode::Physical
    } else if args.contains(&"-L".to_string()) {
        Mode::Logical
    } else {
        DEFAULT_MODE
    };
    mode
}

fn get_physical_current_dir() -> Result<String, std::io::Error> {
    let current_dir = env::current_dir()?;
    let os_string = current_dir.into_os_string();
    let string = os_string.into_string().unwrap();
    Ok(string)
}

fn get_logical_current_dir() -> Result<String, std::io::Error> {
    unimplemented!("`get_logical_current_dir` isn't implemented.");
}
