
pub mod program;
mod structure;
mod function;
mod symbol;
mod block;

mod statement;
mod expression;
mod datatype;
mod common;

// Make these available for submodules
use super::construct;
use crate::lexer::token;
use super::parser;
