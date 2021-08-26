
use std::fmt;

// ===== Instructions =====

#[derive(Clone)]
pub enum ALUOp {
    Add,
    Sub,
    // TODO
}


impl fmt::Display for ALUOp {
    fn fmt (&self, fmt: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ALUOp::Add => write!(fmt, "ALUOp(Add)"),
            ALUOp::Sub => write!(fmt, "ALUOp(Sub)"),
        }
    }
}

#[derive(Clone)]
pub enum Condition {
    Equal,
    LessThan,
    // TODO
}

impl fmt::Display for Condition {
    fn fmt (&self, fmt: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Condition::Equal => write!(fmt, "Branch(Equal)"),
            Condition::LessThan => write!(fmt, "ALUOp(LessThan)"),
        }
    }
}

#[derive(Clone)]
pub enum Instruction {
    Move,  // Assignment. Decide between register / memory later.
    GetArgument, // Move argument(location) -> symbol
    SetReturned, // Move symbol -> returned(location)
    ALUOp(ALUOp),  // Result(args) -> Register
    Jump,  // Unconditional jump to a label
    Branch(Condition), // Jump to label if Condition(RegA, RegB)
    Call,  // Call procedure
    Return, // Return from procedure
    Label, // Put a label here
}

impl fmt::Display for Instruction {
    fn fmt (&self, fmt: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Instruction::Move => write!(fmt, "Instruction(Move)"),
            Instruction::GetArgument => write!(fmt, "Instruction(GetArgument)"),
            Instruction::SetReturned => write!(fmt, "Instruction(SetReturned)"),
            Instruction::ALUOp(alu_op) => write!(fmt, "Instruction({})", alu_op),
            Instruction::Jump => write!(fmt, "Instruction(Jump)"),
            Instruction::Branch(condition) => write!(fmt, "Instruction({})", condition),
            Instruction::Call => write!(fmt, "Instruction(Call)"),
            Instruction::Return => write!(fmt, "Instruction(Return)"),
            Instruction::Label => write!(fmt, "Instruction(Label)"),
        }
    }
}


// ===== Arguments to instructions relating to memory =====

#[derive(Clone)]
pub enum Datatype {
    Integer,
    Float,
    Pointer,
    Struct,
}

impl fmt::Display for Datatype {
    fn fmt (&self, fmt: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Datatype::Integer => write!(fmt, "Datatype(Integer)"),
            Datatype::Float => write!(fmt, "Datatype(Float)"),
            Datatype::Pointer => write!(fmt, "Datatype(Pointer)"),
            Datatype::Struct => write!(fmt, "Datatype(Struct)"),
        }
    }
}

// Used to specify location for passing arguments to procedures and returning
// return values from procedures.
// Registers and stacks are allocated based on size and datatype. Index is used
// to allocate in a consistent order.
#[derive(Clone)]
pub struct PassLocation {
    pub index: usize,
    pub size: usize,
    pub datatype: Datatype,
}

impl fmt::Display for PassLocation {
    fn fmt (&self, fmt: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        write!(fmt,
               "PassLocation(index: {}, size: {}, datatype: {})",
               self.index, self.size, self.datatype)
    }
}

// Generic reference to a local variable.
// Stores variable name and a version number (for SSA).
// Also stores size and datatype to inform instructions that operate on them.
#[derive(Clone)]
pub struct Symbol {
    pub name: String,
    pub version: usize,
    pub size: usize,
    pub datatype: Datatype,
}

impl fmt::Display for Symbol {
    fn fmt (&self, fmt: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        write!(fmt,
               "Symbol(name: {}, version: {}, size: {}, datatype: {}",
               self.name, self.version, self.size, self.datatype)
    }
}

// Immediate value.
#[derive(Clone)]
pub enum Constant {
    Int(i64),
    Float(f64),
}

impl fmt::Display for Constant {
    fn fmt (&self, fmt: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Constant::Int(constant) => write!(fmt, "Contant(Int({}))", constant),
            Constant::Float(constant) => write!(fmt, "Contant(Float({}))", constant),
        }
    }
}

#[derive(Clone)]
pub enum Argument {
    Label(String),       // Assembly label
    PassLocation(PassLocation),
    Symbol(Symbol),      // Generic symbol
    Constant(Constant),
    Integer(i64),        // Offset or stride
}

impl fmt::Display for Argument {
    fn fmt (&self, fmt: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Argument::Label(label) => write!(fmt, "Argument(Label({}))", label),
            Argument::PassLocation(pass_location) => write!(fmt, "Argument({})", pass_location),
            Argument::Symbol(symbol) => write!(fmt, "Argument({})", symbol),
            Argument::Constant(constant) => write!(fmt, "Argument({})", constant),
            Argument::Integer(integer) => write!(fmt, "Argument(Integer({}))", integer),
        }
    }
}

#[derive(Clone)]
pub enum Element {
    Instruction(Instruction),
    Argument(Argument),
    Blank,
}

impl Default for Element {
    fn default() -> Self { Element::Blank }
}

impl fmt::Display for Element {
    fn fmt (&self, fmt: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Element::Instruction(instruction) => write!(fmt, "{}", instruction),
            Element::Argument(argument) => write!(fmt, "{}", argument),
            Element::Blank => write!(fmt, "Blank"),
        }
    }
}
