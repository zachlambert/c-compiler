
use std::fmt;

#[derive(Clone)]
pub enum ALUOp {
    Add,
    Sub,
    // TODO
}

#[derive(Clone)]
pub enum Condition {
    Equal,
    LessThan,
    // TODO
}

#[derive(Clone)]
pub enum Instruction {
    Store, // Register -> Memory
    Load,  // Memory -> Register
    Move,  // Register -> Register
    ALUOp(ALUOp),  // Result(args) -> Register
    Jump,  // Unconditional jump to a label
    Branch(Condition), // Jump to label if Condition(RegA, RegB)
    Call,  // Call procedure
    Return, // Return from procedure
    Label, // Put a label here
}

#[derive(Clone)]
pub enum RegisterType {
    Int,
    Float,
    StackPointer,
}

#[derive(Clone)]
pub struct Register {
    index: u8,       // Index of register
    volatile: bool,  // Volatile or non-volatile register
    reg_type: RegisterType,
}

#[derive(Clone)]
pub enum Memory {
    Label,          // Memory[label]
    Offset,         // Memory[reg + offset]
    OffsetVariable, // Memory[reg + i(reg2)]
    OffsetStride,   // Memory[reg + stride*i(reg2) + j(reg3)]
}

#[derive(Clone)]
pub enum Constant {
    Int(i64),
    Float(f64),
    Str(String),
}

#[derive(Clone)]
pub enum Argument {
    Label(String),       // Assembly label
    Register(Register),
    Memory(Memory),
    Constant(Constant),
    Integer(i64),        // Offset or stride
}

#[derive(Clone)]
pub enum Pseudo { // Pseudoinstructions
    // Code required to save/restore temporary and saved registers.
    CallerBefore, // Code required by caller before a procedure call
    CallerAfter,  // Code requierd by caller after a produre call
    CalleeBefore, // Code required by a callee before procedure content
    CalleeAfter,  // Code required by a callee after procedure content
}

#[derive(Clone)]
pub enum Element {
    Instruction(Instruction),
    Argument(Argument),
    Pseudo(Pseudo),
    Blank,
}

impl Default for Element {
    fn default() -> Self { Element::Blank }
}

// Store an array of elements to encode instructions.
// eg:
// Store RegA -> Mem[SP + RegB]
// == [Instruction(Store), Arg(RegA), Arg(Memory(OfsetVariable)), Arg(SP), Arg(RegB)]

impl fmt::Display for ALUOp {
    fn fmt (&self, fmt: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ALUOp::Add => write!(fmt, "ALUOp(Add)"),
            ALUOp::Sub => write!(fmt, "ALUOp(Sub)"),
        }
    }
}

impl fmt::Display for Condition {
    fn fmt (&self, fmt: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Condition::Equal => write!(fmt, "Branch(Equal)"),
            Condition::LessThan => write!(fmt, "ALUOp(LessThan)"),
        }
    }
}

impl fmt::Display for Instruction {
    fn fmt (&self, fmt: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Instruction::Store => write!(fmt, "Instruction(Store)"),
            Instruction::Load => write!(fmt, "Instruction(Load)"),
            Instruction::Move => write!(fmt, "Instruction(Move)"),
            Instruction::ALUOp(alu_op) => write!(fmt, "Instruction({})", alu_op),
            Instruction::Jump => write!(fmt, "Instruction(Jump)"),
            Instruction::Branch(condition) => write!(fmt, "Instruction({})", condition),
            Instruction::Call => write!(fmt, "Instruction(Call)"),
            Instruction::Return => write!(fmt, "Instruction(Return)"),
            Instruction::Label => write!(fmt, "Instruction(Label)"),
        }
    }
}

impl fmt::Display for RegisterType {
    fn fmt (&self, fmt: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RegisterType::Int => write!(fmt, "RegType(Int)"),
            RegisterType::Float => write!(fmt, "RegType(Float)"),
            RegisterType::StackPointer => write!(fmt, "RegType(StackPointer)"),
        }
    }
}

impl fmt::Display for Register {
    fn fmt (&self, fmt: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        write!(fmt,
               "Register(index: {}, volatile: {}, type: {})",
               self.index, self.volatile, self.reg_type)
    }
}

impl fmt::Display for Memory {
    fn fmt (&self, fmt: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Memory::Label => write!(fmt, "Memory(Label)"),
            Memory::Offset => write!(fmt, "Memory(Offset)"),
            Memory::OffsetVariable => write!(fmt, "Memory(OffsetVariable)"),
            Memory::OffsetStride => write!(fmt, "Memory(OffsetStride)"),
        }
    }
}
impl fmt::Display for Constant {
    fn fmt (&self, fmt: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Constant::Int(constant) => write!(fmt, "Contant(Int({}))", constant),
            Constant::Float(constant) => write!(fmt, "Contant(Float({}))", constant),
            Constant::Str(constant) => write!(fmt, "Contant(Str({}))", constant),
        }
    }
}

impl fmt::Display for Argument {
    fn fmt (&self, fmt: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Argument::Label(label) => write!(fmt, "Argument(Label({}))", label),
            Argument::Register(register) => write!(fmt, "Argument({})", register),
            Argument::Memory(memory) => write!(fmt, "Argument({})", memory),
            Argument::Constant(constant) => write!(fmt, "Argument({})", constant),
            Argument::Integer(integer) => write!(fmt, "Argument(Integer({}))", integer),
        }
    }
}

impl fmt::Display for Pseudo {
    fn fmt (&self, fmt: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Pseudo::CalleeBefore => write!(fmt, "Pseudo(CalleeBefore)"),
            Pseudo::CalleeAfter => write!(fmt, "Pseudo(CalleeAfter)"),
            Pseudo::CallerBefore => write!(fmt, "Pseudo(CallerBefore)"),
            Pseudo::CallerAfter => write!(fmt, "Pseudo(CallerAfter)"),
        }
    }
}

impl fmt::Display for Element {
    fn fmt (&self, fmt: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Element::Instruction(instruction) => write!(fmt, "{}", instruction),
            Element::Argument(argument) => write!(fmt, "{}", argument),
            Element::Pseudo(pseudo) => write!(fmt, "{}", pseudo),
            Element::Blank => write!(fmt, "Blank"),
        }
    }
}

