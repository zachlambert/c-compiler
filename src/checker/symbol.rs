
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
    println!("Added symbol with name {}", name);
}
