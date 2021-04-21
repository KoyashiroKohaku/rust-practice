use anyhow::Result;
use std::fs::read_to_string;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Opt {
    #[structopt(name = "path")]
    path: String,

    #[structopt(short = "n", long = "lines", value_name = "NUM", default_value = "10")]
    lines: usize,
}

fn main() -> Result<()> {
    let opt = Opt::from_args();
    let content = read_to_string(opt.path)?;
    for line in content.lines().take(opt.lines) {
        println!("{}", line);
    }

    Ok(())
}
