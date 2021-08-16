use std::fmt;

use crate::lexer::token::*;

// ===== FUNCTIONS =====

#[derive(Clone)]
pub struct Argument {
    pub name: String,
    pub arg_type: Datatype,
}

impl fmt::Display for Argument {
    fn fmt (&self, fmt: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        write!(fmt, "Argument[arg_type: {}, name: {}]", self.arg_type, self.name)
    }
}

#[derive(Clone)]
pub enum Datatype {
    Unresolved(String),
    Primitive(Primitive),
    Pointer,
    Struct,
}

impl fmt::Display for Datatype {
    fn fmt (&self, fmt: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Datatype::Unresolved(identifier) => write!(fmt, "Datatype(Unresolved: {})", identifier),
            Datatype::Primitive(primitive) => write!(fmt, "Datatype({})", primitive),
            Datatype::Pointer => write!(fmt, "Datatype(Pointer)"),
            Datatype::Struct => write!(fmt, "Datatype(Struct)"),
        }
    }
}

#[derive(Clone)]
pub enum Qualifier {
    Const,
    Static,
    Extern,
}

impl fmt::Display for Qualifier {
    fn fmt (&self, fmt: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Qualifier::Const => write!(fmt, "Qualifier(const)"),
            Qualifier::Static => write!(fmt, "Qualifier(static)"),
            Qualifier::Extern => write!(fmt, "Qualifier(extern)"),
        }
    }
}

#[derive(Clone)]
pub struct Typedef {
    pub name: String,
}


#[derive(Clone)]
pub enum UnaryOp {
    Negate,
    LogicalNot,
}

impl fmt::Display for UnaryOp {
    fn fmt (&self, fmt: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UnaryOp::Negate => write!(fmt, "UnaryOp(Negate)"),
            UnaryOp::LogicalNot => write!(fmt, "UnaryOp(LogicalNot)"),
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
    BitwiseAnd,
    BitwiseOr,
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
            BinaryOp::BitwiseAnd => write!(fmt, "BinaryOp(BitwiseAnd)"),
            BinaryOp::BitwiseOr => write!(fmt, "BinaryOp(BitwiseOr)"),
        }
    }
}

#[derive(Clone)]
pub enum Statement {
    Declare(Datatype, String),    // <type> <identifier>;
    Initialise(Datatype, String), // <type> <identifier> = <expression>;
    Assign(String),           // <identifier> = <expression>;
    Return,                   // return <expression>;
}

impl fmt::Display for Statement {
    fn fmt (&self, fmt: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Statement::Declare(statement_type, identifier) =>
                write!(fmt,
                       "Statement[declare, type: {}, identifier: {}]",
                       statement_type, identifier),
            Statement::Initialise(statement_type, identifier) =>
                write!(fmt,
                       "Statement[initialise, type: {}, identifier: {}]",
                       statement_type, identifier),
            Statement::Assign(identifier) =>
                write!(fmt,
                       "Statement[assign, identifier: {}]",
                       identifier),
            Statement::Return =>
                write!(fmt, "Statement[return]"),
        }
    }
}

#[derive(Clone)]
pub enum Expression {
    Function,
    // - 0: Identifier(name) => FunctionSignature
    // - 1: First argument
    // - 2: Seconds argument
    // - 3: etc...

    UnaryOp(UnaryOp),
    // - 0: Argument

    BinaryOp(BinaryOp),
    // - 0: Left argument
    // - 1: Right argument

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

#[derive(Clone)]
pub enum Construct {
    Program,
    FunctionSignature,
    Statement(Statement),
    Expression(Expression),
    Argument(Argument),
    Datatype(Datatype),
    Identifier(String),
}

impl fmt::Display for Construct {
    fn fmt (&self, fmt: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Construct::Program => write!(fmt, "Program"),
            Construct::Function(function) => write!(fmt, "{}", function),
            Construct::Statement(statement) => write!(fmt, "{}", statement),
            Construct::Expression(expression) => write!(fmt, "{}", expression),
            Construct::Argument(argument) => write!(fmt, "{}", argument),
        }
    }
}
