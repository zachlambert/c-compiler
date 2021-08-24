
use crate::parser::ast::Ast;
use crate::parser::construct::Construct;
use std::collections::HashMap;
use super::instructions::Element;

// Mappings: [ main, func1, global_var, argc, argv, x, ... ]
//           <--  scope 0 ----------> < --- scope 1 --> etc
// Each mapping store:
// - Index of node (in ast)
// - Index of previous mapping with the same name
// - Name of mapping
//
// Symbol table is a HashMap of:
//   name -> index of mapping

struct Mapping {
    pub node_i: usize,
    pub prev: Option<usize>,
    pub name: String,
}

pub struct Generator<'a> {
    pub ast: &'a mut Ast,
    instructions: &'a mut Vec<Element>,
    table: HashMap<String, usize>, // mapping index
    mappings: Vec<Mapping>,
    scope: Vec<usize>, // stack of the start of valid mappings
    tree_stack: Vec<usize>,
}

impl<'a> Generator<'a> {
    pub fn new(ast: &'a mut Ast, instructions: &'a mut Vec<Element>) -> Generator<'a> {
        let start_i = ast.nodes.len() - 1;
        let mut generator = Generator {
            ast: ast,
            instructions: instructions,
            table: HashMap::new(),
            mappings: Vec::new(),
            scope: Vec::new(),
            tree_stack: Vec::new(),
        };
        generator.tree_stack.push(start_i);
        return generator;
    }

    pub fn current_mut(&'a mut self) -> &'a mut Construct {
        let node_i = *self.tree_stack.last()
            .expect("Tried to call current_mut() on an empty tree_stack");
        return &mut self.ast.nodes[node_i].construct;
    }

    pub fn current(&'a self) -> &'a Construct {
        let node_i = *self.tree_stack.last()
            .expect("Tried to call current() on an empty tree_stack");
        return &self.ast.nodes[node_i].construct;
    }

    pub fn down(&mut self) -> bool {
        let node_i = *self.tree_stack.last()
            .expect("Tried to call down() on an empty tree_stack");
        match self.ast.nodes[node_i].child {
            Some(child_i) => {
                self.tree_stack.push(child_i);
                return true;
            },
            None => return false,
        }
    }

    pub fn up(&mut self) {
        self.tree_stack.pop()
            .expect("Tried to call up() on an empty tree_stack");
    }

    pub fn next(&mut self) -> bool {
        let node_i = *self.tree_stack.last()
            .expect("Tried to call next() on an empty tree_stack");
        match self.ast.nodes[node_i].next {
            Some(next_i) => {
                self.tree_stack.pop();
                self.tree_stack.push(next_i);
                return true;
            },
            None => return false,
        }
    }

    pub fn restart(&mut self) {
        self.up();
        self.down();
    }

    pub fn find_symbol(&self, name: &String) -> Option<usize> {
        match self.table.get(name) {
            Some(index) => {
                return Some(self.mappings[*index].node_i);
            },
            _ => return None,
        }
    }

    pub fn add_symbol(&mut self, name: &String) {
        let node_i = *self.tree_stack.last()
            .expect("Tried to call add_symbol() on an empty tree_stack");
        let prev = match self.table.get(name) {
            Some(prev) => Some(*prev),
            None => None,
        };
        let mapping = Mapping {
            node_i: node_i,
            prev: prev,
            name: String::clone(name),
        };
        self.table.insert(String::clone(name), self.mappings.len());
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
            match mapping.prev {
                Some(prev) => {
                    self.table.insert(mapping.name, prev);
                }
                None => {
                    self.table.remove(&mapping.name);
                }
            }
        }
    }

    pub fn replace_construct(&mut self, construct: &Construct) {
        let node_i = *self.tree_stack.last()
            .expect("Tried to call replace_construct() on an empty tree_stack");
        self.ast.nodes[node_i].construct = Construct::clone(construct);
    }
}
