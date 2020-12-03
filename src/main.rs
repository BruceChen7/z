use structopt::StructOpt;
use std::error::Error;

#[derive(StructOpt, Debug)]
#[structopt(name="z", about="z is xxxx")]
pub struct Cli {
    #[structopt(short = "l", long = "list", help="")]
    list: bool,
    #[structopt(short="a", long = "add", help="add `pwd` to data file")]
    add: String,

    #[structopt(short="h", long = "help", help="Print help info")]
    help: bool,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}
