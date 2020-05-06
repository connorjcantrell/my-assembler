use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

pub mod parser;
use parser::Parser;

pub fn run(filename: String) -> std::io::Result<()> {
    let file = File::open(filename)?;
    let mut buffered = BufReader::new(file);
    let mut contents = String::new();
    buffered.read_to_string(&mut contents)?;
    for line in contents.lines() {
        if let Some(i) = filter_line(line) {
            let r = Parser::new(i);
            println!("{:?}", r.command_type());
        }
    }
    Ok(())
}

pub fn filter_line(line: &str) -> Option<String>{
    let idx = line.find("//");
    let line: &str = match idx {
        Some(x) => line[..x].trim(),
        None => line.trim()
    };
    if line.len() == 0 {
        return None
    }
    println!("{:?}", line);
    Some(line.to_string())
}