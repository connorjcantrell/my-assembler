#[derive(Debug, Clone)]
pub struct Parser {
    instruction: String,
}

impl Parser {
    pub fn new(instruction: String) -> Parser { 
        Parser {
            instruction
        }
    }

    pub fn command_type(&self) -> Command {
        if self.instruction.contains("=") || self.instruction.contains(";") {
            Command::C
        } else if self.instruction.contains("@") {
            Command::A
        } else {
            Command::L
        }
    }
}
    

#[derive(Debug, Clone)]
pub enum Command {
    A,  // { String }
    Parser::C,  // { String, String, String }
    L   // { String }
}

pub struct A {
    symbol: String
}


pub struct C {
    dest: Option<String>,
    comp: String,
    jump: Option<String>
}

impl C {
    pub fn new(input: String) {
        let semi = input.find(";");
        let equal = input.find("=");
        if let Some(x) = semi {
            let jump = Some(input[x+1..].to_string());
            let dest = None;
            let comp  = input[..x].to_string();
            C { dest, comp, jump }

        }
    }
}

pub struct L {
    symbol: String
}