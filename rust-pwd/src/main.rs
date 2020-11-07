use std::env;

fn main() -> std::io::Result<()> {
    let current_dir = get_current_dir()?;
    println!("{}", current_dir);
    Ok(())
}

fn get_current_dir() -> Result<String, std::io::Error> {
    let current_dir = env::current_dir()?;
    let os_string = current_dir.into_os_string();
    let string = os_string.into_string().unwrap();
    Ok(string)
}
