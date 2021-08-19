
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

struct Mapping<'a> {
    pub node_i: usize,
    pub prev: Option<usize>,
    pub name: &'a String,
}

pub struct Checker<'a> {
    pub ast: &'a mut Ast,
    table: HashMap<String, Option<usize>>, // mapping index
    mappings: Vec<Mapping<'a>>,
    scope: Vec<usize>, // stack of the start of valid mappings
}

impl<'a> Checker<'a> {
    pub fn new(ast: &'a mut Ast) -> Checker<'a> {
        Checker {
            ast: ast,
            table: HashMap::new(),
            mappings: Vec::new(),
            scope: Vec::new(),
        }
    }

    pub fn find_symbol(&self, name: &String) -> Option<usize> {
        match self.table[name] {
            Some(index) => {
                Some(self.mappings[index].node_i)
            },
            _ => None,
        }
    }

    pub fn add_symbol(&mut self, name: &'a String, node_i: usize) {
        let mapping = Mapping {
            node_i: node_i,
            prev: self.table[name],
            name: name,
        };
        self.table.insert(String::clone(name), Some(self.mappings.len()));
        self.mappings.push(mapping);
    }

    pub fn increase_scope(&mut self) {
        self.scope.push(self.mappings.len());
    }

    pub fn decrease_scope(&mut self) {
        let start = self.scope.pop()
            .expect("Decreasing scope before increasing it");
        while self.mappings.len() > start {
            let mapping = self.mappings.pop()
                .expect("Shouldn't be here");
            self.table.insert(String::clone(mapping.name), mapping.prev);
        }
    }
}
