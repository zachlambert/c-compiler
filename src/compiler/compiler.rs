
use crate::parser::ast::Ast;

pub struct Compiler<'a> {
    pub ast: &'a Ast,
    pub code: &'a mut String,
    // TODO:
    // - Symbol table (functions + global vars)
    // - Types (for now, just primitives so ignore this)
}

impl<'a> Compiler<'a> {
    pub fn new(ast: &'a Ast, code: &'a mut String) -> Compiler<'a> {
        Compiler {
            ast: ast,
            code: code,
        }
    }
}
