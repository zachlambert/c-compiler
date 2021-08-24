
use super::generator::Generator;
use crate::parser::construct::*;


/*
fn get_function_label(checker: &Checker, name: &String) -> String {

    let mut label = String::new();
    label.push_str(&name);

    // Handle different scopes
    label.push_str("__");
    label.push_str(&checker.current_depth().to_string());

    return label;
}
*/

// Standard symbols that can be declared in a scope.
// Excludes arguments and return values
pub fn check_for_symbol(generator: &mut Generator) {
    let name: String;
    match generator.current() {
        Construct::Function(name_) => name = String::clone(name_),
        Construct::Structure(name_, _) => name = String::clone(name_),
        Construct::Variable(name_) => name = String::clone(name_),
        _ => return,
    };
    generator.add_symbol(&name);
}

fn resolve_datatype_terminal(generator: &mut Generator) {
    // Current node is Datatype::Terminal
    // Child is identifier, terminal (or reference when resolved)
    // If it is an identifier, change construct to a reference
    
    generator.down();
    let identifier = match generator.current() {
        Construct::Identifier(identifier) => identifier,
        _ => {
            generator.up();
            return;
        },
    };

    let ref_node_i = match generator.find_symbol(identifier) {
        Some(node_i) => node_i,
        None => panic!("Couldn't find symbol for identifier {}", identifier),
    };
    let construct = Construct::Reference(ref_node_i);

    generator.replace_construct(&construct);
    generator.up();
}

fn resolve_datatype(generator: &mut Generator) {
    // Current node has datatype as one of its children
    // If a datatype is terminal, it will have a primitive or identifier as child
    // If a datatype is a pointer, one of its children will also be a datatype
    generator.down();
    loop {
        if let Construct::Datatype(datatype) = generator.current() {
            match datatype {
                Datatype::Terminal => {
                    resolve_datatype_terminal(generator);
                },
                Datatype::Pointer => {
                    resolve_datatype(generator);
                }
            }
        }
        if !generator.next() {
            break;
        }
    }
    generator.up();
}

fn resolve_function(generator: &mut Generator) {
    generator.down();
    loop {
        match generator.current() {
            Construct::Argument(_) => (),
            Construct::Returned(_) => (),
            _ => break,
        }
        resolve_datatype(generator);
        if !generator.next() {
            break;
        }
    }
    generator.up();
}

fn resolve_structure(generator: &mut Generator) {
    generator.down();
    loop {
        match generator.current() {
            Construct::Member(_, _) => (),
            _ => break,
        }
        resolve_datatype(generator);
        if !generator.next() {
            break;
        }
    }
    generator.up();
}

pub fn resolve_symbol(generator: &mut Generator) {
    match generator.current() {
        Construct::Function(_) => resolve_function(generator),
        Construct::Structure(_, _) => resolve_structure(generator),
        Construct::Variable(_) => resolve_datatype(generator),
        _ => return,
    }
}
