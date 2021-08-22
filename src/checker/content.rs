
use super::checker::*;
use crate::parser::construct::*;
use super::symbol::check_for_symbol;
use super::symbol::resolve_symbol;
use super::function::check_function;
use super::statement::check_statement;

fn check_block(checker: &mut Checker, node_i: usize) {
    checker.increase_scope();
    check_content(checker, node_i);
    checker.decrease_scope();
}

pub fn check_content(checker: &mut Checker, node_i: usize) {
    // Scope is increased/decreased outside this function
    match checker.ast.nodes[node_i].construct {
        Construct::Program => (),
        Construct::Block => (),
        _ => panic!("Invalid node_i passed to resolve_body"),
    }

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

    // 2. Resolve datatypes within structs and functions
    let mut child_opt = checker.ast.nodes[node_i].child;
    loop {
        match child_opt {
            Some(child_i) => {
                resolve_symbol(checker, child_i);
                let child = &checker.ast.nodes[child_i];
                child_opt = child.next;
            },
            None => break,
        }
    }

    // 3. Check statements and expressions by resolving symbols and checking
    //    datatypes are compatible.
    let mut child_opt = checker.ast.nodes[node_i].child;
    loop {
        match child_opt {
            Some(child_i) => {
                match checker.ast.nodes[child_i].construct {
                    Construct::Block => {
                        check_block(checker, child_i);
                    },
                    Construct::Function(_) => {
                        check_function(checker, child_i);
                    },
                    Construct::Statement(_) => {
                        check_statement(checker, child_i);
                    },
                    _ => (),
                }
                let child = &checker.ast.nodes[child_i];
                child_opt = child.next;
            },
            None => break,
        }
    }
}
