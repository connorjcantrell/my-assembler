use structopt::StructOpt;

use assembler;

// mod parser;
// use parser::foo_parser;

#[derive(Debug, StructOpt)]
struct Opt {
    /// Path to input file
    input: String,
}

fn main() {
    let opt = Opt::from_args();
    if let Err(e) = assembler::run(opt.input) {
        eprintln!("Error: {}", e);
        ::std::process::exit(1)
    }
}
