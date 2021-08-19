use std::fmt;

use crate::lexer::token::*;


// ===== DATATYPES and QUALIFIERS =====

#[derive(Clone)]
pub enum Primitive {
    I8,
    I16,
    I32,
    I64,
    U8,
    U16,
    U32,
    U64,
    F32,
    F64,
    C8,
}

impl fmt::Display for Primitive {
    fn fmt (&self, fmt: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Primitive::I8 => write!(fmt, "Primitive(i8)"),
            Primitive::I16 => write!(fmt, "Primitive(i16)"),
            Primitive::I32 => write!(fmt, "Primitive(i32)"),
            Primitive::I64 => write!(fmt, "Primitive(i64)"),
            Primitive::U8 => write!(fmt, "Primitive(u8)"),
            Primitive::U16 => write!(fmt, "Primitive(u16)"),
            Primitive::U32 => write!(fmt, "Primitive(u32)"),
            Primitive::U64 => write!(fmt, "Primitive(u64)"),
            Primitive::F32 => write!(fmt, "Primitive(f32)"),
            Primitive::F64 => write!(fmt, "Primitive(f64)"),
            Primitive::C8 => write!(fmt, "Primitive(c8)"),
        }
    }
}

#[derive(Clone)]
pub enum Datatype {
    Terminal,
    // { qualifier } , ( primitive | identifier | struct )

    Pointer,
    // { qualifier } , datatype

    Inferred,
    // Terminal
}

impl fmt::Display for Datatype {
    fn fmt (&self, fmt: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Datatype::Terminal => write!(fmt, "Datatype(Terminal)"),
            Datatype::Pointer => write!(fmt, "Datatype(Pointer)"),
            Datatype::Inferred => write!(fmt, "Datatype(Inferred)"),
        }
    }
}

#[derive(Clone)]
pub enum Qualifier {
    Mut,
}

impl fmt::Display for Qualifier {
    fn fmt (&self, fmt: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Qualifier::Mut => write!(fmt, "Qualifier(mut)"),
        }
    }
}


// ===== OPERATIONS =====

#[derive(Clone)]
pub enum UnaryOp {
    Negate,
    LogicalNot,
    Deref, // *x
    Ref, // &x
}

impl fmt::Display for UnaryOp {
    fn fmt (&self, fmt: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UnaryOp::Negate => write!(fmt, "UnaryOp(Negate)"),
            UnaryOp::LogicalNot => write!(fmt, "UnaryOp(LogicalNot)"),
            UnaryOp::Deref => write!(fmt, "UnaryOp(Deref)"),
            UnaryOp::Ref => write!(fmt, "UnaryOp(Ref)"),
        }
    }
}

#[derive(Clone)]
pub enum BinaryOp {
    Add,
    Subtract,
    Multiply,
    Divide,

    LogicalAnd,
    LogicalOr,
    LogicalEquals,

    BitwiseAnd,
    BitwiseOr,

    Access, // my_struct.member
}

impl fmt::Display for BinaryOp {
    fn fmt (&self, fmt: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BinaryOp::Add => write!(fmt, "BinaryOp(Add)"),
            BinaryOp::Subtract => write!(fmt, "BinaryOp(Subtract)"),
            BinaryOp::Multiply => write!(fmt, "BinaryOp(Multiply)"),
            BinaryOp::Divide => write!(fmt, "BinaryOp(Divide)"),

            BinaryOp::LogicalAnd => write!(fmt, "BinaryOp(LogicalAnd)"),
            BinaryOp::LogicalOr => write!(fmt, "BinaryOp(LogicalOr)"),
            BinaryOp::LogicalEquals => write!(fmt, "BinaryOp(LogicalEquals)"),

            BinaryOp::BitwiseAnd => write!(fmt, "BinaryOp(BitwiseAnd)"),
            BinaryOp::BitwiseOr => write!(fmt, "BinaryOp(BitwiseOr)"),

            BinaryOp::Access => write!(fmt, "BinaryOp(Access)"),
        }
    }
}


// ===== STATEMENTS and EXPRESSIONS =====

#[derive(Clone)]
pub enum Statement {
    Declare,
    // identifier , ":" datatype , ";"

    Initialise,
    // identifier , ":" , [ type ] , "=" , expression , ";"

    Assign,
    // expression , "=" ,  expression , ";"

    Return,
    // expression
}

impl fmt::Display for Statement {
    fn fmt (&self, fmt: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Statement::Declare => write!(fmt, "Statement(declare)"),
            Statement::Initialise => write!(fmt, "Statement(initialise)"),
            Statement::Assign => write!(fmt, "Statement(assign)"),
            Statement::Return => write!(fmt, "Statement(return)"),
        }
    }
}

#[derive(Clone)]
pub enum Expression {
    Function,
    // identifier , { argument }

    UnaryOp(UnaryOp), // Operator
    // operand

    BinaryOp(BinaryOp), // Operator
    // left operand , right operand

    Constant(Constant),
    // Terminal

    Identifier(String),
    // Terminal
}

impl fmt::Display for Expression {
    fn fmt (&self, fmt: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expression::Function => write!(fmt, "Expression(function)"),
            Expression::UnaryOp(op) => write!(fmt, "Expression({})", op),
            Expression::BinaryOp(op) => write!(fmt, "Expression({})", op),
            Expression::Constant(constant) => write!(fmt, "Expression({})", constant),
            Expression::Identifier(identifier) => write!(fmt, "Expression(Identifier({}))", identifier),
        }
    }
}


// ===== CONSTRUCTS =====

#[derive(Clone)]
pub enum Construct {
    Program,
    // { function | struct }

    Function,
    // identifier , ":" , "=",  "function" , "(" , [ argument , { "," , argument } ] , ")" ,
    // [ "->" , ( return , ( "(" , {return} , ")" ) ) ] , "{" , { statement } , "}"

    Argument,
    // identifier , ":" , datatype
    
    Returned, // May add named return values in future
    // datatype

    Statement(Statement), // Statement type
    // ( Statement::Declare | ... etc )

    Structure,
    // identifier , ":" , "=" , "struct" , { member }

    Member,
    // identifier , ":" , datatype

    Datatype(Datatype),
    // ( Datatype::Terminal | ... )

    Qualifier(Qualifier),
    // Terminal

    Primitive(Primitive),
    // Terminal

    Identifier(String), // name
    // Terminal

    Expression(Expression),
    // ( Expression::Function | ... )
}

impl fmt::Display for Construct {
    fn fmt (&self, fmt: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Construct::Program => write!(fmt, "Program"),
            Construct::Function => write!(fmt, "Function"),
            Construct::Argument => write!(fmt, "Argument"),
            Construct::Returned => write!(fmt, "Returned"),
            Construct::Statement(statement) => write!(fmt, "{}", statement),
            Construct::Structure => write!(fmt, "Structure"),
            Construct::Member => write!(fmt, "Member"),
            Construct::Datatype(datatype) => write!(fmt, "{}", datatype),
            Construct::Identifier(name) => write!(fmt, "Identifier({})", name),
            Construct::Qualifier(qualifier) => write!(fmt, "{}", qualifier),
            Construct::Primitive(primitive) => write!(fmt, "{}", primitive),
            Construct::Expression(expression) => write!(fmt, "{}", expression),
        }
    }
}
