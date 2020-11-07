use std::env;

fn main() -> std::io::Result<()> {
    let mode = get_mode();

    match mode {
        Mode::Physical => println!("mode is pysical"),
        Mode::Logical => println!("mode is logical"),
    }

    let current_dir = get_current_dir()?;
    println!("{}", current_dir);
    Ok(())
}

enum Mode {
    Physical,
    Logical,
}

fn get_mode() -> Mode {
    let args: Vec<String> = env::args().collect();
    let mode: Mode = if args.contains(&"-P".to_string()) {
        Mode::Physical
    } else if args.contains(&"-L".to_string()) {
        Mode::Logical
    } else {
        Mode::Physical
    };
    mode
}

fn get_current_dir() -> Result<String, std::io::Error> {
    let current_dir = env::current_dir()?;
    let os_string = current_dir.into_os_string();
    let string = os_string.into_string().unwrap();
    Ok(string)
}
