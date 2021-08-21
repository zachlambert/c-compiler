
use super::checker::*;
use crate::parser::construct::*;
use super::block::resolve_block;


fn check_for_argument(checker: &mut Checker, node_i: usize) -> bool {
    let name: String;
    let symbol: Symbol;
    match &checker.ast.nodes[node_i].construct {
        Construct::Argument(name_) => {
            symbol = Symbol {
                node_i: node_i,
                data: SymbolData::Variable(Storage::Undefined),
            };
            name = String::clone(name_);
        },
        _ => return false,
    }
    println!("Adding argument {}", name);
    checker.add_symbol(&name, symbol);
    return true;
}

fn check_for_returned(checker: &mut Checker, node_i: usize) -> bool {
    let name: String;
    let symbol: Symbol;
    match &checker.ast.nodes[node_i].construct {
        Construct::Returned(name_) => {
            symbol = Symbol {
                node_i: node_i,
                data: SymbolData::Variable(Storage::Undefined),
            };
            name = String::clone(name_);
        },
        _ => return false,
    }
    println!("Adding return value {}", name);
    checker.add_symbol(&name, symbol);
    return true;
}

pub fn resolve_function(checker: &mut Checker, node_i: usize) {
    println!("Resolving function");

    checker.increase_scope();

    let mut child_opt = checker.ast.nodes[node_i].child;

    // 1. Add symbols for arguments
    loop {
        match child_opt {
            Some(child_i) => {
                if !check_for_argument(checker, child_i) {
                    println!("Encountered construct {}", checker.ast.nodes[child_i].construct);
                    break;
                }
                let child = &checker.ast.nodes[child_i];
                child_opt = child.next;
            },
            None => {
                println!("No more children");
                break;
            }
        }
    }

    // 2. Add symbols for return values
    loop {
        match child_opt {
            Some(child_i) => {
                if !check_for_returned(checker, child_i) {
                    break;
                }
                let child = &checker.ast.nodes[child_i];
                child_opt = child.next;
            },
            None => break,
        }
    }

    // 3. Handle block
    let block_i = child_opt.expect("Expect block node at end of function");
    match checker.ast.nodes[block_i].construct {
        Construct::Block => (),
        _ => panic!("Expected block node at end of function"),
    }
    resolve_block(checker, block_i);

    checker.decrease_scope();
}
