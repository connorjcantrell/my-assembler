use std::fs;
pub mod error;
pub use error::{AsmError, AsmResult};
pub mod parser;
use parser::Parser;
pub mod tables;

pub fn run(filename: String) -> AsmResult {
    let assembly = fs::read_to_string(filename)?;
    let mut filtered_contents = String::new();
    first_pass(&assembly, &mut filtered_contents)?;
    let mut bin = Vec::new();
    second_pass(&filtered_contents, &mut bin)?;
    Ok(())
}

/// Filters out comments and empty lines
/// Writes valid assembly code to new file with BufWriter
/// Returns new file
fn first_pass(contents: &str, filtered: &mut String) -> AsmResult {
    // for each line in file
    //     if line is not a comment
    //         write line to new_buffer
    //             if line is a Label
    //                 write line to labels.json
    // return new_buffer

    for line in contents.lines() {
        if let Some(i) = filter_line(line) {
            let r = Parser::new(i);
            if let Some(i) = r.l_command() {
                todo!("write filtered contents to `filtered`");
            }
        }
    }
    unimplemented!()
}

/// Translates assembly code to binary
fn second_pass(contents: &str, bin: &mut Vec<u8>) -> AsmResult {
    // let (comp,
    //      jump,
    //      dest,
    //      symbol) = Table::predefined()?;

    // for each line in file
    //     determine command type
    //     translate command to binary via corresponding hashmap
    // write binary translation to new_file
    unimplemented!()
}

/// Filters out comments and empty lines
pub fn filter_line(line: &str) -> Option<String> {
    let idx = line.find("//");
    let line: &str = match idx {
        Some(x) => line[..x].trim(),
        None => line.trim(),
    };
    if line.len() == 0 {
        return None;
    }
    Some(line.to_string())
}
