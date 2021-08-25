
use std::fmt;

#[derive(Clone)]
pub enum Control {
    Break,
    Continue,
}

impl fmt::Display for Control {
    fn fmt (&self, fmt: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Control::Break => write!(fmt, "Control(Break)"),
            Control::Continue => write!(fmt, "Control(Continue)"),
        }
    }
}

#[derive(Clone)]
pub enum Statement {
    // Declare,
    // identifier , ":" datatype , ";"

    Assign,
    // expression , "=" ,  expression , ";"

    Return,
    // expression
    
    Conditional,
    // expression , block , [ block ]
    
    Loop,
    // block

    Control(Control),
    // Terminal

    Block,
    // block
}

impl fmt::Display for Statement {
    fn fmt (&self, fmt: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Statement::Assign => write!(fmt, "Statement(Assign)"),
            Statement::Return => write!(fmt, "Statement(Return)"),
            Statement::Conditional => write!(fmt, "Statement(Conditional)"),
            Statement::Loop => write!(fmt, "Statement(Loop)"),
            Statement::Control(control) => write!(fmt, "Statement({})", control),
            Statement::Block => write!(fmt, "Statement(Block)"),
        }
    }
}


