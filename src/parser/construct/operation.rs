
use std::fmt;

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

