
use super::checker::*;
use crate::parser::construct::*;
use super::symbol::check_for_symbol;
use super::function::resolve_function;
use super::node::resolve_node;

pub fn resolve_block(checker: &mut Checker, node_i: usize) {
    // Scope is increased/decreased outside this function
    match checker.ast.nodes[node_i].construct {
        Construct::Program => (),
        Construct::Block => (),
        _ => panic!("Invalid node_i passed to resolve_body"),
    }

    println!("Resolving body");

    println!("Scanning for symbols");
    // 1. Scan for symbols
    let mut child_opt = checker.ast.nodes[node_i].child;
    loop {
        match child_opt {
            Some(child_i) => {
                check_for_symbol(checker, child_i);
                let child = &checker.ast.nodes[child_i];
                child_opt = child.next;
            },
            None => break,
        }
    }

    println!("Resolving child nodes");
    // 2. Resolve each child node
    let mut child_opt = checker.ast.nodes[node_i].child;
    loop {
        match child_opt {
            Some(child_i) => {
                println!("Found child {}", checker.ast.nodes[child_i].construct);
                match checker.ast.nodes[child_i].construct {
                    Construct::Block => {
                        resolve_block(checker, child_i);
                    },
                    Construct::Function(_) => {
                        resolve_function(checker, child_i);
                    },
                    _ => resolve_node(checker, child_i),
                }
                let child = &checker.ast.nodes[child_i];
                child_opt = child.next;
            },
            None => break,
        }
    }
}
