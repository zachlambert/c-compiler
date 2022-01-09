
use std::mem;
use crate::parser::ast::Ast;
use crate::parser::construct::Construct;
use std::collections::HashMap;
use super::instructions::*;

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
    // Depth of internal functions on creation. = 0 if unrestricted access to internal functions.
    pub function_depth: usize,
    pub version: usize,
}

pub struct Generator<'a> {
    pub ast: &'a mut Ast,
    instructions: &'a mut Vec<Element>,
    table: HashMap<String, usize>, // mapping index
    mappings: Vec<Mapping>,
    scope: Vec<usize>, // stack of the start of mappings for each scope
    tree_stack: Vec<usize>,
    function_stack: Vec<usize>, // Stack of index within instructions for function start
    return_datatypes: Vec<usize>,
    temp_version: usize,
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
            function_stack: Vec::new(),
            return_datatypes: Vec::new(),
            temp_version: 0,
        };
        generator.tree_stack.push(start_i);
        return generator;
    }

    // Haven't worked out how to use this without borrow checker complaining
    // pub fn current_mut(&'a mut self) -> &'a mut Construct {
    //     let node_i = *self.tree_stack.last()
    //         .expect("Tried to call current_mut() on an empty tree_stack");
    //     return &mut self.ast.nodes[node_i].construct;
    // }

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

    pub fn down_ref(&mut self, ref_id: usize) {
        self.tree_stack.push(ref_id);
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
                let mapping = &self.mappings[*index];
                if mapping.function_depth == 0 || mapping.function_depth == self.function_stack.len() {
                    return Some(mapping.node_i);
                } else {
                    return None;
                }
            },
            _ => return None,
        }
    }

    pub fn get_symbol_version(&mut self, name: &String, increment: bool) -> usize {
        match self.table.get(name) {
            Some(index) => {
                let mapping = &mut self.mappings[*index];
                if mapping.function_depth == 0 || mapping.function_depth == self.function_stack.len() {
                    let version = mapping.version;
                    if increment {
                        mapping.version+=1;
                    }
                    return version;
                } else {
                    panic!("");
                }
            },
            _ => panic!(""),
        }
    }

    pub fn add_symbol(&mut self, name: &String, block_function_access: bool) {
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
            function_depth: if block_function_access {self.function_stack.len()} else {0},
            version: 0,
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

    pub fn increase_scope_function(&mut self) {
        self.increase_scope();
        self.function_stack.push(self.instructions.len());
        self.return_datatypes.clear();
        self.temp_version = 0;
    }

    pub fn decrease_scope_function(&mut self) {
        self.decrease_scope();
        let internal = self.function_stack.pop().expect("Invalid function_stack");
        let parent = match self.function_stack.pop() {
            Some(parent) => parent,
            None => return, // decrease into global scope, don't need to rearrange
        };
        // Current instructions:
        // [<- parent -><- internal ->]
        // Want:
        // [<- internal -><- parent -> (can continue with parent) ]
        let internal_size = self.instructions.len() - internal;
        let parent_size = internal - parent;

        // Easiest way to do this is:
        // [<- parent -><- internal ->]
        // [<- parent ->--------------<- internal ->]
        // [--------------<- parent -><- internal ->]
        // [<- internal -><- parent ->]
        for i in 0..internal_size {
            let element = mem::take(&mut self.instructions[internal+i]);
            self.instructions.push(element);
        }
        for i in parent_size-1..0 { // Reverse order to avoid overwriting self
            self.instructions[internal-parent_size+i] = mem::take(&mut self.instructions[parent+i]);
        }
        for i in internal_size-1..0 {
            self.instructions[parent+i] = self.instructions.pop().expect("");
        }

        let new_parent = parent + internal_size;
        self.function_stack.push(new_parent);
    }

    pub fn replace_construct(&mut self, construct: &Construct) {
        let node_i = *self.tree_stack.last()
            .expect("Tried to call replace_construct() on an empty tree_stack");
        self.ast.nodes[node_i].construct = Construct::clone(construct);
    }

    pub fn get_ref_id(&self) -> usize {
        return *self.tree_stack.last()
            .expect("Tried to call get_ref_id() on an empty tree_stack");
    }

    pub fn add_element(&mut self, element: Element) {
        self.instructions.push(element);
    }

    pub fn push_return_datatype(&mut self) {
        // Current node = datatype
        let node_i = *self.tree_stack.last().expect("");
        self.return_datatypes.push(node_i);
    }

    pub fn get_return_datatype(&self, index: usize) -> usize {
        return self.return_datatypes[index];
    }

    pub fn get_temp_version(&mut self) -> usize {
        let version = self.temp_version;
        self.temp_version+=1;
        return version;
    }
}
