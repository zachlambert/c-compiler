
use std::fmt;
use crate::lexer::token::Constant;
use super::operation::*;

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

    Identifier,
    // identifier
}

impl fmt::Display for Expression {
    fn fmt (&self, fmt: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expression::Function => write!(fmt, "Expression(function)"),
            Expression::UnaryOp(op) => write!(fmt, "Expression({})", op),
            Expression::BinaryOp(op) => write!(fmt, "Expression({})", op),
            Expression::Constant(constant) => write!(fmt, "Expression({})", constant),
            Expression::Identifier => write!(fmt, "Expression(Identifier)"),
        }
    }
}


