
pub enum Instruction {
    Store, // Register -> Memory
    Load,  // Memory -> Register
    Move,  // Register -> Register
    AluOp(AluOp),  // Result(args) -> Register
    Jump,  // Unconditional jump to a label
    Branch(Branch), // Conditional jump to label, for given expression and condition
    Call,  // Call procedure
    Return, // Return from procedure
    Label, // Put a label here
}

pub enum RegisterType {
    Int,
    Float,
    StackPointer,
}

pub struct Register {
    index: u8,       // Index of register
    volatile: bool,  // Volatile or non-volatile register
    reg_type: RegisterType,
}

pub enum Memory {
    Label,          // Memory[label]
    Offset,         // Memory[reg + offset]
    OffsetVariable, // Memory[reg + i(reg2)]
    OffsetStride,   // Memory[reg + stride*i(reg2) + j(reg3)]
}

pub enum Argument {
    Label(String),       // Assembly label
    Register(Register),
    Memory(Memory),
    Integer(i64),        // Offset or stride
}

pub enum Element {
    Instruction(Instruction),
    Argument(Argument),
}

// Store an array of elements to encode instructions.
// eg:
// Store RegA -> Mem[SP + RegB]
// == [Instruction(Store), Arg(RegA), Arg(Memory(OfsetVariable)), Arg(SP), Arg(RegB)]

