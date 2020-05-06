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
        if self.instruction.chars().nth(0) == Some('@') {
            return Command::A
        }
        return Command::L
    }
}

#[derive(Debug, Clone)]
pub enum Command {
    A,
    C,
    L
}