
use super::generator::Generator;
use crate::parser::construct::*;
use super::datatype::resolve_datatype;


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
    generator.add_symbol(&name, false);
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

pub fn resolve_symbol_datatypes(generator: &mut Generator) {
    match generator.current() {
        Construct::Function(_) => resolve_function(generator),
        Construct::Structure(_, _) => resolve_structure(generator),
        Construct::Variable(_) => resolve_datatype(generator),
        _ => return,
    }
}
