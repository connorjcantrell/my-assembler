use std::path::PathBuf;
use structopt::StructOpt;

use assembler;

mod parser;
use parser::foo_parser;

#[derive(Debug, StructOpt)]
struct Opt {
    /// Path to input file
    input: String,
}

fn main() {
    let opt = Opt::from_args();
    assembler::run(opt.input);
}
