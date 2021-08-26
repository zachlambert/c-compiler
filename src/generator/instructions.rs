
use std::fmt;

// ===== Instructions =====

#[derive(Clone, Copy)]
pub enum ALUOp {
    Add,
    Sub,
    Negate,
    // TODO
}


impl fmt::Display for ALUOp {
    fn fmt (&self, fmt: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ALUOp::Add => write!(fmt, "ALUOp(Add)"),
            ALUOp::Sub => write!(fmt, "ALUOp(Sub)"),
            ALUOp::Negate => write!(fmt, "ALUOp(Negate)"),
        }
    }
}

#[derive(Clone, Copy)]
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

#[derive(Clone, Copy)]
pub enum Instruction {
    Move,  // Move src -> dst
    Load,  // Move Mem[src] -> dst
    Store, // Move src -> Mem[dst]
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
            Instruction::Load => write!(fmt, "Instruction(Load)"),
            Instruction::Store => write!(fmt, "Instruction(Store)"),
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


// ===== Operands to instructions relating to memory =====

#[derive(Clone, Copy)]
pub enum Regtype {
    Integer,
    Float,
    Struct,
    Pointer,
}

impl fmt::Display for Regtype {
    fn fmt (&self, fmt: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Regtype::Integer => write!(fmt, "Regtype(Integer)"),
            Regtype::Float => write!(fmt, "Regtype(Float)"),
            Regtype::Struct => write!(fmt, "Regtype(Struct)"),
            Regtype::Pointer => write!(fmt, "Regtype(Pointer)"),
        }
    }
}

// Used to specify location for passing arguments to procedures and returning
// return values from procedures.
// Registers and stacks are allocated based on size and datatype. Index is used
// to allocate in a consistent order.
#[derive(Clone, Copy)]
pub struct PassLocation {
    pub index: usize,
    pub size: usize,
    pub regtype: Regtype,
}

impl fmt::Display for PassLocation {
    fn fmt (&self, fmt: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        write!(fmt,
               "PassLocation(index: {}, size: {}, regtype: {})",
               self.index, self.size, self.regtype)
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
    pub regtype: Regtype,
}

impl fmt::Display for Symbol {
    fn fmt (&self, fmt: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        write!(fmt,
               "Symbol(name: {}, version: {}, size: {}, regtype: {}",
               self.name, self.version, self.size, self.regtype)
    }
}

// Immediate value.
#[derive(Clone, Copy)]
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
pub enum Operand {
    Label(String),       // Assembly label
    PassLocation(PassLocation),
    Symbol(Symbol),      // Generic symbol
    Constant(Constant),
    Integer(i64),        // Offset or stride
}

impl fmt::Display for Operand {
    fn fmt (&self, fmt: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Operand::Label(label) => write!(fmt, "Operand(Label({}))", label),
            Operand::PassLocation(pass_location) => write!(fmt, "Operand({})", pass_location),
            Operand::Symbol(symbol) => write!(fmt, "Operand({})", symbol),
            Operand::Constant(constant) => write!(fmt, "Operand({})", constant),
            Operand::Integer(integer) => write!(fmt, "Operand(Integer({}))", integer),
        }
    }
}

#[derive(Clone)]
pub enum Element {
    Instruction(Instruction),
    Operand(Operand),
    Blank,
}

impl Default for Element {
    fn default() -> Self { Element::Blank }
}

impl fmt::Display for Element {
    fn fmt (&self, fmt: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Element::Instruction(instruction) => write!(fmt, "{}", instruction),
            Element::Operand(operand) => write!(fmt, "{}", operand),
            Element::Blank => write!(fmt, "Blank"),
        }
    }
}
