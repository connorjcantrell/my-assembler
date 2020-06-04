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
        if let Some(address) = self.a_command() {
           return Some(Command::A(address))
        }
        if let Some(label) = self.l_command() {
            return Some(Command::L(label))
        }
        if let Some((destination, computation, jump)) = self.c_command() {
            return Some(Command::C(destination, computation, jump))
        }
        None
    }

    // TODO: Rename function
    fn a_command(&self) -> Option<Symbol> {
        let a = Regex::new(r"^@(?P<symbol>[a-zA-Z0-9_][a-zA-Z0-9_.$:]{0,}$)").unwrap();
        a.captures(&self.instruction).map(|cap| Symbol::new(&cap["symbol"]))
    }

    // TODO: Rename function
    // TODO: Implement a regex pattern that works for both c-instruction variations
    fn c_command(&self) -> Option<(Dest, Comp, Jump)> {
        let semi = self.instruction.find(';').is_some();
        let equal = self.instruction.find('=').is_some();
        match (semi, equal) {
            // comp;jump
            (true, false) => {
                let c = Regex::new(r"(?P<comp>.{1,3});(?P<jump>.{3})").unwrap();
                c.captures(&self.instruction).map(|caps| {
                    (
                        Dest::empty(),
                        Comp::new(&caps["comp"]),
                        Jump::new(&caps["jump"]),
                    )
                })
            }
            // dest=comp
            (false, true) => {
                let c = Regex::new(r"^(?P<dest>[AMD]{1,3})=(?P<comp>.{1,3})").unwrap();
                c.captures(&self.instruction).map(|caps| {
                    (
                        Dest::new(&caps["dest"]),
                        Comp::new(&caps["comp"]),
                        Jump::empty(),
                    )
                })
            }
            // not valid
            (_, _) => None,
        }
    }

    // TODO: Rename function
    pub(crate) fn l_command(&self) -> Option<Symbol> {
        let l = Regex::new(r"^\((?P<symbol>[^)]+)\)$").unwrap();
        l.captures(&self.instruction).map(|cap| Symbol::new(&cap["symbol"]))
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
