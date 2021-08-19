
use super::checker::*;
use crate::parser::construct::*;

fn add_function(checker: &mut Checker, node_i: usize) {

    let parent = &checker.ast.nodes[node_i];
    match parent.construct {
        Construct::Function => (),
        _ => panic!("get_function_label given incorrect node"),
    }

    let node_i = parent.child
        .expect("Function node has no child");

    let node = &checker.ast.nodes[node_i];
    let identifier = match &node.construct {
        Construct::Identifier(identifier) => String::clone(identifier),
        _ => panic!("First child of function node isn't identifier"),
    };

    let mut label = String::new();
    label.push_str(&identifier);

    // Handle different scopes
    label.push_str("__");
    label.push_str(&checker.current_depth().to_string());

    let symbol = Symbol {
        node_i: node_i,
        data: SymbolData::Function(label),
    };

    checker.add_symbol(&identifier, symbol);
}

fn add_structure(checker: &mut Checker, node_i: usize) {

    let parent = &checker.ast.nodes[node_i];
    match parent.construct {
        Construct::Structure => (),
        _ => panic!("get_function_label given incorrect node"),
    }

    let node_i = parent.child
        .expect("Function node has no child");

    let node = &checker.ast.nodes[node_i];
    let identifier = match &node.construct {
        Construct::Identifier(identifier) => String::clone(identifier),
        _ => panic!("First child of function node isn't identifier"),
    };

pub fn resolve_node(checker: &mut Checker, node_i: usize) {
    // Can't use a reference to the node, because of borrow checker
    let construct = Construct::clone(&checker.ast.nodes[node_i].construct);

    match construct {
        Construct::Identifier(identifier) => {
            match checker.find_symbol(&identifier) {
                Some(symbol) => {
                    checker.ast.nodes[node_i].child = Some(symbol.node_i);
                },
                None => panic!("Could not resolve symbol {}", identifier),
            }
            return;
        },
        Construct::Function => {
            add_function(checker, node_i);
        },
        Construct::Structure => {
            add_structure(checker, node_i);
        },
        // TODO: Variables
        _ => (),
    }

    checker.increase_scope();
    let mut child_opt = checker.ast.nodes[node_i].child;
    loop {
        match child_opt {
            Some(child_i) => {
                resolve_node(checker, child_i);
                let child = &checker.ast.nodes[node_i];
                child_opt = child.next;
            },
            None => break,
        }
    }
    checker.decrease_scope();
}
