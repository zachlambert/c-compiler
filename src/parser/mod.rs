
use crate::lexer::token::*;

pub mod symbol;
pub mod ast;
mod program;
mod statement;
mod expression;

use ast::*;

pub fn create_ast(tokens: &Vec<Token>) -> Option<Ast> {
    let mut ast = Ast::new();
    let state = ParserState {
        token_i: 0,
        node_i: 0,
    };
    if program::match_program(state, tokens, &mut ast).is_none() {
        println!("Failed to match program");
        return None;
    }
    Some(ast)
}
