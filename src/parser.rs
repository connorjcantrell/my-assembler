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

    /// Identifies the type of command, but does not currently parse
    /// Not sure how to use this function in conjunction with the stucts, C, A, and L (below)
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
    
// Is it possible to store different types of structs inside the same enum?
#[derive(Debug, Clone)]
pub enum Command {
    A,  // { String }
    C,  // { String, String, String }
    L   // { String }
}

pub struct A {
    symbol: String
}

impl A {
    pub fn new(input: String) -> Result<A> {

    }
}


pub struct C {
    dest: Option<String>,
    comp: String,
    jump: Option<String>
}

impl C {
    pub fn new(input: String) -> Result<C> {
        let semi = input.find(";");
        let equal = input.find("=");
        if let Some(x) = semi {
            let jump = Some(input[x+1..].to_string());
            let dest = None;
            let comp  = input[..x].to_string();
            return Ok(C { dest, comp, jump })
        }
        if let Some(x) = equal {
            let jump = None;
            let dest = Some(input[..equal]);
            let comp = Some(input[equal+1..]);
            return Ok(C { dest, comp, jump })
        }
        Err(

        )
    }
}

pub struct L {
    symbol: String
}