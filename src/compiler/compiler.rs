
use std::collections::HashMap;

use crate::parser::ast::Ast;
use super::symbol::Symbol;
use super::datatype::Function;
use super::datatype::Struct;

pub struct Compiler<'a> {
    pub ast: &'a Ast,
    pub code: &'a mut String,
    pub symbols: HashMap<&'a String, Symbol<'a>>,
    pub functions: Vec<Function<'a>>,
    pub structs: Vec<Struct<'a>>,
    // TODO: Add types in the future.
}

impl<'a> Compiler<'a> {
    pub fn new(ast: &'a Ast, code: &'a mut String) -> Compiler<'a> {
        Compiler {
            ast: ast,
            code: code,
            symbols: HashMap::new(),
            functions: Vec::new(),
            structs: Vec::new(),
        }
    }

    pub fn add_symbol(&mut self, name: &'a String, symbol: Symbol<'a>) {
        self.symbols.insert(name, symbol);
    }
}
