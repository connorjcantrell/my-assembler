use regex::Regex;

/// Assembly language instruction
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

    /// Identifies the instruction's command type and parses attributes
    pub fn parse(&self) -> Option<Command> {
        if let Some(a) = self.a_command() {
           return Some(Command::A(a))
        }
        if let Some(l) = self.l_command() {
            return Some(Command::L(l))
        }
        if let Some(c) = self.c_command() {
            // TODO: Find an alternative for unpacking values
            return Some(Command::C(c.0, c.1, c.2))
        }
        None
    }

    // TODO: Rename function
    fn a_command(&self) -> Option<Symbol> {
        if let true = self.instruction.starts_with('@') {
            let a = Regex::new(r"^@(?P<value>[a-zA-Z0-9_][a-zA-Z0-9_.$:]{0,}$)").unwrap();
            match a.captures(&self.instruction) {
                Some(cap) => return Some(Symbol::new(&cap["value"])),
                None => return None
            }
        } 
        None
    }

    // TODO: Rename function
    // TODO: Implement a regex pattern that works for both c-instruction variations
    fn c_command(&self) -> Option<(Dest, Comp, Jump)> {
        let semi = self.instruction.find(';').is_some();
        let equal = self.instruction.find('=').is_some();
        match (semi, equal) {
            (true, false) => {  // comp;jump
                let c = Regex::new(r"(?P<comp>.{1,3});(?P<jump>.{3})").unwrap();
                match c.captures(&self.instruction) {
                    Some(caps) => return Some(
                        (
                            Dest::empty(),
                            Comp::new(&caps["comp"]),
                            Jump::new(&caps["jump"]),
                        )
                    ),
                    None => return None
                }
            },
            (false , true) => {  // dest=comp
                let c = Regex::new(r"^(?P<dest>[AMD]{1,3})=(?P<comp>.{1,3})").unwrap();
                match c.captures(&self.instruction) {
                    Some(caps) => return Some(
                        (
                            Dest::new(&caps["dest"]),
                            Comp::new(&caps["comp"]),
                            Jump::empty(),
                        )
                    ),
                    None => return None
                }

            },
            (_, _) => { return None },  // not valid
        }
    }

    // TODO: Rename function
    fn l_command(&self) -> Option<Symbol> {
        let left_parenthesis = self.instruction.starts_with('(');
        let right_parenthesis = self.instruction.ends_with(')');
        match (left_parenthesis, right_parenthesis) {
            (true, true) => {
                let l = Regex::new(r"^\((?P<symbol>[^)]+)\)$").unwrap();
                match l.captures(&self.instruction) {
                    Some(cap) => Some(Symbol::new(&cap["symbol"])),
                    None => None
                }
            },
            _ => None
        }
    }
}
    
#[derive(Debug, Clone)]
pub enum Command {
    A(Symbol),
    C(Dest, Comp, Jump),
    L(Symbol),
}

#[derive(Debug, Clone)]
pub struct Symbol {
    value: String
}

impl Symbol {
    pub fn new(input: &str) -> Symbol {
        Symbol {
            value: input.to_string()
        }
    }
}

#[derive(Debug, Clone)]
pub struct Comp {
    mnemonic: String,
    binary: Option<String>,
}

impl Comp {
    pub fn new(mnemonic: &str) -> Comp {
        Comp {mnemonic: mnemonic.to_string(), binary: None}
    }
}

#[derive(Debug, Clone)]
pub struct Dest {
    mnemonic: Option<String>,
    binary: Option<String>,
}

impl Dest {
    pub fn new(mnemonic: &str) -> Dest {
        Dest {
            mnemonic: Some(mnemonic.to_string()),
            binary: None,
        }
    }
    
    pub fn empty() -> Dest {
        Dest {
            mnemonic: None,
            binary: None
        }
    }
}

#[derive(Debug, Clone)]
pub struct Jump {
    mnemonic: Option<String>,
    binary: Option<String>,
}

impl Jump {
    pub fn new(mnemonic: &str) -> Jump {
        Jump {
            mnemonic: Some(mnemonic.to_string()),
            binary: None,
        }
    }
    pub fn empty() -> Jump {
        Jump {
            mnemonic: None,
            binary: None
        }
    }
}