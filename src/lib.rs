use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

pub mod parser;
use parser::Parser;
pub mod tables;
use tables::Table;

pub fn run(filename: String) -> std::io::Result<()> {
    let assembly = File::open(filename)?;
    let mut buffered = BufReader::new(assembly);
    let mut contents = String::new();
    buffered.read_to_string(&mut contents)?;
    
    let mut success = 0;
    let mut failure: i32 = 0;
    let table = Table::new().unwrap();
    for line in contents.lines() {
        if let Some(i) = filter_line(line) {
            let r = Parser::new(i);
            match r.parse() {
                Some(_) => success += 1,
                None => {
                    failure += 1;
                    println!("{}", line)
                }
            }
        }
    }
    // println!("{:?}", table);
    println!("success: {}\nfailure: {}", success, failure);
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
    Some(line.to_string())
}