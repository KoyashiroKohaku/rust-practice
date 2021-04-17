fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    for arg in args {
        let content = std::fs::read_to_string(arg).expect("error: `{}` is not exists!");
        for line in content.lines() {
            println!("{}", line);
        }
    }
}
