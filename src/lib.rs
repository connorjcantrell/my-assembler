use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::io::prelude::*;
use std::error::Error;

pub mod error;
pub use error::{AsmError, AsmResult};
pub mod parser;
use parser::Parser;
pub mod tables;
use tables::Table;

pub fn run(filename: String) -> AsmResult {
    let assembly = File::open(filename)?;
    let mut buffered = BufReader::new(assembly);
    let mut contents = String::new();
    buffered.read_to_string(&mut contents)?;
    let filtered_contents = first_pass(contents)?;
    second_pass(filtered_contents)?;
    Ok(())
}

/// Filters out comments and empty lines
/// Writes valid assembly code to new file with BufWriter
/// Returns new file
fn first_pass(contents: BufReader<R>) -> AsmResult<R> {
    // for each line in file
    //     if line is not a comment
    //         write line to new_buffer
    //             if line is a Label
    //                 write line to labels.json
    // return new_buffer
    let lines = contents.lines.collect();
    let total = lines.len();
    let mut writer = BufWriter::with_capacity(total, Vec::new());

    for line in contents.lines() {
        if let Some(i) = filter_line(line) {
            let r = Parser::new(i);
            if let Some(i) = r.l_command() {
                todo!()  // 
            }
        }
    }
    unimplemented!()
}

/// Translates assembly code to binary
fn second_pass(contents: BufReader<R>) -> AsmResult {
    //
    let (comp,
         jump,  
         dest, 
         symbol) = Table::predefined().unwrap();  // Improve error handling

    // for each line in file
    //     determine command type
    //     translate command to binary via corresponding hashmap
    // write binary translation to new_file
}

/// Filters out comments and empty lines
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
