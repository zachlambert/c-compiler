use std::fmt;

use crate::lexer::token::*;

#[derive(Clone)]
pub struct Function {
    pub name: String,
    pub ret_type: Type,
}

impl fmt::Display for Function {
    fn fmt (&self, fmt: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        write!(fmt, "Function[ret_type: {}, name: {}]", self.ret_type, self.name)
    }
}

#[derive(Clone)]
pub enum Type {
    Primitive(Primitive),
    Identifier(String),
}

impl fmt::Display for Type {
    fn fmt (&self, fmt: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Type::Primitive(primitive) => write!(fmt, "Type({})", primitive),
            Type::Identifier(identifier) => write!(fmt, "Type({})", identifier),
        }
    }
}

#[derive(Clone)]
pub struct Argument {
    pub name: String,
    pub arg_type: Type,
}

impl fmt::Display for Argument {
    fn fmt (&self, fmt: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        write!(fmt, "Argument[arg_type: {}, name: {}]", self.arg_type, self.name)
    }
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
    Declare(Type, String),    // <type> <identifier>;
    Initialise(Type, String), // <type> <identifier> = <expression>;
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
    Function(String), // n child expressions for args
    UnaryOp(UnaryOp), // one child expression
    BinaryOp(BinaryOp), // Two child expressions
    Constant(Constant), // Terminal
    Identifier(String), // Terminal
}

impl fmt::Display for Expression {
    fn fmt (&self, fmt: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expression::Function(name) => write!(fmt, "Expression(function, name: {})", name),
            Expression::UnaryOp(op) => write!(fmt, "Expression({})", op),
            Expression::BinaryOp(op) => write!(fmt, "Expression({})", op),
            Expression::Constant(constant) => write!(fmt, "Expression({})", constant),
            Expression::Identifier(identifier) => write!(fmt, "Expression(Identifier({}))", identifier),
        }
    }
}

#[derive(Clone)]
pub enum Symbol {
    Program,
    Function(Function),
    Statement(Statement),
    Expression(Expression),
    Argument(Argument),
}

impl fmt::Display for Symbol {
    fn fmt (&self, fmt: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Symbol::Program => write!(fmt, "Program"),
            Symbol::Function(function) => write!(fmt, "{}", function),
            Symbol::Statement(statement) => write!(fmt, "{}", statement),
            Symbol::Expression(expression) => write!(fmt, "{}", expression),
            Symbol::Argument(argument) => write!(fmt, "{}", argument),
        }
    }
}
