
use super::checker::*;
use crate::parser::construct::*;


pub fn resolve_identifier(checker: &mut Checker, node_i: usize) -> bool {
    let identifier : String;
    {
        let node = &mut checker.ast.nodes[node_i];
        identifier = match &node.construct {
            Construct::Identifier(identifier) => String::clone(identifier),
            _ => return false,
        };
    }
    match checker.find_symbol(&identifier) {
        Some(symbol) => {
            checker.ast.nodes[node_i].child = Some(symbol.node_i);
        },
        None => panic!("Could not resolve symbol for identifier {}", identifier),
    }
    println!("Resolved identifier {}", identifier);
    return true;
}

pub fn resolve_node(checker: &mut Checker, node_i: usize) {
    if resolve_identifier(checker, node_i) {
        return;
    }
    let mut child_opt = checker.ast.nodes[node_i].child;
    loop {
        match child_opt {
            Some(child_i) => {
                resolve_node(checker, child_i);
                let child = &checker.ast.nodes[child_i];
                child_opt = child.next;
            },
            None => break,
        }
    }
}
