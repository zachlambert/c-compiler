
mod checker;
mod content;
mod symbol;
mod function;
mod statement;
mod expression;

use crate::parser::ast::Ast;
use checker::Checker;
use content::check_content;

pub fn resolve_ast(ast: &mut Ast) {
    let mut checker = Checker::new(ast);
    let root_i = checker.ast.nodes.len() - 1;
    check_content(&mut checker, root_i);
}
