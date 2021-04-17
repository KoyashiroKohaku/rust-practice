fn main() {
    let mut args: Vec<String> = Vec::new();
    for arg in std::env::args().skip(1) {
        args.push(arg);
    }
    let result = args.join(" ");
    println!("{}", result);
}
