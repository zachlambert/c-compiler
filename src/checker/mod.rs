

mod checker;
mod block;
mod function;
mod symbol;
mod node;

use crate::parser::ast::Ast;
use checker::Checker;
use block::resolve_block;

pub fn resolve_ast(ast: &mut Ast) {
    let mut checker = Checker::new(ast);
    let root_i = checker.ast.nodes.len() - 1;
    resolve_block(&mut checker, root_i);
}
