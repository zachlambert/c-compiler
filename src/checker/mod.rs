

mod checker;
mod resolve;

use crate::parser::ast::Ast;
use checker::Checker;

pub fn resolve_ast(ast: &mut Ast) -> bool {
    let mut checker = Checker::new(ast);
    return resolve::resolve_ast(&mut checker);
}

pub fn validate_ast(ast: &Ast) -> bool {
    return true;
}
