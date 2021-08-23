
use crate::parser::ast::Ast;
use crate::parser::construct::Construct;
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
    Structure,
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

#[derive(Clone, Copy)]
pub struct Config {
    num_temporary: usize,
    num_saved: usize,
    num_floats: usize,
}

pub struct Generator<'a> {
    pub ast: &'a mut Ast,
    config: Config,
    table: HashMap<String, usize>, // mapping index
    mappings: Vec<Mapping>,
    scope: Vec<usize>, // stack of the start of valid mappings
    tree_stack: Vec<usize>,
    stack_size: usize, // Size of stack frame in a function
}

impl<'a> Generator<'a> {
    pub fn new(ast: &'a mut Ast, config: Config) -> Generator<'a> {
        let generator = Generator {
            ast: ast,
            config: config,
            table: HashMap::new(),
            mappings: Vec::new(),
            scope: Vec::new(),
            tree_stack: Vec::new(),
        };
        generator.tree_stack.push(ast.nodes.len() - 1);
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

    pub fn find_symbol(&'a self, name: &String) -> Option<&'a Symbol> {
        match self.table.get(name) {
            Some(index) => {
                Some(&self.mappings[*index].symbol)
            },
            _ => None,
        }
    }

    pub fn add_symbol(&mut self, name: &String, symbol: Symbol) {
        let prev = match self.table.get(name) {
            Some(prev) => Some(*prev),
            None => None,
        };
        let mapping = Mapping {
            symbol: Symbol::clone(&symbol),
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

    pub fn new_stack_frame(&mut self) {
        self.stack_size = 0;
    }

    pub fn allocate_stack_space(&mut self, size: usize) -> usize {
        let result = self.stack_size;
        self.stack_size += size;
        return result;
    }
}
