

mod checker;
mod resolve;

use crate::parser::ast::Ast;
use checker::Checker;
use resolve::resolve_node;

pub fn resolve_ast(ast: &mut Ast) {
    let mut checker = Checker::new(ast);
    let root_i = checker.ast.nodes.len() - 1;
    resolve_node(&mut checker, root_i);
}
