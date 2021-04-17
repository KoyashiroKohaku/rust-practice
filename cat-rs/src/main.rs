fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    for arg in args {
        let result = std::fs::read_to_string(&arg);
        match result {
            Ok(content) => {
                for line in content.lines() {
                    println!("{}", line);
                }
            }
            Err(error) => {
                eprintln!("[cat-rs error]: `{}`: {}", &arg, error);
                std::process::exit(1);
            }
        };
    }
}
