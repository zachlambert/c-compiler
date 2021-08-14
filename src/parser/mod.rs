
use crate::lexer::token::*;

pub mod symbol;
pub mod ast;
pub mod parser;
mod program;
// mod statement;
// mod expression;

use ast::Ast;
use parser::Parser;

pub fn create_ast(tokens: &Vec<Token>) -> Option<Ast> {
    let mut ast = Ast::new();
    let mut parser = Parser::new(
        &mut ast,
        tokens,
        128,
        32
    );
    if !program::match_program(&mut parser) {
        println!("Failed to match program");
        return None;
    }
    Some(ast)
}
