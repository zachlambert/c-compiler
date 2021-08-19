
use crate::parser::ast::Ast;
use std::collections::HashMap;

// Mappings: [ main, func1, global_var, argc, argv, x, ... ]
//           <--  scope 0 ----------> < --- scope 1 --> etc
// Each mapping store:
// - Index of node (in ast)
// - Index of previous mapping with the same name
// - Name of mapping
//
// Symbol table is a HashMap of:
//   name -> index of mapping


#[derive(Clone)]
pub enum Storage {
    Local(i64),   // Stack offset
    Global(String), // Label
}

#[derive(Clone)]
pub enum SymbolData {
    Variable(Storage),
    Function(String),  // Label
    Struct,
}

#[derive(Clone)]
pub struct Symbol {
    pub node_i: usize,
    pub data: SymbolData,
}

struct Mapping {
    pub symbol: Symbol,
    pub prev: Option<usize>,
    pub name: String,
}

pub struct Checker<'a> {
    pub ast: &'a mut Ast,
    table: HashMap<String, Option<usize>>, // mapping index
    mappings: Vec<Mapping>,
    scope: Vec<usize>, // stack of the start of valid mappings
    depth: usize,
}

impl<'a> Checker<'a> {
    pub fn new(ast: &'a mut Ast) -> Checker<'a> {
        Checker {
            ast: ast,
            table: HashMap::new(),
            mappings: Vec::new(),
            scope: Vec::new(),
            depth: 0,
        }
    }

    pub fn find_symbol(&'a self, name: &String) -> Option<&'a Symbol> {
        match self.table[name] {
            Some(index) => {
                Some(&self.mappings[index].symbol)
            },
            _ => None,
        }
    }

    pub fn add_symbol(&mut self, name: &String, symbol: Symbol) {
        let mapping = Mapping {
            symbol: Symbol::clone(&symbol),
            prev: self.table[name],
            name: String::clone(name),
        };
        self.table.insert(String::clone(name), Some(self.mappings.len()));
        self.mappings.push(mapping);
    }

    pub fn increase_scope(&mut self) {
        self.scope.push(self.mappings.len());
        self.depth += 1;
    }

    pub fn decrease_scope(&mut self) {
        let start = self.scope.pop()
            .expect("Decreasing scope before increasing it");
        while self.mappings.len() > start {
            let mapping = self.mappings.pop()
                .expect("Shouldn't be here");
            self.table.insert(String::clone(&mapping.name), mapping.prev);
        }
        self.depth -= 1;
    }

    pub fn current_depth(&self) -> usize {
        self.depth
    }
}
