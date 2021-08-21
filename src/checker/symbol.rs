
use super::checker::*;
use crate::parser::construct::*;


fn get_function_label(checker: &Checker, name: &String) -> String {

    let mut label = String::new();
    label.push_str(&name);

    // Handle different scopes
    label.push_str("__");
    label.push_str(&checker.current_depth().to_string());

    return label;
}

// Standard symbols that can be declared in a scope.
// Excludes arguments and return values
pub fn check_for_symbol(checker: &mut Checker, node_i: usize) {
    let name : String;
    let symbol : Symbol;
    {
        let node = &checker.ast.nodes[node_i];
        match &node.construct {
            Construct::Function(name_) => {
                symbol = Symbol {
                    node_i: node_i,
                    data: SymbolData::Function(get_function_label(checker, name_)),
                };
                name = String::clone(name_);
            },
            Construct::Structure(name_) => {
                symbol = Symbol {
                    node_i: node_i,
                    data: SymbolData::Structure,
                };
                name = String::clone(name_);
            },
            Construct::Variable(name_) => {
                symbol = Symbol {
                    node_i: node_i,
                    data: SymbolData::Structure,
                };
                name = String::clone(name_);
            },
            _ => return,
        }
    }
    println!("Adding symbol with name {}", name);
    checker.add_symbol(&name, symbol);
}

fn resolve_datatype_terminal(checker: &mut Checker, node_i: usize) {
    let node = &checker.ast.nodes[node_i];
    let identifier: String;
    {
        let child = &checker.ast.nodes[node.child
            .expect("Node passed to resolve_datatype_terminal has no child")];
        match &child.construct {
            Construct::Identifier(identifier_) => identifier = String::clone(identifier_),
            _ => return,
        };
    }
    let symbol_node_i = match checker.find_symbol(&identifier) {
        Some(symbol) => symbol.node_i,
        _ => panic!("Couldn't find symbol for identifier {}", identifier),
    };
    let node = &mut checker.ast.nodes[node_i];
    node.child = Some(symbol_node_i);
}

fn resolve_datatype(checker: &mut Checker, node_i: usize) {
    // node_i is some node, where first child should be a datatype
    // In the case of pointers, need to recursively scan down until reaching
    // an identifier.
    let mut child_opt = checker.ast.nodes[node_i].child;
    loop {
        match child_opt {
            Some(child_i) => {
                resolve_datatype(checker, child_i);
                match &checker.ast.nodes[child_i].construct {
                    Construct::Datatype(datatype) => match datatype {
                        Datatype::Terminal => {
                            resolve_datatype_terminal(checker, child_i);
                        }
                        _ => (),
                    }
                    _ => (),
                }
                let child = &checker.ast.nodes[child_i];
                child_opt = child.next;
            },
            None => break,
        }
    }
}

fn resolve_function(checker: &mut Checker, node_i: usize) {
    let mut child_opt = checker.ast.nodes[node_i].child;
    loop {
        match child_opt {
            Some(child_i) => {
                match checker.ast.nodes[child_i].construct {
                    Construct::Argument(_) => (),
                    Construct::Returned(_) => (),
                    _ => break,
                }
                resolve_datatype(checker, child_i);
                let child = &checker.ast.nodes[child_i];
                child_opt = child.next;
            },
            None => break,
        }
    }
}

fn resolve_structure(checker: &mut Checker, node_i: usize) {
    let mut child_opt = checker.ast.nodes[node_i].child;
    loop {
        match child_opt {
            Some(child_i) => {
                match checker.ast.nodes[child_i].construct {
                    Construct::Member(_) => (),
                    _ => break,
                }
                resolve_datatype(checker, child_i);
                let child = &checker.ast.nodes[child_i];
                child_opt = child.next;
            },
            None => break,
        }
    }
}

pub fn resolve_symbol(checker: &mut Checker, node_i: usize) {
    match &checker.ast.nodes[node_i].construct {
        Construct::Function(_) => resolve_function(checker, node_i),
        Construct::Structure(_) => resolve_structure(checker, node_i),
        Construct::Variable(_) => resolve_datatype(checker, node_i),
        _ => return,
    }
}
