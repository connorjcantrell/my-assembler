use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

pub mod parser;

pub fn run(filename: String) -> std::io::Result<()> {
    let file = File::open(filename)?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;
    println!("{:?}", contents);
    Ok(())
}