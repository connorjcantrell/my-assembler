use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Serialize, Deserialize, Debug)]
pub struct Table {
    comp: HashMap<String, String>,
    jump: HashMap<String, String>,
    dest: HashMap<String, String>,
    symbol: HashMap<String, i32>,
    variable: HashMap<String, String>,
}

impl Table {
    pub fn predefined(labels: HashMap<&str, &str>) -> Result<Table, Box<dyn Error>> {
        // Read input file of a string
        let mut file = File::open("tables.json")?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        // Deserialize data structure
        let data: Table = serde_json::from_str(&contents)?;
        println!("{:?}", data);
        Ok(data)
    }
}
